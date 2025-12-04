/// Trait for measuring distance between two objects.
/// 
/// Obj x Obj -> R
pub trait Metrizable {
  fn dist(&self, other: &Self) -> f64;
}

pub trait BoundedVariation {

  fn min(&self) -> f64;

  fn max(&self) -> f64;

  /// Clipping a value in case of greater / smaller of predefined
  /// `min`, `max` value.
  fn clip(&self, value : f64) -> f64 {
    value.min(self.max()).max(self.min())
  }

  /// Normalize a value to closed interval Icc(0, 1).
  /// 
  fn normalize(&self, value : f64) -> f64 {
    let value = self.clip(value);
    (value - &self.min()) / (&self.max() - &self.min())
  }
}

pub trait BoundedMetrizable : Metrizable + BoundedVariation {
  fn norm_dist(&self, other: &Self) -> f64 {
    self.normalize(self.dist(other))
  }
}