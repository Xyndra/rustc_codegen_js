#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DataTransferItemList` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransferItemList)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DataTransferItemList`*"]
#[js::opaque]
#[derive(Copy)]
pub struct DataTransferItemList(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `length` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransferItemList/length)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DataTransferItemList`*"]
impl DataTransferItemList {
    #[js::import = "$0.length"]
pub fn length(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "DataTransferItem")]
    #[doc = "The `add()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransferItemList/add)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DataTransferItem`, `DataTransferItemList`*"]
impl DataTransferItemList {
    #[js::import = "$0.add($1, $2)"]
pub fn add_with_str_and_type(&self, data: &str, type_: &str) -> Result<Option<DataTransferItem>, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "DataTransferItem", feature = "File",))]
    #[doc = "The `add()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransferItemList/add)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DataTransferItem`, `DataTransferItemList`, `File`*"]
impl DataTransferItemList {
    #[js::import = "$0.add($1)"]
pub fn add_with_file(&self, data: &File) -> Result<Option<DataTransferItem>, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `clear()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransferItemList/clear)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DataTransferItemList`*"]
impl DataTransferItemList {
    #[js::import = "$0.clear()"]
pub fn clear(&self) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `remove()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransferItemList/remove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DataTransferItemList`*"]
impl DataTransferItemList {
    #[js::import = "$0.remove($1)"]
pub fn remove(&self, index: u32) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "DataTransferItem")]
    #[doc = "Indexing getter. As in the literal Javascript `this[key]`."]
    #[doc = ""]
    #[doc = ""]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DataTransferItem`, `DataTransferItemList`*"]
impl DataTransferItemList {
    #[js::import = "$0.None($1)"]
pub fn get(&self, index: u32) -> Option<DataTransferItem> { panic!("web-sys intrinsic") }
}
