#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PageTransitionEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PageTransitionEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PageTransitionEvent`*"]
#[js::opaque]
#[derive(Copy)]
pub struct PageTransitionEvent(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `persisted` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PageTransitionEvent/persisted)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PageTransitionEvent`*"]
impl PageTransitionEvent {
    #[js::import = "$0.persisted"]
pub fn persisted(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "The `new PageTransitionEvent(..)` constructor, creating a new instance of `PageTransitionEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PageTransitionEvent/PageTransitionEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PageTransitionEvent`*"]
impl PageTransitionEvent {
    #[js::import = "new PageTransitionEvent($0)"]
pub fn new(type_: &str) -> Result<PageTransitionEvent, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "PageTransitionEventInit")]
    #[doc = "The `new PageTransitionEvent(..)` constructor, creating a new instance of `PageTransitionEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PageTransitionEvent/PageTransitionEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PageTransitionEvent`, `PageTransitionEventInit`*"]
impl PageTransitionEvent {
    #[js::import = "new PageTransitionEvent($0, $1)"]
pub fn new_with_event_init_dict(type_: &str, event_init_dict: &PageTransitionEventInit,) -> Result<PageTransitionEvent, JsValue> { panic!("web-sys intrinsic") }
}
