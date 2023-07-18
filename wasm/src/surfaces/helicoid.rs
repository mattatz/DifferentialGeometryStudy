use crate::domain::Domain;
use crate::plane::Plane;
use crate::surfaces::surface::Surface;
use crate::types::Point3f;
use crate::types::Vector3f;

pub struct Helicoid {
    plane: Plane,
    c: f64,
    domain: Domain,
}

impl Default for Helicoid {
    fn default() -> Self {
        Self::new(Plane::default(), 2.0, Domain::new(-2.0, 2.0))
    }
}

impl Helicoid {
    pub fn new(plane: Plane, c: f64, domain: Domain) -> Self {
        Self { plane, c, domain }
    }

    fn map(&self, u: f64, v: f64) -> (f64, f64) {
        (self.domain.map(u), self.domain.map(v))
    }
}

impl Surface for Helicoid {
    fn point_at(&self, u: f64, v: f64) -> Point3f {
        let (u, v) = self.map(u, v);
        self.plane.origin
            + self.plane.x_axis * u * v.cos()
            + self.plane.y_axis * u * v.sin()
            + self.plane.normal * self.c * v
    }

    fn du_at(&self, u: f64, v: f64) -> Vector3f {
        let (_u, v) = self.map(u, v);
        self.plane.x_axis * v.cos() + self.plane.y_axis * v.sin()
    }

    fn dv_at(&self, u: f64, v: f64) -> Vector3f {
        let (u, v) = self.map(u, v);
        -self.plane.x_axis * u * v.sin()
            + self.plane.y_axis * u * v.cos()
            + self.plane.normal * self.c
    }

    fn dudu_at(&self, _: f64, _: f64) -> Vector3f {
        Vector3f::new(0.0, 0.0, 0.0)
    }

    fn dudv_at(&self, u: f64, v: f64) -> Vector3f {
        let (_u, v) = self.map(u, v);
        -self.plane.x_axis * v.sin() + self.plane.y_axis * v.cos()
    }

    fn dvdv_at(&self, u: f64, v: f64) -> Vector3f {
        let (u, v) = self.map(u, v);
        -self.plane.x_axis * u * v.cos() - self.plane.y_axis * u * v.sin()
    }
}
