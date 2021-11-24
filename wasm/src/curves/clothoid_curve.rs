
use crate::log::log;
use core::f64::consts::TAU;
use crate::optimizer::df::F;
use crate::curves::parametric_curve::ParametricCurve;
use crate::types::{Point3f, Vector3f};
use crate::curves::curve::Curve;
use crate::domain::Domain;
use crate::plane::Plane;

pub struct ClothoidCurve {
  plane: Plane,
  a: f64,
  start_angle: f64,
  end_angle: f64,
  delta: f64,
}

impl Default for ClothoidCurve {
  fn default() -> Self { 
    ClothoidCurve::new(Plane::default(), 1.0, -TAU, TAU, Some(1e-3))
  }
}

impl ClothoidCurve {
  pub fn new (plane: Plane, a: f64, start_angle: f64, end_angle: f64, delta: Option<f64>) -> Self {
    Self {
      plane, a, start_angle, end_angle,
      delta: delta.unwrap_or(1e-2)
    }
  }
}

impl Curve for ClothoidCurve {

  fn domain(&self) -> Domain {
    Domain::new(self.start_angle, self.end_angle)
  }

  fn length(&self) -> f64 {
    self.integral_length(None)
  }

  fn point_at(&self, s: f64) -> Point3f {
    let t = s * (self.end_angle - self.start_angle) + self.start_angle;
    let sine = t.signum();
    let t = (t / self.delta) as i64;
    let abs = t.abs();
    let parameters = (0..=abs).into_iter().map(|i| {
      let tt = (i as f64) * self.delta;
      self.a * tt * tt * 0.5
    });
    let dx: f64 = parameters.clone().into_iter().map(|p| {
      p.cos() * self.delta
    }).sum();
    let dy: f64 = parameters.into_iter().map(|p| {
      p.sin() * self.delta
    }).sum();
    self.plane.origin + self.plane.x_axis * dx * sine + self.plane.y_axis * dy * sine
  }

  fn velocity_at(&self, s: f64) -> Vector3f {
    let t = s * (self.end_angle - self.start_angle) + self.start_angle;
    let sine = t.signum();
    let p = self.a * t * t * 0.5;
    let dx = p.cos();
    let dy = p.sin();
    self.plane.x_axis * dx * sine + self.plane.y_axis * dy * sine
  }

  fn acceleration_at(&self, s: f64) -> Vector3f {
    let t = s * (self.end_angle - self.start_angle) + self.start_angle;
    let sine = t.signum();
    let p = self.a * t * t * 0.5;
    let k = self.a * t; // difference of p
    let dx = k * - p.sin();
    let dy = k * p.cos();
    self.plane.x_axis * dx * sine + self.plane.y_axis * dy * sine
  }

}
