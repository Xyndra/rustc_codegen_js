#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ClipboardEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ClipboardEventInit`*"]
#[js::opaque]
#[derive(Copy)]
pub struct ClipboardEventInit(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ClipboardEventInit`*"]
#[js::import = "None"]
pub fn get_bubbles(this: &ClipboardEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ClipboardEventInit`*"]
#[js::import = "None = $1"]
pub fn set_bubbles(this: &ClipboardEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ClipboardEventInit`*"]
#[js::import = "None"]
pub fn get_cancelable(this: &ClipboardEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ClipboardEventInit`*"]
#[js::import = "None = $1"]
pub fn set_cancelable(this: &ClipboardEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ClipboardEventInit`*"]
#[js::import = "None"]
pub fn get_composed(this: &ClipboardEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ClipboardEventInit`*"]
#[js::import = "None = $1"]
pub fn set_composed(this: &ClipboardEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[cfg(feature = "DataTransfer")]
    #[doc = "Get the `clipboardData` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ClipboardEventInit`, `DataTransfer`*"]
#[js::import = "None"]
pub fn get_clipboard_data(this: &ClipboardEventInit) -> Option<DataTransfer> { panic!("web-sys intrinsic") }
    #[cfg(feature = "DataTransfer")]
    #[doc = "Change the `clipboardData` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ClipboardEventInit`, `DataTransfer`*"]
#[js::import = "None = $1"]
pub fn set_clipboard_data(this: &ClipboardEventInit, val: Option<&DataTransfer>) { panic!("web-sys intrinsic") }
impl ClipboardEventInit {
    #[doc = "Construct a new `ClipboardEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ClipboardEventInit`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_bubbles()` instead."]
#[js::import = "None"]
pub fn bubbles(&mut self, val: bool) -> &mut Self { self.set_bubbles(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_cancelable()` instead."]
#[js::import = "None"]
pub fn cancelable(&mut self, val: bool) -> &mut Self { self.set_cancelable(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_composed()` instead."]
#[js::import = "None"]
pub fn composed(&mut self, val: bool) -> &mut Self { self.set_composed(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(feature = "DataTransfer")]
    #[deprecated = "Use `set_clipboard_data()` instead."]
#[js::import = "None"]
pub fn clipboard_data(&mut self, val: Option<&DataTransfer>) -> &mut Self { self.set_clipboard_data(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for ClipboardEventInit {
    fn default() -> Self {
        Self::new()
    }
}
