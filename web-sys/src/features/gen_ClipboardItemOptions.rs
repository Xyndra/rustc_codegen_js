#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ClipboardItemOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ClipboardItemOptions`*"]
#[js::opaque]
#[derive(Copy)]
pub struct ClipboardItemOptions(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "PresentationStyle")]
    #[doc = "Get the `presentationStyle` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ClipboardItemOptions`, `PresentationStyle`*"]
#[js::import = "None"]
pub fn get_presentation_style(this: &ClipboardItemOptions) -> Option<PresentationStyle> { panic!("web-sys intrinsic") }
    #[cfg(feature = "PresentationStyle")]
    #[doc = "Change the `presentationStyle` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ClipboardItemOptions`, `PresentationStyle`*"]
#[js::import = "None = $1"]
pub fn set_presentation_style(this: &ClipboardItemOptions, val: PresentationStyle) { panic!("web-sys intrinsic") }
impl ClipboardItemOptions {
    #[doc = "Construct a new `ClipboardItemOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ClipboardItemOptions`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[cfg(feature = "PresentationStyle")]
    #[deprecated = "Use `set_presentation_style()` instead."]
#[js::import = "None"]
pub fn presentation_style(&mut self, val: PresentationStyle) -> &mut Self { self.set_presentation_style(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for ClipboardItemOptions {
    fn default() -> Self {
        Self::new()
    }
}
