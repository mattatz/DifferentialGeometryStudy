mod utils;

pub mod log;
pub mod types;
pub mod domain;
pub mod plane;
pub mod frenet_frame;
pub mod optimizer;
pub mod curves;
pub mod surfaces;
pub mod curve_tessellation;
pub mod surface_tessellation;
pub mod app;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
