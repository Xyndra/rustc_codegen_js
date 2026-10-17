#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MessageEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessageEventInit`*"]
#[js::opaque]
#[derive(Copy)]
pub struct MessageEventInit(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessageEventInit`*"]
#[js::import = "None"]
pub fn get_bubbles(this: &MessageEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessageEventInit`*"]
#[js::import = "None = $1"]
pub fn set_bubbles(this: &MessageEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessageEventInit`*"]
#[js::import = "None"]
pub fn get_cancelable(this: &MessageEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessageEventInit`*"]
#[js::import = "None = $1"]
pub fn set_cancelable(this: &MessageEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessageEventInit`*"]
#[js::import = "None"]
pub fn get_composed(this: &MessageEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessageEventInit`*"]
#[js::import = "None = $1"]
pub fn set_composed(this: &MessageEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `data` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessageEventInit`*"]
#[js::import = "None"]
pub fn get_data(this: &MessageEventInit) -> ::wasm_bindgen::JsValue { panic!("web-sys intrinsic") }
    #[doc = "Change the `data` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessageEventInit`*"]
#[js::import = "None = $1"]
pub fn set_data(this: &MessageEventInit, val: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
    #[doc = "Get the `lastEventId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessageEventInit`*"]
#[js::import = "None"]
pub fn get_last_event_id(this: &MessageEventInit) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `lastEventId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessageEventInit`*"]
#[js::import = "None = $1"]
pub fn set_last_event_id(this: &MessageEventInit, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `origin` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessageEventInit`*"]
#[js::import = "None"]
pub fn get_origin(this: &MessageEventInit) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `origin` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessageEventInit`*"]
#[js::import = "None = $1"]
pub fn set_origin(this: &MessageEventInit, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `ports` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessageEventInit`*"]
#[js::import = "None"]
pub fn get_ports(this: &MessageEventInit) -> Option<::js_sys::Array> { panic!("web-sys intrinsic") }
    #[doc = "Change the `ports` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessageEventInit`*"]
#[js::import = "None = $1"]
pub fn set_ports(this: &MessageEventInit, val: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
    #[doc = "Get the `source` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessageEventInit`*"]
#[js::import = "None"]
pub fn get_source(this: &MessageEventInit) -> Option<::js_sys::Object> { panic!("web-sys intrinsic") }
    #[doc = "Change the `source` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessageEventInit`*"]
#[js::import = "None = $1"]
pub fn set_source(this: &MessageEventInit, val: Option<&::js_sys::Object>) { panic!("web-sys intrinsic") }
impl MessageEventInit {
    #[doc = "Construct a new `MessageEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessageEventInit`*"]
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
impl Default for MessageEventInit {
    fn default() -> Self {
        Self::new()
    }
}
