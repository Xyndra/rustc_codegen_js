#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `FileSystemDirectoryEntry` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryEntry)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemDirectoryEntry`*"]
#[js::opaque]
#[derive(Copy)]
pub struct FileSystemDirectoryEntry(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "FileSystemDirectoryReader")]
    #[doc = "The `createReader()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryEntry/createReader)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemDirectoryEntry`, `FileSystemDirectoryReader`*"]
impl FileSystemDirectoryEntry {
    #[js::import = "$0.createReader()"]
pub fn create_reader(&self) -> FileSystemDirectoryReader { panic!("web-sys intrinsic") }
}
    #[doc = "The `getDirectory()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryEntry/getDirectory)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemDirectoryEntry`*"]
impl FileSystemDirectoryEntry {
    #[js::import = "$0.getDirectory()"]
pub fn get_directory(&self) { panic!("web-sys intrinsic") }
}
    #[doc = "The `getDirectory()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryEntry/getDirectory)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemDirectoryEntry`*"]
impl FileSystemDirectoryEntry {
    #[js::import = "$0.getDirectory($1)"]
pub fn get_directory_with_path(&self, path: Option<&str>) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "FileSystemFlags")]
    #[doc = "The `getDirectory()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryEntry/getDirectory)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemDirectoryEntry`, `FileSystemFlags`*"]
impl FileSystemDirectoryEntry {
    #[js::import = "$0.getDirectory($1, $2)"]
pub fn get_directory_with_path_and_options(&self, path: Option<&str>, options: &FileSystemFlags) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "FileSystemFlags")]
    #[doc = "The `getDirectory()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryEntry/getDirectory)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemDirectoryEntry`, `FileSystemFlags`*"]
impl FileSystemDirectoryEntry {
    #[js::import = "$0.getDirectory($1, $2, $3)"]
pub fn get_directory_with_path_and_options_and_callback(&self, path: Option<&str>, options: &FileSystemFlags, success_callback: &::js_sys::Function) { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "FileSystemEntryCallback", feature = "FileSystemFlags",))]
    #[doc = "The `getDirectory()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryEntry/getDirectory)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemDirectoryEntry`, `FileSystemEntryCallback`, `FileSystemFlags`*"]
impl FileSystemDirectoryEntry {
    #[js::import = "$0.getDirectory($1, $2, $3)"]
pub fn get_directory_with_path_and_options_and_file_system_entry_callback(&self, path: Option<&str>, options: &FileSystemFlags, success_callback: &FileSystemEntryCallback) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "FileSystemFlags")]
    #[doc = "The `getDirectory()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryEntry/getDirectory)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemDirectoryEntry`, `FileSystemFlags`*"]
impl FileSystemDirectoryEntry {
    #[js::import = "$0.getDirectory($1, $2, $3, $4)"]
pub fn get_directory_with_path_and_options_and_callback_and_callback(&self, path: Option<&str>, options: &FileSystemFlags, success_callback: &::js_sys::Function, error_callback: &::js_sys::Function) { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "FileSystemEntryCallback", feature = "FileSystemFlags",))]
    #[doc = "The `getDirectory()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryEntry/getDirectory)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemDirectoryEntry`, `FileSystemEntryCallback`, `FileSystemFlags`*"]
impl FileSystemDirectoryEntry {
    #[js::import = "$0.getDirectory($1, $2, $3, $4)"]
pub fn get_directory_with_path_and_options_and_file_system_entry_callback_and_callback(&self, path: Option<&str>, options: &FileSystemFlags, success_callback: &FileSystemEntryCallback, error_callback: &::js_sys::Function) { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "ErrorCallback", feature = "FileSystemFlags",))]
    #[doc = "The `getDirectory()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryEntry/getDirectory)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ErrorCallback`, `FileSystemDirectoryEntry`, `FileSystemFlags`*"]
impl FileSystemDirectoryEntry {
    #[js::import = "$0.getDirectory($1, $2, $3, $4)"]
pub fn get_directory_with_path_and_options_and_callback_and_error_callback(&self, path: Option<&str>, options: &FileSystemFlags, success_callback: &::js_sys::Function, error_callback: &ErrorCallback) { panic!("web-sys intrinsic") }
}
    #[cfg(all(
        feature = "ErrorCallback",
        feature = "FileSystemEntryCallback",
        feature = "FileSystemFlags",
    ))]
    #[doc = "The `getDirectory()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryEntry/getDirectory)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ErrorCallback`, `FileSystemDirectoryEntry`, `FileSystemEntryCallback`, `FileSystemFlags`*"]
impl FileSystemDirectoryEntry {
    #[js::import = "$0.getDirectory($1, $2, $3, $4)"]
pub fn get_directory_with_path_and_options_and_file_system_entry_callback_and_error_callback(&self, path: Option<&str>, options: &FileSystemFlags, success_callback: &FileSystemEntryCallback, error_callback: &ErrorCallback) { panic!("web-sys intrinsic") }
}
    #[doc = "The `getFile()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryEntry/getFile)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemDirectoryEntry`*"]
impl FileSystemDirectoryEntry {
    #[js::import = "$0.getFile()"]
pub fn get_file(&self) { panic!("web-sys intrinsic") }
}
    #[doc = "The `getFile()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryEntry/getFile)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemDirectoryEntry`*"]
impl FileSystemDirectoryEntry {
    #[js::import = "$0.getFile($1)"]
pub fn get_file_with_path(&self, path: Option<&str>) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "FileSystemFlags")]
    #[doc = "The `getFile()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryEntry/getFile)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemDirectoryEntry`, `FileSystemFlags`*"]
impl FileSystemDirectoryEntry {
    #[js::import = "$0.getFile($1, $2)"]
pub fn get_file_with_path_and_options(&self, path: Option<&str>, options: &FileSystemFlags) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "FileSystemFlags")]
    #[doc = "The `getFile()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryEntry/getFile)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemDirectoryEntry`, `FileSystemFlags`*"]
impl FileSystemDirectoryEntry {
    #[js::import = "$0.getFile($1, $2, $3)"]
pub fn get_file_with_path_and_options_and_callback(&self, path: Option<&str>, options: &FileSystemFlags, success_callback: &::js_sys::Function) { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "FileSystemEntryCallback", feature = "FileSystemFlags",))]
    #[doc = "The `getFile()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryEntry/getFile)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemDirectoryEntry`, `FileSystemEntryCallback`, `FileSystemFlags`*"]
impl FileSystemDirectoryEntry {
    #[js::import = "$0.getFile($1, $2, $3)"]
pub fn get_file_with_path_and_options_and_file_system_entry_callback(&self, path: Option<&str>, options: &FileSystemFlags, success_callback: &FileSystemEntryCallback) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "FileSystemFlags")]
    #[doc = "The `getFile()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryEntry/getFile)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemDirectoryEntry`, `FileSystemFlags`*"]
impl FileSystemDirectoryEntry {
    #[js::import = "$0.getFile($1, $2, $3, $4)"]
pub fn get_file_with_path_and_options_and_callback_and_callback(&self, path: Option<&str>, options: &FileSystemFlags, success_callback: &::js_sys::Function, error_callback: &::js_sys::Function) { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "FileSystemEntryCallback", feature = "FileSystemFlags",))]
    #[doc = "The `getFile()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryEntry/getFile)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemDirectoryEntry`, `FileSystemEntryCallback`, `FileSystemFlags`*"]
impl FileSystemDirectoryEntry {
    #[js::import = "$0.getFile($1, $2, $3, $4)"]
pub fn get_file_with_path_and_options_and_file_system_entry_callback_and_callback(&self, path: Option<&str>, options: &FileSystemFlags, success_callback: &FileSystemEntryCallback, error_callback: &::js_sys::Function) { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "ErrorCallback", feature = "FileSystemFlags",))]
    #[doc = "The `getFile()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryEntry/getFile)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ErrorCallback`, `FileSystemDirectoryEntry`, `FileSystemFlags`*"]
impl FileSystemDirectoryEntry {
    #[js::import = "$0.getFile($1, $2, $3, $4)"]
pub fn get_file_with_path_and_options_and_callback_and_error_callback(&self, path: Option<&str>, options: &FileSystemFlags, success_callback: &::js_sys::Function, error_callback: &ErrorCallback) { panic!("web-sys intrinsic") }
}
    #[cfg(all(
        feature = "ErrorCallback",
        feature = "FileSystemEntryCallback",
        feature = "FileSystemFlags",
    ))]
    #[doc = "The `getFile()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryEntry/getFile)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ErrorCallback`, `FileSystemDirectoryEntry`, `FileSystemEntryCallback`, `FileSystemFlags`*"]
impl FileSystemDirectoryEntry {
    #[js::import = "$0.getFile($1, $2, $3, $4)"]
pub fn get_file_with_path_and_options_and_file_system_entry_callback_and_error_callback(&self, path: Option<&str>, options: &FileSystemFlags, success_callback: &FileSystemEntryCallback, error_callback: &ErrorCallback) { panic!("web-sys intrinsic") }
}
