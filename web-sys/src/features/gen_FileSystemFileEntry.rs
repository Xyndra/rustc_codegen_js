#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `FileSystemFileEntry` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemFileEntry)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemFileEntry`*"]
#[js::opaque]
#[derive(Copy)]
pub struct FileSystemFileEntry(pub ::wasm_bindgen::JsValue);
    #[doc = "The `file()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemFileEntry/file)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemFileEntry`*"]
impl FileSystemFileEntry {
    #[js::import = "$0.file($1)"]
pub fn file_with_callback(&self, success_callback: &::js_sys::Function) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "FileCallback")]
    #[doc = "The `file()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemFileEntry/file)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileCallback`, `FileSystemFileEntry`*"]
impl FileSystemFileEntry {
    #[js::import = "$0.file($1)"]
pub fn file_with_file_callback(&self, success_callback: &FileCallback) { panic!("web-sys intrinsic") }
}
    #[doc = "The `file()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemFileEntry/file)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemFileEntry`*"]
impl FileSystemFileEntry {
    #[js::import = "$0.file($1, $2)"]
pub fn file_with_callback_and_callback(&self, success_callback: &::js_sys::Function, error_callback: &::js_sys::Function) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "FileCallback")]
    #[doc = "The `file()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemFileEntry/file)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileCallback`, `FileSystemFileEntry`*"]
impl FileSystemFileEntry {
    #[js::import = "$0.file($1, $2)"]
pub fn file_with_file_callback_and_callback(&self, success_callback: &FileCallback, error_callback: &::js_sys::Function) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "ErrorCallback")]
    #[doc = "The `file()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemFileEntry/file)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ErrorCallback`, `FileSystemFileEntry`*"]
impl FileSystemFileEntry {
    #[js::import = "$0.file($1, $2)"]
pub fn file_with_callback_and_error_callback(&self, success_callback: &::js_sys::Function, error_callback: &ErrorCallback) { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "ErrorCallback", feature = "FileCallback",))]
    #[doc = "The `file()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemFileEntry/file)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ErrorCallback`, `FileCallback`, `FileSystemFileEntry`*"]
impl FileSystemFileEntry {
    #[js::import = "$0.file($1, $2)"]
pub fn file_with_file_callback_and_error_callback(&self, success_callback: &FileCallback, error_callback: &ErrorCallback) { panic!("web-sys intrinsic") }
}
