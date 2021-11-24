
use crate::curve_tessellation::CurveTessellation;
use crate::domain::Domain;
use crate::frenet_frame::FrenetFrame;
use crate::types::*;

pub trait Curve {
  fn domain (&self) -> Domain;

  fn length(&self) -> f64;

  fn integral_length (&self, delta: Option<f64>) -> f64 {
    let delta = delta.unwrap_or(1e-4);
    let mut l: f64 = 0.0;
    let step: usize = (1.0 / delta) as usize;
    // log(&format!("step {:?}", step));
    let mut prev = self.point_at(0.0);
    for i in (1..=step).step_by(1) {
      let s: f64 = (i as f64) * delta;
      let current = self.point_at(s);
      l += (current - prev).norm();
      prev = current;
    }
    l
  }

  fn point_at(&self, s: f64) -> Point3f;
  fn velocity_at(&self, s: f64) -> Vector3f;
  fn acceleration_at(&self, s: f64) -> Vector3f;

  fn tangent_at(&self, s: f64) -> Vector3f {
    self.velocity_at(s).normalize()
  }

  fn normal_at(&self, s: f64) -> Vector3f {
    self.acceleration_at(s).normalize()
  }

  fn curvature_at(&self, s: f64) -> f64 {
    let n = self.acceleration_at(s);
    n.norm()
  }

  fn curvature_radius_at(&self, s: f64) -> f64 {
    1.0 / self.curvature_at(s)
  }

  fn binormal_at(&self, s: f64) -> Vector3f {
    let tangent = self.tangent_at(s);
    let normal = self.normal_at(s);
    tangent.cross(&normal)
  }

  fn frame_at(&self, s: f64) -> FrenetFrame {
    let p = self.point_at(s);
    let tangent = self.tangent_at(s).normalize();
    let normal = self.normal_at(s).normalize();
    let binormal = tangent.cross(&normal);
    FrenetFrame::new(&p, &tangent, &normal, &binormal)
  }

  fn tessellate(&self, delta: Option<f64>) -> CurveTessellation { 
    let delta = delta.unwrap_or(1e-4);
    let count: usize = (1.0 / delta) as usize;
    let parameters = (0..count).into_iter().map(|i| {
      let s = i as f64 / (count - 1) as f64;
      s
    });
    let frames: Vec<FrenetFrame> = parameters.clone().map(|s| {
      self.frame_at(s)
    }).collect();
    let curvatures: Vec<f64> = parameters.map(|s| {
      self.curvature_at(s)
    }).collect();
    CurveTessellation::new(frames, curvatures)
  }

  fn divide_by_count(&self, count: i64) -> Vec<Point3f> {
    (0..count).into_iter().map(|i| {
      let s = i as f64 / (count - 1) as f64;
      self.point_at(s)
    }).collect()
  }

  fn divide_by_length(&self, length: f64) -> Vec<Point3f> {
    todo!()
  }

}
