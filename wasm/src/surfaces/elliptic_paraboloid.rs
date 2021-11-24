
use crate::domain::Domain;
use crate::types::Vector3f;
use crate::types::Point3f;
use crate::surfaces::surface::Surface;
use crate::plane::Plane;

pub struct EllipticParaboloid {
  plane: Plane,
  a: f64,
  b: f64,
  domain: Domain,
}

impl Default for EllipticParaboloid {
  fn default() -> Self { 
    Self::new(
      Plane::new(
        Point3f::new(0.0, 0.0, 1.0),
        Vector3f::new(1.0, 0.0, 0.0),
        Vector3f::new(0.0, 1.0, 0.0),
        Vector3f::new(0.0, 0.0, -1.0),
      ),
      2.0, 2.0, Domain::new(-2.0, 2.0)
    )
  }
}

impl EllipticParaboloid {
  pub fn new (plane: Plane, a: f64, b: f64, domain: Domain) -> Self {
    Self {
      plane,
      a,
      b,
      domain,
    }
  }

  fn map (&self, u: f64, v: f64) -> (f64, f64) {
    (self.domain.map(u), self.domain.map(v))
  }
}

impl Surface for EllipticParaboloid {

  fn point_at(&self, u: f64, v: f64) -> Point3f {
    let (u, v) = self.map(u, v);
    let w = u * u + v * v;
    self.plane.origin + self.plane.x_axis * self.a * u + self.plane.y_axis * self.b * v + self.plane.normal * w
  }

  fn du_at(&self, u: f64, v: f64) -> Vector3f {
    let (u, _v) = self.map(u, v);
    self.plane.x_axis * self.a + self.plane.normal * 2.0 * u
  }

  fn dv_at(&self, u: f64, v: f64) -> Vector3f {
    let (_u, v) = self.map(u, v);
    self.plane.y_axis * self.b + self.plane.normal * 2.0 * v
  }

  fn dudu_at(&self, _: f64, _: f64) -> Vector3f {
    self.plane.normal * 2.0
  }

  fn dudv_at(&self, _: f64, _: f64) -> Vector3f {
    Vector3f::new(0.0, 0.0, 0.0)
  }

  fn dvdv_at(&self, _: f64, _: f64) -> Vector3f {
    self.plane.normal * 2.0
  }

}

