
use crate::domain::Domain;
use core::f64::consts::PI;
use crate::curves::curve::Curve;
use crate::plane::Plane;
use crate::types::*;
use crate::log::*;

pub struct ArcCurve {
  pub plane: Plane,
  pub radius: f64,
  pub angle: f64,
}

impl Default for ArcCurve {
  fn default() -> Self { 
    ArcCurve::new(Plane::default(), 2.0, PI)
  }
}

impl ArcCurve {
  pub fn new (plane: Plane, radius: f64, angle: f64) -> Self {
    Self {
      plane, radius, angle,
    }
  }
}

impl Curve for ArcCurve {

  fn domain (&self) -> Domain {
    Domain::new(0.0, self.angle)
  }

  fn length(&self) -> f64 {
    // 2.0 * PI * self.radius * (self.angle / (2.0 * PI))
    return self.radius * self.angle;
  }

  fn point_at(&self, s: f64) -> Point3f {
    let r = s * self.angle;
    let dx = self.radius * r.cos();
    let dy = self.radius * r.sin();
    let o = self.plane.origin.clone();
    let p = o + self.plane.x_axis * dx + self.plane.y_axis * dy;
    // log(&format!("{:?}", p));
    return p;
  }

  fn velocity_at(&self, s: f64) -> Vector3f {
    let r = s * self.angle;
    let dx = - self.radius * r.sin();
    let dy = self.radius * r.cos();
    self.plane.x_axis * dx + self.plane.y_axis * dy
  }

  fn acceleration_at(&self, s: f64) -> Vector3f {
    let r = s * self.angle;
    let dx = - self.radius * r.cos();
    let dy = - self.radius * r.sin();
    self.plane.x_axis * dx + self.plane.y_axis * dy
  }

  /*
  fn jerk_at(&self, s: f64) -> Vector3f {
    let r = s * self.angle;
    let dx = self.radius * r.sin();
    let dy = - self.radius * r.cos();
    self.plane.x_axis * dx + self.plane.y_axis * dy
  }
  */

}
