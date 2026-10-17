#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PresentationConnectionAvailableEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnectionAvailableEventInit`*"]
#[js::opaque]
#[derive(Copy)]
pub struct PresentationConnectionAvailableEventInit(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnectionAvailableEventInit`*"]
#[js::import = "None"]
pub fn get_bubbles(this: &PresentationConnectionAvailableEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnectionAvailableEventInit`*"]
#[js::import = "None = $1"]
pub fn set_bubbles(this: &PresentationConnectionAvailableEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnectionAvailableEventInit`*"]
#[js::import = "None"]
pub fn get_cancelable(this: &PresentationConnectionAvailableEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnectionAvailableEventInit`*"]
#[js::import = "None = $1"]
pub fn set_cancelable(this: &PresentationConnectionAvailableEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnectionAvailableEventInit`*"]
#[js::import = "None"]
pub fn get_composed(this: &PresentationConnectionAvailableEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnectionAvailableEventInit`*"]
#[js::import = "None = $1"]
pub fn set_composed(this: &PresentationConnectionAvailableEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[cfg(feature = "PresentationConnection")]
    #[doc = "Get the `connection` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnection`, `PresentationConnectionAvailableEventInit`*"]
#[js::import = "None"]
pub fn get_connection(this: &PresentationConnectionAvailableEventInit,) -> PresentationConnection { panic!("web-sys intrinsic") }
    #[cfg(feature = "PresentationConnection")]
    #[doc = "Change the `connection` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnection`, `PresentationConnectionAvailableEventInit`*"]
#[js::import = "None = $1"]
pub fn set_connection(this: &PresentationConnectionAvailableEventInit, val: &PresentationConnection,) { panic!("web-sys intrinsic") }
impl PresentationConnectionAvailableEventInit {
    #[cfg(feature = "PresentationConnection")]
    #[doc = "Construct a new `PresentationConnectionAvailableEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnection`, `PresentationConnectionAvailableEventInit`*"]
#[js::import = "None"]
pub fn new(connection: &PresentationConnection) -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret.set_connection(connection);
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
    #[cfg(feature = "PresentationConnection")]
    #[deprecated = "Use `set_connection()` instead."]
#[js::import = "None"]
pub fn connection(&mut self, val: &PresentationConnection) -> &mut Self { self.set_connection(val) { panic!("web-sys intrinsic") }
        self
    }
}
