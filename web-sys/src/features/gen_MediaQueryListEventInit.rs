#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaQueryListEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaQueryListEventInit`*"]
#[js::opaque]
#[derive(Copy)]
pub struct MediaQueryListEventInit(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaQueryListEventInit`*"]
#[js::import = "None"]
pub fn get_bubbles(this: &MediaQueryListEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaQueryListEventInit`*"]
#[js::import = "None = $1"]
pub fn set_bubbles(this: &MediaQueryListEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaQueryListEventInit`*"]
#[js::import = "None"]
pub fn get_cancelable(this: &MediaQueryListEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaQueryListEventInit`*"]
#[js::import = "None = $1"]
pub fn set_cancelable(this: &MediaQueryListEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaQueryListEventInit`*"]
#[js::import = "None"]
pub fn get_composed(this: &MediaQueryListEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaQueryListEventInit`*"]
#[js::import = "None = $1"]
pub fn set_composed(this: &MediaQueryListEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `matches` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaQueryListEventInit`*"]
#[js::import = "None"]
pub fn get_matches(this: &MediaQueryListEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `matches` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaQueryListEventInit`*"]
#[js::import = "None = $1"]
pub fn set_matches(this: &MediaQueryListEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `media` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaQueryListEventInit`*"]
#[js::import = "None"]
pub fn get_media(this: &MediaQueryListEventInit) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `media` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaQueryListEventInit`*"]
#[js::import = "None = $1"]
pub fn set_media(this: &MediaQueryListEventInit, val: &str) { panic!("web-sys intrinsic") }
impl MediaQueryListEventInit {
    #[doc = "Construct a new `MediaQueryListEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaQueryListEventInit`*"]
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
    #[deprecated = "Use `set_matches()` instead."]
#[js::import = "None"]
pub fn matches(&mut self, val: bool) -> &mut Self { self.set_matches(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_media()` instead."]
#[js::import = "None"]
pub fn media(&mut self, val: &str) -> &mut Self { self.set_media(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for MediaQueryListEventInit {
    fn default() -> Self {
        Self::new()
    }
}
