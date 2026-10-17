#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `OesVertexArrayObject` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OES_vertex_array_object)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OesVertexArrayObject`*"]
#[js::opaque]
#[derive(Copy)]
pub struct OesVertexArrayObject(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "WebGlVertexArrayObject")]
    #[doc = "The `bindVertexArrayOES()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OES_vertex_array_object/bindVertexArrayOES)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OesVertexArrayObject`, `WebGlVertexArrayObject`*"]
impl OesVertexArrayObject {
    #[js::import = "$0.bindVertexArrayOES($1)"]
pub fn bind_vertex_array_oes(&self, array_object: Option<&WebGlVertexArrayObject>) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "WebGlVertexArrayObject")]
    #[doc = "The `createVertexArrayOES()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OES_vertex_array_object/createVertexArrayOES)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OesVertexArrayObject`, `WebGlVertexArrayObject`*"]
impl OesVertexArrayObject {
    #[js::import = "$0.createVertexArrayOES()"]
pub fn create_vertex_array_oes(&self) -> Option<WebGlVertexArrayObject> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "WebGlVertexArrayObject")]
    #[doc = "The `deleteVertexArrayOES()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OES_vertex_array_object/deleteVertexArrayOES)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OesVertexArrayObject`, `WebGlVertexArrayObject`*"]
impl OesVertexArrayObject {
    #[js::import = "$0.deleteVertexArrayOES($1)"]
pub fn delete_vertex_array_oes(&self, array_object: Option<&WebGlVertexArrayObject>) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "WebGlVertexArrayObject")]
    #[doc = "The `isVertexArrayOES()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OES_vertex_array_object/isVertexArrayOES)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OesVertexArrayObject`, `WebGlVertexArrayObject`*"]
impl OesVertexArrayObject {
    #[js::import = "$0.isVertexArrayOES($1)"]
pub fn is_vertex_array_oes(&self, array_object: Option<&WebGlVertexArrayObject>) -> bool { panic!("web-sys intrinsic") }
}
impl OesVertexArrayObject {
    #[doc = "The `OES_vertex_array_object.VERTEX_ARRAY_BINDING_OES` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OesVertexArrayObject`*"]
    pub const VERTEX_ARRAY_BINDING_OES: u32 = 34229u64 as u32;
}
