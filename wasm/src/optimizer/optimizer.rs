use crate::optimizer::df::grad;
use crate::optimizer::df::hessian;
use crate::optimizer::df::F2x1;
use crate::types::Matrix2x1f;
use crate::types::Matrix2x2f;
use wasm_bindgen::prelude::*;

pub trait Optimizer {
    fn new(x: &Matrix2x1f, delta: f64) -> Self
    where
        Self: Sized;
    fn init(&mut self, x: &Matrix2x1f);
    fn iterate(&mut self, expr: &F2x1) -> Option<Matrix2x1f>;
    fn next(&self, expr: &F2x1, x: &Matrix2x1f) -> Option<Matrix2x1f>;

    fn backtrack(&self, expr: &F2x1, x: &Matrix2x1f, p: &Matrix2x1f, rho: f64, c1: f64) -> f64 {
        let x0 = expr(x);
        let d = c1
            * (grad(expr, x, self.delta()).transpose() * p)
                .get(0)
                .unwrap();
        let mut alpha = 1.0;
        // search first step size to meet the armijo condition
        while expr(&(x + p * alpha)) > x0 + alpha * d {
            alpha *= rho;
        }
        alpha
    }

    fn delta(&self) -> f64;
}

#[wasm_bindgen]
pub struct GradientDescentOptimizer {
    x: Matrix2x1f,
    delta: f64,
}

#[wasm_bindgen]
pub struct NewtonOptimizer {
    x: Matrix2x1f,
    delta: f64,
}

#[wasm_bindgen]
pub struct QuasiNewtonOptimizer {
    x: Matrix2x1f,
    b: Matrix2x2f,
    delta: f64,
}

impl Optimizer for GradientDescentOptimizer {
    fn new(x: &Matrix2x1f, delta: f64) -> Self {
        GradientDescentOptimizer { x: *x, delta }
    }

    fn init(&mut self, x: &Matrix2x1f) {
        self.x = *x;
    }

    fn delta(&self) -> f64 {
        self.delta
    }

    fn iterate(&mut self, expr: &F2x1) -> Option<Matrix2x1f> {
        let next = self.next(expr, &self.x);
        match next {
            Some(x) => self.x = x,
            None => {}
        }
        next
    }

    fn next(&self, expr: &F2x1, x: &Matrix2x1f) -> Option<Matrix2x1f> {
        let p = -grad(expr, x, self.delta);
        let alpha = self.backtrack(expr, x, &p, 0.95, 0.5);
        Some(x + p * alpha)
    }
}

impl Optimizer for NewtonOptimizer {
    fn new(x: &Matrix2x1f, delta: f64) -> Self {
        NewtonOptimizer { x: *x, delta }
    }

    fn init(&mut self, x: &Matrix2x1f) {
        self.x = *x;
    }

    fn delta(&self) -> f64 {
        self.delta
    }

    fn iterate(&mut self, expr: &F2x1) -> Option<Matrix2x1f> {
        let next = self.next(expr, &self.x);
        match next {
            Some(x) => self.x = x,
            None => {}
        }
        next
    }

    fn next(&self, expr: &F2x1, x: &Matrix2x1f) -> Option<Matrix2x1f> {
        let g = grad(expr, x, self.delta);
        let h = hessian(expr, x, self.delta);
        match h.try_inverse() {
            Some(ih) => {
                let p: Matrix2x1f = -ih * g;
                let alpha = self.backtrack(expr, x, &p, 0.95, 0.8);
                Some(x + p * alpha)
            }
            None => None,
        }
    }
}

impl Optimizer for QuasiNewtonOptimizer {
    fn new(x: &Matrix2x1f, delta: f64) -> Self {
        QuasiNewtonOptimizer {
            x: *x,
            b: Matrix2x2f::new(1.0, 0.0, 0.0, 1.0),
            delta,
        }
    }

    fn init(&mut self, x: &Matrix2x1f) {
        self.x = *x;
        self.b = Matrix2x2f::new(1.0, 0.0, 0.0, 1.0);
    }

    fn delta(&self) -> f64 {
        self.delta
    }

    fn iterate(&mut self, expr: &F2x1) -> Option<Matrix2x1f> {
        let prev = self.x;
        let next = self.next(expr, &self.x);

        match next {
            Some(x) => {
                // s = f(x') - f(x)
                let s = x - prev;

                // y = ⊿f(x') - ⊿f(x)
                let y = grad(expr, &x, self.delta) - grad(expr, &prev, self.delta);

                // SR1
                /*
                let tmp = y - self.b * s;
                let tmp_t = tmp.transpose();
                let denom = (tmp_t * s).get(0).unwrap().clone();
                self.b = self.b + (tmp * tmp_t) / denom;
                */

                // BFGS
                let s_t = s.transpose();
                let y_t = y.transpose();
                self.b = self.b - (self.b * s * s_t * self.b) / *(s_t * self.b * s).get(0).unwrap()
                    + (y * y_t) / *(y_t * s).get(0).unwrap();
                self.x = x;
                Some(x)
            }
            None => None,
        }
    }

    fn next(&self, expr: &F2x1, x: &Matrix2x1f) -> Option<Matrix2x1f> {
        let g = grad(expr, &self.x, self.delta);
        let p = -self.b * g;
        let alpha = self.backtrack(expr, x, &p, 0.95, 0.8);
        Some(x + p * alpha)
    }
}
