#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DataTransfer` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransfer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DataTransfer`*"]
#[js::opaque]
#[derive(Copy)]
pub struct DataTransfer(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `dropEffect` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransfer/dropEffect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DataTransfer`*"]
impl DataTransfer {
    #[js::import = "$0.dropEffect"]
pub fn drop_effect(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `dropEffect` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransfer/dropEffect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DataTransfer`*"]
impl DataTransfer {
    #[js::import = "$0.dropEffect = $1"]
pub fn set_drop_effect(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `effectAllowed` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransfer/effectAllowed)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DataTransfer`*"]
impl DataTransfer {
    #[js::import = "$0.effectAllowed"]
pub fn effect_allowed(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `effectAllowed` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransfer/effectAllowed)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DataTransfer`*"]
impl DataTransfer {
    #[js::import = "$0.effectAllowed = $1"]
pub fn set_effect_allowed(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "DataTransferItemList")]
    #[doc = "Getter for the `items` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransfer/items)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DataTransfer`, `DataTransferItemList`*"]
impl DataTransfer {
    #[js::import = "$0.items"]
pub fn items(&self) -> DataTransferItemList { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `types` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransfer/types)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DataTransfer`*"]
impl DataTransfer {
    #[js::import = "$0.types"]
pub fn types(&self) -> ::js_sys::Array { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "FileList")]
    #[doc = "Getter for the `files` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransfer/files)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DataTransfer`, `FileList`*"]
impl DataTransfer {
    #[js::import = "$0.files"]
pub fn files(&self) -> Option<FileList> { panic!("web-sys intrinsic") }
}
    #[doc = "The `new DataTransfer(..)` constructor, creating a new instance of `DataTransfer`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransfer/DataTransfer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DataTransfer`*"]
impl DataTransfer {
    #[js::import = "new DataTransfer()"]
pub fn new() -> Result<DataTransfer, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `clearData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransfer/clearData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DataTransfer`*"]
impl DataTransfer {
    #[js::import = "$0.clearData()"]
pub fn clear_data(&self) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `clearData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransfer/clearData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DataTransfer`*"]
impl DataTransfer {
    #[js::import = "$0.clearData($1)"]
pub fn clear_data_with_format(&self, format: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `getData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransfer/getData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DataTransfer`*"]
impl DataTransfer {
    #[js::import = "$0.getData($1)"]
pub fn get_data(&self, format: &str) -> Result<::alloc::string::String, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `getFiles()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransfer/getFiles)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DataTransfer`*"]
impl DataTransfer {
    #[js::import = "$0.getFiles()"]
pub fn get_files(&self) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `getFiles()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransfer/getFiles)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DataTransfer`*"]
impl DataTransfer {
    #[js::import = "$0.getFiles($1)"]
pub fn get_files_with_recursive_flag(&self, recursive_flag: bool) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `getFilesAndDirectories()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransfer/getFilesAndDirectories)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DataTransfer`*"]
impl DataTransfer {
    #[js::import = "$0.getFilesAndDirectories()"]
pub fn get_files_and_directories(&self) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `setData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransfer/setData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DataTransfer`*"]
impl DataTransfer {
    #[js::import = "$0.setData($1, $2)"]
pub fn set_data(&self, format: &str, data: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Element")]
    #[doc = "The `setDragImage()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransfer/setDragImage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DataTransfer`, `Element`*"]
impl DataTransfer {
    #[js::import = "$0.setDragImage($1, $2, $3)"]
pub fn set_drag_image(&self, image: &Element, x: i32, y: i32) { panic!("web-sys intrinsic") }
}
