#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RequestMediaKeySystemAccessNotification` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RequestMediaKeySystemAccessNotification`*"]
#[js::opaque]
#[derive(Copy)]
pub struct RequestMediaKeySystemAccessNotification(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `keySystem` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RequestMediaKeySystemAccessNotification`*"]
#[js::import = "None"]
pub fn get_key_system(this: &RequestMediaKeySystemAccessNotification,) -> ::alloc::string::String { panic!("web-sys intrinsic") }
    #[doc = "Change the `keySystem` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RequestMediaKeySystemAccessNotification`*"]
#[js::import = "None = $1"]
pub fn set_key_system(this: &RequestMediaKeySystemAccessNotification, val: &str) { panic!("web-sys intrinsic") }
    #[cfg(feature = "MediaKeySystemStatus")]
    #[doc = "Get the `status` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySystemStatus`, `RequestMediaKeySystemAccessNotification`*"]
#[js::import = "None"]
pub fn get_status(this: &RequestMediaKeySystemAccessNotification) -> MediaKeySystemStatus { panic!("web-sys intrinsic") }
    #[cfg(feature = "MediaKeySystemStatus")]
    #[doc = "Change the `status` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySystemStatus`, `RequestMediaKeySystemAccessNotification`*"]
#[js::import = "None = $1"]
pub fn set_status(this: &RequestMediaKeySystemAccessNotification, val: MediaKeySystemStatus) { panic!("web-sys intrinsic") }
impl RequestMediaKeySystemAccessNotification {
    #[cfg(feature = "MediaKeySystemStatus")]
    #[doc = "Construct a new `RequestMediaKeySystemAccessNotification`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySystemStatus`, `RequestMediaKeySystemAccessNotification`*"]
#[js::import = "None"]
pub fn new(key_system: &str, status: MediaKeySystemStatus) -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret.set_key_system(key_system);
        ret.set_status(status);
        ret
    }
    #[deprecated = "Use `set_key_system()` instead."]
#[js::import = "None"]
pub fn key_system(&mut self, val: &str) -> &mut Self { self.set_key_system(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(feature = "MediaKeySystemStatus")]
    #[deprecated = "Use `set_status()` instead."]
#[js::import = "None"]
pub fn status(&mut self, val: MediaKeySystemStatus) -> &mut Self { self.set_status(val) { panic!("web-sys intrinsic") }
        self
    }
}
