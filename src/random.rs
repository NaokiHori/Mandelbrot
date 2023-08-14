#![deny(missing_docs)]

//! Takes care of random stuffs.

use rand::Rng;

/// Stores the random number generator (rng)
pub struct Random {
    /// random number generator (rng)
    rng: rand::rngs::StdRng,
}

/// Stores the random number generator (rng)
impl Random {
    /// Constructor.  
    /// This is just a wrapper of [`rand::SeedableRng::seed_from_u64()`],
    /// and store the returned *rng*.
    ///   
    /// * `seed` - Random seed used as an input of the rng.
    pub fn new(seed: u64) -> Random {
        let rng: rand::rngs::StdRng = rand::SeedableRng::seed_from_u64(seed);
        return Random { rng };
    }
    /// Returns a random number which is larger than `min` and smaller than `max`.
    ///   
    /// * `min` - Small limit.
    /// * `max` - Large limit.
    pub fn gen_range(&mut self, min: f64, max: f64) -> f64 {
        let rng: &mut rand::rngs::StdRng = &mut self.rng;
        return rng.gen_range(min..max);
    }
}
