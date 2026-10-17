#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Storage` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Storage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Storage`*"]
#[js::opaque]
#[derive(Copy)]
pub struct Storage(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `length` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Storage/length)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Storage`*"]
impl Storage {
    #[js::import = "$0.length"]
pub fn length(&self) -> Result<u32, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `clear()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Storage/clear)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Storage`*"]
impl Storage {
    #[js::import = "$0.clear()"]
pub fn clear(&self) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `getItem()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Storage/getItem)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Storage`*"]
impl Storage {
    #[js::import = "$0.getItem($1)"]
pub fn get_item(&self, key: &str) -> Result<Option<::alloc::string::String>, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `key()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Storage/key)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Storage`*"]
impl Storage {
    #[js::import = "$0.key($1)"]
pub fn key(&self, index: u32) -> Result<Option<::alloc::string::String>, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `removeItem()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Storage/removeItem)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Storage`*"]
impl Storage {
    #[js::import = "$0.removeItem($1)"]
pub fn remove_item(&self, key: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `setItem()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Storage/setItem)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Storage`*"]
impl Storage {
    #[js::import = "$0.setItem($1, $2)"]
pub fn set_item(&self, key: &str, value: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Indexing getter. As in the literal Javascript `this[key]`."]
    #[doc = ""]
    #[doc = ""]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Storage`*"]
impl Storage {
    #[js::import = "$0.None($1)"]
pub fn get(&self, key: &str) -> Result<Option<::alloc::string::String>, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Indexing setter. As in the literal Javascript `this[key] = value`."]
    #[doc = ""]
    #[doc = ""]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Storage`*"]
impl Storage {
    #[js::import = "$0.None($1, $2)"]
pub fn set(&self, key: &str, value: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Indexing deleter. As in the literal Javascript `delete this[key]`."]
    #[doc = ""]
    #[doc = ""]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Storage`*"]
impl Storage {
    #[js::import = "$0.None($1)"]
pub fn delete(&self, key: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
