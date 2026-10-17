#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `FileSystemGetFileOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemGetFileOptions`*"]
#[js::opaque]
#[derive(Copy)]
pub struct FileSystemGetFileOptions(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `create` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemGetFileOptions`*"]
#[js::import = "None"]
pub fn get_create(this: &FileSystemGetFileOptions) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `create` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemGetFileOptions`*"]
#[js::import = "None = $1"]
pub fn set_create(this: &FileSystemGetFileOptions, val: bool) { panic!("web-sys intrinsic") }
impl FileSystemGetFileOptions {
    #[doc = "Construct a new `FileSystemGetFileOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemGetFileOptions`*"]
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
impl Default for FileSystemGetFileOptions {
    fn default() -> Self {
        Self::new()
    }
}
