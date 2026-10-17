#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PushSubscription` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PushSubscription)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushSubscription`*"]
#[js::opaque]
#[derive(Copy)]
pub struct PushSubscription(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `endpoint` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PushSubscription/endpoint)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushSubscription`*"]
impl PushSubscription {
    #[js::import = "$0.endpoint"]
pub fn endpoint(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "PushSubscriptionOptions")]
    #[doc = "Getter for the `options` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PushSubscription/options)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushSubscription`, `PushSubscriptionOptions`*"]
impl PushSubscription {
    #[js::import = "$0.options"]
pub fn options(&self) -> PushSubscriptionOptions { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "PushEncryptionKeyName")]
    #[doc = "The `getKey()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PushSubscription/getKey)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushEncryptionKeyName`, `PushSubscription`*"]
impl PushSubscription {
    #[js::import = "$0.getKey($1)"]
pub fn get_key(&self, name: PushEncryptionKeyName) -> Result<Option<::js_sys::ArrayBuffer>, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "PushSubscriptionJson")]
    #[doc = "The `toJSON()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PushSubscription/toJSON)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushSubscription`, `PushSubscriptionJson`*"]
impl PushSubscription {
    #[js::import = "$0.toJSON()"]
pub fn to_json(&self) -> Result<PushSubscriptionJson, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `unsubscribe()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PushSubscription/unsubscribe)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushSubscription`*"]
impl PushSubscription {
    #[js::import = "$0.unsubscribe()"]
pub fn unsubscribe(&self) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
