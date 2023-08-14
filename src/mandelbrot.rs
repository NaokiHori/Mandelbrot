#![deny(missing_docs)]

//! Plays the central role in this crate.

mod common;
mod find_center;
mod solve;

use crate::coord::Coord;
use crate::options::Options;

/// Stores the result of the recurrence relations of the Mandelbrot set
pub struct Point {
    /// flag to tell the convergence of the recurrence relation
    pub is_diverged: bool,
    /// number of iterations to diverge
    pub iter: u64,
}

/// Extracts a fairly complex structure from the Mandelbrot set.
///   
/// See also: [`crate::mandelbrot::find_center::find_center`].
pub fn find_center(options: &Options) -> Result<Coord<f64>, ()> {
    return find_center::find_center(options);
}

/// Solves the recurrence relation of the Mandelbrot set.  
///
/// In particular, for each point (=pixel), this function checks  
/// 1. whether each point (pixel) diverges or not,  
/// 2. how many iterations are needed if the point diverges.
///   
/// See also: [`crate::mandelbrot::solve::solve`].
pub fn solve_recurrence_relation(options: &Options, center: &Coord<f64>) -> Result<Vec<Point>, ()> {
    return Ok(solve::solve(&options.resols, center, options.grid_size));
}
