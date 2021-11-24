
use web_sys::console::log_1;
use wasm_bindgen::prelude::*;

pub fn log(s: &String) {
    log_1(&JsValue::from(s));
}
