#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `VrLayer` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrLayer`*"]
#[js::opaque]
#[derive(Copy)]
pub struct VrLayer(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `leftBounds` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrLayer`*"]
#[js::import = "None"]
pub fn get_left_bounds(this: &VrLayer) -> Option<::js_sys::Array> { panic!("web-sys intrinsic") }
    #[doc = "Change the `leftBounds` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrLayer`*"]
#[js::import = "None = $1"]
pub fn set_left_bounds(this: &VrLayer, val: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
    #[doc = "Get the `rightBounds` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrLayer`*"]
#[js::import = "None"]
pub fn get_right_bounds(this: &VrLayer) -> Option<::js_sys::Array> { panic!("web-sys intrinsic") }
    #[doc = "Change the `rightBounds` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrLayer`*"]
#[js::import = "None = $1"]
pub fn set_right_bounds(this: &VrLayer, val: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
    #[cfg(feature = "HtmlCanvasElement")]
    #[doc = "Get the `source` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlCanvasElement`, `VrLayer`*"]
#[js::import = "None"]
pub fn get_source(this: &VrLayer) -> Option<HtmlCanvasElement> { panic!("web-sys intrinsic") }
    #[cfg(feature = "HtmlCanvasElement")]
    #[doc = "Change the `source` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlCanvasElement`, `VrLayer`*"]
#[js::import = "None = $1"]
pub fn set_source(this: &VrLayer, val: Option<&HtmlCanvasElement>) { panic!("web-sys intrinsic") }
impl VrLayer {
    #[doc = "Construct a new `VrLayer`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrLayer`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_left_bounds()` instead."]
#[js::import = "None"]
pub fn left_bounds(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self { self.set_left_bounds(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_right_bounds()` instead."]
#[js::import = "None"]
pub fn right_bounds(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self { self.set_right_bounds(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(feature = "HtmlCanvasElement")]
    #[deprecated = "Use `set_source()` instead."]
#[js::import = "None"]
pub fn source(&mut self, val: Option<&HtmlCanvasElement>) -> &mut Self { self.set_source(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for VrLayer {
    fn default() -> Self {
        Self::new()
    }
}
