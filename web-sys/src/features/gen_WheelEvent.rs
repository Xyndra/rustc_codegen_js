#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WheelEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WheelEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WheelEvent`*"]
#[js::opaque]
#[derive(Copy)]
pub struct WheelEvent(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `deltaX` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WheelEvent/deltaX)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WheelEvent`*"]
impl WheelEvent {
    #[js::import = "$0.deltaX"]
pub fn delta_x(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `deltaY` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WheelEvent/deltaY)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WheelEvent`*"]
impl WheelEvent {
    #[js::import = "$0.deltaY"]
pub fn delta_y(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `deltaZ` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WheelEvent/deltaZ)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WheelEvent`*"]
impl WheelEvent {
    #[js::import = "$0.deltaZ"]
pub fn delta_z(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `deltaMode` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WheelEvent/deltaMode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WheelEvent`*"]
impl WheelEvent {
    #[js::import = "$0.deltaMode"]
pub fn delta_mode(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[doc = "The `new WheelEvent(..)` constructor, creating a new instance of `WheelEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WheelEvent/WheelEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WheelEvent`*"]
impl WheelEvent {
    #[js::import = "new WheelEvent($0)"]
pub fn new(type_: &str) -> Result<WheelEvent, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "WheelEventInit")]
    #[doc = "The `new WheelEvent(..)` constructor, creating a new instance of `WheelEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WheelEvent/WheelEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WheelEvent`, `WheelEventInit`*"]
impl WheelEvent {
    #[js::import = "new WheelEvent($0, $1)"]
pub fn new_with_event_init_dict(type_: &str, event_init_dict: &WheelEventInit,) -> Result<WheelEvent, JsValue> { panic!("web-sys intrinsic") }
}
impl WheelEvent {
    #[doc = "The `WheelEvent.DOM_DELTA_PIXEL` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WheelEvent`*"]
    pub const DOM_DELTA_PIXEL: u32 = 0u64 as u32;
    #[doc = "The `WheelEvent.DOM_DELTA_LINE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WheelEvent`*"]
    pub const DOM_DELTA_LINE: u32 = 1u64 as u32;
    #[doc = "The `WheelEvent.DOM_DELTA_PAGE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WheelEvent`*"]
    pub const DOM_DELTA_PAGE: u32 = 2u64 as u32;
}
