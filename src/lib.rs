#![feature(map_first_last)]

//! # Positioning
//!
//! This is a library to encapsulate some code I've found myself repeating recently around game
//! coordinates. The particularly useful thing to abstract out is pathfinding, as that's quite
//! intricate to implement from scratch every time I need it.

/// Contains an implementation of A* and all-pairs-shortest paths with fun a twist where the latter
/// performs well as a heuristic in a specific context.
pub mod pathfinding;
mod position;

/// Contains an implementation of an iterator which returns positions out from a given start in
/// breadth first order.
pub mod bfs;

pub use position::Position;
