
use crate::types::{ Point3f, Vector3f };

pub struct FrenetFrame {
  pub position: Point3f,
  pub tangent: Vector3f,
  pub normal: Vector3f,
  pub binormal: Vector3f,
}

impl FrenetFrame {

  pub fn new (p: &Point3f, t: &Vector3f, n: &Vector3f, b: &Vector3f) -> FrenetFrame {
    FrenetFrame {
      position: p.clone(),
      tangent: t.clone(),
      normal: n.clone(),
      binormal: b.clone(),
    }
  }

}
