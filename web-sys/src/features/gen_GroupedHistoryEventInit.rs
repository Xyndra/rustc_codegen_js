#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GroupedHistoryEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GroupedHistoryEventInit`*"]
#[js::opaque]
#[derive(Copy)]
pub struct GroupedHistoryEventInit(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GroupedHistoryEventInit`*"]
#[js::import = "None"]
pub fn get_bubbles(this: &GroupedHistoryEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GroupedHistoryEventInit`*"]
#[js::import = "None = $1"]
pub fn set_bubbles(this: &GroupedHistoryEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GroupedHistoryEventInit`*"]
#[js::import = "None"]
pub fn get_cancelable(this: &GroupedHistoryEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GroupedHistoryEventInit`*"]
#[js::import = "None = $1"]
pub fn set_cancelable(this: &GroupedHistoryEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GroupedHistoryEventInit`*"]
#[js::import = "None"]
pub fn get_composed(this: &GroupedHistoryEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GroupedHistoryEventInit`*"]
#[js::import = "None = $1"]
pub fn set_composed(this: &GroupedHistoryEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[cfg(feature = "Element")]
    #[doc = "Get the `otherBrowser` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`, `GroupedHistoryEventInit`*"]
#[js::import = "None"]
pub fn get_other_browser(this: &GroupedHistoryEventInit) -> Option<Element> { panic!("web-sys intrinsic") }
    #[cfg(feature = "Element")]
    #[doc = "Change the `otherBrowser` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`, `GroupedHistoryEventInit`*"]
#[js::import = "None = $1"]
pub fn set_other_browser(this: &GroupedHistoryEventInit, val: Option<&Element>) { panic!("web-sys intrinsic") }
impl GroupedHistoryEventInit {
    #[doc = "Construct a new `GroupedHistoryEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GroupedHistoryEventInit`*"]
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
    #[cfg(feature = "Element")]
    #[deprecated = "Use `set_other_browser()` instead."]
#[js::import = "None"]
pub fn other_browser(&mut self, val: Option<&Element>) -> &mut Self { self.set_other_browser(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for GroupedHistoryEventInit {
    fn default() -> Self {
        Self::new()
    }
}
