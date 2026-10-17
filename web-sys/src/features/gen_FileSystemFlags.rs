#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `FileSystemFlags` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemFlags`*"]
#[js::opaque]
#[derive(Copy)]
pub struct FileSystemFlags(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `create` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemFlags`*"]
#[js::import = "None"]
pub fn get_create(this: &FileSystemFlags) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `create` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemFlags`*"]
#[js::import = "None = $1"]
pub fn set_create(this: &FileSystemFlags, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `exclusive` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemFlags`*"]
#[js::import = "None"]
pub fn get_exclusive(this: &FileSystemFlags) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `exclusive` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemFlags`*"]
#[js::import = "None = $1"]
pub fn set_exclusive(this: &FileSystemFlags, val: bool) { panic!("web-sys intrinsic") }
impl FileSystemFlags {
    #[doc = "Construct a new `FileSystemFlags`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemFlags`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_create()` instead."]
#[js::import = "None"]
pub fn create(&mut self, val: bool) -> &mut Self { self.set_create(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_exclusive()` instead."]
#[js::import = "None"]
pub fn exclusive(&mut self, val: bool) -> &mut Self { self.set_exclusive(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for FileSystemFlags {
    fn default() -> Self {
        Self::new()
    }
}
