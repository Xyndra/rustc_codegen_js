#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `StorageManager` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StorageManager)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StorageManager`*"]
#[js::opaque]
#[derive(Copy)]
pub struct StorageManager(pub ::wasm_bindgen::JsValue);
    #[doc = "The `estimate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StorageManager/estimate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StorageManager`*"]
impl StorageManager {
    #[js::import = "$0.estimate()"]
pub fn estimate(&self) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `getDirectory()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StorageManager/getDirectory)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StorageManager`*"]
impl StorageManager {
    #[js::import = "$0.getDirectory()"]
pub fn get_directory(&self) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[doc = "The `persist()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StorageManager/persist)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StorageManager`*"]
impl StorageManager {
    #[js::import = "$0.persist()"]
pub fn persist(&self) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `persisted()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StorageManager/persisted)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StorageManager`*"]
impl StorageManager {
    #[js::import = "$0.persisted()"]
pub fn persisted(&self) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
