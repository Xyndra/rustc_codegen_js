#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Permissions` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Permissions)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Permissions`*"]
#[js::opaque]
#[derive(Copy)]
pub struct Permissions(pub ::wasm_bindgen::JsValue);
    #[doc = "The `query()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Permissions/query)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Permissions`*"]
impl Permissions {
    #[js::import = "$0.query($1)"]
pub fn query(&self, permission: &::js_sys::Object) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `revoke()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Permissions/revoke)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Permissions`*"]
impl Permissions {
    #[js::import = "$0.revoke($1)"]
pub fn revoke(&self, permission: &::js_sys::Object) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
