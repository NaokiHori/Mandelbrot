#![deny(missing_docs)]

//! Detects complex structures in the Mandelbrot set automatically and outputs the results as an image.  
//! <div>  
//! <img class="thumbnail" src="../static.files/image.jpg" onerror="this.onerror=null; this.remove();" alt="" width="50%" />  
//! </div>  

mod coord;
mod mandelbrot;
mod options;
mod output;
mod random;

/// Entrypoint of this crate.  
///
/// This function calls the following four sub-routines to complete the whole job.
///   
/// # Load command-line arguments
///
/// At first run-time parameters are loaded, which are given as the command-line arguments.  
/// For the time being they are optional and default parameters are assigned if not specified.
///   
/// See also: [`options::init()`].
///
/// # Find a nice image center
///
/// Extracting a fairly complex structure from the Mandelbrot set (or other fractals) is not a simple task.  
/// This function takes care of such *nice* extraction.
///   
/// See also: [`mandelbrot::find_center()`].
///
/// # Solve recurrence relation
///
/// In the previous step, the center of an image is determined.  
/// Here by solving the recurrence relation, we check whether each point (pixel) diverges or not,
/// and how many iterations are needed if it does.  
/// These results are stored in the struct [`mandelbrot::Point`].  
///   
/// See also: [`mandelbrot::solve_recurrence_relation()`].
///
/// # Generate an image
///
/// In the final step, we convert the pixel information into a simple image in the [Portable Any Map](https://netpbm.sourceforge.net/doc/ppm.html) format.  
///
/// See also: [`output::execute()`].
pub fn main() -> () {
    use coord::Coord;
    use mandelbrot::Point;
    use options::Options;
    // load command-line arguments to set options
    let options: Options = match options::init() {
        Ok(options) => options,
        Err(_) => std::process::exit(1),
    };
    // find a nice image center so that the resulting image has something to display
    let center: Coord<f64> = match mandelbrot::find_center(&options) {
        Ok(center) => center,
        Err(_) => std::process::exit(1),
    };
    // obtain the number of iterations to diverge
    let points: Vec<Point> = match mandelbrot::solve_recurrence_relation(&options, &center) {
        Ok(points) => points,
        Err(_) => std::process::exit(1),
    };
    // convert the results of the recurrence relation to an image
    match output::execute(&options, points) {
        Ok(_) => {}
        Err(_) => std::process::exit(1),
    };
}
