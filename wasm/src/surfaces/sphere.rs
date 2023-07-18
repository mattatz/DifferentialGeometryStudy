use crate::plane::Plane;
use crate::surfaces::surface::Surface;
use crate::types::Point3f;
use crate::types::Vector3f;
use core::f64::consts::PI;
use core::f64::consts::TAU;

pub struct Sphere {
    plane: Plane,
    radius: f64,
}

impl Default for Sphere {
    fn default() -> Self {
        Sphere::new(Plane::default(), 2.0)
    }
}

impl Sphere {
    pub fn new(plane: Plane, radius: f64) -> Self {
        Self { plane, radius }
    }

    fn sin_cos(&self, u: f64, v: f64) -> (f64, f64, f64, f64) {
        let delta = 1e-8;
        let theta = u * PI * (1.0 - delta * 2.0) + delta; // 0.0 < u < pi
        let phi = v * TAU * (1.0 - delta); // 0.0 <= v < 2pi
        let st = theta.sin();
        let ct = theta.cos();
        let sp = phi.sin();
        let cp = phi.cos();
        (st, ct, sp, cp)
    }
}

impl Surface for Sphere {
    fn area(&self) -> f64 {
        4.0 * PI * self.radius * self.radius
    }

    fn point_at(&self, u: f64, v: f64) -> Point3f {
        let (st, ct, sp, cp) = self.sin_cos(u, v);
        let dx = self.radius * st * cp * self.plane.x_axis; // sin(theta) * cos(phi)
        let dy = self.radius * st * sp * self.plane.y_axis; // sin(theta) * sin(phi)
        let dz = self.radius * ct * self.plane.normal; // cos(theta)
        self.plane.origin + dx + dy + dz
    }

    fn du_at(&self, u: f64, v: f64) -> Vector3f {
        let (st, ct, sp, cp) = self.sin_cos(u, v);
        let dx = self.radius * ct * cp * self.plane.x_axis; // cos(theta) * cos(phi)
        let dy = self.radius * ct * sp * self.plane.y_axis; // cos(theta) * sin(phi)
        let dz = -self.radius * st * self.plane.normal; // - sin(theta)
        dx + dy + dz
    }

    fn dv_at(&self, u: f64, v: f64) -> Vector3f {
        let (st, _ct, sp, cp) = self.sin_cos(u, v);
        let dx = -self.radius * st * sp * self.plane.x_axis; // - sin(theta) * sin(phi)
        let dy = self.radius * st * cp * self.plane.y_axis; // sin(theta) * cos(phi)
        let dz = 0.0 * self.plane.normal; // 0
        dx + dy + dz
    }

    fn dudu_at(&self, u: f64, v: f64) -> Vector3f {
        let (st, ct, sp, cp) = self.sin_cos(u, v);
        let dx = -self.radius * st * cp * self.plane.x_axis; // - sin(theta) * cos(phi)
        let dy = -self.radius * st * sp * self.plane.y_axis; // - sin(theta) * sin(phi)
        let dz = -self.radius * ct * self.plane.normal; // - cos(theta)
        dx + dy + dz
    }

    fn dvdv_at(&self, u: f64, v: f64) -> Vector3f {
        let (st, _ct, sp, cp) = self.sin_cos(u, v);
        let dx = -self.radius * st * cp * self.plane.x_axis; // - sin(theta) * cos(phi)
        let dy = -self.radius * st * sp * self.plane.y_axis; // - sin(theta) * sin(phi)
        let dz = 0.0 * self.plane.normal; // 0
        dx + dy + dz
    }

    fn dudv_at(&self, u: f64, v: f64) -> Vector3f {
        let (_st, ct, sp, cp) = self.sin_cos(u, v);
        let dx = -self.radius * ct * sp * self.plane.x_axis; // - cos(theta) * sin(phi)
        let dy = self.radius * ct * cp * self.plane.y_axis; // cos(theta) * cos(phi)
        let dz = 0.0 * self.plane.normal; // 0.0
        dx + dy + dz
    }
}
