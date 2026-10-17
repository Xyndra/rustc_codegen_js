#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `IdbTransaction` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBTransaction)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbTransaction`*"]
#[js::opaque]
#[derive(Copy)]
pub struct IdbTransaction(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "DomStringList")]
    #[doc = "Getter for the `objectStoreNames` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBTransaction/objectStoreNames)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomStringList`, `IdbTransaction`*"]
impl IdbTransaction {
    #[js::import = "$0.objectStoreNames"]
pub fn object_store_names(&self) -> DomStringList { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "IdbTransactionMode")]
    #[doc = "Getter for the `mode` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBTransaction/mode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbTransaction`, `IdbTransactionMode`*"]
impl IdbTransaction {
    #[js::import = "$0.mode"]
pub fn mode(&self) -> Result<IdbTransactionMode, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "IdbDatabase")]
    #[doc = "Getter for the `db` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBTransaction/db)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbDatabase`, `IdbTransaction`*"]
impl IdbTransaction {
    #[js::import = "$0.db"]
pub fn db(&self) -> IdbDatabase { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "DomException")]
    #[doc = "Getter for the `error` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBTransaction/error)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomException`, `IdbTransaction`*"]
impl IdbTransaction {
    #[js::import = "$0.error"]
pub fn error(&self) -> Option<DomException> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onabort` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBTransaction/onabort)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbTransaction`*"]
impl IdbTransaction {
    #[js::import = "$0.onabort"]
pub fn onabort(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onabort` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBTransaction/onabort)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbTransaction`*"]
impl IdbTransaction {
    #[js::import = "$0.onabort = $1"]
pub fn set_onabort(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `oncomplete` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBTransaction/oncomplete)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbTransaction`*"]
impl IdbTransaction {
    #[js::import = "$0.oncomplete"]
pub fn oncomplete(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `oncomplete` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBTransaction/oncomplete)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbTransaction`*"]
impl IdbTransaction {
    #[js::import = "$0.oncomplete = $1"]
pub fn set_oncomplete(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBTransaction/onerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbTransaction`*"]
impl IdbTransaction {
    #[js::import = "$0.onerror"]
pub fn onerror(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBTransaction/onerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbTransaction`*"]
impl IdbTransaction {
    #[js::import = "$0.onerror = $1"]
pub fn set_onerror(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "The `abort()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBTransaction/abort)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbTransaction`*"]
impl IdbTransaction {
    #[js::import = "$0.abort()"]
pub fn abort(&self) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `commit()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBTransaction/commit)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbTransaction`*"]
    #[deprecated]
impl IdbTransaction {
    #[js::import = "$0.commit()"]
pub fn commit(&self) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "IdbObjectStore")]
    #[doc = "The `objectStore()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBTransaction/objectStore)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbObjectStore`, `IdbTransaction`*"]
impl IdbTransaction {
    #[js::import = "$0.objectStore($1)"]
pub fn object_store(&self, name: &str) -> Result<IdbObjectStore, JsValue> { panic!("web-sys intrinsic") }
}
