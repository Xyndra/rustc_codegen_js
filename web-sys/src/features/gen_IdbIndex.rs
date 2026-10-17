#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `IdbIndex` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbIndex`*"]
#[js::opaque]
#[derive(Copy)]
pub struct IdbIndex(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `name` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex/name)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbIndex`*"]
impl IdbIndex {
    #[js::import = "$0.name"]
pub fn name(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `name` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex/name)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbIndex`*"]
impl IdbIndex {
    #[js::import = "$0.name = $1"]
pub fn set_name(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "IdbObjectStore")]
    #[doc = "Getter for the `objectStore` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex/objectStore)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbIndex`, `IdbObjectStore`*"]
impl IdbIndex {
    #[js::import = "$0.objectStore"]
pub fn object_store(&self) -> IdbObjectStore { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `keyPath` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex/keyPath)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbIndex`*"]
impl IdbIndex {
    #[js::import = "$0.keyPath"]
pub fn key_path(&self) -> Result<::wasm_bindgen::JsValue, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `multiEntry` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex/multiEntry)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbIndex`*"]
impl IdbIndex {
    #[js::import = "$0.multiEntry"]
pub fn multi_entry(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `unique` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex/unique)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbIndex`*"]
impl IdbIndex {
    #[js::import = "$0.unique"]
pub fn unique(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `locale` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex/locale)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbIndex`*"]
    #[deprecated]
impl IdbIndex {
    #[js::import = "$0.locale"]
pub fn locale(&self) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `isAutoLocale` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex/isAutoLocale)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbIndex`*"]
    #[deprecated]
impl IdbIndex {
    #[js::import = "$0.isAutoLocale"]
pub fn is_auto_locale(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "IdbRequest")]
    #[doc = "The `count()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex/count)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbIndex`, `IdbRequest`*"]
impl IdbIndex {
    #[js::import = "$0.count()"]
pub fn count(&self) -> Result<IdbRequest, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "IdbRequest")]
    #[doc = "The `count()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex/count)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbIndex`, `IdbRequest`*"]
impl IdbIndex {
    #[js::import = "$0.count($1)"]
pub fn count_with_key(&self, query: &::wasm_bindgen::JsValue) -> Result<IdbRequest, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "IdbRequest")]
    #[doc = "The `get()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex/get)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbIndex`, `IdbRequest`*"]
impl IdbIndex {
    #[js::import = "$0.get($1)"]
pub fn get(&self, query: &::wasm_bindgen::JsValue) -> Result<IdbRequest, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "IdbRequest")]
    #[doc = "The `getAll()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex/getAll)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbIndex`, `IdbRequest`*"]
impl IdbIndex {
    #[js::import = "$0.getAll()"]
pub fn get_all(&self) -> Result<IdbRequest, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "IdbRequest")]
    #[doc = "The `getAll()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex/getAll)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbIndex`, `IdbRequest`*"]
impl IdbIndex {
    #[js::import = "$0.getAll($1)"]
pub fn get_all_with_key(&self, query: &::wasm_bindgen::JsValue) -> Result<IdbRequest, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "IdbRequest")]
    #[doc = "The `getAll()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex/getAll)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbIndex`, `IdbRequest`*"]
impl IdbIndex {
    #[js::import = "$0.getAll($1, $2)"]
pub fn get_all_with_key_and_limit(&self, query: &::wasm_bindgen::JsValue, count: u32) -> Result<IdbRequest, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "IdbRequest")]
    #[doc = "The `getAllKeys()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex/getAllKeys)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbIndex`, `IdbRequest`*"]
impl IdbIndex {
    #[js::import = "$0.getAllKeys()"]
pub fn get_all_keys(&self) -> Result<IdbRequest, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "IdbRequest")]
    #[doc = "The `getAllKeys()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex/getAllKeys)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbIndex`, `IdbRequest`*"]
impl IdbIndex {
    #[js::import = "$0.getAllKeys($1)"]
pub fn get_all_keys_with_key(&self, query: &::wasm_bindgen::JsValue) -> Result<IdbRequest, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "IdbRequest")]
    #[doc = "The `getAllKeys()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex/getAllKeys)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbIndex`, `IdbRequest`*"]
impl IdbIndex {
    #[js::import = "$0.getAllKeys($1, $2)"]
pub fn get_all_keys_with_key_and_limit(&self, query: &::wasm_bindgen::JsValue, count: u32) -> Result<IdbRequest, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "IdbRequest")]
    #[doc = "The `getKey()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex/getKey)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbIndex`, `IdbRequest`*"]
impl IdbIndex {
    #[js::import = "$0.getKey($1)"]
pub fn get_key(&self, query: &::wasm_bindgen::JsValue) -> Result<IdbRequest, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "IdbRequest")]
    #[doc = "The `openCursor()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex/openCursor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbIndex`, `IdbRequest`*"]
impl IdbIndex {
    #[js::import = "$0.openCursor()"]
pub fn open_cursor(&self) -> Result<IdbRequest, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "IdbRequest")]
    #[doc = "The `openCursor()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex/openCursor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbIndex`, `IdbRequest`*"]
impl IdbIndex {
    #[js::import = "$0.openCursor($1)"]
pub fn open_cursor_with_range(&self, query: &::wasm_bindgen::JsValue) -> Result<IdbRequest, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "IdbCursorDirection", feature = "IdbRequest",))]
    #[doc = "The `openCursor()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex/openCursor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbCursorDirection`, `IdbIndex`, `IdbRequest`*"]
impl IdbIndex {
    #[js::import = "$0.openCursor($1, $2)"]
pub fn open_cursor_with_range_and_direction(&self, query: &::wasm_bindgen::JsValue, direction: IdbCursorDirection) -> Result<IdbRequest, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "IdbRequest")]
    #[doc = "The `openKeyCursor()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex/openKeyCursor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbIndex`, `IdbRequest`*"]
impl IdbIndex {
    #[js::import = "$0.openKeyCursor()"]
pub fn open_key_cursor(&self) -> Result<IdbRequest, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "IdbRequest")]
    #[doc = "The `openKeyCursor()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex/openKeyCursor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbIndex`, `IdbRequest`*"]
impl IdbIndex {
    #[js::import = "$0.openKeyCursor($1)"]
pub fn open_key_cursor_with_range(&self, query: &::wasm_bindgen::JsValue) -> Result<IdbRequest, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "IdbCursorDirection", feature = "IdbRequest",))]
    #[doc = "The `openKeyCursor()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBIndex/openKeyCursor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbCursorDirection`, `IdbIndex`, `IdbRequest`*"]
impl IdbIndex {
    #[js::import = "$0.openKeyCursor($1, $2)"]
pub fn open_key_cursor_with_range_and_direction(&self, query: &::wasm_bindgen::JsValue, direction: IdbCursorDirection) -> Result<IdbRequest, JsValue> { panic!("web-sys intrinsic") }
}
