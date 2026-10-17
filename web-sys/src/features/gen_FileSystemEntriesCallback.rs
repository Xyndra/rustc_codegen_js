#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `FileSystemEntriesCallback` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemEntriesCallback`*"]
#[js::opaque]
#[derive(Copy)]
pub struct FileSystemEntriesCallback(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `handleEvent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemEntriesCallback`*"]
#[js::import = "None"]
pub fn get_handle_event(this: &FileSystemEntriesCallback) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
    #[doc = "Change the `handleEvent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemEntriesCallback`*"]
#[js::import = "None = $1"]
pub fn set_handle_event(this: &FileSystemEntriesCallback, val: &::js_sys::Function) { panic!("web-sys intrinsic") }
impl FileSystemEntriesCallback {
    #[doc = "Construct a new `FileSystemEntriesCallback`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemEntriesCallback`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_handle_event()` instead."]
#[js::import = "None"]
pub fn handle_event(&mut self, val: &::js_sys::Function) -> &mut Self { self.set_handle_event(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for FileSystemEntriesCallback {
    fn default() -> Self {
        Self::new()
    }
}
