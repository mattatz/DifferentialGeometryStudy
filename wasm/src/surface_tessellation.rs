
use nalgebra::{ Point3, Vector3 };
use wasm_bindgen::prelude::*;

type P = f64;

#[wasm_bindgen]
pub struct SurfaceTessellation {
  rows: usize,
  columns: usize,
  points: Vec<Point3<P>>,
  normals: Vec<Vector3<P>>,
  gauss_curvature: Vec<f64>,
  mean_curvature: Vec<f64>,
  indices: Vec<usize>,
}

impl SurfaceTessellation {

  pub fn new (
    rows: usize, columns: usize,
    points: Vec<Point3<P>>, normals: Vec<Vector3<P>>, gauss_curvature: Vec<f64>, mean_curvature: Vec<f64>, indices: Vec<usize>) -> Self {
    Self {
      rows,
      columns,
      points,
      normals,
      gauss_curvature,
      mean_curvature,
      indices
    }
  }

}

#[wasm_bindgen]
impl SurfaceTessellation {

  pub fn rows (&self) -> usize {
    self.rows
  }

  pub fn columns (&self) -> usize {
    self.columns
  }

  pub fn points_count (&self) -> usize {
    self.points.len()
  }

  pub fn indices_count (&self) -> usize {
    self.indices.len()
  }

  pub fn stride (&self) -> usize {
    3
  }

  pub fn points(&self) -> *const Point3<P> {
    return self.points.as_ptr();
  }

  pub fn normals(&self) -> *const Vector3<P> {
    return self.normals.as_ptr();
  }

  pub fn gauss_curvature(&self) -> *const f64 {
    return self.gauss_curvature.as_ptr();
  }

  pub fn mean_curvature(&self) -> *const f64 {
    return self.mean_curvature.as_ptr();
  }

  pub fn indices(&self) -> *const usize {
    return self.indices.as_ptr();
  }

}
