
use crate::surfaces::surface_curvature::SurfaceCurvature;
use crate::log::log;
use crate::surface_tessellation::SurfaceTessellation;
use crate::plane::Plane;
use crate::frenet_frame::FrenetFrame;
use crate::types::Vector3f;
use crate::types::Point3f;

pub trait Surface {

  fn area (&self) -> f64 {
    todo!()
  }

  fn point_at(&self, u: f64, v: f64) -> Point3f;

  fn tangent_plane_at(&self, u: f64, v: f64) -> Plane {
    let o = self.point_at(u, v);
    let (x, y) = self.derivatives_at(u, v);
    let n = x.cross(&y);
    Plane::new(o, x, y, n)
  }

  fn derivatives_at(&self, u: f64, v: f64) -> (Vector3f, Vector3f) {
    let x = self.du_at(u, v).normalize();
    let y = self.dv_at(u, v).normalize();
    (x, y)
  }

  fn normal_at(&self, u: f64, v: f64) -> Vector3f {
    let (x, y) = self.derivatives_at(u, v);
    x.cross(&y)
  }

  fn du_at(&self, u: f64, v: f64) -> Vector3f;
  fn dv_at(&self, u: f64, v: f64) -> Vector3f;
  fn dudu_at(&self, u: f64, v: f64) -> Vector3f;
  fn dudv_at(&self, u: f64, v: f64) -> Vector3f;
  fn dvdv_at(&self, u: f64, v: f64) -> Vector3f;

  // https://mathworld.wolfram.com/GaussianCurvature.html
  fn curvature_at(&self, u: f64, v: f64) -> SurfaceCurvature {
    let normal = self.normal_at(u, v);
    let du = self.du_at(u, v);
    let dv = self.dv_at(u, v);
    let l = self.dudu_at(u, v).dot(&normal); // dudu * e
    let m = self.dudv_at(u, v).dot(&normal); // dudv * e
    let n = self.dvdv_at(u, v).dot(&normal); // dvdv * e
    let e = du.dot(&du); // du * du
    let f = du.dot(&dv); // du * dv
    let g = dv.dot(&dv); // dv * dv

    // k1k2 = (LN - MM) / (EG - FF)
    // (k1 + k1) / 2 = (EN + GL - 2FM) / 2(EG - FF)
    SurfaceCurvature {
      point: self.point_at(u, v),
      uv: (u, v),
      normal: normal,
      gaussian: (l * n - m * m) / (e * g - f * f),
      mean: (e * n + g * l - 2.0 * f * m) / 2.0 * (e * g - f * f),
    }
  }

  // https://mathworld.wolfram.com/AreaElement.html
  fn area_element_at(&self, u: f64, v: f64, delta_u: f64, delta_v: f64) -> f64 {
    let du = self.du_at(u, v);
    let dv = self.dv_at(u, v);
    let e = du.dot(&du); // du * du
    let f = du.dot(&dv); // du * dv
    let g = dv.dot(&dv); // dv * dv
    // √(EG - FF)dudv
    (e * g - f * f).sqrt() * delta_u * delta_v
  }

  fn tessellate(&self, delta: Option<f64>) -> SurfaceTessellation { 
    let delta = delta.unwrap_or(1e-4);
    let count: usize = (1.0 / delta) as usize;
    let parameters: Vec<f64> = (0..count).into_iter().map(|i| {
      let s = i as f64 / (count - 1) as f64;
      // log(&format!("{:?}", s));
      s
    }).collect();
    let mut points: Vec<Point3f> = vec![];
    let mut normals: Vec<Vector3f> = vec![];
    let mut gauss_curvature: Vec<f64> = vec![];
    let mut mean_curvature: Vec<f64> = vec![];
    let mut indices: Vec<usize> = vec![];
    for iu in 0..count {
      let i = iu * count;
      let u = parameters.get(iu).unwrap().clone();
      for iv in 0..count {
        let j = i + iv;
        let v = parameters.get(iv).unwrap().clone();
        let curvature = self.curvature_at(u, v);
        // let point = self.point_at(u, v);
        // let normal = self.normal_at(u, v);
        points.push(curvature.point);
        normals.push(curvature.normal);
        gauss_curvature.push(curvature.gaussian);
        mean_curvature.push(curvature.mean);
        if iu < count - 1 {
          if iv < count - 1 {
            let k = j + count; // next row
            indices.push(j);
            indices.push(j + 1);
            indices.push(k);

            indices.push(k + 1);
            indices.push(k);
            indices.push(j + 1);
          }
        }
      }
    }

    SurfaceTessellation::new(
      count, count,
      points, normals, gauss_curvature, mean_curvature, indices
    )
  }

}
