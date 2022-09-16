//! # Position
//!
//! A data type representing coordinates in a 3D grid using [`i32`] coordinates, with wrapping
//! arithmetic at the boundaries.

use std::ops::{Add, Mul, Sub};

/// Represents a position in a 3D grid, bounded in each dimension by the maximum size of an [`i64`].
/// Around the bounds, all arithmetic in this module with be done in a wrapping way, so be aware
/// of that in your usage of this module.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bevy", derive(bevy::prelude::Component))]
pub struct Position {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

impl Position {
    /// Creates a new [`Position`].
    pub fn new(x: i64, y: i64, z: i64) -> Self {
        Position { x, y, z }
    }

    /// Returns whether or not we are adjacent to another position. Diagonal adjacency is not
    /// counted:
    /// ```
    /// use positioning::position::Position;
    ///
    /// assert!(!Position::new(0, 0, 0).is_adjacent_to(Position::new(1, 1, 0)))
    /// ```
    pub fn is_adjacent_to(self, other: Position) -> bool {
        self.hamming_distance(other) == 1
    }

    /// Returns an iterator over all adjacent positions.
    pub fn adjacent(self) -> Box<dyn Iterator<Item = Position>> {
        use itertools::Itertools;
        Box::new(
            (-1..=1)
                .cartesian_product(-1..=1)
                .cartesian_product(-1..=1)
                .map(move |((dx, dy), dz)| Position {
                    x: self.x.wrapping_add(dx),
                    y: self.y.wrapping_add(dy),
                    z: self.z.wrapping_add(dz),
                })
                .filter(move |pos| self.is_adjacent_to(*pos)),
        )
    }

    /// Computes the Hamming distance between two points.
    pub fn hamming_distance(self, other: Position) -> u64 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y) + self.z.abs_diff(other.z)
    }
}

#[test]
fn position_adjacency_test() {
    let a = Position { x: 1, y: 1, z: 1 };
    let b = Position { x: 2, ..a };
    assert!(a.is_adjacent_to(b));
    let c = Position { y: 2, ..a };
    assert!(a.is_adjacent_to(c));
    let d = Position { z: 2, ..a };
    assert!(a.is_adjacent_to(d));
    let e = Position { y: 2, ..b };
    assert!(!a.is_adjacent_to(e));
    let f = Position { z: 2, ..b };
    assert!(!a.is_adjacent_to(f));
    let g = Position { z: 10, ..e };
    assert!(!a.is_adjacent_to(g));
}

#[cfg(feature = "bevy")]
impl From<bevy::prelude::Vec3> for Position {
    fn from(v: bevy::prelude::Vec3) -> Self {
        fn convert(f: f32) -> i64 {
            if f < 0. {
                -(-f as i64)
            } else {
                f as i64 + 1
            }
        }
        Position {
            x: convert(v.x),
            y: convert(v.y),
            z: convert(v.z),
        }
    }
}

impl Add<Position> for Position {
    type Output = Position;

    fn add(self, rhs: Position) -> Self::Output {
        Position {
            x: self.x.wrapping_add(rhs.x),
            y: self.y.wrapping_add(rhs.y),
            z: self.z.wrapping_add(rhs.z),
        }
    }
}

impl Sub<Position> for Position {
    type Output = Position;

    fn sub(self, rhs: Position) -> Self::Output {
        Position {
            x: self.x.wrapping_sub(rhs.x),
            y: self.y.wrapping_sub(rhs.y),
            z: self.z.wrapping_sub(rhs.z),
        }
    }
}

impl Mul<i64> for Position {
    type Output = Position;

    fn mul(self, rhs: i64) -> Self::Output {
        Position {
            x: self.x.wrapping_mul(rhs),
            y: self.y.wrapping_mul(rhs),
            z: self.z.wrapping_mul(rhs),
        }
    }
}

#[cfg(feature = "rand")]
impl rand::distributions::Distribution<Position> for rand::distributions::Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Position {
        Position::new(rng.gen(), rng.gen(), rng.gen())
    }
}
