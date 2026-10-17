#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PushSubscriptionInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushSubscriptionInit`*"]
#[js::opaque]
#[derive(Copy)]
pub struct PushSubscriptionInit(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `appServerKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushSubscriptionInit`*"]
#[js::import = "None"]
pub fn get_app_server_key(this: &PushSubscriptionInit) -> Option<::js_sys::Object> { panic!("web-sys intrinsic") }
    #[doc = "Change the `appServerKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushSubscriptionInit`*"]
#[js::import = "None = $1"]
pub fn set_app_server_key(this: &PushSubscriptionInit, val: Option<&::js_sys::Object>) { panic!("web-sys intrinsic") }
    #[doc = "Get the `authSecret` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushSubscriptionInit`*"]
#[js::import = "None"]
pub fn get_auth_secret(this: &PushSubscriptionInit) -> Option<::js_sys::ArrayBuffer> { panic!("web-sys intrinsic") }
    #[doc = "Change the `authSecret` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushSubscriptionInit`*"]
#[js::import = "None = $1"]
pub fn set_auth_secret(this: &PushSubscriptionInit, val: Option<&::js_sys::ArrayBuffer>) { panic!("web-sys intrinsic") }
    #[doc = "Get the `endpoint` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushSubscriptionInit`*"]
#[js::import = "None"]
pub fn get_endpoint(this: &PushSubscriptionInit) -> ::alloc::string::String { panic!("web-sys intrinsic") }
    #[doc = "Change the `endpoint` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushSubscriptionInit`*"]
#[js::import = "None = $1"]
pub fn set_endpoint(this: &PushSubscriptionInit, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `p256dhKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushSubscriptionInit`*"]
#[js::import = "None"]
pub fn get_p256dh_key(this: &PushSubscriptionInit) -> Option<::js_sys::ArrayBuffer> { panic!("web-sys intrinsic") }
    #[doc = "Change the `p256dhKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushSubscriptionInit`*"]
#[js::import = "None = $1"]
pub fn set_p256dh_key(this: &PushSubscriptionInit, val: Option<&::js_sys::ArrayBuffer>) { panic!("web-sys intrinsic") }
    #[doc = "Get the `scope` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushSubscriptionInit`*"]
#[js::import = "None"]
pub fn get_scope(this: &PushSubscriptionInit) -> ::alloc::string::String { panic!("web-sys intrinsic") }
    #[doc = "Change the `scope` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushSubscriptionInit`*"]
#[js::import = "None = $1"]
pub fn set_scope(this: &PushSubscriptionInit, val: &str) { panic!("web-sys intrinsic") }
impl PushSubscriptionInit {
    #[doc = "Construct a new `PushSubscriptionInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushSubscriptionInit`*"]
#[js::import = "None"]
pub fn new(endpoint: &str, scope: &str) -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret.set_endpoint(endpoint);
        ret.set_scope(scope);
        ret
    }
    #[deprecated = "Use `set_app_server_key()` instead."]
#[js::import = "None"]
pub fn app_server_key(&mut self, val: Option<&::js_sys::Object>) -> &mut Self { self.set_app_server_key(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_auth_secret()` instead."]
#[js::import = "None"]
pub fn auth_secret(&mut self, val: Option<&::js_sys::ArrayBuffer>) -> &mut Self { self.set_auth_secret(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_endpoint()` instead."]
#[js::import = "None"]
pub fn endpoint(&mut self, val: &str) -> &mut Self { self.set_endpoint(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_p256dh_key()` instead."]
#[js::import = "None"]
pub fn p256dh_key(&mut self, val: Option<&::js_sys::ArrayBuffer>) -> &mut Self { self.set_p256dh_key(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_scope()` instead."]
#[js::import = "None"]
pub fn scope(&mut self, val: &str) -> &mut Self { self.set_scope(val) { panic!("web-sys intrinsic") }
        self
    }
}
