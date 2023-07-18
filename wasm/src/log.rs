use wasm_bindgen::prelude::*;
use web_sys::console::log_1;

pub fn log(s: &String) {
    log_1(&JsValue::from(s));
}
