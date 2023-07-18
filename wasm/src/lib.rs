mod utils;

pub mod app;
pub mod curve_tessellation;
pub mod curves;
pub mod domain;
pub mod frenet_frame;
pub mod log;
pub mod optimizer;
pub mod plane;
pub mod surface_tessellation;
pub mod surfaces;
pub mod types;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
