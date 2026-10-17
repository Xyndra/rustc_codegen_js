#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `FileSystemCreateWritableOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemCreateWritableOptions`*"]
#[js::opaque]
#[derive(Copy)]
pub struct FileSystemCreateWritableOptions(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `keepExistingData` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemCreateWritableOptions`*"]
#[js::import = "None"]
pub fn get_keep_existing_data(this: &FileSystemCreateWritableOptions) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `keepExistingData` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemCreateWritableOptions`*"]
#[js::import = "None = $1"]
pub fn set_keep_existing_data(this: &FileSystemCreateWritableOptions, val: bool) { panic!("web-sys intrinsic") }
impl FileSystemCreateWritableOptions {
    #[doc = "Construct a new `FileSystemCreateWritableOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemCreateWritableOptions`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_keep_existing_data()` instead."]
#[js::import = "None"]
pub fn keep_existing_data(&mut self, val: bool) -> &mut Self { self.set_keep_existing_data(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for FileSystemCreateWritableOptions {
    fn default() -> Self {
        Self::new()
    }
}
