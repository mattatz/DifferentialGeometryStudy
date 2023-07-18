use crate::surface_tessellation::SurfaceTessellation;
use crate::{
    curves::curve::Curve,
    surfaces::{
        cylinder::Cylinder, elliptic_paraboloid::EllipticParaboloid, helicoid::Helicoid,
        hyperbolic_paraboloid::HyperbolicParaboloid, mobius::Mobius,
        single_leaf_paraboloid::SingleLeafParaboloid, sphere::Sphere, surface::Surface,
        torus::Torus,
    },
};
use wasm_bindgen::prelude::*;

pub enum CurveType {}

#[wasm_bindgen]
pub enum SurfaceType {
    Sphere,
    Cylinder,
    Torus,
    Mobius,
    Helicoid,
    EllipticParaboloid,
    HyperbolicParaboloid,
    SingleLeafParaboloid,
}

#[wasm_bindgen]
pub struct App {}

#[wasm_bindgen]
impl App {
    pub fn new() -> App {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        App {}
    }

    pub fn create_surface(
        &self,
        t: SurfaceType,
        delta: Option<f64>,
    ) -> Option<SurfaceTessellation> {
        match t {
            SurfaceType::Sphere => Some(Sphere::default().tessellate(delta)),
            SurfaceType::Cylinder => Some(Cylinder::default().tessellate(delta)),
            SurfaceType::Torus => Some(Torus::default().tessellate(delta)),
            SurfaceType::Mobius => Some(Mobius::default().tessellate(delta)),
            SurfaceType::Helicoid => Some(Helicoid::default().tessellate(delta)),
            SurfaceType::EllipticParaboloid => {
                Some(EllipticParaboloid::default().tessellate(delta))
            }
            SurfaceType::HyperbolicParaboloid => {
                Some(HyperbolicParaboloid::default().tessellate(delta))
            }
            SurfaceType::SingleLeafParaboloid => {
                Some(SingleLeafParaboloid::default().tessellate(delta))
            }
            _ => None,
        }
    }
}
