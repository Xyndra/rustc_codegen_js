#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ExtendableMessageEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtendableMessageEventInit`*"]
#[js::opaque]
#[derive(Copy)]
pub struct ExtendableMessageEventInit(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtendableMessageEventInit`*"]
#[js::import = "None"]
pub fn get_bubbles(this: &ExtendableMessageEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtendableMessageEventInit`*"]
#[js::import = "None = $1"]
pub fn set_bubbles(this: &ExtendableMessageEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtendableMessageEventInit`*"]
#[js::import = "None"]
pub fn get_cancelable(this: &ExtendableMessageEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtendableMessageEventInit`*"]
#[js::import = "None = $1"]
pub fn set_cancelable(this: &ExtendableMessageEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtendableMessageEventInit`*"]
#[js::import = "None"]
pub fn get_composed(this: &ExtendableMessageEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtendableMessageEventInit`*"]
#[js::import = "None = $1"]
pub fn set_composed(this: &ExtendableMessageEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `data` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtendableMessageEventInit`*"]
#[js::import = "None"]
pub fn get_data(this: &ExtendableMessageEventInit) -> ::wasm_bindgen::JsValue { panic!("web-sys intrinsic") }
    #[doc = "Change the `data` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtendableMessageEventInit`*"]
#[js::import = "None = $1"]
pub fn set_data(this: &ExtendableMessageEventInit, val: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
    #[doc = "Get the `lastEventId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtendableMessageEventInit`*"]
#[js::import = "None"]
pub fn get_last_event_id(this: &ExtendableMessageEventInit) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `lastEventId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtendableMessageEventInit`*"]
#[js::import = "None = $1"]
pub fn set_last_event_id(this: &ExtendableMessageEventInit, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `origin` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtendableMessageEventInit`*"]
#[js::import = "None"]
pub fn get_origin(this: &ExtendableMessageEventInit) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `origin` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtendableMessageEventInit`*"]
#[js::import = "None = $1"]
pub fn set_origin(this: &ExtendableMessageEventInit, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `ports` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtendableMessageEventInit`*"]
#[js::import = "None"]
pub fn get_ports(this: &ExtendableMessageEventInit) -> Option<::js_sys::Array> { panic!("web-sys intrinsic") }
    #[doc = "Change the `ports` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtendableMessageEventInit`*"]
#[js::import = "None = $1"]
pub fn set_ports(this: &ExtendableMessageEventInit, val: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
    #[doc = "Get the `source` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtendableMessageEventInit`*"]
#[js::import = "None"]
pub fn get_source(this: &ExtendableMessageEventInit) -> Option<::js_sys::Object> { panic!("web-sys intrinsic") }
    #[doc = "Change the `source` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtendableMessageEventInit`*"]
#[js::import = "None = $1"]
pub fn set_source(this: &ExtendableMessageEventInit, val: Option<&::js_sys::Object>) { panic!("web-sys intrinsic") }
impl ExtendableMessageEventInit {
    #[doc = "Construct a new `ExtendableMessageEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtendableMessageEventInit`*"]
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
    #[deprecated = "Use `set_data()` instead."]
#[js::import = "None"]
pub fn data(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self { self.set_data(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_last_event_id()` instead."]
#[js::import = "None"]
pub fn last_event_id(&mut self, val: &str) -> &mut Self { self.set_last_event_id(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_origin()` instead."]
#[js::import = "None"]
pub fn origin(&mut self, val: &str) -> &mut Self { self.set_origin(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_ports()` instead."]
#[js::import = "None"]
pub fn ports(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self { self.set_ports(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_source()` instead."]
#[js::import = "None"]
pub fn source(&mut self, val: Option<&::js_sys::Object>) -> &mut Self { self.set_source(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for ExtendableMessageEventInit {
    fn default() -> Self {
        Self::new()
    }
}
