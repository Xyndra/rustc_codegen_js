#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ErrorEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ErrorEventInit`*"]
#[js::opaque]
#[derive(Copy)]
pub struct ErrorEventInit(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ErrorEventInit`*"]
#[js::import = "None"]
pub fn get_bubbles(this: &ErrorEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ErrorEventInit`*"]
#[js::import = "None = $1"]
pub fn set_bubbles(this: &ErrorEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ErrorEventInit`*"]
#[js::import = "None"]
pub fn get_cancelable(this: &ErrorEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ErrorEventInit`*"]
#[js::import = "None = $1"]
pub fn set_cancelable(this: &ErrorEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ErrorEventInit`*"]
#[js::import = "None"]
pub fn get_composed(this: &ErrorEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ErrorEventInit`*"]
#[js::import = "None = $1"]
pub fn set_composed(this: &ErrorEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `colno` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ErrorEventInit`*"]
#[js::import = "None"]
pub fn get_colno(this: &ErrorEventInit) -> Option<u32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `colno` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ErrorEventInit`*"]
#[js::import = "None = $1"]
pub fn set_colno(this: &ErrorEventInit, val: u32) { panic!("web-sys intrinsic") }
    #[doc = "Get the `error` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ErrorEventInit`*"]
#[js::import = "None"]
pub fn get_error(this: &ErrorEventInit) -> ::wasm_bindgen::JsValue { panic!("web-sys intrinsic") }
    #[doc = "Change the `error` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ErrorEventInit`*"]
#[js::import = "None = $1"]
pub fn set_error(this: &ErrorEventInit, val: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
    #[doc = "Get the `filename` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ErrorEventInit`*"]
#[js::import = "None"]
pub fn get_filename(this: &ErrorEventInit) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `filename` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ErrorEventInit`*"]
#[js::import = "None = $1"]
pub fn set_filename(this: &ErrorEventInit, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `lineno` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ErrorEventInit`*"]
#[js::import = "None"]
pub fn get_lineno(this: &ErrorEventInit) -> Option<u32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `lineno` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ErrorEventInit`*"]
#[js::import = "None = $1"]
pub fn set_lineno(this: &ErrorEventInit, val: u32) { panic!("web-sys intrinsic") }
    #[doc = "Get the `message` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ErrorEventInit`*"]
#[js::import = "None"]
pub fn get_message(this: &ErrorEventInit) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `message` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ErrorEventInit`*"]
#[js::import = "None = $1"]
pub fn set_message(this: &ErrorEventInit, val: &str) { panic!("web-sys intrinsic") }
impl ErrorEventInit {
    #[doc = "Construct a new `ErrorEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ErrorEventInit`*"]
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
    #[deprecated = "Use `set_colno()` instead."]
#[js::import = "None"]
pub fn colno(&mut self, val: u32) -> &mut Self { self.set_colno(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_error()` instead."]
#[js::import = "None"]
pub fn error(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self { self.set_error(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_filename()` instead."]
#[js::import = "None"]
pub fn filename(&mut self, val: &str) -> &mut Self { self.set_filename(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_lineno()` instead."]
#[js::import = "None"]
pub fn lineno(&mut self, val: u32) -> &mut Self { self.set_lineno(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_message()` instead."]
#[js::import = "None"]
pub fn message(&mut self, val: &str) -> &mut Self { self.set_message(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for ErrorEventInit {
    fn default() -> Self {
        Self::new()
    }
}
