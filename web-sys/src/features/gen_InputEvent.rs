#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `InputEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/InputEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `InputEvent`*"]
#[js::opaque]
#[derive(Copy)]
pub struct InputEvent(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `isComposing` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/InputEvent/isComposing)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `InputEvent`*"]
impl InputEvent {
    #[js::import = "$0.isComposing"]
pub fn is_composing(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `inputType` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/InputEvent/inputType)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `InputEvent`*"]
impl InputEvent {
    #[js::import = "$0.inputType"]
pub fn input_type(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `data` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/InputEvent/data)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `InputEvent`*"]
impl InputEvent {
    #[js::import = "$0.data"]
pub fn data(&self) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "DataTransfer")]
    #[doc = "Getter for the `dataTransfer` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/InputEvent/dataTransfer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DataTransfer`, `InputEvent`*"]
impl InputEvent {
    #[js::import = "$0.dataTransfer"]
pub fn data_transfer(&self) -> Option<DataTransfer> { panic!("web-sys intrinsic") }
}
    #[doc = "The `new InputEvent(..)` constructor, creating a new instance of `InputEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/InputEvent/InputEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `InputEvent`*"]
impl InputEvent {
    #[js::import = "new InputEvent($0)"]
pub fn new(type_: &str) -> Result<InputEvent, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "InputEventInit")]
    #[doc = "The `new InputEvent(..)` constructor, creating a new instance of `InputEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/InputEvent/InputEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `InputEvent`, `InputEventInit`*"]
impl InputEvent {
    #[js::import = "new InputEvent($0, $1)"]
pub fn new_with_event_init_dict(type_: &str, event_init_dict: &InputEventInit,) -> Result<InputEvent, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `getTargetRanges()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/InputEvent/getTargetRanges)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `InputEvent`*"]
impl InputEvent {
    #[js::import = "$0.getTargetRanges()"]
pub fn get_target_ranges(&self) -> ::js_sys::Array { panic!("web-sys intrinsic") }
}
