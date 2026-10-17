#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaKeyNeededEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeyNeededEventInit`*"]
#[js::opaque]
#[derive(Copy)]
pub struct MediaKeyNeededEventInit(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeyNeededEventInit`*"]
#[js::import = "None"]
pub fn get_bubbles(this: &MediaKeyNeededEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeyNeededEventInit`*"]
#[js::import = "None = $1"]
pub fn set_bubbles(this: &MediaKeyNeededEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeyNeededEventInit`*"]
#[js::import = "None"]
pub fn get_cancelable(this: &MediaKeyNeededEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeyNeededEventInit`*"]
#[js::import = "None = $1"]
pub fn set_cancelable(this: &MediaKeyNeededEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeyNeededEventInit`*"]
#[js::import = "None"]
pub fn get_composed(this: &MediaKeyNeededEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeyNeededEventInit`*"]
#[js::import = "None = $1"]
pub fn set_composed(this: &MediaKeyNeededEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `initData` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeyNeededEventInit`*"]
#[js::import = "None"]
pub fn get_init_data(this: &MediaKeyNeededEventInit) -> Option<::js_sys::ArrayBuffer> { panic!("web-sys intrinsic") }
    #[doc = "Change the `initData` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeyNeededEventInit`*"]
#[js::import = "None = $1"]
pub fn set_init_data(this: &MediaKeyNeededEventInit, val: Option<&::js_sys::ArrayBuffer>) { panic!("web-sys intrinsic") }
    #[doc = "Get the `initDataType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeyNeededEventInit`*"]
#[js::import = "None"]
pub fn get_init_data_type(this: &MediaKeyNeededEventInit) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `initDataType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeyNeededEventInit`*"]
#[js::import = "None = $1"]
pub fn set_init_data_type(this: &MediaKeyNeededEventInit, val: &str) { panic!("web-sys intrinsic") }
impl MediaKeyNeededEventInit {
    #[doc = "Construct a new `MediaKeyNeededEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeyNeededEventInit`*"]
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
    #[deprecated = "Use `set_init_data()` instead."]
#[js::import = "None"]
pub fn init_data(&mut self, val: Option<&::js_sys::ArrayBuffer>) -> &mut Self { self.set_init_data(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_init_data_type()` instead."]
#[js::import = "None"]
pub fn init_data_type(&mut self, val: &str) -> &mut Self { self.set_init_data_type(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for MediaKeyNeededEventInit {
    fn default() -> Self {
        Self::new()
    }
}
