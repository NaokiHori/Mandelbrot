#![deny(missing_docs)]

//! Contains auxiliary routines used by [`crate::mandelbrot`] internally.  

/// Computes the negtive / positive boundary of the given domain.
///   
/// This is a private function intended to be used only inside [`crate::mandelbrot`].
///   
/// * `sign`   - Side of the boundary of interest: negtive (-1) or positive (+1)
/// * `resol`  - Number of pixels in the direction
/// * `center` - Center of the domain in the direction
/// * `delta`  - Inter-pixel distance
/// * `factor` - Retraction factor
pub fn get_bound(sign: f64, resol: usize, center: f64, delta: f64, factor: f64) -> f64 {
    return center + sign * 0.5 * factor * resol as f64 * delta;
}
