#![deny(missing_docs)]

//! Pixelises two-dimensional array and outputs the result as an image.

use crate::coord::Coord;
use crate::mandelbrot::Point;
use crate::options::Options;

/// Main function
///   
/// * `options` - Fixed configuration parameters
/// * `point`   - Result of the recurrence relation for each point
pub fn execute(options: &Options, points: Vec<Point>) -> Result<(), ()> {
    let pixels: Vec<u8> = match pixelise(options, points) {
        Ok(pixels) => pixels,
        Err(_) => return Err(()),
    };
    let _ = match dump(options, pixels) {
        Ok(()) => return Ok(()),
        Err(_) => return Err(()),
    };
}

/// Converts a two-dimensional array telling the result of the
/// recurrence relation to a set of pixels  
///   
/// * `options` - Fixed parameters: the number of pixels
/// * `point`   - Result of the recurrence relation for each point
fn pixelise(options: &Options, points: Vec<Point>) -> Result<Vec<u8>, ()> {
    let resols: &Coord<usize> = &options.resols;
    let nitems = resols.x * resols.y;
    let mut pixels: Vec<u8> = vec![0u8; nitems * 3];
    // find extrema
    let min: u64 = match points.iter().min_by_key(|point| point.iter) {
        Some(point) => point.iter,
        None => {
            println!("failed to find min");
            return Err(());
        }
    };
    let max: u64 = match points.iter().max_by_key(|point| point.iter) {
        Some(point) => point.iter,
        None => {
            println!("failed to find max");
            return Err(());
        }
    };
    let min: f64 = min as f64;
    let max: f64 = max as f64;
    // convert
    for n in 0..nitems {
        let is_diverged: bool = points[n].is_diverged;
        let iter: f64 = points[n].iter as f64;
        // enforce [0f64:1f64]
        let val: f64 = if is_diverged {
            (iter - min) / (max - min)
        } else {
            1.
        };
        // transform
        let x: f64 = (n % resols.x) as f64 / resols.x as f64 - 0.5f64;
        let y: f64 = (n / resols.x) as f64 / resols.y as f64 - 0.5f64;
        let theta: f64 = y.atan2(x);
        let val: [f64; 3] = to_rgb(theta, val);
        // convert to u8
        for m in 0..3 {
            pixels[3 * n + m] = if val[m] > 1. {
                255
            } else if val[m] < 0. {
                0
            } else {
                (255. * val[m]) as u8
            };
        }
    }
    return Ok(pixels);
}

/// Outputs pixelised information as an image  
///   
/// * `options` - Fixed parameters: output file name, the number of pixels
/// * `pixels`  - Main data to be dumped
fn dump(options: &Options, pixels: Vec<u8>) -> Result<(), ()> {
    // I assume *.ppm
    const MAGIC_NUMBER: &str = "P6";
    // open and prepare stream
    let file: std::fs::File = match std::fs::File::create(&options.fname) {
        Ok(file) => file,
        Err(_) => {
            println!("failed to open file");
            return Err(());
        }
    };
    let mut stream: std::io::BufWriter<std::fs::File> = std::io::BufWriter::new(file);
    // fwrite
    let _size: usize = match std::io::Write::write(
        &mut stream,
        format!(
            "{}\n{} {}\n255\n",
            MAGIC_NUMBER, &options.resols.x, &options.resols.y
        )
        .as_bytes(),
    ) {
        Ok(size) => size,
        Err(_) => {
            println!("file write failed");
            return Err(());
        }
    };
    let _size: usize = match std::io::Write::write(&mut stream, &pixels) {
        Ok(size) => size,
        Err(_) => {
            println!("file write failed");
            return Err(());
        }
    };
    return Ok(());
}

/// Maps a scalar value to an RGB pair
///   
/// * `theta` - Random phase angle.
/// * `val`   - Scalar value.
fn to_rgb(theta: f64, val: f64) -> [f64; 3] {
    const PI: f64 = std::f64::consts::PI;
    return [
        0.5f64 * val * (1. + f64::sin(0. / 3. * PI + theta)),
        0.5f64 * val * (1. + f64::sin(2. / 3. * PI + theta)),
        0.5f64 * val * (1. + f64::sin(4. / 3. * PI + theta)),
    ];
}
