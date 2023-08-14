#![deny(missing_docs)]

//! Defines a generic struct to store two-dimensional information.

/// Stores general two-dimensional information.
pub struct Coord<T> {
    /// horizontal
    pub x: T,
    /// vertical
    pub y: T,
}
