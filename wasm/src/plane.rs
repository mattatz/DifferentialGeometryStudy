use crate::types::*;

pub struct Plane {
    pub origin: Point3f,
    pub normal: Vector3f,
    pub x_axis: Vector3f,
    pub y_axis: Vector3f,
}

impl Default for Plane {
    fn default() -> Plane {
        Plane {
            origin: Point3f::new(0.0, 0.0, 0.0),
            x_axis: Vector3f::new(1.0, 0.0, 0.0),
            y_axis: Vector3f::new(0.0, 1.0, 0.0),
            normal: Vector3f::new(0.0, 0.0, -1.0),
        }
    }
}

impl Plane {
    pub fn new(o: Point3f, x: Vector3f, y: Vector3f, n: Vector3f) -> Plane {
        Plane {
            origin: o,
            x_axis: x,
            y_axis: y,
            normal: n,
        }
    }
}
