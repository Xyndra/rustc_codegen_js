#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HashChangeEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HashChangeEventInit`*"]
#[js::opaque]
#[derive(Copy)]
pub struct HashChangeEventInit(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HashChangeEventInit`*"]
#[js::import = "None"]
pub fn get_bubbles(this: &HashChangeEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HashChangeEventInit`*"]
#[js::import = "None = $1"]
pub fn set_bubbles(this: &HashChangeEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HashChangeEventInit`*"]
#[js::import = "None"]
pub fn get_cancelable(this: &HashChangeEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HashChangeEventInit`*"]
#[js::import = "None = $1"]
pub fn set_cancelable(this: &HashChangeEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HashChangeEventInit`*"]
#[js::import = "None"]
pub fn get_composed(this: &HashChangeEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HashChangeEventInit`*"]
#[js::import = "None = $1"]
pub fn set_composed(this: &HashChangeEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `newURL` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HashChangeEventInit`*"]
#[js::import = "None"]
pub fn get_new_url(this: &HashChangeEventInit) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `newURL` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HashChangeEventInit`*"]
#[js::import = "None = $1"]
pub fn set_new_url(this: &HashChangeEventInit, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `oldURL` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HashChangeEventInit`*"]
#[js::import = "None"]
pub fn get_old_url(this: &HashChangeEventInit) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `oldURL` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HashChangeEventInit`*"]
#[js::import = "None = $1"]
pub fn set_old_url(this: &HashChangeEventInit, val: &str) { panic!("web-sys intrinsic") }
impl HashChangeEventInit {
    #[doc = "Construct a new `HashChangeEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HashChangeEventInit`*"]
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
    #[deprecated = "Use `set_new_url()` instead."]
#[js::import = "None"]
pub fn new_url(&mut self, val: &str) -> &mut Self { self.set_new_url(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_old_url()` instead."]
#[js::import = "None"]
pub fn old_url(&mut self, val: &str) -> &mut Self { self.set_old_url(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for HashChangeEventInit {
    fn default() -> Self {
        Self::new()
    }
}
