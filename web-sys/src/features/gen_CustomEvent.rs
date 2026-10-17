#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CustomEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CustomEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CustomEvent`*"]
#[js::opaque]
#[derive(Copy)]
pub struct CustomEvent(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `detail` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CustomEvent/detail)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CustomEvent`*"]
impl CustomEvent {
    #[js::import = "$0.detail"]
pub fn detail(&self) -> ::wasm_bindgen::JsValue { panic!("web-sys intrinsic") }
}
    #[doc = "The `new CustomEvent(..)` constructor, creating a new instance of `CustomEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CustomEvent/CustomEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CustomEvent`*"]
impl CustomEvent {
    #[js::import = "new CustomEvent($0)"]
pub fn new(type_: &str) -> Result<CustomEvent, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "CustomEventInit")]
    #[doc = "The `new CustomEvent(..)` constructor, creating a new instance of `CustomEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CustomEvent/CustomEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CustomEvent`, `CustomEventInit`*"]
impl CustomEvent {
    #[js::import = "new CustomEvent($0, $1)"]
pub fn new_with_event_init_dict(type_: &str, event_init_dict: &CustomEventInit,) -> Result<CustomEvent, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `initCustomEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CustomEvent/initCustomEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CustomEvent`*"]
impl CustomEvent {
    #[js::import = "$0.initCustomEvent($1)"]
pub fn init_custom_event(&self, type_: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "The `initCustomEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CustomEvent/initCustomEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CustomEvent`*"]
impl CustomEvent {
    #[js::import = "$0.initCustomEvent($1, $2)"]
pub fn init_custom_event_with_can_bubble(&self, type_: &str, can_bubble: bool) { panic!("web-sys intrinsic") }
}
    #[doc = "The `initCustomEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CustomEvent/initCustomEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CustomEvent`*"]
impl CustomEvent {
    #[js::import = "$0.initCustomEvent($1, $2, $3)"]
pub fn init_custom_event_with_can_bubble_and_cancelable(&self, type_: &str, can_bubble: bool, cancelable: bool) { panic!("web-sys intrinsic") }
}
    #[doc = "The `initCustomEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CustomEvent/initCustomEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CustomEvent`*"]
impl CustomEvent {
    #[js::import = "$0.initCustomEvent($1, $2, $3, $4)"]
pub fn init_custom_event_with_can_bubble_and_cancelable_and_detail(&self, type_: &str, can_bubble: bool, cancelable: bool, detail: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
}
