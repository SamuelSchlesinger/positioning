//! # Pathfinding
//!
//! Imagine we have a fixed 3D grid world with some set of blocked and non-blocked tiles that we
//! know of statically. Dynamically, as the game or whatever other system is running, those tiles
//! may be blocked by other things, but the statically known blocked tiles will never be open in
//! any circumstance.
//!
//! Then, we can compute all-pairs shortest paths on the statically known paths and use that
//! information in order to inform a clever heuristic for the A* algorithm. That is the strategy
//! taken in this module.
//!
//! A major potential improvement would be to implement a Jump Point Search on top of that, but
//! that is going to be a lot harder for me to verify the correctness of and this is what I'm going
//! for this time.

/// A number which has an Infinity variant in order to indicate that there is no path. It is
/// convenient to be consistent with the pathfinding literature by using this type, otherwise I
/// have to diverge from the algorithms as written up occasionally more than I'd like.
use std::collections::{BTreeMap, BTreeSet, VecDeque};

use priority_queue::DoublePriorityQueue; // TODO Replace with PriorityQueue<_, Reverse<_>>

use crate::position::Position;
#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Copy, Clone)]
pub enum WithInfinity<I> {
    Normal(I),
    Infinity,
}

impl<I> Default for WithInfinity<I> {
    fn default() -> Self {
        WithInfinity::Infinity
    }
}

impl<I: std::ops::Add<I, Output = I>> std::ops::Add<WithInfinity<I>> for WithInfinity<I> {
    type Output = WithInfinity<I>;

    fn add(self, rhs: WithInfinity<I>) -> Self::Output {
        match self {
            WithInfinity::Normal(i) => match rhs {
                WithInfinity::Infinity => WithInfinity::Infinity,
                WithInfinity::Normal(j) => WithInfinity::Normal(i + j),
            },
            WithInfinity::Infinity => WithInfinity::Infinity,
        }
    }
}

#[test]
fn with_infinity_test() {
    let x = WithInfinity::Normal(1i32);
    let z = WithInfinity::Normal(0i32);
    let y = WithInfinity::Infinity;
    assert!(y > x);
    assert!(z < x);
    assert!(x + z < y);
    for i in 0..100000 {
        assert!(y > WithInfinity::Normal(i));
    }
}

/// An admissible heuristic for the A* pathfinding algorithm is one which always returns an
/// optimistic result. Oftentimes, a good heuristic can make a big performance difference.
pub trait Heuristic {
    fn heuristic_distance(&self, start: Position, end: Position) -> WithInfinity<u64>;

    fn find_shortest_path(
        &self,
        open_positions: &BTreeSet<Position>,
        start: Position,
        end: Position,
    ) -> Option<VecDeque<Position>> {
        let mut distances_from_start: BTreeMap<Position, WithInfinity<u64>> = BTreeMap::new();
        let mut predecessor: BTreeMap<Position, Position> = BTreeMap::new();
        let mut queue: DoublePriorityQueue<Position, WithInfinity<u64>> =
            DoublePriorityQueue::new();
        distances_from_start.insert(start, WithInfinity::Normal(0));
        match self.heuristic_distance(start, end) {
            WithInfinity::Infinity => return None,
            n => {
                queue.push(start, n);
            }
        }

        if !open_positions.contains(&end) {
            return None;
        }

        loop {
            match queue.pop_min() {
                None => {
                    break;
                }
                Some((position, _distance)) => {
                    if position == end {
                        break;
                    }
                    for neighbor in position
                        .adjacent()
                        .filter(|neighbor| open_positions.contains(neighbor))
                    {
                        let alt = distances_from_start
                            .get(&position)
                            .map(|e| *e)
                            .unwrap_or_default()
                            + WithInfinity::Normal(1);
                        if alt < *distances_from_start.entry(neighbor).or_default() {
                            let halt = alt + self.heuristic_distance(position, end);
                            distances_from_start.insert(neighbor, alt);
                            predecessor.insert(neighbor, position);
                            if queue.change_priority(&neighbor, halt).is_none() {
                                queue.push(neighbor, halt);
                            }
                        }
                    }
                }
            }
        }

        match distances_from_start.entry(end).or_default() {
            WithInfinity::Infinity => None,
            WithInfinity::Normal(distance) => {
                let mut current_position = end;
                let mut path = VecDeque::new();
                loop {
                    if current_position == start {
                        break;
                    }
                    path.push_front(current_position);
                    if let Some(pre) = predecessor.get(&current_position) {
                        current_position = *pre;
                    } else {
                        panic!("should always have a path home");
                    }
                }
                assert_eq!(*distance, path.len() as u64);
                Some(path)
            }
        }
    }
}

/// The all-pairs shortest paths on the static graph described above will always be an admissible
/// heuristic for the dynamic graph as long as the statically closed tiles are never unblocked.
///
/// This heuristic is extremely valuable for the use case described in this module. Imagine the
/// following scenario:
///
/// ```text
/// ____________
/// |e X   start|
/// |n X        |
/// |d X        |
/// |  XXXXXXX  |
/// |           |
/// -------------
/// ```
///
/// The [`HammingDistance`] heuristic will send you looking all throughout the top right region
/// before you realize you're meant to go around, whereas the all pairs metric will immediately be
/// able to bypass this obstacle. In the presence of dynamically but not statically blocked tiles,
/// we can run into these problems again easily, but we avoid them in many cases even then. The
/// more tight the heuristic is to the real distance, the better.
pub struct AllPairsShortestPaths(BTreeMap<(Position, Position), WithInfinity<u64>>);

impl Heuristic for AllPairsShortestPaths {
    fn heuristic_distance(&self, start: Position, end: Position) -> WithInfinity<u64> {
        self.distance_between(start, end).unwrap_or_default()
    }
}

/// The 3D hamming distance metric is always fine to use.
pub struct HammingDistance;

impl Heuristic for HammingDistance {
    fn heuristic_distance(&self, start: Position, end: Position) -> WithInfinity<u64> {
        WithInfinity::Normal(
            start.x.abs_diff(end.x) + start.y.abs_diff(end.y) + start.z.abs_diff(end.z),
        )
    }
}

impl AllPairsShortestPaths {
    pub fn distance_between(
        &self,
        position: Position,
        other_position: Position,
    ) -> Option<WithInfinity<u64>> {
        self.0.get(&(position, other_position)).map(|p| *p)
    }
}

/// Computes a data structure caching the distances between all open positions
pub fn all_pairs_shortest_paths(open_positions: &BTreeSet<Position>) -> AllPairsShortestPaths {
    let mut distances = BTreeMap::new();
    for position in open_positions.iter().copied() {
        for adjacent in position.adjacent() {
            distances.insert((position, adjacent), WithInfinity::Normal(1));
        }
    }
    for position in open_positions.iter().copied() {
        for other_position in open_positions.iter().copied() {
            let _ = distances
                .entry((position, other_position))
                .or_insert(WithInfinity::Infinity);
        }
    }
    for position in open_positions.iter().copied() {
        distances.insert((position, position), WithInfinity::Normal(0));
    }

    for k in open_positions.iter().copied() {
        for i in open_positions.iter().copied() {
            for j in open_positions.iter().copied() {
                let new_distance_candidate =
                    *distances.get(&(i, k)).unwrap() + *distances.get(&(k, j)).unwrap();
                let old_distance_candidate = *distances.get(&(i, j)).unwrap();
                if new_distance_candidate < old_distance_candidate {
                    distances.insert((i, j), new_distance_candidate);
                }
            }
        }
    }

    AllPairsShortestPaths(distances)
}

#[test]
fn all_pairs_test() {
    let open_positions: BTreeSet<Position> = vec![
        Position::new(0, 0, 0),
        Position::new(0, 0, 1),
        Position::new(0, 0, 2),
    ]
    .into_iter()
    .collect();

    let all_pairs = all_pairs_shortest_paths(&open_positions);

    assert_eq!(
        all_pairs.distance_between(Position::new(0, 0, 0), Position::new(0, 0, 0)),
        Some(WithInfinity::Normal(0))
    );

    assert_eq!(
        all_pairs.distance_between(Position::new(0, 0, 0), Position::new(0, 0, 1)),
        Some(WithInfinity::Normal(1)),
    );

    assert_eq!(
        all_pairs.distance_between(Position::new(0, 0, 0), Position::new(0, 0, 2)),
        Some(WithInfinity::Normal(2)),
    );

    assert_eq!(
        all_pairs.distance_between(Position::new(0, 0, 1), Position::new(0, 0, 0)),
        Some(WithInfinity::Normal(1)),
    );

    assert_eq!(
        all_pairs.distance_between(Position::new(0, 0, 2), Position::new(0, 0, 0)),
        Some(WithInfinity::Normal(2)),
    );

    assert_eq!(
        all_pairs.distance_between(Position::new(0, 0, 2), Position::new(0, 0, 1)),
        Some(WithInfinity::Normal(1)),
    );
}

#[test]
fn shortest_path_test() {
    use itertools::Itertools;

    let static_open_positions: BTreeSet<Position> = vec![
        Position::new(0, 0, 0),
        Position::new(0, 0, 1),
        Position::new(0, 0, 2),
    ]
    .into_iter()
    .collect();

    let dynamic_open_positions: BTreeSet<Position> =
        vec![Position::new(0, 0, 0), Position::new(0, 0, 2)]
            .into_iter()
            .collect();

    let all_pairs = all_pairs_shortest_paths(&static_open_positions);

    assert_eq!(
        all_pairs.find_shortest_path(
            &dynamic_open_positions,
            Position::new(0, 0, 0),
            Position::new(0, 0, 2)
        ),
        None
    );

    assert_eq!(
        all_pairs.find_shortest_path(
            &dynamic_open_positions,
            Position::new(0, 0, 0),
            Position::new(0, 0, 1),
        ),
        None
    );

    assert_eq!(
        all_pairs.find_shortest_path(
            &dynamic_open_positions,
            Position::new(0, 0, 0),
            Position::new(0, 0, 0),
        ),
        Some(VecDeque::new())
    );

    const N: i64 = 3;

    let static_open_positions: BTreeSet<Position> = (0..N)
        .cartesian_product(0..N)
        .map(|(i, j)| Position::new(i, j, 0))
        .collect();

    let all_pairs = all_pairs_shortest_paths(&static_open_positions);

    let mut dynamic_open_positions: BTreeSet<Position> = static_open_positions.clone();
    for y in 1..N {
        dynamic_open_positions.remove(&Position::new(N / 2, y, 0));
    }

    for position in dynamic_open_positions.iter().copied() {
        for other in dynamic_open_positions.iter().copied() {
            if !(other.x == N / 2) || other.y == 0 {
                continue;
            }
            if position == Position::new(N / 2, 0, 0)
                || (position.x < N / 2 && other.x < N / 2)
                || (position.x > N / 2 && other.x > N / 2)
            {
                assert_eq!(
                    all_pairs
                        .find_shortest_path(&dynamic_open_positions, position, other)
                        .map_or_else(
                            || WithInfinity::Infinity,
                            |e| WithInfinity::Normal(e.len() as u64)
                        ),
                    all_pairs.heuristic_distance(position, other),
                );
            } else {
                assert_eq!(
                    all_pairs
                        .find_shortest_path(&dynamic_open_positions, position, other)
                        .map_or_else(
                            || WithInfinity::Infinity,
                            |e| WithInfinity::Normal(e.len() as u64)
                        ),
                    WithInfinity::Normal(
                        position.x.abs_diff(other.x)
                            + (N - position.y) as u64
                            + (N - other.y) as u64
                    ),
                );
            }
        }
    }
}
