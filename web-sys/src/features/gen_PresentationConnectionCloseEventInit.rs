#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PresentationConnectionCloseEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnectionCloseEventInit`*"]
#[js::opaque]
#[derive(Copy)]
pub struct PresentationConnectionCloseEventInit(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnectionCloseEventInit`*"]
#[js::import = "None"]
pub fn get_bubbles(this: &PresentationConnectionCloseEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnectionCloseEventInit`*"]
#[js::import = "None = $1"]
pub fn set_bubbles(this: &PresentationConnectionCloseEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnectionCloseEventInit`*"]
#[js::import = "None"]
pub fn get_cancelable(this: &PresentationConnectionCloseEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnectionCloseEventInit`*"]
#[js::import = "None = $1"]
pub fn set_cancelable(this: &PresentationConnectionCloseEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnectionCloseEventInit`*"]
#[js::import = "None"]
pub fn get_composed(this: &PresentationConnectionCloseEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnectionCloseEventInit`*"]
#[js::import = "None = $1"]
pub fn set_composed(this: &PresentationConnectionCloseEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `message` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnectionCloseEventInit`*"]
#[js::import = "None"]
pub fn get_message(this: &PresentationConnectionCloseEventInit,) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `message` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnectionCloseEventInit`*"]
#[js::import = "None = $1"]
pub fn set_message(this: &PresentationConnectionCloseEventInit, val: &str) { panic!("web-sys intrinsic") }
    #[cfg(feature = "PresentationConnectionClosedReason")]
    #[doc = "Get the `reason` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnectionCloseEventInit`, `PresentationConnectionClosedReason`*"]
#[js::import = "None"]
pub fn get_reason(this: &PresentationConnectionCloseEventInit,) -> PresentationConnectionClosedReason { panic!("web-sys intrinsic") }
    #[cfg(feature = "PresentationConnectionClosedReason")]
    #[doc = "Change the `reason` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnectionCloseEventInit`, `PresentationConnectionClosedReason`*"]
#[js::import = "None = $1"]
pub fn set_reason(this: &PresentationConnectionCloseEventInit, val: PresentationConnectionClosedReason,) { panic!("web-sys intrinsic") }
impl PresentationConnectionCloseEventInit {
    #[cfg(feature = "PresentationConnectionClosedReason")]
    #[doc = "Construct a new `PresentationConnectionCloseEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnectionCloseEventInit`, `PresentationConnectionClosedReason`*"]
#[js::import = "None"]
pub fn new(reason: PresentationConnectionClosedReason) -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret.set_reason(reason);
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
pub fn message(&mut self, val: &str) -> &mut Self { self.set_message(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(feature = "PresentationConnectionClosedReason")]
    #[deprecated = "Use `set_reason()` instead."]
#[js::import = "None"]
pub fn reason(&mut self, val: PresentationConnectionClosedReason) -> &mut Self { self.set_reason(val) { panic!("web-sys intrinsic") }
        self
    }
}
