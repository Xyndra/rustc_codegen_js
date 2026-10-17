#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Crypto` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Crypto)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Crypto`*"]
#[js::opaque]
#[derive(Copy)]
pub struct Crypto(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "SubtleCrypto")]
    #[doc = "Getter for the `subtle` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Crypto/subtle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Crypto`, `SubtleCrypto`*"]
impl Crypto {
    #[js::import = "$0.subtle"]
pub fn subtle(&self) -> SubtleCrypto { panic!("web-sys intrinsic") }
}
    #[doc = "The `getRandomValues()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Crypto/getRandomValues)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Crypto`*"]
impl Crypto {
    #[js::import = "$0.getRandomValues($1)"]
pub fn get_random_values_with_array_buffer_view(&self, array: &::js_sys::Object) -> Result<::js_sys::Object, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `getRandomValues()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Crypto/getRandomValues)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Crypto`*"]
impl Crypto {
    #[js::import = "$0.getRandomValues($1)"]
pub fn get_random_values_with_u8_array(&self, array: &mut [u8]) -> Result<::js_sys::Object, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `getRandomValues()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Crypto/getRandomValues)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Crypto`*"]
impl Crypto {
    #[js::import = "$0.getRandomValues($1)"]
pub fn get_random_values_with_js_u8_array(&self, array: &::js_sys::Uint8Array) -> Result<::js_sys::Object, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `randomUUID()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Crypto/randomUUID)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Crypto`*"]
impl Crypto {
    #[js::import = "$0.randomUUID()"]
pub fn random_uuid(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
