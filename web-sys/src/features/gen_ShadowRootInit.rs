#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ShadowRootInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ShadowRootInit`*"]
#[js::opaque]
#[derive(Copy)]
pub struct ShadowRootInit(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "ShadowRootMode")]
    #[doc = "Get the `mode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ShadowRootInit`, `ShadowRootMode`*"]
#[js::import = "None"]
pub fn get_mode(this: &ShadowRootInit) -> ShadowRootMode { panic!("web-sys intrinsic") }
    #[cfg(feature = "ShadowRootMode")]
    #[doc = "Change the `mode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ShadowRootInit`, `ShadowRootMode`*"]
#[js::import = "None = $1"]
pub fn set_mode(this: &ShadowRootInit, val: ShadowRootMode) { panic!("web-sys intrinsic") }
impl ShadowRootInit {
    #[cfg(feature = "ShadowRootMode")]
    #[doc = "Construct a new `ShadowRootInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ShadowRootInit`, `ShadowRootMode`*"]
#[js::import = "None"]
pub fn new(mode: ShadowRootMode) -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret.set_mode(mode);
        ret
    }
    #[cfg(feature = "ShadowRootMode")]
    #[deprecated = "Use `set_mode()` instead."]
#[js::import = "None"]
pub fn mode(&mut self, val: ShadowRootMode) -> &mut Self { self.set_mode(val) { panic!("web-sys intrinsic") }
        self
    }
}
