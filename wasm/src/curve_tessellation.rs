
use nalgebra::{ Point3, Vector3 };
use crate::frenet_frame::{ FrenetFrame };
use wasm_bindgen::prelude::*;

type P = f64;

#[wasm_bindgen]
pub struct CurveTessellation {
  points: Vec<Point3<P>>,
  tangents: Vec<Vector3<P>>,
  normals: Vec<Vector3<P>>,
  binormals: Vec<Vector3<P>>,
  curvatures: Vec<P>,
}

impl CurveTessellation {
  pub fn new (frames: Vec<FrenetFrame>, curvatures: Vec<f64>) -> Self {
    let points: Vec<Point3<P>> = frames.iter().map(|f| {
      f.position
      // Point3::new(p.x as f32, p.y as f32, p.z as f32)
    }).collect();
    let tangents: Vec<Vector3<P>> = frames.iter().map(|f| {
      f.tangent
    }).collect();
    let normals: Vec<Vector3<P>> = frames.iter().map(|f| {
      f.normal
    }).collect();
    let binormals : Vec<Vector3<P>> = frames.iter().map(|f| {
      f.binormal
    }).collect();

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

  pub fn count (&self) -> usize {
    self.points.len()
  }

  pub fn stride (&self) -> usize {
    3
  }

  pub fn points(&self) -> *const Point3<P> {
    return self.points.as_ptr();
  }

  pub fn tangents(&self) -> *const Vector3<P> {
    return self.tangents.as_ptr();
  }

  pub fn normals(&self) -> *const Vector3<P> {
    return self.normals.as_ptr();
  }

  pub fn binormals(&self) -> *const Vector3<P> {
    return self.binormals.as_ptr();
  }

  pub fn curvatures(&self) -> *const P {
    return self.curvatures.as_ptr();
  }

}
