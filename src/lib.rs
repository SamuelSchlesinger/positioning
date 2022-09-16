//! # Positioning
//!
//! This is a library to encapsulate some code I've found myself repeating recently around game
//! coordinates. The particularly useful thing to abstract out is pathfinding, as that's quite
//! intricate to implement from scratch every time I need it.

pub mod pathfinding;
pub mod position;
