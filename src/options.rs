#![deny(missing_docs)]

//! Loads run-time parameters given as the command-line arguments.

mod test_f64;
mod test_string;
mod test_u64;
mod test_usize;

use crate::coord::Coord;
use std::env;

/// Stores fixed control parameters.
pub struct Options {
    /// Random seed which will be used to initialise random number generator.
    pub seed: u64,
    /// Inter-pixel distance of the final image.
    pub grid_size: f64,
    /// Number of pixels in the two dimensions.
    pub resols: Coord<usize>,
    /// Name of the result image (should be `*.ppm`).
    pub fname: String,
}

/// Loads and validates the control parameters.
pub fn init() -> Result<Options, ()> {
    // create an Options instance with default parameters
    let mut options: Options = Options {
        seed: 0u64,
        grid_size: 5.0e-7f64,
        resols: Coord {
            x: 1280usize,
            y: 800usize,
        },
        fname: String::from("image.ppm"),
    };
    // load parameters
    let args: Vec<String> = env::args().collect();
    // update default parameters if exist
    options.seed = match load_seed(&args, options.seed) {
        Ok(seed) => seed,
        Err(_) => {
            print_error_message();
            return Err(());
        }
    };
    options.grid_size = match load_grid_size(&args, options.grid_size) {
        Ok(grid_size) => grid_size,
        Err(_) => {
            print_error_message();
            return Err(());
        }
    };
    options.resols = match load_resols(&args, options.resols) {
        Ok(resols) => resols,
        Err(_) => {
            print_error_message();
            return Err(());
        }
    };
    options.fname = match load_fname(&args, options.fname) {
        Ok(fname) => fname,
        Err(_) => {
            print_error_message();
            return Err(());
        }
    };
    print_last_message(&options);
    return Ok(options);
}

/// Outputs an error message before abort.
fn print_error_message() -> () {
    println!("ERROR: failed to load command-line arguments.");
    println!("USAGE: cargo run -- --<key>=<value>");
    println!("Available keys and their types / requirements are listed below.");
    println!("    seed      : positive integer (u64)");
    println!("    grid_size : positive floating number (f64)");
    println!("    width     : positive integer number (usize)");
    println!("    height    : positive integer number (usize)");
    println!("    fname     : string which ends with 'ppm'");
    println!("See also README.rst.");
}

/// Outputs an log message before return.
///   
/// * `options` - Parameters loaded and / or assigned.
fn print_last_message(options: &Options) -> () {
    println!("The settings are configured as follows.");
    println!("    random seed     : {}", options.seed);
    println!("    grid size       : {}", options.grid_size);
    println!("    width           : {}", options.resols.x);
    println!("    height          : {}", options.resols.y);
    println!("    image file name : {}", options.fname);
}

/// Loads a random seed from the command-line arguments and try to interpret it as a `u64` value.
///   
/// * `args`    - All command-line arguments as a vector of strings.
/// * `default` - Default value filled in the absence of the user specification.
fn load_seed(args: &Vec<String>, default: u64) -> Result<u64, ()> {
    const KEY: &str = "seed";
    let value: u64 = match extract_value::<u64>(KEY, args, default) {
        Ok(value) => value,
        Err(msg) => {
            println!("{}: {}", KEY, msg);
            return Err(());
        }
    };
    return Ok(value);
}

/// Loads a grid size from the command-line arguments and try to interpret it as a `f64` value.
///   
/// * `args`    - All command-line arguments as a vector of strings.
/// * `default` - Default value filled in the absence of the user specification.
fn load_grid_size(args: &Vec<String>, default: f64) -> Result<f64, ()> {
    const KEY: &str = "grid_size";
    let value: f64 = match extract_value::<f64>(KEY, args, default) {
        Ok(value) => value,
        Err(msg) => {
            println!("{}: {}", KEY, msg);
            return Err(());
        }
    };
    // check being positive
    if value > 0. {
        return Ok(value);
    } else {
        println!("{}: expect positive number", KEY);
        return Err(());
    }
}

/// Loads `width` and `height` from the command-line arguments and try to interpret them as `usize` values.
///   
/// * `args`    - All command-line arguments as a vector of strings.
/// * `default` - Default value filled in the absence of the user specification.
fn load_resols(args: &Vec<String>, default: Coord<usize>) -> Result<Coord<usize>, ()> {
    const KEYS: Coord<&str> = Coord {
        x: "width",
        y: "height",
    };
    let resols: Coord<usize> = Coord {
        x: match extract_value::<usize>(KEYS.x, args, default.x) {
            Ok(value) => value,
            Err(msg) => {
                println!("{}: {}", KEYS.x, msg);
                return Err(());
            }
        },
        y: match extract_value::<usize>(KEYS.y, args, default.y) {
            Ok(value) => value,
            Err(msg) => {
                println!("{}: {}", KEYS.y, msg);
                return Err(());
            }
        },
    };
    return Ok(resols);
}

/// Load the name of the final image from the command-line arguments and validate its suffix.
///   
/// * `args`    - All command-line arguments as a vector of strings.
/// * `default` - Default value filled in the absence of the user specification.
fn load_fname(args: &Vec<String>, default: String) -> Result<String, ()> {
    const KEY: &str = "fname";
    let value: String = match extract_value::<String>(KEY, args, default) {
        Ok(value) => value,
        Err(msg) => {
            println!("{}: {}", KEY, msg);
            return Err(());
        }
    };
    // check suffix
    if value.ends_with(".ppm") {
        return Ok(value);
    } else {
        println!("{}: expect suffix \".ppm\"", KEY);
        return Err(());
    }
}

/// Kernel function used to extract a value attached with the given key and convert it to the desired data type.
///   
/// * `key`     - Key whose attached value will be checked.
/// * `args`    - All command-line arguments as a vector of strings.
/// * `default` - Default value filled in the absence of the user specification.
fn extract_value<T: std::str::FromStr>(
    key: &str,
    args: &Vec<String>,
    default: T,
) -> Result<T, &'static str> {
    // from --<key>=<value>, extract value
    let header: String = format!("--{}=", key);
    for arg in args.iter() {
        if !arg.starts_with(&header) {
            continue;
        }
        return parse_and_extract::<T>(&header, arg);
    }
    // not found, use default
    return Ok(default);
}

/// Extract the value by parsing a key-value pair.
///   
/// * `header` - "--key=".
/// * `arg`    - "--key=value".
fn parse_and_extract<T: std::str::FromStr>(
    header: &String,
    arg: &String,
) -> Result<T, &'static str> {
    const MSG1: &'static str = "empty value";
    const MSG2: &'static str = "invalid value";
    // try to take the value out
    let value: &str = match arg.split(header).nth(1) {
        Some(value) => value,
        None => return Err(MSG1),
    };
    if "" == value {
        return Err(MSG1);
    }
    // try to convert from &str to T
    let value: T = match value.parse::<T>() {
        Ok(value) => value,
        Err(_) => return Err(MSG2),
    };
    return Ok(value);
}
