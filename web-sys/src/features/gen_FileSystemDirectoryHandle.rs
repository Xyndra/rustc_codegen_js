#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `FileSystemDirectoryHandle` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryHandle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemDirectoryHandle`*"]
#[js::opaque]
#[derive(Copy)]
pub struct FileSystemDirectoryHandle(pub ::wasm_bindgen::JsValue);
    #[doc = "The `entries()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryHandle/entries)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemDirectoryHandle`*"]
impl FileSystemDirectoryHandle {
    #[js::import = "$0.entries()"]
pub fn entries(&self) -> ::js_sys::AsyncIterator { panic!("web-sys intrinsic") }
}
    #[doc = "The `getDirectoryHandle()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryHandle/getDirectoryHandle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemDirectoryHandle`*"]
impl FileSystemDirectoryHandle {
    #[js::import = "$0.getDirectoryHandle($1)"]
pub fn get_directory_handle(&self, name: &str) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "FileSystemGetDirectoryOptions")]
    #[doc = "The `getDirectoryHandle()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryHandle/getDirectoryHandle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemDirectoryHandle`, `FileSystemGetDirectoryOptions`*"]
impl FileSystemDirectoryHandle {
    #[js::import = "$0.getDirectoryHandle($1, $2)"]
pub fn get_directory_handle_with_options(&self, name: &str, options: &FileSystemGetDirectoryOptions) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[doc = "The `getFileHandle()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryHandle/getFileHandle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemDirectoryHandle`*"]
impl FileSystemDirectoryHandle {
    #[js::import = "$0.getFileHandle($1)"]
pub fn get_file_handle(&self, name: &str) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "FileSystemGetFileOptions")]
    #[doc = "The `getFileHandle()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryHandle/getFileHandle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemDirectoryHandle`, `FileSystemGetFileOptions`*"]
impl FileSystemDirectoryHandle {
    #[js::import = "$0.getFileHandle($1, $2)"]
pub fn get_file_handle_with_options(&self, name: &str, options: &FileSystemGetFileOptions) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[doc = "The `keys()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryHandle/keys)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemDirectoryHandle`*"]
impl FileSystemDirectoryHandle {
    #[js::import = "$0.keys()"]
pub fn keys(&self) -> ::js_sys::AsyncIterator { panic!("web-sys intrinsic") }
}
    #[doc = "The `removeEntry()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryHandle/removeEntry)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemDirectoryHandle`*"]
impl FileSystemDirectoryHandle {
    #[js::import = "$0.removeEntry($1)"]
pub fn remove_entry(&self, name: &str) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "FileSystemRemoveOptions")]
    #[doc = "The `removeEntry()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryHandle/removeEntry)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemDirectoryHandle`, `FileSystemRemoveOptions`*"]
impl FileSystemDirectoryHandle {
    #[js::import = "$0.removeEntry($1, $2)"]
pub fn remove_entry_with_options(&self, name: &str, options: &FileSystemRemoveOptions) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[doc = "The `resolve()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryHandle/resolve)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemDirectoryHandle`*"]
impl FileSystemDirectoryHandle {
    #[js::import = "$0.resolve($1)"]
pub fn resolve(&self, possible_descendant: &FileSystemHandle) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[doc = "The `values()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryHandle/values)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemDirectoryHandle`*"]
impl FileSystemDirectoryHandle {
    #[js::import = "$0.values()"]
pub fn values(&self) -> ::js_sys::AsyncIterator { panic!("web-sys intrinsic") }
}
