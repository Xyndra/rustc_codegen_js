#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `FileSystemGetDirectoryOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemGetDirectoryOptions`*"]
#[js::opaque]
#[derive(Copy)]
pub struct FileSystemGetDirectoryOptions(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `create` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemGetDirectoryOptions`*"]
#[js::import = "None"]
pub fn get_create(this: &FileSystemGetDirectoryOptions) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `create` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemGetDirectoryOptions`*"]
#[js::import = "None = $1"]
pub fn set_create(this: &FileSystemGetDirectoryOptions, val: bool) { panic!("web-sys intrinsic") }
impl FileSystemGetDirectoryOptions {
    #[doc = "Construct a new `FileSystemGetDirectoryOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemGetDirectoryOptions`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_create()` instead."]
#[js::import = "None"]
pub fn create(&mut self, val: bool) -> &mut Self { self.set_create(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for FileSystemGetDirectoryOptions {
    fn default() -> Self {
        Self::new()
    }
}
