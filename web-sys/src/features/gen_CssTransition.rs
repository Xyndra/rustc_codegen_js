#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CssTransition` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSTransition)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssTransition`*"]
#[js::opaque]
#[derive(Copy)]
pub struct CssTransition(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `transitionProperty` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSTransition/transitionProperty)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssTransition`*"]
impl CssTransition {
    #[js::import = "$0.transitionProperty"]
pub fn transition_property(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
