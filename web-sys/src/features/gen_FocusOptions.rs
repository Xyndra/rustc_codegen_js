#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `FocusOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FocusOptions`*"]
#[js::opaque]
#[derive(Copy)]
pub struct FocusOptions(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `focusVisible` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FocusOptions`*"]
#[js::import = "None"]
pub fn get_focus_visible(this: &FocusOptions) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `focusVisible` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FocusOptions`*"]
#[js::import = "None = $1"]
pub fn set_focus_visible(this: &FocusOptions, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `preventScroll` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FocusOptions`*"]
#[js::import = "None"]
pub fn get_prevent_scroll(this: &FocusOptions) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `preventScroll` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FocusOptions`*"]
#[js::import = "None = $1"]
pub fn set_prevent_scroll(this: &FocusOptions, val: bool) { panic!("web-sys intrinsic") }
impl FocusOptions {
    #[doc = "Construct a new `FocusOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FocusOptions`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_focus_visible()` instead."]
#[js::import = "None"]
pub fn focus_visible(&mut self, val: bool) -> &mut Self { self.set_focus_visible(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_prevent_scroll()` instead."]
#[js::import = "None"]
pub fn prevent_scroll(&mut self, val: bool) -> &mut Self { self.set_prevent_scroll(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for FocusOptions {
    fn default() -> Self {
        Self::new()
    }
}
