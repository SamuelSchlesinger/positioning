//! # Positioning
//!
//! This is a library to encapsulate some code I've found myself repeating recently around game
//! coordinates. The particularly useful thing to abstract out is pathfinding, as that's quite
//! intricate to implement from scratch every time I need it.

/// The pathfinding module consists of an implementation of A* and a domain specific heuristic which
/// is very useful in the context of a 3D grid.
pub mod pathfinding;
mod position;

pub use position::Position;
