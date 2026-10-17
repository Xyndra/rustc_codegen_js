#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `IdbDatabase` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbDatabase`*"]
#[js::opaque]
#[derive(Copy)]
pub struct IdbDatabase(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `name` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/name)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbDatabase`*"]
impl IdbDatabase {
    #[js::import = "$0.name"]
pub fn name(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `version` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/version)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbDatabase`*"]
impl IdbDatabase {
    #[js::import = "$0.version"]
pub fn version(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "DomStringList")]
    #[doc = "Getter for the `objectStoreNames` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/objectStoreNames)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomStringList`, `IdbDatabase`*"]
impl IdbDatabase {
    #[js::import = "$0.objectStoreNames"]
pub fn object_store_names(&self) -> DomStringList { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onabort` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/onabort)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbDatabase`*"]
impl IdbDatabase {
    #[js::import = "$0.onabort"]
pub fn onabort(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onabort` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/onabort)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbDatabase`*"]
impl IdbDatabase {
    #[js::import = "$0.onabort = $1"]
pub fn set_onabort(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onclose` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/onclose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbDatabase`*"]
impl IdbDatabase {
    #[js::import = "$0.onclose"]
pub fn onclose(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onclose` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/onclose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbDatabase`*"]
impl IdbDatabase {
    #[js::import = "$0.onclose = $1"]
pub fn set_onclose(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/onerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbDatabase`*"]
impl IdbDatabase {
    #[js::import = "$0.onerror"]
pub fn onerror(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/onerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbDatabase`*"]
impl IdbDatabase {
    #[js::import = "$0.onerror = $1"]
pub fn set_onerror(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onversionchange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/onversionchange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbDatabase`*"]
impl IdbDatabase {
    #[js::import = "$0.onversionchange"]
pub fn onversionchange(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onversionchange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/onversionchange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbDatabase`*"]
impl IdbDatabase {
    #[js::import = "$0.onversionchange = $1"]
pub fn set_onversionchange(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "StorageType")]
    #[doc = "Getter for the `storage` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/storage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbDatabase`, `StorageType`*"]
    #[deprecated]
impl IdbDatabase {
    #[js::import = "$0.storage"]
pub fn storage(&self) -> StorageType { panic!("web-sys intrinsic") }
}
    #[doc = "The `close()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/close)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbDatabase`*"]
impl IdbDatabase {
    #[js::import = "$0.close()"]
pub fn close(&self) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "IdbRequest")]
    #[doc = "The `createMutableFile()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/createMutableFile)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbDatabase`, `IdbRequest`*"]
    #[deprecated]
impl IdbDatabase {
    #[js::import = "$0.createMutableFile($1)"]
pub fn create_mutable_file(&self, name: &str) -> Result<IdbRequest, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "IdbRequest")]
    #[doc = "The `createMutableFile()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/createMutableFile)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbDatabase`, `IdbRequest`*"]
    #[deprecated]
impl IdbDatabase {
    #[js::import = "$0.createMutableFile($1, $2)"]
pub fn create_mutable_file_with_type(&self, name: &str, type_: &str) -> Result<IdbRequest, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "IdbObjectStore")]
    #[doc = "The `createObjectStore()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/createObjectStore)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbDatabase`, `IdbObjectStore`*"]
impl IdbDatabase {
    #[js::import = "$0.createObjectStore($1)"]
pub fn create_object_store(&self, name: &str) -> Result<IdbObjectStore, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "IdbObjectStore", feature = "IdbObjectStoreParameters",))]
    #[doc = "The `createObjectStore()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/createObjectStore)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbDatabase`, `IdbObjectStore`, `IdbObjectStoreParameters`*"]
impl IdbDatabase {
    #[js::import = "$0.createObjectStore($1, $2)"]
pub fn create_object_store_with_optional_parameters(&self, name: &str, options: &IdbObjectStoreParameters) -> Result<IdbObjectStore, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `deleteObjectStore()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/deleteObjectStore)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbDatabase`*"]
impl IdbDatabase {
    #[js::import = "$0.deleteObjectStore($1)"]
pub fn delete_object_store(&self, name: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "IdbTransaction")]
    #[doc = "The `transaction()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/transaction)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbDatabase`, `IdbTransaction`*"]
impl IdbDatabase {
    #[js::import = "$0.transaction($1)"]
pub fn transaction_with_str(&self, store_names: &str) -> Result<IdbTransaction, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "IdbTransaction")]
    #[doc = "The `transaction()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/transaction)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbDatabase`, `IdbTransaction`*"]
impl IdbDatabase {
    #[js::import = "$0.transaction($1)"]
pub fn transaction_with_str_sequence(&self, store_names: &::wasm_bindgen::JsValue) -> Result<IdbTransaction, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "IdbTransaction", feature = "IdbTransactionMode",))]
    #[doc = "The `transaction()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/transaction)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbDatabase`, `IdbTransaction`, `IdbTransactionMode`*"]
impl IdbDatabase {
    #[js::import = "$0.transaction($1, $2)"]
pub fn transaction_with_str_and_mode(&self, store_names: &str, mode: IdbTransactionMode) -> Result<IdbTransaction, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "IdbTransaction", feature = "IdbTransactionMode",))]
    #[doc = "The `transaction()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/transaction)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbDatabase`, `IdbTransaction`, `IdbTransactionMode`*"]
impl IdbDatabase {
    #[js::import = "$0.transaction($1, $2)"]
pub fn transaction_with_str_sequence_and_mode(&self, store_names: &::wasm_bindgen::JsValue, mode: IdbTransactionMode) -> Result<IdbTransaction, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(
        feature = "IdbTransaction",
        feature = "IdbTransactionMode",
        feature = "IdbTransactionOptions",
    ))]
    #[doc = "The `transaction()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/transaction)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbDatabase`, `IdbTransaction`, `IdbTransactionMode`, `IdbTransactionOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl IdbDatabase {
    #[js::import = "$0.transaction($1, $2, $3)"]
pub fn transaction_with_str_and_mode_and_options(&self, store_names: &str, mode: IdbTransactionMode, options: &IdbTransactionOptions) -> Result<IdbTransaction, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(
        feature = "IdbTransaction",
        feature = "IdbTransactionMode",
        feature = "IdbTransactionOptions",
    ))]
    #[doc = "The `transaction()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/transaction)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbDatabase`, `IdbTransaction`, `IdbTransactionMode`, `IdbTransactionOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl IdbDatabase {
    #[js::import = "$0.transaction($1, $2, $3)"]
pub fn transaction_with_str_sequence_and_mode_and_options(&self, store_names: &::wasm_bindgen::JsValue, mode: IdbTransactionMode, options: &IdbTransactionOptions) -> Result<IdbTransaction, JsValue> { panic!("web-sys intrinsic") }
}
