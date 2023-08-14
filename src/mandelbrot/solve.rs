#![deny(missing_docs)]

//! Solves the recurrence relation for each point in the given complex domain.

use crate::coord::Coord;
use crate::mandelbrot::{common, Point};

/// maximum number of iterations, above which the recurrence relation is considered to converge.
const MAX_ITER: u64 = 1024u64;

/// Solves the recurrence relation for each pixel to see the convergence
/// and to count the number of iterations needed to diverge.
///   
/// * `resols` - The number of pixels in two directions.
/// * `center` - The center of the domain.
/// * `delta`  - The inter-pixel size.
pub fn solve(resols: &Coord<usize>, center: &Coord<f64>, delta: f64) -> Vec<Point> {
    // prepare buffers to store the results
    let nitems: usize = resols.x * resols.y;
    let mut points: Vec<Point> = Vec::with_capacity(nitems);
    for _ in 0..nitems {
        points.push(Point {
            is_diverged: false,
            iter: 0u64,
        });
    }
    // check left-bottom domain corner
    let corner: Coord<f64> = Coord {
        x: common::get_bound(-1., resols.x, center.x, delta, 1.),
        y: common::get_bound(-1., resols.y, center.y, delta, 1.),
    };
    // for each pixel, solve recurrence relation
    for n in 0..nitems {
        let point: Coord<f64> = {
            let i: usize = n % resols.x;
            let j: usize = n / resols.x;
            let x: f64 = corner.x + i as f64 * delta;
            let y: f64 = corner.y + j as f64 * delta;
            Coord { x, y }
        };
        points[n] = kernel(&point);
    }
    return points;
}

/// Solves the recurrence relation for a single given point.
///   
/// * `p0` - A specific point in the complex plane to which the recurrence relation is considered.
fn kernel(p0: &Coord<f64>) -> Point {
    // p0: given   complex number (c)
    // p1: current complex number (z^n)
    // p2: next    complex number (z^{n+1})
    let mut p1: Coord<f64> = {
        let x: f64 = 0.;
        let y: f64 = 0.;
        Coord { x, y }
    };
    // solve recurrence relation to determine
    //   this pixel is inside/outside the Mandelbrot set
    let mut iter: u64 = 0;
    loop {
        // compute z^{n+1}
        let p2: Coord<f64> = {
            let x: f64 = p0.x + p1.x * p1.x - p1.y * p1.y;
            let y: f64 = p0.y + 2. * p1.x * p1.y;
            Coord { x, y }
        };
        iter = iter + 1;
        // check whether maximum number of iteration has been reached,
        //   i.e., this point has not diverged and thus inside Mandelbrot set
        if MAX_ITER < iter {
            return Point {
                is_diverged: false,
                iter: MAX_ITER,
            };
        }
        // check L^2 on the complex plane to see the divergence
        if 4. < p2.x.powi(2i32) + p2.y.powi(2i32) {
            return Point {
                is_diverged: true,
                iter: iter,
            };
        }
        // update z^n
        p1 = p2;
    }
}

#[cfg(test)]
mod test_kernel {
    use crate::mandelbrot::solve::kernel;
    use crate::mandelbrot::solve::MAX_ITER;
    use crate::mandelbrot::Coord;
    use crate::mandelbrot::Point;
    #[test]
    fn test0() -> () {
        let point: Point = kernel(&Coord::<f64> { x: 2., y: 0. });
        assert_eq!(true, point.is_diverged);
        assert_eq!(2, point.iter);
    }
    #[test]
    fn test1() -> () {
        let point: Point = kernel(&Coord::<f64> { x: 0., y: 0. });
        assert_eq!(false, point.is_diverged);
        assert_eq!(MAX_ITER, point.iter);
    }
    #[test]
    fn test2() -> () {
        let point: Point = kernel(&Coord::<f64> { x: -2., y: 0. });
        assert_eq!(false, point.is_diverged);
        assert_eq!(MAX_ITER, point.iter);
    }
}
