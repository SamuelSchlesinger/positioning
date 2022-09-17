use std::collections::BTreeSet;

use crate::Position;

/// # Breadth First Search
///
/// An iterator which performs a breadth first search of a set of open tiles, returning
/// [`Position`]s and their distances away from the given start in order of those distances.
/// If there are multiple positions at the same distance away, the first one in the order defined on
/// [`Position`] will be returned first.
pub struct Bfs<'a> {
    open_positions: &'a BTreeSet<Position>,
    visited: BTreeSet<Position>,
    distance: u64,
    current_frontier: BTreeSet<Position>,
    next_frontier: BTreeSet<Position>,
}

impl<'a> Bfs<'a> {
    pub fn new(open_positions: &'a BTreeSet<Position>, start: Position) -> Self {
        let mut current_frontier = BTreeSet::new();

        if open_positions.contains(&start) {
            current_frontier.insert(start);
        }

        Bfs {
            open_positions,
            visited: BTreeSet::new(),
            distance: 0,
            current_frontier,
            next_frontier: BTreeSet::new(),
        }
    }
}

impl<'a> Iterator for Bfs<'a> {
    type Item = (Position, u64);

    fn next(&mut self) -> Option<Self::Item> {
        match self.current_frontier.pop_first() {
            None => {
                if self.next_frontier.is_empty() {
                    return None;
                }
                std::mem::swap(&mut self.current_frontier, &mut self.next_frontier);
                self.distance += 1;
                self.next()
            }
            Some(cursor) => {
                self.visited.insert(cursor);
                for neighbor in cursor
                    .adjacent()
                    .filter(|neighbor| self.open_positions.contains(&neighbor))
                {
                    if self.visited.contains(&neighbor) {
                        continue;
                    }
                    self.next_frontier.insert(neighbor);
                }
                return Some((cursor, self.distance));
            }
        }
    }
}

#[test]
fn test_bfs() {
    let mut open_positions = BTreeSet::new();
    for i in 0..10 {
        for j in 0..10 {
            open_positions.insert(Position::new(i, j, 0));
        }
    }

    let origin = (0, 0, 0i64).into();

    let bfs = Bfs::new(&open_positions, origin);

    for (position, distance) in bfs {
        eprintln!("position = {:?}, distance = {}", position, distance);
        assert_eq!(distance, origin.hamming_distance(position));
    }
}
