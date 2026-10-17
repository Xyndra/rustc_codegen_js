#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `IdbLocaleAwareKeyRange` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBLocaleAwareKeyRange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbLocaleAwareKeyRange`*"]
    #[deprecated]
#[js::opaque]
#[derive(Copy)]
pub struct IdbLocaleAwareKeyRange(pub ::wasm_bindgen::JsValue);
    #[doc = "The `bound()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBLocaleAwareKeyRange/bound_static)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbLocaleAwareKeyRange`*"]
    #[deprecated]
impl IdbLocaleAwareKeyRange {
    #[js::import = "IDBLocaleAwareKeyRange.bound"]
pub fn bound(lower: &::wasm_bindgen::JsValue, upper: &::wasm_bindgen::JsValue,) -> Result<IdbLocaleAwareKeyRange, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `bound()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBLocaleAwareKeyRange/bound_static)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbLocaleAwareKeyRange`*"]
    #[deprecated]
impl IdbLocaleAwareKeyRange {
    #[js::import = "IDBLocaleAwareKeyRange.bound"]
pub fn bound_with_lower_open(lower: &::wasm_bindgen::JsValue, upper: &::wasm_bindgen::JsValue, lower_open: bool,) -> Result<IdbLocaleAwareKeyRange, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `bound()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBLocaleAwareKeyRange/bound_static)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbLocaleAwareKeyRange`*"]
    #[deprecated]
impl IdbLocaleAwareKeyRange {
    #[js::import = "IDBLocaleAwareKeyRange.bound"]
pub fn bound_with_lower_open_and_upper_open(lower: &::wasm_bindgen::JsValue, upper: &::wasm_bindgen::JsValue, lower_open: bool, upper_open: bool,) -> Result<IdbLocaleAwareKeyRange, JsValue> { panic!("web-sys intrinsic") }
}
