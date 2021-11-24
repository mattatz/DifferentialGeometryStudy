
use crate::types::Matrix2x1f;
use crate::types::Matrix2x2f;

pub type F = dyn Fn(f64) -> f64;
pub type F2x1 = dyn Fn(&Matrix2x1f) -> f64;

pub fn df(f: &F, n: i32, x: f64, delta: f64) -> f64 {
    if n <= 0 {
        return f(x);
    } else {
        return (
            df(f, n - 1, x + delta, delta)
            -
            df(f, n - 1, x - delta, delta)
        ) / (delta * 2.0);
    }
}

pub fn df2x1(f: &F2x1, n: i32, x: &Matrix2x1f, delta: &Matrix2x1f) -> f64 {
    if n <= 0 {
        return f(&x);
    } else {
        let norm = delta.norm();
        return (
            df2x1(f, n - 1, &(x + delta), &delta)
            -
            df2x1(f, n - 1, &(x - delta), &delta)
        ) / (norm * 2.0);
    }
}

pub fn grad(f: &F2x1, x: &Matrix2x1f, delta: f64) -> Matrix2x1f {
    Matrix2x1f::new(
        df2x1(f, 1, &x, &Matrix2x1f::new(delta, 0.0)),
        df2x1(f, 1, &x, &Matrix2x1f::new(0.0, delta))
    )
}

pub fn hessian(f: &F2x1, x: &Matrix2x1f, delta: f64) -> Matrix2x2f {
    let xs = Matrix2x1f::new(delta, 0.0);
    let ys = Matrix2x1f::new(0.0, delta);

    let h = delta * 2.0;
    let dxx = df2x1(f, 2, &x, &xs);
    let dxy = (df2x1(f, 1, &(x + ys), &xs) - df2x1(f, 1, &(x - ys), &xs)) / h;
    let dyx = (df2x1(f, 1, &(x + xs), &ys) - df2x1(f, 1, &(x - xs), &ys)) / h;
    let dyy = df2x1(f, 2, &x, &ys);

    Matrix2x2f::new(
        dxx, dxy, dyx, dyy
    )
}
