#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `IdbObjectStore` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbObjectStore`*"]
#[js::opaque]
#[derive(Copy)]
pub struct IdbObjectStore(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `name` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/name)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbObjectStore`*"]
impl IdbObjectStore {
    #[js::import = "$0.name"]
pub fn name(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `name` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/name)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbObjectStore`*"]
impl IdbObjectStore {
    #[js::import = "$0.name = $1"]
pub fn set_name(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `keyPath` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/keyPath)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbObjectStore`*"]
impl IdbObjectStore {
    #[js::import = "$0.keyPath"]
pub fn key_path(&self) -> Result<::wasm_bindgen::JsValue, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "DomStringList")]
    #[doc = "Getter for the `indexNames` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/indexNames)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomStringList`, `IdbObjectStore`*"]
impl IdbObjectStore {
    #[js::import = "$0.indexNames"]
pub fn index_names(&self) -> DomStringList { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "IdbTransaction")]
    #[doc = "Getter for the `transaction` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/transaction)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbObjectStore`, `IdbTransaction`*"]
impl IdbObjectStore {
    #[js::import = "$0.transaction"]
pub fn transaction(&self) -> IdbTransaction { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `autoIncrement` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/autoIncrement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbObjectStore`*"]
impl IdbObjectStore {
    #[js::import = "$0.autoIncrement"]
pub fn auto_increment(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "IdbRequest")]
    #[doc = "The `add()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/add)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbObjectStore`, `IdbRequest`*"]
impl IdbObjectStore {
    #[js::import = "$0.add($1)"]
pub fn add(&self, value: &::wasm_bindgen::JsValue) -> Result<IdbRequest, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "IdbRequest")]
    #[doc = "The `add()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/add)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbObjectStore`, `IdbRequest`*"]
impl IdbObjectStore {
    #[js::import = "$0.add($1, $2)"]
pub fn add_with_key(&self, value: &::wasm_bindgen::JsValue, key: &::wasm_bindgen::JsValue) -> Result<IdbRequest, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "IdbRequest")]
    #[doc = "The `clear()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/clear)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbObjectStore`, `IdbRequest`*"]
impl IdbObjectStore {
    #[js::import = "$0.clear()"]
pub fn clear(&self) -> Result<IdbRequest, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "IdbRequest")]
    #[doc = "The `count()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/count)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbObjectStore`, `IdbRequest`*"]
impl IdbObjectStore {
    #[js::import = "$0.count()"]
pub fn count(&self) -> Result<IdbRequest, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "IdbRequest")]
    #[doc = "The `count()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/count)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbObjectStore`, `IdbRequest`*"]
impl IdbObjectStore {
    #[js::import = "$0.count($1)"]
pub fn count_with_key(&self, query: &::wasm_bindgen::JsValue) -> Result<IdbRequest, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "IdbIndex")]
    #[doc = "The `createIndex()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/createIndex)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbIndex`, `IdbObjectStore`*"]
impl IdbObjectStore {
    #[js::import = "$0.createIndex($1, $2)"]
pub fn create_index_with_str(&self, name: &str, key_path: &str) -> Result<IdbIndex, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "IdbIndex")]
    #[doc = "The `createIndex()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/createIndex)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbIndex`, `IdbObjectStore`*"]
impl IdbObjectStore {
    #[js::import = "$0.createIndex($1, $2)"]
pub fn create_index_with_str_sequence(&self, name: &str, key_path: &::wasm_bindgen::JsValue) -> Result<IdbIndex, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "IdbIndex", feature = "IdbIndexParameters",))]
    #[doc = "The `createIndex()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/createIndex)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbIndex`, `IdbIndexParameters`, `IdbObjectStore`*"]
impl IdbObjectStore {
    #[js::import = "$0.createIndex($1, $2, $3)"]
pub fn create_index_with_str_and_optional_parameters(&self, name: &str, key_path: &str, options: &IdbIndexParameters) -> Result<IdbIndex, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "IdbIndex", feature = "IdbIndexParameters",))]
    #[doc = "The `createIndex()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/createIndex)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbIndex`, `IdbIndexParameters`, `IdbObjectStore`*"]
impl IdbObjectStore {
    #[js::import = "$0.createIndex($1, $2, $3)"]
pub fn create_index_with_str_sequence_and_optional_parameters(&self, name: &str, key_path: &::wasm_bindgen::JsValue, options: &IdbIndexParameters) -> Result<IdbIndex, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "IdbRequest")]
    #[doc = "The `delete()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/delete)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbObjectStore`, `IdbRequest`*"]
impl IdbObjectStore {
    #[js::import = "$0.delete($1)"]
pub fn delete(&self, query: &::wasm_bindgen::JsValue) -> Result<IdbRequest, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `deleteIndex()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/deleteIndex)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbObjectStore`*"]
impl IdbObjectStore {
    #[js::import = "$0.deleteIndex($1)"]
pub fn delete_index(&self, name: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "IdbRequest")]
    #[doc = "The `get()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/get)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbObjectStore`, `IdbRequest`*"]
impl IdbObjectStore {
    #[js::import = "$0.get($1)"]
pub fn get(&self, query: &::wasm_bindgen::JsValue) -> Result<IdbRequest, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "IdbRequest")]
    #[doc = "The `getAll()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/getAll)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbObjectStore`, `IdbRequest`*"]
impl IdbObjectStore {
    #[js::import = "$0.getAll()"]
pub fn get_all(&self) -> Result<IdbRequest, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "IdbRequest")]
    #[doc = "The `getAll()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/getAll)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbObjectStore`, `IdbRequest`*"]
impl IdbObjectStore {
    #[js::import = "$0.getAll($1)"]
pub fn get_all_with_key(&self, query: &::wasm_bindgen::JsValue) -> Result<IdbRequest, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "IdbRequest")]
    #[doc = "The `getAll()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/getAll)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbObjectStore`, `IdbRequest`*"]
impl IdbObjectStore {
    #[js::import = "$0.getAll($1, $2)"]
pub fn get_all_with_key_and_limit(&self, query: &::wasm_bindgen::JsValue, count: u32) -> Result<IdbRequest, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "IdbRequest")]
    #[doc = "The `getAllKeys()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/getAllKeys)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbObjectStore`, `IdbRequest`*"]
impl IdbObjectStore {
    #[js::import = "$0.getAllKeys()"]
pub fn get_all_keys(&self) -> Result<IdbRequest, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "IdbRequest")]
    #[doc = "The `getAllKeys()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/getAllKeys)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbObjectStore`, `IdbRequest`*"]
impl IdbObjectStore {
    #[js::import = "$0.getAllKeys($1)"]
pub fn get_all_keys_with_key(&self, query: &::wasm_bindgen::JsValue) -> Result<IdbRequest, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "IdbRequest")]
    #[doc = "The `getAllKeys()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/getAllKeys)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbObjectStore`, `IdbRequest`*"]
impl IdbObjectStore {
    #[js::import = "$0.getAllKeys($1, $2)"]
pub fn get_all_keys_with_key_and_limit(&self, query: &::wasm_bindgen::JsValue, count: u32) -> Result<IdbRequest, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "IdbRequest")]
    #[doc = "The `getKey()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/getKey)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbObjectStore`, `IdbRequest`*"]
impl IdbObjectStore {
    #[js::import = "$0.getKey($1)"]
pub fn get_key(&self, query: &::wasm_bindgen::JsValue) -> Result<IdbRequest, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "IdbIndex")]
    #[doc = "The `index()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/index)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbIndex`, `IdbObjectStore`*"]
impl IdbObjectStore {
    #[js::import = "$0.index($1)"]
pub fn index(&self, name: &str) -> Result<IdbIndex, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "IdbRequest")]
    #[doc = "The `openCursor()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/openCursor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbObjectStore`, `IdbRequest`*"]
impl IdbObjectStore {
    #[js::import = "$0.openCursor()"]
pub fn open_cursor(&self) -> Result<IdbRequest, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "IdbRequest")]
    #[doc = "The `openCursor()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/openCursor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbObjectStore`, `IdbRequest`*"]
impl IdbObjectStore {
    #[js::import = "$0.openCursor($1)"]
pub fn open_cursor_with_range(&self, query: &::wasm_bindgen::JsValue) -> Result<IdbRequest, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "IdbCursorDirection", feature = "IdbRequest",))]
    #[doc = "The `openCursor()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/openCursor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbCursorDirection`, `IdbObjectStore`, `IdbRequest`*"]
impl IdbObjectStore {
    #[js::import = "$0.openCursor($1, $2)"]
pub fn open_cursor_with_range_and_direction(&self, query: &::wasm_bindgen::JsValue, direction: IdbCursorDirection) -> Result<IdbRequest, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "IdbRequest")]
    #[doc = "The `openKeyCursor()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/openKeyCursor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbObjectStore`, `IdbRequest`*"]
impl IdbObjectStore {
    #[js::import = "$0.openKeyCursor()"]
pub fn open_key_cursor(&self) -> Result<IdbRequest, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "IdbRequest")]
    #[doc = "The `openKeyCursor()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/openKeyCursor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbObjectStore`, `IdbRequest`*"]
impl IdbObjectStore {
    #[js::import = "$0.openKeyCursor($1)"]
pub fn open_key_cursor_with_range(&self, query: &::wasm_bindgen::JsValue) -> Result<IdbRequest, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "IdbCursorDirection", feature = "IdbRequest",))]
    #[doc = "The `openKeyCursor()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/openKeyCursor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbCursorDirection`, `IdbObjectStore`, `IdbRequest`*"]
impl IdbObjectStore {
    #[js::import = "$0.openKeyCursor($1, $2)"]
pub fn open_key_cursor_with_range_and_direction(&self, query: &::wasm_bindgen::JsValue, direction: IdbCursorDirection) -> Result<IdbRequest, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "IdbRequest")]
    #[doc = "The `put()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/put)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbObjectStore`, `IdbRequest`*"]
impl IdbObjectStore {
    #[js::import = "$0.put($1)"]
pub fn put(&self, value: &::wasm_bindgen::JsValue) -> Result<IdbRequest, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "IdbRequest")]
    #[doc = "The `put()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/put)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbObjectStore`, `IdbRequest`*"]
impl IdbObjectStore {
    #[js::import = "$0.put($1, $2)"]
pub fn put_with_key(&self, value: &::wasm_bindgen::JsValue, key: &::wasm_bindgen::JsValue) -> Result<IdbRequest, JsValue> { panic!("web-sys intrinsic") }
}
