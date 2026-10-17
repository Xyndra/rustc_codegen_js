#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `IdbCursorWithValue` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBCursorWithValue)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbCursorWithValue`*"]
#[js::opaque]
#[derive(Copy)]
pub struct IdbCursorWithValue(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `value` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBCursorWithValue/value)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbCursorWithValue`*"]
impl IdbCursorWithValue {
    #[js::import = "$0.value"]
pub fn value(&self) -> Result<::wasm_bindgen::JsValue, JsValue> { panic!("web-sys intrinsic") }
}
