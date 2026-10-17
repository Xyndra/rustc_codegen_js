#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `StyleSheetChangeEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StyleSheetChangeEventInit`*"]
#[js::opaque]
#[derive(Copy)]
pub struct StyleSheetChangeEventInit(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StyleSheetChangeEventInit`*"]
#[js::import = "None"]
pub fn get_bubbles(this: &StyleSheetChangeEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StyleSheetChangeEventInit`*"]
#[js::import = "None = $1"]
pub fn set_bubbles(this: &StyleSheetChangeEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StyleSheetChangeEventInit`*"]
#[js::import = "None"]
pub fn get_cancelable(this: &StyleSheetChangeEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StyleSheetChangeEventInit`*"]
#[js::import = "None = $1"]
pub fn set_cancelable(this: &StyleSheetChangeEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StyleSheetChangeEventInit`*"]
#[js::import = "None"]
pub fn get_composed(this: &StyleSheetChangeEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StyleSheetChangeEventInit`*"]
#[js::import = "None = $1"]
pub fn set_composed(this: &StyleSheetChangeEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `documentSheet` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StyleSheetChangeEventInit`*"]
#[js::import = "None"]
pub fn get_document_sheet(this: &StyleSheetChangeEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `documentSheet` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StyleSheetChangeEventInit`*"]
#[js::import = "None = $1"]
pub fn set_document_sheet(this: &StyleSheetChangeEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[cfg(feature = "CssStyleSheet")]
    #[doc = "Get the `stylesheet` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssStyleSheet`, `StyleSheetChangeEventInit`*"]
#[js::import = "None"]
pub fn get_stylesheet(this: &StyleSheetChangeEventInit) -> Option<CssStyleSheet> { panic!("web-sys intrinsic") }
    #[cfg(feature = "CssStyleSheet")]
    #[doc = "Change the `stylesheet` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssStyleSheet`, `StyleSheetChangeEventInit`*"]
#[js::import = "None = $1"]
pub fn set_stylesheet(this: &StyleSheetChangeEventInit, val: Option<&CssStyleSheet>) { panic!("web-sys intrinsic") }
impl StyleSheetChangeEventInit {
    #[doc = "Construct a new `StyleSheetChangeEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StyleSheetChangeEventInit`*"]
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
    #[deprecated = "Use `set_document_sheet()` instead."]
#[js::import = "None"]
pub fn document_sheet(&mut self, val: bool) -> &mut Self { self.set_document_sheet(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(feature = "CssStyleSheet")]
    #[deprecated = "Use `set_stylesheet()` instead."]
#[js::import = "None"]
pub fn stylesheet(&mut self, val: Option<&CssStyleSheet>) -> &mut Self { self.set_stylesheet(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for StyleSheetChangeEventInit {
    fn default() -> Self {
        Self::new()
    }
}
