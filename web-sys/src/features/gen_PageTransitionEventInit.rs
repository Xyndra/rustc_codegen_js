#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PageTransitionEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PageTransitionEventInit`*"]
#[js::opaque]
#[derive(Copy)]
pub struct PageTransitionEventInit(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PageTransitionEventInit`*"]
#[js::import = "None"]
pub fn get_bubbles(this: &PageTransitionEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PageTransitionEventInit`*"]
#[js::import = "None = $1"]
pub fn set_bubbles(this: &PageTransitionEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PageTransitionEventInit`*"]
#[js::import = "None"]
pub fn get_cancelable(this: &PageTransitionEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PageTransitionEventInit`*"]
#[js::import = "None = $1"]
pub fn set_cancelable(this: &PageTransitionEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PageTransitionEventInit`*"]
#[js::import = "None"]
pub fn get_composed(this: &PageTransitionEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PageTransitionEventInit`*"]
#[js::import = "None = $1"]
pub fn set_composed(this: &PageTransitionEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `inFrameSwap` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PageTransitionEventInit`*"]
#[js::import = "None"]
pub fn get_in_frame_swap(this: &PageTransitionEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `inFrameSwap` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PageTransitionEventInit`*"]
#[js::import = "None = $1"]
pub fn set_in_frame_swap(this: &PageTransitionEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `persisted` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PageTransitionEventInit`*"]
#[js::import = "None"]
pub fn get_persisted(this: &PageTransitionEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `persisted` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PageTransitionEventInit`*"]
#[js::import = "None = $1"]
pub fn set_persisted(this: &PageTransitionEventInit, val: bool) { panic!("web-sys intrinsic") }
impl PageTransitionEventInit {
    #[doc = "Construct a new `PageTransitionEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PageTransitionEventInit`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_bubbles()` instead."]
#[js::import = "None"]
pub fn bubbles(&mut self, val: bool) -> &mut Self { self.set_bubbles(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_cancelable()` instead."]
#[js::import = "None"]
pub fn cancelable(&mut self, val: bool) -> &mut Self { self.set_cancelable(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_composed()` instead."]
#[js::import = "None"]
pub fn composed(&mut self, val: bool) -> &mut Self { self.set_composed(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_in_frame_swap()` instead."]
#[js::import = "None"]
pub fn in_frame_swap(&mut self, val: bool) -> &mut Self { self.set_in_frame_swap(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_persisted()` instead."]
#[js::import = "None"]
pub fn persisted(&mut self, val: bool) -> &mut Self { self.set_persisted(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for PageTransitionEventInit {
    fn default() -> Self {
        Self::new()
    }
}
