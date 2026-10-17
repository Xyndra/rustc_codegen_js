#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CompositionEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CompositionEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CompositionEvent`*"]
#[js::opaque]
#[derive(Copy)]
pub struct CompositionEvent(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `data` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CompositionEvent/data)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CompositionEvent`*"]
impl CompositionEvent {
    #[js::import = "$0.data"]
pub fn data(&self) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `locale` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CompositionEvent/locale)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CompositionEvent`*"]
impl CompositionEvent {
    #[js::import = "$0.locale"]
pub fn locale(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "The `new CompositionEvent(..)` constructor, creating a new instance of `CompositionEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CompositionEvent/CompositionEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CompositionEvent`*"]
impl CompositionEvent {
    #[js::import = "new CompositionEvent($0)"]
pub fn new(type_: &str) -> Result<CompositionEvent, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "CompositionEventInit")]
    #[doc = "The `new CompositionEvent(..)` constructor, creating a new instance of `CompositionEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CompositionEvent/CompositionEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CompositionEvent`, `CompositionEventInit`*"]
impl CompositionEvent {
    #[js::import = "new CompositionEvent($0, $1)"]
pub fn new_with_event_init_dict(type_: &str, event_init_dict: &CompositionEventInit,) -> Result<CompositionEvent, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `initCompositionEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CompositionEvent/initCompositionEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CompositionEvent`*"]
impl CompositionEvent {
    #[js::import = "$0.initCompositionEvent($1)"]
pub fn init_composition_event(&self, type_arg: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "The `initCompositionEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CompositionEvent/initCompositionEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CompositionEvent`*"]
impl CompositionEvent {
    #[js::import = "$0.initCompositionEvent($1, $2)"]
pub fn init_composition_event_with_can_bubble_arg(&self, type_arg: &str, can_bubble_arg: bool) { panic!("web-sys intrinsic") }
}
    #[doc = "The `initCompositionEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CompositionEvent/initCompositionEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CompositionEvent`*"]
impl CompositionEvent {
    #[js::import = "$0.initCompositionEvent($1, $2, $3)"]
pub fn init_composition_event_with_can_bubble_arg_and_cancelable_arg(&self, type_arg: &str, can_bubble_arg: bool, cancelable_arg: bool) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Window")]
    #[doc = "The `initCompositionEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CompositionEvent/initCompositionEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CompositionEvent`, `Window`*"]
impl CompositionEvent {
    #[js::import = "$0.initCompositionEvent($1, $2, $3, $4)"]
pub fn init_composition_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg(&self, type_arg: &str, can_bubble_arg: bool, cancelable_arg: bool, view_arg: Option<&Window>) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Window")]
    #[doc = "The `initCompositionEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CompositionEvent/initCompositionEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CompositionEvent`, `Window`*"]
impl CompositionEvent {
    #[js::import = "$0.initCompositionEvent($1, $2, $3, $4, $5)"]
pub fn init_composition_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg_and_data_arg(&self, type_arg: &str, can_bubble_arg: bool, cancelable_arg: bool, view_arg: Option<&Window>, data_arg: Option<&str>) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Window")]
    #[doc = "The `initCompositionEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CompositionEvent/initCompositionEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CompositionEvent`, `Window`*"]
impl CompositionEvent {
    #[js::import = "$0.initCompositionEvent($1, $2, $3, $4, $5, $6)"]
pub fn init_composition_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg_and_data_arg_and_locale_arg(&self, type_arg: &str, can_bubble_arg: bool, cancelable_arg: bool, view_arg: Option<&Window>, data_arg: Option<&str>, locale_arg: &str) { panic!("web-sys intrinsic") }
}
