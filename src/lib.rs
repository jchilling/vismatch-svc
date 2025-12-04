pub mod descriptor;
pub mod vec_ops;
pub mod metric;

use num_traits::Float;
use ndarray::Array1;

pub trait L2Norm<B: Float> {
    /// Implement the l2-norm.
    fn norm(&self) -> B;
}

impl L2Norm<f32> for Array1<f32> {
    fn norm(&self) -> f32 {
        let x = self.dot(self);
        x.sqrt()
    }
}

impl L2Norm<f64> for Array1<f64> {
    fn norm(&self) -> f64 {
        let x = self.dot(self);
        x.sqrt()
    }
}

pub trait UnitVector<B: Float> {
    fn unit(&self) -> Array1<B>; 
}


impl UnitVector<f32> for Array1<f32> {
    fn unit(&self) -> Array1<f32> {
        self.clone() / self.norm()
    }
}

impl UnitVector<f64> for Array1<f64> {
    fn unit(&self) -> Array1<f64> {
        self.clone() / self.norm()
    }
}


// Add support of casting a result of image hashing to a 
// floating type vector.
// 
// # Example
// 
// ```rust,no_run
// 
// imagehash::perceptual_hash(&image).as_vector::<f32>();
// 
// ```
//pub trait AsFloatVector {
//    fn as_vector<B: Float + 'static>(&self) -> Vec<B>
//        where f32: AsPrimitive<B>; // F32 is acceptable here, since the source is from uint8.
//
//    fn as_array<B: Float + 'static>(&self) -> ndarray::Array1<B>
//        where f32: AsPrimitive<B> { // F32 is acceptable here, since the source is from uint8.
//            let x = self.as_vector();
//            return ndarray::Array1::from_vec(x);
//        } 
//
//}

//impl AsFloatVector for imagehash::Hash {
//    /// Casting a result of image hashing to a floating type vector.
//    /// 
//    /// # Example
//    /// 
//    /// ```rust,no_run
//    /// 
//    /// imagehash::perceptual_hash(&image).as_vector::<f32>();
//    /// 
//    /// ```
//    fn as_vector<B: Float + 'static>(&self) -> Vec<B> 
//        where f32: AsPrimitive<B>{
//        let f_vec: Vec<B> = self.to_bytes().iter()
//            .map(|x: &u8| (x.clone() as f32 / 255.0).as_())
//            .collect();
//        return f_vec;
//    }
//}