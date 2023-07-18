use crate::curves::curve::Curve;
use crate::domain::Domain;
use crate::plane::Plane;
use crate::types::Point3f;
use crate::types::Vector3f;
use core::f64::consts::PI;

pub struct EllipseArcCurve {
    pub plane: Plane,
    pub a: f64,
    pub b: f64,
    pub angle: f64,
}

impl Default for EllipseArcCurve {
    fn default() -> Self {
        EllipseArcCurve::new(Plane::default(), 1.0, 2.0, PI)
    }
}

impl EllipseArcCurve {
    pub fn new(plane: Plane, a: f64, b: f64, angle: f64) -> Self {
        Self { plane, a, b, angle }
    }
}

impl Curve for EllipseArcCurve {
    fn domain(&self) -> Domain {
        Domain::new(0.0, self.angle)
    }

    fn length(&self) -> f64 {
        self.integral_length(None)
    }

    fn point_at(&self, s: f64) -> Point3f {
        let r = s * self.angle;
        let dx = self.a * r.cos();
        let dy = self.b * r.sin();
        let o = self.plane.origin;
        o + self.plane.x_axis * dx + self.plane.y_axis * dy
    }

    fn velocity_at(&self, s: f64) -> Vector3f {
        let r = s * self.angle;
        let dx = -self.a * r.sin();
        let dy = self.b * r.cos();
        self.plane.x_axis * dx + self.plane.y_axis * dy
    }

    fn acceleration_at(&self, s: f64) -> Vector3f {
        let r = s * self.angle;
        let dx = -self.a * r.cos();
        let dy = -self.b * r.sin();
        self.plane.x_axis * dx + self.plane.y_axis * dy
    }
}
