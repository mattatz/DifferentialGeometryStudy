use crate::plane::Plane;
use crate::surfaces::surface::Surface;
use crate::types::Point3f;
use crate::types::Vector3f;
use core::f64::consts::PI;
use core::f64::consts::TAU;

pub struct Mobius {
    plane: Plane,
}

impl Default for Mobius {
    fn default() -> Self {
        Self::new(Plane::default())
    }
}

impl Mobius {
    pub fn new(plane: Plane) -> Self {
        Self { plane }
    }

    fn theta_phi(&self, u: f64, v: f64) -> (f64, f64) {
        let theta = u * TAU - PI;
        let phi = v - 0.5;
        (theta, phi)
    }
}

impl Surface for Mobius {
    fn point_at(&self, u: f64, v: f64) -> Point3f {
        let (theta, phi) = self.theta_phi(u, v);
        let dx = theta.cos() + phi * (theta * 0.5).sin() * theta.cos(); // cos(theta) * phi * sin(theta / 2) * cos(theta)
        let dy = theta.sin() + phi * (theta * 0.5).sin() * theta.sin(); // sin(theta) * phi * sin(theta / 2) * sin(theta)
        let dz = phi * (theta * 0.5).cos(); // phi * cos(theta / 2)
        self.plane.origin + self.plane.x_axis * dx + self.plane.y_axis * dy + self.plane.normal * dz
    }

    fn du_at(&self, u: f64, v: f64) -> Vector3f {
        let (theta, phi) = self.theta_phi(u, v);
        let dx = -theta.sin() + -phi * 0.5 * (theta * 0.5).cos() * theta.sin();
        let dy = theta.cos() + phi * 0.5 * (theta * 0.5).cos() * theta.cos();
        let dz = -phi * 0.5 * (theta * 0.5).sin();
        self.plane.x_axis * dx + self.plane.y_axis * dy + self.plane.normal * dz
    }

    fn dv_at(&self, u: f64, v: f64) -> Vector3f {
        let (theta, _) = self.theta_phi(u, v);
        let dx = (theta * 0.5).sin() * theta.cos();
        let dy = (theta * 0.5).sin() * theta.sin();
        let dz = (theta * 0.5).cos();
        self.plane.x_axis * dx + self.plane.y_axis * dy + self.plane.normal * dz
    }

    fn dudu_at(&self, u: f64, v: f64) -> Vector3f {
        let (theta, phi) = self.theta_phi(u, v);
        let dx = -theta.sin() + -phi * 0.5 * (theta * 0.5).cos() * theta.sin();
        let dy = theta.cos() + phi * 0.5 * (theta * 0.5).cos() * theta.cos();
        let dz = -phi * 0.5 * (theta * 0.5).sin();
        self.plane.x_axis * dx + self.plane.y_axis * dy + self.plane.normal * dz
    }

    fn dvdv_at(&self, u: f64, v: f64) -> Vector3f {
        let (theta, _) = self.theta_phi(u, v);
        let dx = (theta * 0.5).sin() * theta.cos();
        let dy = (theta * 0.5).sin() * theta.sin();
        let dz = (theta * 0.5).cos();
        self.plane.x_axis * dx + self.plane.y_axis * dy + self.plane.normal * dz
    }

    fn dudv_at(&self, u: f64, v: f64) -> Vector3f {
        let (theta, phi) = self.theta_phi(u, v);
        let dx = -theta.sin() + -phi * 0.5 * (theta * 0.5).cos() * theta.sin();
        let dy = theta.cos() + phi * 0.5 * (theta * 0.5).cos() * theta.cos();
        let dz = -phi * 0.5 * (theta * 0.5).sin();
        self.plane.x_axis * dx + self.plane.y_axis * dy + self.plane.normal * dz
    }
}
