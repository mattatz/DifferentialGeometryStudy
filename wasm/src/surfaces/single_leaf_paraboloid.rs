
use std::f64::consts::TAU;
use crate::domain::Domain;
use crate::types::Vector3f;
use crate::types::Point3f;
use crate::surfaces::surface::Surface;
use crate::plane::Plane;

pub struct SingleLeafParaboloid {
  plane: Plane,
  a: f64,
  b: f64,
  c: f64,
  domain: Domain,
}

impl Default for SingleLeafParaboloid {
  fn default() -> Self { 
    Self::new(
      Plane::default(), 1.0, 1.0, 2.0, Domain::new(-2.0, 2.0)
    )
  }
}

impl SingleLeafParaboloid {
  pub fn new (plane: Plane, a: f64, b: f64, c: f64, domain: Domain) -> Self {
    Self {
      plane,
      a,
      b,
      c,
      domain,
    }
  }

  fn map (&self, u: f64, v: f64) -> (f64, f64) {
    (self.domain.map(u), v * TAU)
  }
}

impl Surface for SingleLeafParaboloid {

  fn point_at(&self, u: f64, v: f64) -> Point3f {
    let (u, v) = self.map(u, v);
    let dx = self.a * u.cosh() * v.cos();
    let dy = self.b * u.cosh() * v.sin();
    let dz = self.c * u.sinh();
    self.plane.origin + self.plane.x_axis * dx + self.plane.y_axis * dy + self.plane.normal * dz
  }

  fn du_at(&self, u: f64, v: f64) -> Vector3f {
    let (u, v) = self.map(u, v);
    let dx = self.a * u.sinh() * v.cos();
    let dy = self.b * u.sinh() * v.sin();
    let dz = self.c * u.cosh();
    self.plane.x_axis * dx + self.plane.y_axis * dy + self.plane.normal * dz
  }

  fn dv_at(&self, u: f64, v: f64) -> Vector3f {
    let (u, v) = self.map(u, v);
    let dx = - self.a * u.cosh() * v.sin();
    let dy = self.b * u.cosh() * v.cos();
    let dz = 0.0;
    self.plane.x_axis * dx + self.plane.y_axis * dy + self.plane.normal * dz
  }

  fn dudu_at(&self, u: f64, v: f64) -> Vector3f {
    let (u, v) = self.map(u, v);
    let dx = self.a * u.cosh() * v.cos();
    let dy = self.b * u.cosh() * v.sin();
    let dz = self.c * u.sinh();
    self.plane.x_axis * dx + self.plane.y_axis * dy + self.plane.normal * dz
  }

  fn dudv_at(&self, u: f64, v: f64) -> Vector3f {
    let (u, v) = self.map(u, v);
    let dx = - self.a * u.sinh() * v.sin();
    let dy = self.b * u.sinh() * v.cos();
    let dz = 0.0;
    self.plane.x_axis * dx + self.plane.y_axis * dy + self.plane.normal * dz
  }

  fn dvdv_at(&self, u: f64, v: f64) -> Vector3f {
    let (u, v) = self.map(u, v);
    let dx = - self.a * u.cosh() * v.cos();
    let dy = - self.b * u.cosh() * v.sin();
    let dz = 0.0;
    self.plane.x_axis * dx + self.plane.y_axis * dy + self.plane.normal * dz
  }

}

