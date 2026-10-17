#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PushSubscriptionOptionsInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushSubscriptionOptionsInit`*"]
#[js::opaque]
#[derive(Copy)]
pub struct PushSubscriptionOptionsInit(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `applicationServerKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushSubscriptionOptionsInit`*"]
#[js::import = "None"]
pub fn get_application_server_key(this: &PushSubscriptionOptionsInit,) -> ::wasm_bindgen::JsValue { panic!("web-sys intrinsic") }
    #[doc = "Change the `applicationServerKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushSubscriptionOptionsInit`*"]
#[js::import = "None = $1"]
pub fn set_application_server_key(this: &PushSubscriptionOptionsInit, val: &::wasm_bindgen::JsValue,) { panic!("web-sys intrinsic") }
    #[doc = "Get the `userVisibleOnly` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushSubscriptionOptionsInit`*"]
#[js::import = "None"]
pub fn get_user_visible_only(this: &PushSubscriptionOptionsInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `userVisibleOnly` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushSubscriptionOptionsInit`*"]
#[js::import = "None = $1"]
pub fn set_user_visible_only(this: &PushSubscriptionOptionsInit, val: bool) { panic!("web-sys intrinsic") }
impl PushSubscriptionOptionsInit {
    #[doc = "Construct a new `PushSubscriptionOptionsInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushSubscriptionOptionsInit`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_application_server_key()` instead."]
#[js::import = "None"]
pub fn application_server_key(&mut self, val: Option<&::wasm_bindgen::JsValue>) -> &mut Self { self.set_application_server_key(val.unwrap_or(&::wasm_bindgen::JsValue::NULL)) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_user_visible_only()` instead."]
#[js::import = "None"]
pub fn user_visible_only(&mut self, val: bool) -> &mut Self { self.set_user_visible_only(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for PushSubscriptionOptionsInit {
    fn default() -> Self {
        Self::new()
    }
}
