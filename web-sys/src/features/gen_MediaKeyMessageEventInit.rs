#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaKeyMessageEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeyMessageEventInit`*"]
#[js::opaque]
#[derive(Copy)]
pub struct MediaKeyMessageEventInit(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeyMessageEventInit`*"]
#[js::import = "None"]
pub fn get_bubbles(this: &MediaKeyMessageEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeyMessageEventInit`*"]
#[js::import = "None = $1"]
pub fn set_bubbles(this: &MediaKeyMessageEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeyMessageEventInit`*"]
#[js::import = "None"]
pub fn get_cancelable(this: &MediaKeyMessageEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeyMessageEventInit`*"]
#[js::import = "None = $1"]
pub fn set_cancelable(this: &MediaKeyMessageEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeyMessageEventInit`*"]
#[js::import = "None"]
pub fn get_composed(this: &MediaKeyMessageEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeyMessageEventInit`*"]
#[js::import = "None = $1"]
pub fn set_composed(this: &MediaKeyMessageEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `message` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeyMessageEventInit`*"]
#[js::import = "None"]
pub fn get_message(this: &MediaKeyMessageEventInit) -> ::js_sys::ArrayBuffer { panic!("web-sys intrinsic") }
    #[doc = "Change the `message` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeyMessageEventInit`*"]
#[js::import = "None = $1"]
pub fn set_message(this: &MediaKeyMessageEventInit, val: &::js_sys::ArrayBuffer) { panic!("web-sys intrinsic") }
    #[cfg(feature = "MediaKeyMessageType")]
    #[doc = "Get the `messageType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeyMessageEventInit`, `MediaKeyMessageType`*"]
#[js::import = "None"]
pub fn get_message_type(this: &MediaKeyMessageEventInit) -> MediaKeyMessageType { panic!("web-sys intrinsic") }
    #[cfg(feature = "MediaKeyMessageType")]
    #[doc = "Change the `messageType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeyMessageEventInit`, `MediaKeyMessageType`*"]
#[js::import = "None = $1"]
pub fn set_message_type(this: &MediaKeyMessageEventInit, val: MediaKeyMessageType) { panic!("web-sys intrinsic") }
impl MediaKeyMessageEventInit {
    #[cfg(feature = "MediaKeyMessageType")]
    #[doc = "Construct a new `MediaKeyMessageEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeyMessageEventInit`, `MediaKeyMessageType`*"]
#[js::import = "None"]
pub fn new(message: &::js_sys::ArrayBuffer, message_type: MediaKeyMessageType) -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret.set_message(message);
        ret.set_message_type(message_type);
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
    #[deprecated = "Use `set_message()` instead."]
#[js::import = "None"]
pub fn message(&mut self, val: &::js_sys::ArrayBuffer) -> &mut Self { self.set_message(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(feature = "MediaKeyMessageType")]
    #[deprecated = "Use `set_message_type()` instead."]
#[js::import = "None"]
pub fn message_type(&mut self, val: MediaKeyMessageType) -> &mut Self { self.set_message_type(val) { panic!("web-sys intrinsic") }
        self
    }
}
