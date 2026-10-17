#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `FileSystemFileHandle` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemFileHandle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemFileHandle`*"]
#[js::opaque]
#[derive(Copy)]
pub struct FileSystemFileHandle(pub ::wasm_bindgen::JsValue);
    #[doc = "The `createSyncAccessHandle()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemFileHandle/createSyncAccessHandle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemFileHandle`*"]
impl FileSystemFileHandle {
    #[js::import = "$0.createSyncAccessHandle()"]
pub fn create_sync_access_handle(&self) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[doc = "The `createWritable()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemFileHandle/createWritable)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemFileHandle`*"]
impl FileSystemFileHandle {
    #[js::import = "$0.createWritable()"]
pub fn create_writable(&self) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "FileSystemCreateWritableOptions")]
    #[doc = "The `createWritable()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemFileHandle/createWritable)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemCreateWritableOptions`, `FileSystemFileHandle`*"]
impl FileSystemFileHandle {
    #[js::import = "$0.createWritable($1)"]
pub fn create_writable_with_options(&self, options: &FileSystemCreateWritableOptions) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[doc = "The `getFile()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemFileHandle/getFile)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemFileHandle`*"]
impl FileSystemFileHandle {
    #[js::import = "$0.getFile()"]
pub fn get_file(&self) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
