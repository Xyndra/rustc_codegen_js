#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `UnderlyingSink` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UnderlyingSink`*"]
#[js::opaque]
#[derive(Copy)]
pub struct UnderlyingSink(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `abort` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UnderlyingSink`*"]
#[js::import = "None"]
pub fn get_abort(this: &UnderlyingSink) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
    #[doc = "Change the `abort` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UnderlyingSink`*"]
#[js::import = "None = $1"]
pub fn set_abort(this: &UnderlyingSink, val: &::js_sys::Function) { panic!("web-sys intrinsic") }
    #[doc = "Get the `close` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UnderlyingSink`*"]
#[js::import = "None"]
pub fn get_close(this: &UnderlyingSink) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
    #[doc = "Change the `close` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UnderlyingSink`*"]
#[js::import = "None = $1"]
pub fn set_close(this: &UnderlyingSink, val: &::js_sys::Function) { panic!("web-sys intrinsic") }
    #[doc = "Get the `start` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UnderlyingSink`*"]
#[js::import = "None"]
pub fn get_start(this: &UnderlyingSink) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
    #[doc = "Change the `start` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UnderlyingSink`*"]
#[js::import = "None = $1"]
pub fn set_start(this: &UnderlyingSink, val: &::js_sys::Function) { panic!("web-sys intrinsic") }
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UnderlyingSink`*"]
#[js::import = "None"]
pub fn get_type(this: &UnderlyingSink) -> ::wasm_bindgen::JsValue { panic!("web-sys intrinsic") }
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UnderlyingSink`*"]
#[js::import = "None = $1"]
pub fn set_type(this: &UnderlyingSink, val: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
    #[doc = "Get the `write` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UnderlyingSink`*"]
#[js::import = "None"]
pub fn get_write(this: &UnderlyingSink) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
    #[doc = "Change the `write` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UnderlyingSink`*"]
#[js::import = "None = $1"]
pub fn set_write(this: &UnderlyingSink, val: &::js_sys::Function) { panic!("web-sys intrinsic") }
impl UnderlyingSink {
    #[doc = "Construct a new `UnderlyingSink`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UnderlyingSink`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_abort()` instead."]
#[js::import = "None"]
pub fn abort(&mut self, val: &::js_sys::Function) -> &mut Self { self.set_abort(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_close()` instead."]
#[js::import = "None"]
pub fn close(&mut self, val: &::js_sys::Function) -> &mut Self { self.set_close(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_start()` instead."]
#[js::import = "None"]
pub fn start(&mut self, val: &::js_sys::Function) -> &mut Self { self.set_start(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_type()` instead."]
#[js::import = "None"]
pub fn type_(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self { self.set_type(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_write()` instead."]
#[js::import = "None"]
pub fn write(&mut self, val: &::js_sys::Function) -> &mut Self { self.set_write(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for UnderlyingSink {
    fn default() -> Self {
        Self::new()
    }
}
