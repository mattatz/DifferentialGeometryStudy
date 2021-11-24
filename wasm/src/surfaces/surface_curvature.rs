
use crate::types::Vector3f;
use crate::types::Point3f;

pub struct SurfaceCurvature {
  pub point: Point3f,
  pub uv: (f64, f64),
  pub normal: Vector3f,
  pub gaussian: f64,
  pub mean: f64,
}

impl SurfaceCurvature {
}

