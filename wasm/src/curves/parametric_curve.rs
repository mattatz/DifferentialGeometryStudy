use crate::curves::curve::Curve;
use crate::domain::Domain;

use crate::optimizer::df::{df, F};
use crate::types::{Point3f, Vector3f};

pub trait ParametricCurve {
    fn domain(&self) -> Domain;
    fn x_expr(&self) -> &Box<F>;
    fn y_expr(&self) -> &Box<F>;
    fn z_expr(&self) -> &Box<F>;
}

impl<T> Curve for T
where
    T: ParametricCurve,
{
    fn domain(&self) -> Domain {
        self.domain()
    }

    fn length(&self) -> f64 {
        let delta = 1e-4;
        let mut l: f64 = 0.0;
        let step: usize = (1.0 / delta) as usize;
        // log(&format!("step {:?}", step));
        let mut prev = self.point_at(0.0);
        for i in (1..=step).step_by(1) {
            let s: f64 = (i as f64) * delta;
            let current = self.point_at(s);
            l += (current - prev).norm();
            prev = current;
        }
        l
    }

    fn point_at(&self, s: f64) -> Point3f {
        let d = self.domain();
        let t = d.start() + s * d.end();
        let x = (self.x_expr())(t);
        let y = (self.y_expr())(t);
        let z = (self.z_expr())(t);
        Point3f::new(x, y, z)
    }

    fn velocity_at(&self, s: f64) -> Vector3f {
        let d = self.domain();
        let t = d.start() + s * d.end();
        let dx = df(self.x_expr(), 1, t, 1e-4);
        let dy = df(self.y_expr(), 1, t, 1e-4);
        let dz = df(self.z_expr(), 1, t, 1e-4);
        Vector3f::new(dx, dy, dz)
    }

    fn acceleration_at(&self, s: f64) -> Vector3f {
        let d = self.domain();
        let t = d.start() + s * d.end();
        let dx = df(self.x_expr(), 2, t, 1e-4);
        let dy = df(self.y_expr(), 2, t, 1e-4);
        let dz = df(self.z_expr(), 2, t, 1e-4);
        Vector3f::new(dx, dy, dz)
    }
}
