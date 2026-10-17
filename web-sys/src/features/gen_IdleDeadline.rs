#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `IdleDeadline` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IdleDeadline)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdleDeadline`*"]
#[js::opaque]
#[derive(Copy)]
pub struct IdleDeadline(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `didTimeout` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IdleDeadline/didTimeout)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdleDeadline`*"]
impl IdleDeadline {
    #[js::import = "$0.didTimeout"]
pub fn did_timeout(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "The `timeRemaining()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IdleDeadline/timeRemaining)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdleDeadline`*"]
impl IdleDeadline {
    #[js::import = "$0.timeRemaining()"]
pub fn time_remaining(&self) -> f64 { panic!("web-sys intrinsic") }
}
