#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Directory` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Directory)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Directory`*"]
#[js::opaque]
#[derive(Copy)]
pub struct Directory(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `name` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Directory/name)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Directory`*"]
impl Directory {
    #[js::import = "$0.name"]
pub fn name(&self) -> Result<::alloc::string::String, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `path` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Directory/path)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Directory`*"]
impl Directory {
    #[js::import = "$0.path"]
pub fn path(&self) -> Result<::alloc::string::String, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `getFiles()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Directory/getFiles)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Directory`*"]
impl Directory {
    #[js::import = "$0.getFiles()"]
pub fn get_files(&self) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `getFiles()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Directory/getFiles)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Directory`*"]
impl Directory {
    #[js::import = "$0.getFiles($1)"]
pub fn get_files_with_recursive_flag(&self, recursive_flag: bool) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `getFilesAndDirectories()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Directory/getFilesAndDirectories)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Directory`*"]
impl Directory {
    #[js::import = "$0.getFilesAndDirectories()"]
pub fn get_files_and_directories(&self) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
