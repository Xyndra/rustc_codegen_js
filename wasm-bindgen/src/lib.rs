#![no_std]
#![feature(register_tool)]
#![register_tool(js)]
#![allow(dead_code)]

extern crate alloc;

pub use js_util::JsValue;

pub mod prelude {
    pub use super::JsValue;
}
