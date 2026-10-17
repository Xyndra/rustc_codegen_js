#![no_std]
#![feature(register_tool)]
#![register_tool(js)]
#![allow(dead_code)]

extern crate alloc;

pub use wasm_bindgen::JsValue;
pub type Object = JsValue;
pub type Array = JsValue;
pub type Function = JsValue;

#[js::import = "globalThis"]
pub fn global() -> JsValue {
    panic!("js intrinsic")
}

pub type Promise = JsValue;
