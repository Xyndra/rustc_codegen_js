#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaStreamEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamEventInit`*"]
#[js::opaque]
#[derive(Copy)]
pub struct MediaStreamEventInit(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamEventInit`*"]
#[js::import = "None"]
pub fn get_bubbles(this: &MediaStreamEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamEventInit`*"]
#[js::import = "None = $1"]
pub fn set_bubbles(this: &MediaStreamEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamEventInit`*"]
#[js::import = "None"]
pub fn get_cancelable(this: &MediaStreamEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamEventInit`*"]
#[js::import = "None = $1"]
pub fn set_cancelable(this: &MediaStreamEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamEventInit`*"]
#[js::import = "None"]
pub fn get_composed(this: &MediaStreamEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamEventInit`*"]
#[js::import = "None = $1"]
pub fn set_composed(this: &MediaStreamEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[cfg(feature = "MediaStream")]
    #[doc = "Get the `stream` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStream`, `MediaStreamEventInit`*"]
#[js::import = "None"]
pub fn get_stream(this: &MediaStreamEventInit) -> Option<MediaStream> { panic!("web-sys intrinsic") }
    #[cfg(feature = "MediaStream")]
    #[doc = "Change the `stream` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStream`, `MediaStreamEventInit`*"]
#[js::import = "None = $1"]
pub fn set_stream(this: &MediaStreamEventInit, val: Option<&MediaStream>) { panic!("web-sys intrinsic") }
impl MediaStreamEventInit {
    #[doc = "Construct a new `MediaStreamEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamEventInit`*"]
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
    #[cfg(feature = "MediaStream")]
    #[deprecated = "Use `set_stream()` instead."]
#[js::import = "None"]
pub fn stream(&mut self, val: Option<&MediaStream>) -> &mut Self { self.set_stream(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for MediaStreamEventInit {
    fn default() -> Self {
        Self::new()
    }
}
