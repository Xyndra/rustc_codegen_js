#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AngleInstancedArrays` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ANGLE_instanced_arrays)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AngleInstancedArrays`*"]
#[js::opaque]
#[derive(Copy)]
pub struct AngleInstancedArrays(pub ::wasm_bindgen::JsValue);
    #[doc = "The `drawArraysInstancedANGLE()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ANGLE_instanced_arrays/drawArraysInstancedANGLE)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AngleInstancedArrays`*"]
impl AngleInstancedArrays {
    #[js::import = "$0.drawArraysInstancedANGLE($1, $2, $3, $4)"]
pub fn draw_arrays_instanced_angle(&self, mode: u32, first: i32, count: i32, primcount: i32) { panic!("web-sys intrinsic") }
}
    #[doc = "The `drawElementsInstancedANGLE()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ANGLE_instanced_arrays/drawElementsInstancedANGLE)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AngleInstancedArrays`*"]
impl AngleInstancedArrays {
    #[js::import = "$0.drawElementsInstancedANGLE($1, $2, $3, $4, $5)"]
pub fn draw_elements_instanced_angle_with_i32(&self, mode: u32, count: i32, type_: u32, offset: i32, primcount: i32) { panic!("web-sys intrinsic") }
}
    #[doc = "The `drawElementsInstancedANGLE()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ANGLE_instanced_arrays/drawElementsInstancedANGLE)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AngleInstancedArrays`*"]
impl AngleInstancedArrays {
    #[js::import = "$0.drawElementsInstancedANGLE($1, $2, $3, $4, $5)"]
pub fn draw_elements_instanced_angle_with_f64(&self, mode: u32, count: i32, type_: u32, offset: f64, primcount: i32) { panic!("web-sys intrinsic") }
}
    #[doc = "The `vertexAttribDivisorANGLE()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ANGLE_instanced_arrays/vertexAttribDivisorANGLE)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AngleInstancedArrays`*"]
impl AngleInstancedArrays {
    #[js::import = "$0.vertexAttribDivisorANGLE($1, $2)"]
pub fn vertex_attrib_divisor_angle(&self, index: u32, divisor: u32) { panic!("web-sys intrinsic") }
}
impl AngleInstancedArrays {
    #[doc = "The `ANGLE_instanced_arrays.VERTEX_ATTRIB_ARRAY_DIVISOR_ANGLE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AngleInstancedArrays`*"]
    pub const VERTEX_ATTRIB_ARRAY_DIVISOR_ANGLE: u32 = 35070u64 as u32;
}
