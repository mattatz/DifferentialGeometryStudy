
use core::f64::consts::TAU;
use crate::plane::Plane;
use crate::types::Vector3f;
use crate::types::Point3f;
use crate::surfaces::surface::Surface;

pub struct Torus {
  pub plane: Plane,
  pub a: f64,
  pub b: f64,
}

impl Default for Torus {
  fn default() -> Self { 
    Self::new(Plane::default(), 1.0, 3.0)
  }
}

impl Torus {
  pub fn new (plane: Plane, a: f64, b: f64) -> Self {
    Self {
      plane, a, b
    }
  }

  fn sin_cos (&self, u: f64, v: f64) -> (f64, f64, f64, f64) {
    let theta = u * TAU;
    let phi = v * TAU;
    let st = theta.sin();
    let ct = theta.cos();
    let sp = phi.sin();
    let cp = phi.cos();
    (st, ct, sp, cp)
  }
}

impl Surface for Torus {

  fn point_at(&self, u: f64, v: f64) -> Point3f {
    let (st, ct, sp, cp) = self.sin_cos(u, v);
    let dx = (self.a * ct + self.b) * cp; // (acost + b) cosp
    let dy = (self.a * ct + self.b) * sp; // (acost + b) sinp
    let dz = self.a * st; // asint
    Point3f::new(dx, dy, dz)
  }

  fn du_at(&self, u: f64, v: f64) -> Vector3f {
    let (st, ct, sp, cp) = self.sin_cos(u, v);
    let dx = - self.a * st * cp; // - asint cosp
    let dy = - self.a * st * sp; // - asint sinp
    let dz = self.a * ct; // acost
    Vector3f::new(dx, dy, dz)
  }

  fn dv_at(&self, u: f64, v: f64) -> Vector3f {
    let (_st, ct, sp, cp) = self.sin_cos(u, v);
    let dx = - (self.a * ct + self.b) * sp; // - (acost + b) sinp
    let dy = (self.a * ct + self.b) * cp; // (acost + b) cosp
    let dz = 0.0; // 0
    Vector3f::new(dx, dy, dz)
  }

  fn dudu_at(&self, u: f64, v: f64) -> Vector3f {
    let (st, ct, sp, cp) = self.sin_cos(u, v);
    let dx = - self.a * ct * cp; // - acost cosp
    let dy = - self.a * ct * sp; // - acost sinp
    let dz = - self.a * st; // -asint
    Vector3f::new(dx, dy, dz)
  }

  fn dvdv_at(&self, u: f64, v: f64) -> Vector3f {
    let (_st, ct, sp, cp) = self.sin_cos(u, v);
    let dx = - (self.a * ct + self.b) * cp; // - (acost + b) cosp
    let dy = - (self.a * ct + self.b) * sp; // - (acost + b) sinp
    let dz = 0.0; // 0
    Vector3f::new(dx, dy, dz)
  }

  fn dudv_at(&self, u: f64, v: f64) -> Vector3f {
    let (st, _ct, sp, cp) = self.sin_cos(u, v);
    let dx = self.a * st * sp; // asint sinp
    let dy = - self.a * st * cp; // - asint cosp
    let dz = 0.0; // 0.0
    Vector3f::new(dx, dy, dz)
  }

}
