
use core::f64::consts::PI;
use core::f64::consts::TAU;
use crate::types::Vector3f;
use crate::types::Point3f;
use crate::plane::Plane;
use crate::surfaces::surface::Surface;

pub struct Cylinder {
  plane: Plane,
  radius: f64,
  height: f64,
}

impl Default for Cylinder {
  fn default() -> Self { 
    Self::new(
      Plane::new(
        Point3f::new(0.0, 0.0, 1.5),
        Vector3f::new(1.0, 0.0, 0.0),
        Vector3f::new(0.0, 1.0, 0.0),
        Vector3f::new(0.0, 0.0, -1.0),
      ),
      2.0,
      3.0
    )
  }
}

impl Cylinder {
  pub fn new (plane: Plane, radius: f64, height: f64) -> Self {
    Self {
      plane, radius, height
    }
  }

  fn sin_cos (&self, u: f64) -> (f64, f64) {
    let delta = 1e-8;
    let theta = u * TAU * (1.0 - delta); // 0.0 <= v < 2pi
    let st = theta.sin();
    let ct = theta.cos();
    (st, ct)
  }
}

impl Surface for Cylinder {
  fn area(&self) -> f64 {
    2.0 * PI * self.radius * self.height
  }

  fn point_at(&self, u: f64, v: f64) -> Point3f {
    let (st, ct) = self.sin_cos(u);
    let dx = self.radius * st * self.plane.x_axis; // sin(theta)
    let dy = self.radius * ct * self.plane.y_axis; // cos(theta)
    let dz = self.height * v * self.plane.normal;
    self.plane.origin + dx + dy + dz
  }

  fn du_at(&self, u: f64, _v: f64) -> Vector3f {
    let (st, ct) = self.sin_cos(u);
    let dx = self.radius * ct * self.plane.x_axis; // cos(theta)
    let dy = - self.radius * st * self.plane.y_axis; // - sin(theta)
    dx + dy
  }

  fn dv_at(&self, _u: f64, _v: f64) -> Vector3f {
    self.height * self.plane.normal
  }

  fn dudu_at(&self, u: f64, _v: f64) -> Vector3f {
    let (st, ct) = self.sin_cos(u);
    let dx = - self.radius * st * self.plane.x_axis; // - sin(theta)
    let dy = - self.radius * ct * self.plane.y_axis; // - cos(theta)
    dx + dy
  }

  fn dvdv_at(&self, _u: f64, _v: f64) -> Vector3f {
    Vector3f::new(0.0, 0.0, 0.0)
  }

  fn dudv_at(&self, _u: f64, _v: f64) -> Vector3f {
    Vector3f::new(0.0, 0.0, 0.0)
  }
}

