#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `TransitionEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TransitionEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TransitionEvent`*"]
#[js::opaque]
#[derive(Copy)]
pub struct TransitionEvent(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `propertyName` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TransitionEvent/propertyName)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TransitionEvent`*"]
impl TransitionEvent {
    #[js::import = "$0.propertyName"]
pub fn property_name(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `elapsedTime` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TransitionEvent/elapsedTime)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TransitionEvent`*"]
impl TransitionEvent {
    #[js::import = "$0.elapsedTime"]
pub fn elapsed_time(&self) -> f32 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `pseudoElement` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TransitionEvent/pseudoElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TransitionEvent`*"]
impl TransitionEvent {
    #[js::import = "$0.pseudoElement"]
pub fn pseudo_element(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "The `new TransitionEvent(..)` constructor, creating a new instance of `TransitionEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TransitionEvent/TransitionEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TransitionEvent`*"]
impl TransitionEvent {
    #[js::import = "new TransitionEvent($0)"]
pub fn new(type_: &str) -> Result<TransitionEvent, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "TransitionEventInit")]
    #[doc = "The `new TransitionEvent(..)` constructor, creating a new instance of `TransitionEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TransitionEvent/TransitionEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TransitionEvent`, `TransitionEventInit`*"]
impl TransitionEvent {
    #[js::import = "new TransitionEvent($0, $1)"]
pub fn new_with_event_init_dict(type_: &str, event_init_dict: &TransitionEventInit,) -> Result<TransitionEvent, JsValue> { panic!("web-sys intrinsic") }
}
