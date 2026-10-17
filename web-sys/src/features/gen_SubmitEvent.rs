#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SubmitEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubmitEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SubmitEvent`*"]
#[js::opaque]
#[derive(Copy)]
pub struct SubmitEvent(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "HtmlElement")]
    #[doc = "Getter for the `submitter` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubmitEvent/submitter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlElement`, `SubmitEvent`*"]
impl SubmitEvent {
    #[js::import = "$0.submitter"]
pub fn submitter(&self) -> Option<HtmlElement> { panic!("web-sys intrinsic") }
}
    #[doc = "The `new SubmitEvent(..)` constructor, creating a new instance of `SubmitEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubmitEvent/SubmitEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SubmitEvent`*"]
impl SubmitEvent {
    #[js::import = "new SubmitEvent($0)"]
pub fn new(type_: &str) -> Result<SubmitEvent, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SubmitEventInit")]
    #[doc = "The `new SubmitEvent(..)` constructor, creating a new instance of `SubmitEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SubmitEvent/SubmitEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SubmitEvent`, `SubmitEventInit`*"]
impl SubmitEvent {
    #[js::import = "new SubmitEvent($0, $1)"]
pub fn new_with_event_init_dict(type_: &str, event_init_dict: &SubmitEventInit,) -> Result<SubmitEvent, JsValue> { panic!("web-sys intrinsic") }
}
