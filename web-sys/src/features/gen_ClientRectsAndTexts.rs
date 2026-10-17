#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ClientRectsAndTexts` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ClientRectsAndTexts`*"]
#[js::opaque]
#[derive(Copy)]
pub struct ClientRectsAndTexts(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "DomRectList")]
    #[doc = "Get the `rectList` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ClientRectsAndTexts`, `DomRectList`*"]
#[js::import = "None"]
pub fn get_rect_list(this: &ClientRectsAndTexts) -> DomRectList { panic!("web-sys intrinsic") }
    #[cfg(feature = "DomRectList")]
    #[doc = "Change the `rectList` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ClientRectsAndTexts`, `DomRectList`*"]
#[js::import = "None = $1"]
pub fn set_rect_list(this: &ClientRectsAndTexts, val: &DomRectList) { panic!("web-sys intrinsic") }
    #[doc = "Get the `textList` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ClientRectsAndTexts`*"]
#[js::import = "None"]
pub fn get_text_list(this: &ClientRectsAndTexts) -> ::js_sys::Array { panic!("web-sys intrinsic") }
    #[doc = "Change the `textList` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ClientRectsAndTexts`*"]
#[js::import = "None = $1"]
pub fn set_text_list(this: &ClientRectsAndTexts, val: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
impl ClientRectsAndTexts {
    #[cfg(feature = "DomRectList")]
    #[doc = "Construct a new `ClientRectsAndTexts`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ClientRectsAndTexts`, `DomRectList`*"]
#[js::import = "None"]
pub fn new(rect_list: &DomRectList, text_list: &::wasm_bindgen::JsValue) -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret.set_rect_list(rect_list);
        ret.set_text_list(text_list);
        ret
    }
    #[cfg(feature = "DomRectList")]
    #[deprecated = "Use `set_rect_list()` instead."]
#[js::import = "None"]
pub fn rect_list(&mut self, val: &DomRectList) -> &mut Self { self.set_rect_list(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_text_list()` instead."]
#[js::import = "None"]
pub fn text_list(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self { self.set_text_list(val) { panic!("web-sys intrinsic") }
        self
    }
}
