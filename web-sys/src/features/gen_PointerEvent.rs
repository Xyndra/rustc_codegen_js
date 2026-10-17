#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PointerEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PointerEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEvent`*"]
#[js::opaque]
#[derive(Copy)]
pub struct PointerEvent(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `pointerId` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PointerEvent/pointerId)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEvent`*"]
impl PointerEvent {
    #[js::import = "$0.pointerId"]
pub fn pointer_id(&self) -> i32 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `width` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PointerEvent/width)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEvent`*"]
impl PointerEvent {
    #[js::import = "$0.width"]
pub fn width(&self) -> i32 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `height` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PointerEvent/height)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEvent`*"]
impl PointerEvent {
    #[js::import = "$0.height"]
pub fn height(&self) -> i32 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `pressure` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PointerEvent/pressure)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEvent`*"]
impl PointerEvent {
    #[js::import = "$0.pressure"]
pub fn pressure(&self) -> f32 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `tangentialPressure` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PointerEvent/tangentialPressure)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEvent`*"]
impl PointerEvent {
    #[js::import = "$0.tangentialPressure"]
pub fn tangential_pressure(&self) -> f32 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `tiltX` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PointerEvent/tiltX)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEvent`*"]
impl PointerEvent {
    #[js::import = "$0.tiltX"]
pub fn tilt_x(&self) -> i32 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `tiltY` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PointerEvent/tiltY)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEvent`*"]
impl PointerEvent {
    #[js::import = "$0.tiltY"]
pub fn tilt_y(&self) -> i32 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `twist` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PointerEvent/twist)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEvent`*"]
impl PointerEvent {
    #[js::import = "$0.twist"]
pub fn twist(&self) -> i32 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `pointerType` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PointerEvent/pointerType)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEvent`*"]
impl PointerEvent {
    #[js::import = "$0.pointerType"]
pub fn pointer_type(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `isPrimary` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PointerEvent/isPrimary)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEvent`*"]
impl PointerEvent {
    #[js::import = "$0.isPrimary"]
pub fn is_primary(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "The `new PointerEvent(..)` constructor, creating a new instance of `PointerEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PointerEvent/PointerEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEvent`*"]
impl PointerEvent {
    #[js::import = "new PointerEvent($0)"]
pub fn new(type_: &str) -> Result<PointerEvent, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "PointerEventInit")]
    #[doc = "The `new PointerEvent(..)` constructor, creating a new instance of `PointerEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PointerEvent/PointerEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEvent`, `PointerEventInit`*"]
impl PointerEvent {
    #[js::import = "new PointerEvent($0, $1)"]
pub fn new_with_event_init_dict(type_: &str, event_init_dict: &PointerEventInit,) -> Result<PointerEvent, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `getCoalescedEvents()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PointerEvent/getCoalescedEvents)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PointerEvent`*"]
impl PointerEvent {
    #[js::import = "$0.getCoalescedEvents()"]
pub fn get_coalesced_events(&self) -> ::js_sys::Array { panic!("web-sys intrinsic") }
}
