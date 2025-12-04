mod traits;

use std::iter::zip;

/// Implement metrize for image hash bytes.
/// 
/// In this implementation, we use hamming distance.
impl traits::Metrizable for imagehash::Hash {
    fn dist(&self, other: &Self) -> f64 {
      let lhs = self.bits.clone();
      let rhs = other.bits.clone();
      let hm_diff = zip(lhs.iter(), rhs.iter())
        .map(|(x, y)| x != y)
        .map(|x| if x {1} else {0})
        .fold(0, |acc, x| acc + x);
      
      hm_diff as f64
    }
}

impl traits::BoundedVariation for imagehash::Hash {
    fn min(&self) -> f64 {
        // The minimum possible difference between two 64-bit
        // arrays, is 0.
        0.0
    }

    fn max(&self) -> f64 {
        // The maximum possible difference between two 64-bit
        // arrays, is 64.
        64.0
    }

}


//fn diff(lhs : &Vec<bool>, rhs : &Vec<bool>) -> i32 {
//    zip(lhs.iter(), rhs.iter())
//    .map(|(x, y)| x != y)
//    .map(|x| if x {1} else {0})
//    .fold(0, |acc, x| acc + x)
//}


impl traits::BoundedMetrizable for imagehash::Hash { }

pub use traits::{Metrizable, BoundedVariation, BoundedMetrizable};