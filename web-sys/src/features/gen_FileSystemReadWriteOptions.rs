#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `FileSystemReadWriteOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemReadWriteOptions`*"]
#[js::opaque]
#[derive(Copy)]
pub struct FileSystemReadWriteOptions(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `at` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemReadWriteOptions`*"]
#[js::import = "None"]
pub fn get_at(this: &FileSystemReadWriteOptions) -> Option<f64> { panic!("web-sys intrinsic") }
    #[doc = "Change the `at` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemReadWriteOptions`*"]
#[js::import = "None = $1"]
pub fn set_at(this: &FileSystemReadWriteOptions, val: f64) { panic!("web-sys intrinsic") }
impl FileSystemReadWriteOptions {
    #[doc = "Construct a new `FileSystemReadWriteOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemReadWriteOptions`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_at()` instead."]
#[js::import = "None"]
pub fn at(&mut self, val: f64) -> &mut Self { self.set_at(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for FileSystemReadWriteOptions {
    fn default() -> Self {
        Self::new()
    }
}
