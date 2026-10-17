#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ResizeObserverOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ResizeObserverOptions`*"]
#[js::opaque]
#[derive(Copy)]
pub struct ResizeObserverOptions(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "ResizeObserverBoxOptions")]
    #[doc = "Get the `box` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ResizeObserverBoxOptions`, `ResizeObserverOptions`*"]
#[js::import = "None"]
pub fn get_box(this: &ResizeObserverOptions) -> Option<ResizeObserverBoxOptions> { panic!("web-sys intrinsic") }
    #[cfg(feature = "ResizeObserverBoxOptions")]
    #[doc = "Change the `box` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ResizeObserverBoxOptions`, `ResizeObserverOptions`*"]
#[js::import = "None = $1"]
pub fn set_box(this: &ResizeObserverOptions, val: ResizeObserverBoxOptions) { panic!("web-sys intrinsic") }
impl ResizeObserverOptions {
    #[doc = "Construct a new `ResizeObserverOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ResizeObserverOptions`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[cfg(feature = "ResizeObserverBoxOptions")]
    #[deprecated = "Use `set_box()` instead."]
#[js::import = "None"]
pub fn box_(&mut self, val: ResizeObserverBoxOptions) -> &mut Self { self.set_box(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for ResizeObserverOptions {
    fn default() -> Self {
        Self::new()
    }
}
