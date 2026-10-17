#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `FileSystemRemoveOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemRemoveOptions`*"]
#[js::opaque]
#[derive(Copy)]
pub struct FileSystemRemoveOptions(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `recursive` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemRemoveOptions`*"]
#[js::import = "None"]
pub fn get_recursive(this: &FileSystemRemoveOptions) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `recursive` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemRemoveOptions`*"]
#[js::import = "None = $1"]
pub fn set_recursive(this: &FileSystemRemoveOptions, val: bool) { panic!("web-sys intrinsic") }
impl FileSystemRemoveOptions {
    #[doc = "Construct a new `FileSystemRemoveOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemRemoveOptions`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_recursive()` instead."]
#[js::import = "None"]
pub fn recursive(&mut self, val: bool) -> &mut Self { self.set_recursive(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for FileSystemRemoveOptions {
    fn default() -> Self {
        Self::new()
    }
}
