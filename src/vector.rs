pub struct Vector3 {
  pub x: f32,
  pub y: f32,
  pub z: f32
}

impl Vector3 {
  pub fn from_scalar(v: f32) -> Vector3 {
      Vector3{x: v, y: v, z: v}
  }

  pub fn zero() -> Vector3 {
    Vector3::from_scalar(0.0)
  }
}