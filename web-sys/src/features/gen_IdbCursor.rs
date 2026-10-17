#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `IdbCursor` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBCursor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbCursor`*"]
#[js::opaque]
#[derive(Copy)]
pub struct IdbCursor(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `source` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBCursor/source)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbCursor`*"]
impl IdbCursor {
    #[js::import = "$0.source"]
pub fn source(&self) -> ::js_sys::Object { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "IdbCursorDirection")]
    #[doc = "Getter for the `direction` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBCursor/direction)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbCursor`, `IdbCursorDirection`*"]
impl IdbCursor {
    #[js::import = "$0.direction"]
pub fn direction(&self) -> IdbCursorDirection { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `key` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBCursor/key)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbCursor`*"]
impl IdbCursor {
    #[js::import = "$0.key"]
pub fn key(&self) -> Result<::wasm_bindgen::JsValue, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `primaryKey` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBCursor/primaryKey)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbCursor`*"]
impl IdbCursor {
    #[js::import = "$0.primaryKey"]
pub fn primary_key(&self) -> Result<::wasm_bindgen::JsValue, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "IdbRequest")]
    #[doc = "Getter for the `request` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBCursor/request)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbCursor`, `IdbRequest`*"]
    #[deprecated]
impl IdbCursor {
    #[js::import = "$0.request"]
pub fn request(&self) -> IdbRequest { panic!("web-sys intrinsic") }
}
    #[doc = "The `advance()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBCursor/advance)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbCursor`*"]
impl IdbCursor {
    #[js::import = "$0.advance($1)"]
pub fn advance(&self, count: u32) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `continue()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBCursor/continue)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbCursor`*"]
impl IdbCursor {
    #[js::import = "$0.continue()"]
pub fn continue_(&self) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `continue()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBCursor/continue)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbCursor`*"]
impl IdbCursor {
    #[js::import = "$0.continue($1)"]
pub fn continue_with_key(&self, key: &::wasm_bindgen::JsValue) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `continuePrimaryKey()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBCursor/continuePrimaryKey)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbCursor`*"]
impl IdbCursor {
    #[js::import = "$0.continuePrimaryKey($1, $2)"]
pub fn continue_primary_key(&self, key: &::wasm_bindgen::JsValue, primary_key: &::wasm_bindgen::JsValue) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "IdbRequest")]
    #[doc = "The `delete()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBCursor/delete)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbCursor`, `IdbRequest`*"]
impl IdbCursor {
    #[js::import = "$0.delete()"]
pub fn delete(&self) -> Result<IdbRequest, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "IdbRequest")]
    #[doc = "The `update()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBCursor/update)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbCursor`, `IdbRequest`*"]
impl IdbCursor {
    #[js::import = "$0.update($1)"]
pub fn update(&self, value: &::wasm_bindgen::JsValue) -> Result<IdbRequest, JsValue> { panic!("web-sys intrinsic") }
}
