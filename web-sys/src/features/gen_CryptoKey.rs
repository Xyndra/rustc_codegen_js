#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CryptoKey` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CryptoKey)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CryptoKey`*"]
#[js::opaque]
#[derive(Copy)]
pub struct CryptoKey(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CryptoKey/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CryptoKey`*"]
impl CryptoKey {
    #[js::import = "$0.type"]
pub fn type_(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `extractable` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CryptoKey/extractable)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CryptoKey`*"]
impl CryptoKey {
    #[js::import = "$0.extractable"]
pub fn extractable(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `algorithm` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CryptoKey/algorithm)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CryptoKey`*"]
impl CryptoKey {
    #[js::import = "$0.algorithm"]
pub fn algorithm(&self) -> Result<::js_sys::Object, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `usages` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CryptoKey/usages)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CryptoKey`*"]
impl CryptoKey {
    #[js::import = "$0.usages"]
pub fn usages(&self) -> ::js_sys::Array { panic!("web-sys intrinsic") }
}
