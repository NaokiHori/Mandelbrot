#![deny(missing_docs)]

//! Detects complex structures in the Mandelbrot set automatically.

use crate::coord::Coord;
use crate::mandelbrot::{common, solve, Point};
use crate::options::Options;
use crate::random::Random;

/// Finding a nice image center so that the resulting image has something to display.
///   
/// * `options` - Fixed parameters which control the overall behaviour.
pub fn find_center(options: &Options) -> Result<Coord<f64>, ()> {
    const DEBUG_ON: bool = false;
    // initial image range candidate
    const RANGE: [f64; 2] = [-2., 2.];
    // image shrinkage rate, which should be [0:1]
    // NOTE: the smaller (0~) the more efficient,
    //       the larger (~1) the more robust
    const FACTOR: f64 = 0.75;
    // initialise random number generator
    let mut rng: Random = Random::new(options.seed);
    // consider rough image for faster check
    const SHRINKAGE: usize = 4;
    let resols: Coord<usize> = Coord {
        x: options.resols.x / SHRINKAGE,
        y: options.resols.y / SHRINKAGE,
    };
    let mut center: Coord<f64> = {
        let origin: f64 = 0.5 * RANGE[0] + 0.5 * RANGE[1];
        Coord {
            x: origin + rng.gen_range(-1., 1.),
            y: origin + rng.gen_range(-1., 1.),
        }
    };
    let mut delta: f64 = {
        let deltas: Coord<f64> = Coord {
            x: (RANGE[1] - RANGE[0]) / resols.x as f64,
            y: (RANGE[1] - RANGE[0]) / resols.y as f64,
        };
        if deltas.y < deltas.x {
            deltas.x
        } else {
            deltas.y
        }
    };
    println!("looking for a center, hang on...");
    loop {
        let points: Vec<Point> = solve::solve(&resols, &center, delta);
        if delta < options.grid_size {
            break;
        }
        let complexity = match update_center(&resols, delta, FACTOR, &points, &mut center) {
            Ok(complexity) => complexity,
            Err(_) => {
                println!("no structure is found inside the domain");
                println!("try another random seed to change the initial condition");
                return Err(());
            }
        };
        if DEBUG_ON {
            println!(
                "    delta: {:.1e} center: ({:+.1e}, {:+.1e}), comp: {:8}",
                delta, center.x, center.y, complexity
            );
        }
        delta = FACTOR * delta;
    }
    println!("it is found at ({:+.15e}, {:+.15e})", center.x, center.y);
    return Ok(center);
}

/// Finds the most complex part of the given range inside the given domain and zooms-in.
///   
/// * `resols` - The number of pixels in two directions.
/// * `delta`  - Inter-pixel distance.
/// * `factor` - Zoom-in speed, (0 = infinite, 1 = no zoom).
/// * `points` - Result of the reccurence relation for each pixel, from which the complexity is computed.
/// * `center` - Center of the domain, which is intent-in/out.
fn update_center(
    resols: &Coord<usize>,
    delta: f64,
    factor: f64,
    points: &Vec<Point>,
    center: &mut Coord<f64>,
) -> Result<u64, ()> {
    // compute complexity for each sub-domain
    let mut complexities: [u64; 4] = [0; 4];
    for n in 0..resols.x * resols.y {
        let i: usize = n % resols.x;
        let j: usize = n / resols.x;
        // four sub-domains
        let ci: usize = if i < resols.x / 2 { 0 } else { 1 };
        let cj: usize = if j < resols.y / 2 { 0 } else { 1 };
        // skip edge
        if 0 == i || resols.x - 1 == i {
            continue;
        }
        if 0 == j || resols.y - 1 == j {
            continue;
        }
        // interested in four neighbouring points
        let indices: [usize; 5] = [
            j * resols.x + i,
            j * resols.x + (i - 1),
            j * resols.x + (i + 1),
            (j - 1) * resols.x + i,
            (j + 1) * resols.x + i,
        ];
        // compute XOR to check the boundary exists
        let mut are_diverged: [u64; 5] = [0; 5];
        for m in 0..5 {
            are_diverged[m] = if points[indices[m]].is_diverged { 1 } else { 0 };
        }
        let mut complexity: u64 = 0;
        for m in 1..5 {
            complexity += are_diverged[m] ^ are_diverged[0];
        }
        complexities[cj * 2 + ci] += complexity;
    }
    // take out maximum and its index
    let (index, complexity): (usize, u64) = {
        let (index, &value) = complexities
            .iter()
            .enumerate()
            .max_by_key(|&(_, value)| value)
            .unwrap();
        (index, value)
    };
    if 0 == complexity {
        return Err(());
    }
    let conditions: [[Coord<f64>; 2]; 4] = {
        // store two scalars for pretty print
        let sngl: f64 = 1.;
        let fctr: f64 = factor;
        [
            [Coord { x: sngl, y: sngl }, Coord { x: fctr, y: fctr }],
            [Coord { x: fctr, y: sngl }, Coord { x: sngl, y: fctr }],
            [Coord { x: sngl, y: fctr }, Coord { x: fctr, y: sngl }],
            [Coord { x: fctr, y: fctr }, Coord { x: sngl, y: sngl }],
        ]
    };
    let corners: [Coord<f64>; 2] = [
        Coord {
            x: common::get_bound(-1., resols.x, center.x, delta, conditions[index][0].x),
            y: common::get_bound(-1., resols.y, center.y, delta, conditions[index][0].y),
        },
        Coord {
            x: common::get_bound(1., resols.x, center.x, delta, conditions[index][1].x),
            y: common::get_bound(1., resols.y, center.y, delta, conditions[index][1].y),
        },
    ];
    center.x = 0.5 * corners[0].x + 0.5 * corners[1].x;
    center.y = 0.5 * corners[0].y + 0.5 * corners[1].y;
    return Ok(complexity);
}
