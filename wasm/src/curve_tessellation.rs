use crate::frenet_frame::FrenetFrame;
use nalgebra::{Point3, Vector3};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct CurveTessellation {
    points: Vec<Point3<f32>>,
    tangents: Vec<Vector3<f32>>,
    normals: Vec<Vector3<f32>>,
    binormals: Vec<Vector3<f32>>,
    curvatures: Vec<f32>,
}

impl CurveTessellation {
    pub fn new(frames: Vec<FrenetFrame>, curvatures: Vec<f32>) -> Self {
        let points: Vec<Point3<f32>> = frames
            .iter()
            .map(|f| {
                f.position.cast()
                // Point3::new(p.x as f32, p.y as f32, p.z as f32)
            })
            .collect();
        let tangents: Vec<Vector3<f32>> = frames.iter().map(|f| f.tangent.cast()).collect();
        let normals: Vec<Vector3<f32>> = frames.iter().map(|f| f.normal.cast()).collect();
        let binormals: Vec<Vector3<f32>> = frames.iter().map(|f| f.binormal.cast()).collect();

        Self {
            points,
            tangents,
            normals,
            binormals,
            curvatures,
        }
    }
}

#[wasm_bindgen]
impl CurveTessellation {
    pub fn count(&self) -> usize {
        self.points.len()
    }

    pub fn stride(&self) -> usize {
        3
    }

    pub fn points(&self) -> *const Point3<f32> {
        self.points.as_ptr()
    }

    pub fn tangents(&self) -> *const Vector3<f32> {
        self.tangents.as_ptr()
    }

    pub fn normals(&self) -> *const Vector3<f32> {
        self.normals.as_ptr()
    }

    pub fn binormals(&self) -> *const Vector3<f32> {
        self.binormals.as_ptr()
    }

    pub fn curvatures(&self) -> *const f32 {
        self.curvatures.as_ptr()
    }
}
