use nalgebra::Point3;
use nalgebra::Scalar;
use nalgebra::Vector3;

pub struct SurfaceCurvature<T: Scalar> {
    pub point: Point3<T>,
    pub uv: (T, T),
    pub normal: Vector3<T>,
    pub gaussian: T,
    pub mean: T,
}

impl<T: Scalar> SurfaceCurvature<T> {}
