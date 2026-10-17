#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DataTransferItem` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransferItem)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DataTransferItem`*"]
#[js::opaque]
#[derive(Copy)]
pub struct DataTransferItem(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `kind` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransferItem/kind)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DataTransferItem`*"]
impl DataTransferItem {
    #[js::import = "$0.kind"]
pub fn kind(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransferItem/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DataTransferItem`*"]
impl DataTransferItem {
    #[js::import = "$0.type"]
pub fn type_(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "File")]
    #[doc = "The `getAsFile()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransferItem/getAsFile)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DataTransferItem`, `File`*"]
impl DataTransferItem {
    #[js::import = "$0.getAsFile()"]
pub fn get_as_file(&self) -> Result<Option<File>, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `getAsFileSystemHandle()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransferItem/getAsFileSystemHandle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DataTransferItem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl DataTransferItem {
    #[js::import = "$0.getAsFileSystemHandle()"]
pub fn get_as_file_system_handle(&self) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[doc = "The `getAsString()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransferItem/getAsString)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DataTransferItem`*"]
impl DataTransferItem {
    #[js::import = "$0.getAsString($1)"]
pub fn get_as_string(&self, callback: Option<&::js_sys::Function>) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "FileSystemEntry")]
    #[doc = "The `webkitGetAsEntry()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransferItem/webkitGetAsEntry)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DataTransferItem`, `FileSystemEntry`*"]
impl DataTransferItem {
    #[js::import = "$0.webkitGetAsEntry()"]
pub fn webkit_get_as_entry(&self) -> Result<Option<FileSystemEntry>, JsValue> { panic!("web-sys intrinsic") }
}
