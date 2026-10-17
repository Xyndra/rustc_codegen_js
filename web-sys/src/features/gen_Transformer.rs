#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Transformer` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Transformer`*"]
#[js::opaque]
#[derive(Copy)]
pub struct Transformer(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `flush` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Transformer`*"]
#[js::import = "None"]
pub fn get_flush(this: &Transformer) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
    #[doc = "Change the `flush` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Transformer`*"]
#[js::import = "None = $1"]
pub fn set_flush(this: &Transformer, val: &::js_sys::Function) { panic!("web-sys intrinsic") }
    #[doc = "Get the `readableType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Transformer`*"]
#[js::import = "None"]
pub fn get_readable_type(this: &Transformer) -> ::wasm_bindgen::JsValue { panic!("web-sys intrinsic") }
    #[doc = "Change the `readableType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Transformer`*"]
#[js::import = "None = $1"]
pub fn set_readable_type(this: &Transformer, val: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
    #[doc = "Get the `start` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Transformer`*"]
#[js::import = "None"]
pub fn get_start(this: &Transformer) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
    #[doc = "Change the `start` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Transformer`*"]
#[js::import = "None = $1"]
pub fn set_start(this: &Transformer, val: &::js_sys::Function) { panic!("web-sys intrinsic") }
    #[doc = "Get the `transform` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Transformer`*"]
#[js::import = "None"]
pub fn get_transform(this: &Transformer) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
    #[doc = "Change the `transform` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Transformer`*"]
#[js::import = "None = $1"]
pub fn set_transform(this: &Transformer, val: &::js_sys::Function) { panic!("web-sys intrinsic") }
    #[doc = "Get the `writableType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Transformer`*"]
#[js::import = "None"]
pub fn get_writable_type(this: &Transformer) -> ::wasm_bindgen::JsValue { panic!("web-sys intrinsic") }
    #[doc = "Change the `writableType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Transformer`*"]
#[js::import = "None = $1"]
pub fn set_writable_type(this: &Transformer, val: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
impl Transformer {
    #[doc = "Construct a new `Transformer`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Transformer`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_flush()` instead."]
#[js::import = "None"]
pub fn flush(&mut self, val: &::js_sys::Function) -> &mut Self { self.set_flush(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_readable_type()` instead."]
#[js::import = "None"]
pub fn readable_type(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self { self.set_readable_type(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_start()` instead."]
#[js::import = "None"]
pub fn start(&mut self, val: &::js_sys::Function) -> &mut Self { self.set_start(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_transform()` instead."]
#[js::import = "None"]
pub fn transform(&mut self, val: &::js_sys::Function) -> &mut Self { self.set_transform(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_writable_type()` instead."]
#[js::import = "None"]
pub fn writable_type(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self { self.set_writable_type(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for Transformer {
    fn default() -> Self {
        Self::new()
    }
}
