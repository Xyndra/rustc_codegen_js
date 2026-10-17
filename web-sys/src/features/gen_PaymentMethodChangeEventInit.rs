#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PaymentMethodChangeEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PaymentMethodChangeEventInit`*"]
#[js::opaque]
#[derive(Copy)]
pub struct PaymentMethodChangeEventInit(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PaymentMethodChangeEventInit`*"]
#[js::import = "None"]
pub fn get_bubbles(this: &PaymentMethodChangeEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PaymentMethodChangeEventInit`*"]
#[js::import = "None = $1"]
pub fn set_bubbles(this: &PaymentMethodChangeEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PaymentMethodChangeEventInit`*"]
#[js::import = "None"]
pub fn get_cancelable(this: &PaymentMethodChangeEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PaymentMethodChangeEventInit`*"]
#[js::import = "None = $1"]
pub fn set_cancelable(this: &PaymentMethodChangeEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PaymentMethodChangeEventInit`*"]
#[js::import = "None"]
pub fn get_composed(this: &PaymentMethodChangeEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PaymentMethodChangeEventInit`*"]
#[js::import = "None = $1"]
pub fn set_composed(this: &PaymentMethodChangeEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `methodDetails` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PaymentMethodChangeEventInit`*"]
#[js::import = "None"]
pub fn get_method_details(this: &PaymentMethodChangeEventInit) -> Option<::js_sys::Object> { panic!("web-sys intrinsic") }
    #[doc = "Change the `methodDetails` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PaymentMethodChangeEventInit`*"]
#[js::import = "None = $1"]
pub fn set_method_details(this: &PaymentMethodChangeEventInit, val: Option<&::js_sys::Object>) { panic!("web-sys intrinsic") }
    #[doc = "Get the `methodName` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PaymentMethodChangeEventInit`*"]
#[js::import = "None"]
pub fn get_method_name(this: &PaymentMethodChangeEventInit) -> ::alloc::string::String { panic!("web-sys intrinsic") }
    #[doc = "Change the `methodName` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PaymentMethodChangeEventInit`*"]
#[js::import = "None = $1"]
pub fn set_method_name(this: &PaymentMethodChangeEventInit, val: &str) { panic!("web-sys intrinsic") }
impl PaymentMethodChangeEventInit {
    #[doc = "Construct a new `PaymentMethodChangeEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PaymentMethodChangeEventInit`*"]
#[js::import = "None"]
pub fn new(method_name: &str) -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret.set_method_name(method_name);
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
    #[deprecated = "Use `set_method_details()` instead."]
#[js::import = "None"]
pub fn method_details(&mut self, val: Option<&::js_sys::Object>) -> &mut Self { self.set_method_details(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_method_name()` instead."]
#[js::import = "None"]
pub fn method_name(&mut self, val: &str) -> &mut Self { self.set_method_name(val) { panic!("web-sys intrinsic") }
        self
    }
}
