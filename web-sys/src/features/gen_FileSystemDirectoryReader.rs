#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `FileSystemDirectoryReader` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryReader)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemDirectoryReader`*"]
#[js::opaque]
#[derive(Copy)]
pub struct FileSystemDirectoryReader(pub ::wasm_bindgen::JsValue);
    #[doc = "The `readEntries()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryReader/readEntries)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemDirectoryReader`*"]
impl FileSystemDirectoryReader {
    #[js::import = "$0.readEntries($1)"]
pub fn read_entries_with_callback(&self, success_callback: &::js_sys::Function) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "FileSystemEntriesCallback")]
    #[doc = "The `readEntries()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryReader/readEntries)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemDirectoryReader`, `FileSystemEntriesCallback`*"]
impl FileSystemDirectoryReader {
    #[js::import = "$0.readEntries($1)"]
pub fn read_entries_with_file_system_entries_callback(&self, success_callback: &FileSystemEntriesCallback) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `readEntries()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryReader/readEntries)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemDirectoryReader`*"]
impl FileSystemDirectoryReader {
    #[js::import = "$0.readEntries($1, $2)"]
pub fn read_entries_with_callback_and_callback(&self, success_callback: &::js_sys::Function, error_callback: &::js_sys::Function) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "FileSystemEntriesCallback")]
    #[doc = "The `readEntries()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryReader/readEntries)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemDirectoryReader`, `FileSystemEntriesCallback`*"]
impl FileSystemDirectoryReader {
    #[js::import = "$0.readEntries($1, $2)"]
pub fn read_entries_with_file_system_entries_callback_and_callback(&self, success_callback: &FileSystemEntriesCallback, error_callback: &::js_sys::Function) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "ErrorCallback")]
    #[doc = "The `readEntries()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryReader/readEntries)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ErrorCallback`, `FileSystemDirectoryReader`*"]
impl FileSystemDirectoryReader {
    #[js::import = "$0.readEntries($1, $2)"]
pub fn read_entries_with_callback_and_error_callback(&self, success_callback: &::js_sys::Function, error_callback: &ErrorCallback) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "ErrorCallback", feature = "FileSystemEntriesCallback",))]
    #[doc = "The `readEntries()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryReader/readEntries)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ErrorCallback`, `FileSystemDirectoryReader`, `FileSystemEntriesCallback`*"]
impl FileSystemDirectoryReader {
    #[js::import = "$0.readEntries($1, $2)"]
pub fn read_entries_with_file_system_entries_callback_and_error_callback(&self, success_callback: &FileSystemEntriesCallback, error_callback: &ErrorCallback) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
