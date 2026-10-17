#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `FileSystem` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystem)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystem`*"]
#[js::opaque]
#[derive(Copy)]
pub struct FileSystem(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `name` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystem/name)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystem`*"]
impl FileSystem {
    #[js::import = "$0.name"]
pub fn name(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "FileSystemDirectoryEntry")]
    #[doc = "Getter for the `root` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystem/root)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystem`, `FileSystemDirectoryEntry`*"]
impl FileSystem {
    #[js::import = "$0.root"]
pub fn root(&self) -> FileSystemDirectoryEntry { panic!("web-sys intrinsic") }
}
