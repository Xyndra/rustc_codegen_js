#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `FileSystemEntry` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemEntry)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemEntry`*"]
#[js::opaque]
#[derive(Copy)]
pub struct FileSystemEntry(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `isFile` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemEntry/isFile)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemEntry`*"]
impl FileSystemEntry {
    #[js::import = "$0.isFile"]
pub fn is_file(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `isDirectory` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemEntry/isDirectory)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemEntry`*"]
impl FileSystemEntry {
    #[js::import = "$0.isDirectory"]
pub fn is_directory(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `name` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemEntry/name)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemEntry`*"]
impl FileSystemEntry {
    #[js::import = "$0.name"]
pub fn name(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `fullPath` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemEntry/fullPath)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemEntry`*"]
impl FileSystemEntry {
    #[js::import = "$0.fullPath"]
pub fn full_path(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "FileSystem")]
    #[doc = "Getter for the `filesystem` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemEntry/filesystem)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystem`, `FileSystemEntry`*"]
impl FileSystemEntry {
    #[js::import = "$0.filesystem"]
pub fn filesystem(&self) -> FileSystem { panic!("web-sys intrinsic") }
}
    #[doc = "The `getParent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemEntry/getParent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemEntry`*"]
impl FileSystemEntry {
    #[js::import = "$0.getParent()"]
pub fn get_parent(&self) { panic!("web-sys intrinsic") }
}
    #[doc = "The `getParent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemEntry/getParent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemEntry`*"]
impl FileSystemEntry {
    #[js::import = "$0.getParent($1)"]
pub fn get_parent_with_callback(&self, success_callback: &::js_sys::Function) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "FileSystemEntryCallback")]
    #[doc = "The `getParent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemEntry/getParent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemEntry`, `FileSystemEntryCallback`*"]
impl FileSystemEntry {
    #[js::import = "$0.getParent($1)"]
pub fn get_parent_with_file_system_entry_callback(&self, success_callback: &FileSystemEntryCallback) { panic!("web-sys intrinsic") }
}
    #[doc = "The `getParent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemEntry/getParent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemEntry`*"]
impl FileSystemEntry {
    #[js::import = "$0.getParent($1, $2)"]
pub fn get_parent_with_callback_and_callback(&self, success_callback: &::js_sys::Function, error_callback: &::js_sys::Function) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "FileSystemEntryCallback")]
    #[doc = "The `getParent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemEntry/getParent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemEntry`, `FileSystemEntryCallback`*"]
impl FileSystemEntry {
    #[js::import = "$0.getParent($1, $2)"]
pub fn get_parent_with_file_system_entry_callback_and_callback(&self, success_callback: &FileSystemEntryCallback, error_callback: &::js_sys::Function) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "ErrorCallback")]
    #[doc = "The `getParent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemEntry/getParent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ErrorCallback`, `FileSystemEntry`*"]
impl FileSystemEntry {
    #[js::import = "$0.getParent($1, $2)"]
pub fn get_parent_with_callback_and_error_callback(&self, success_callback: &::js_sys::Function, error_callback: &ErrorCallback) { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "ErrorCallback", feature = "FileSystemEntryCallback",))]
    #[doc = "The `getParent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemEntry/getParent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ErrorCallback`, `FileSystemEntry`, `FileSystemEntryCallback`*"]
impl FileSystemEntry {
    #[js::import = "$0.getParent($1, $2)"]
pub fn get_parent_with_file_system_entry_callback_and_error_callback(&self, success_callback: &FileSystemEntryCallback, error_callback: &ErrorCallback) { panic!("web-sys intrinsic") }
}
