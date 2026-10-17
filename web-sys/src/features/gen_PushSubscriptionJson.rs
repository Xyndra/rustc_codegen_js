#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PushSubscriptionJson` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushSubscriptionJson`*"]
#[js::opaque]
#[derive(Copy)]
pub struct PushSubscriptionJson(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `endpoint` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushSubscriptionJson`*"]
#[js::import = "None"]
pub fn get_endpoint(this: &PushSubscriptionJson) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `endpoint` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushSubscriptionJson`*"]
#[js::import = "None = $1"]
pub fn set_endpoint(this: &PushSubscriptionJson, val: &str) { panic!("web-sys intrinsic") }
    #[cfg(feature = "PushSubscriptionKeys")]
    #[doc = "Get the `keys` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushSubscriptionJson`, `PushSubscriptionKeys`*"]
#[js::import = "None"]
pub fn get_keys(this: &PushSubscriptionJson) -> Option<PushSubscriptionKeys> { panic!("web-sys intrinsic") }
    #[cfg(feature = "PushSubscriptionKeys")]
    #[doc = "Change the `keys` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushSubscriptionJson`, `PushSubscriptionKeys`*"]
#[js::import = "None = $1"]
pub fn set_keys(this: &PushSubscriptionJson, val: &PushSubscriptionKeys) { panic!("web-sys intrinsic") }
impl PushSubscriptionJson {
    #[doc = "Construct a new `PushSubscriptionJson`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushSubscriptionJson`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_endpoint()` instead."]
#[js::import = "None"]
pub fn endpoint(&mut self, val: &str) -> &mut Self { self.set_endpoint(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(feature = "PushSubscriptionKeys")]
    #[deprecated = "Use `set_keys()` instead."]
#[js::import = "None"]
pub fn keys(&mut self, val: &PushSubscriptionKeys) -> &mut Self { self.set_keys(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for PushSubscriptionJson {
    fn default() -> Self {
        Self::new()
    }
}
