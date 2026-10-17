#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AttributeNameValue` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AttributeNameValue`*"]
#[js::opaque]
#[derive(Copy)]
pub struct AttributeNameValue(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AttributeNameValue`*"]
#[js::import = "None"]
pub fn get_name(this: &AttributeNameValue) -> ::alloc::string::String { panic!("web-sys intrinsic") }
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AttributeNameValue`*"]
#[js::import = "None = $1"]
pub fn set_name(this: &AttributeNameValue, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `value` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AttributeNameValue`*"]
#[js::import = "None"]
pub fn get_value(this: &AttributeNameValue) -> ::alloc::string::String { panic!("web-sys intrinsic") }
    #[doc = "Change the `value` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AttributeNameValue`*"]
#[js::import = "None = $1"]
pub fn set_value(this: &AttributeNameValue, val: &str) { panic!("web-sys intrinsic") }
impl AttributeNameValue {
    #[doc = "Construct a new `AttributeNameValue`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AttributeNameValue`*"]
#[js::import = "None"]
pub fn new(name: &str, value: &str) -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret.set_name(name);
        ret.set_value(value);
        ret
    }
    #[deprecated = "Use `set_name()` instead."]
#[js::import = "None"]
pub fn name(&mut self, val: &str) -> &mut Self { self.set_name(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_value()` instead."]
#[js::import = "None"]
pub fn value(&mut self, val: &str) -> &mut Self { self.set_value(val) { panic!("web-sys intrinsic") }
        self
    }
}
