#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `IdbFactory` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFactory)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbFactory`*"]
#[js::opaque]
#[derive(Copy)]
pub struct IdbFactory(pub ::wasm_bindgen::JsValue);
    #[doc = "The `cmp()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFactory/cmp)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbFactory`*"]
impl IdbFactory {
    #[js::import = "$0.cmp($1, $2)"]
pub fn cmp(&self, first: &::wasm_bindgen::JsValue, second: &::wasm_bindgen::JsValue) -> Result<i16, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "IdbOpenDbRequest")]
    #[doc = "The `deleteDatabase()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFactory/deleteDatabase)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbFactory`, `IdbOpenDbRequest`*"]
impl IdbFactory {
    #[js::import = "$0.deleteDatabase($1)"]
pub fn delete_database(&self, name: &str) -> Result<IdbOpenDbRequest, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "IdbOpenDbOptions", feature = "IdbOpenDbRequest",))]
    #[doc = "The `deleteDatabase()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFactory/deleteDatabase)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbFactory`, `IdbOpenDbOptions`, `IdbOpenDbRequest`*"]
    #[deprecated]
impl IdbFactory {
    #[js::import = "$0.deleteDatabase($1, $2)"]
pub fn delete_database_with_options(&self, name: &str, options: &IdbOpenDbOptions) -> Result<IdbOpenDbRequest, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "IdbOpenDbRequest")]
    #[doc = "The `open()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFactory/open)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbFactory`, `IdbOpenDbRequest`*"]
impl IdbFactory {
    #[js::import = "$0.open($1)"]
pub fn open(&self, name: &str) -> Result<IdbOpenDbRequest, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "IdbOpenDbRequest")]
    #[doc = "The `open()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFactory/open)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbFactory`, `IdbOpenDbRequest`*"]
impl IdbFactory {
    #[js::import = "$0.open($1, $2)"]
pub fn open_with_u32(&self, name: &str, version: u32) -> Result<IdbOpenDbRequest, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "IdbOpenDbRequest")]
    #[doc = "The `open()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFactory/open)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbFactory`, `IdbOpenDbRequest`*"]
impl IdbFactory {
    #[js::import = "$0.open($1, $2)"]
pub fn open_with_f64(&self, name: &str, version: f64) -> Result<IdbOpenDbRequest, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "IdbOpenDbOptions", feature = "IdbOpenDbRequest",))]
    #[doc = "The `open()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFactory/open)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbFactory`, `IdbOpenDbOptions`, `IdbOpenDbRequest`*"]
    #[deprecated]
impl IdbFactory {
    #[js::import = "$0.open($1, $2)"]
pub fn open_with_idb_open_db_options(&self, name: &str, options: &IdbOpenDbOptions) -> Result<IdbOpenDbRequest, JsValue> { panic!("web-sys intrinsic") }
}
