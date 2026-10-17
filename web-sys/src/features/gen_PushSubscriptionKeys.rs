#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PushSubscriptionKeys` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushSubscriptionKeys`*"]
#[js::opaque]
#[derive(Copy)]
pub struct PushSubscriptionKeys(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `auth` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushSubscriptionKeys`*"]
#[js::import = "None"]
pub fn get_auth(this: &PushSubscriptionKeys) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `auth` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushSubscriptionKeys`*"]
#[js::import = "None = $1"]
pub fn set_auth(this: &PushSubscriptionKeys, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `p256dh` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushSubscriptionKeys`*"]
#[js::import = "None"]
pub fn get_p256dh(this: &PushSubscriptionKeys) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `p256dh` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushSubscriptionKeys`*"]
#[js::import = "None = $1"]
pub fn set_p256dh(this: &PushSubscriptionKeys, val: &str) { panic!("web-sys intrinsic") }
impl PushSubscriptionKeys {
    #[doc = "Construct a new `PushSubscriptionKeys`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushSubscriptionKeys`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_auth()` instead."]
#[js::import = "None"]
pub fn auth(&mut self, val: &str) -> &mut Self { self.set_auth(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_p256dh()` instead."]
#[js::import = "None"]
pub fn p256dh(&mut self, val: &str) -> &mut Self { self.set_p256dh(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for PushSubscriptionKeys {
    fn default() -> Self {
        Self::new()
    }
}
