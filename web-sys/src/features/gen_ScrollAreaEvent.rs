#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ScrollAreaEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScrollAreaEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScrollAreaEvent`*"]
#[js::opaque]
#[derive(Copy)]
pub struct ScrollAreaEvent(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `x` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScrollAreaEvent/x)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScrollAreaEvent`*"]
impl ScrollAreaEvent {
    #[js::import = "$0.x"]
pub fn x(&self) -> f32 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `y` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScrollAreaEvent/y)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScrollAreaEvent`*"]
impl ScrollAreaEvent {
    #[js::import = "$0.y"]
pub fn y(&self) -> f32 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `width` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScrollAreaEvent/width)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScrollAreaEvent`*"]
impl ScrollAreaEvent {
    #[js::import = "$0.width"]
pub fn width(&self) -> f32 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `height` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScrollAreaEvent/height)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScrollAreaEvent`*"]
impl ScrollAreaEvent {
    #[js::import = "$0.height"]
pub fn height(&self) -> f32 { panic!("web-sys intrinsic") }
}
    #[doc = "The `initScrollAreaEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScrollAreaEvent/initScrollAreaEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScrollAreaEvent`*"]
impl ScrollAreaEvent {
    #[js::import = "$0.initScrollAreaEvent($1)"]
pub fn init_scroll_area_event(&self, type_: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "The `initScrollAreaEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScrollAreaEvent/initScrollAreaEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScrollAreaEvent`*"]
impl ScrollAreaEvent {
    #[js::import = "$0.initScrollAreaEvent($1, $2)"]
pub fn init_scroll_area_event_with_can_bubble(&self, type_: &str, can_bubble: bool) { panic!("web-sys intrinsic") }
}
    #[doc = "The `initScrollAreaEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScrollAreaEvent/initScrollAreaEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScrollAreaEvent`*"]
impl ScrollAreaEvent {
    #[js::import = "$0.initScrollAreaEvent($1, $2, $3)"]
pub fn init_scroll_area_event_with_can_bubble_and_cancelable(&self, type_: &str, can_bubble: bool, cancelable: bool) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Window")]
    #[doc = "The `initScrollAreaEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScrollAreaEvent/initScrollAreaEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScrollAreaEvent`, `Window`*"]
impl ScrollAreaEvent {
    #[js::import = "$0.initScrollAreaEvent($1, $2, $3, $4)"]
pub fn init_scroll_area_event_with_can_bubble_and_cancelable_and_view(&self, type_: &str, can_bubble: bool, cancelable: bool, view: Option<&Window>) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Window")]
    #[doc = "The `initScrollAreaEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScrollAreaEvent/initScrollAreaEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScrollAreaEvent`, `Window`*"]
impl ScrollAreaEvent {
    #[js::import = "$0.initScrollAreaEvent($1, $2, $3, $4, $5)"]
pub fn init_scroll_area_event_with_can_bubble_and_cancelable_and_view_and_detail(&self, type_: &str, can_bubble: bool, cancelable: bool, view: Option<&Window>, detail: i32) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Window")]
    #[doc = "The `initScrollAreaEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScrollAreaEvent/initScrollAreaEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScrollAreaEvent`, `Window`*"]
impl ScrollAreaEvent {
    #[js::import = "$0.initScrollAreaEvent($1, $2, $3, $4, $5, $6)"]
pub fn init_scroll_area_event_with_can_bubble_and_cancelable_and_view_and_detail_and_x(&self, type_: &str, can_bubble: bool, cancelable: bool, view: Option<&Window>, detail: i32, x: f32) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Window")]
    #[doc = "The `initScrollAreaEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScrollAreaEvent/initScrollAreaEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScrollAreaEvent`, `Window`*"]
impl ScrollAreaEvent {
    #[js::import = "$0.initScrollAreaEvent($1, $2, $3, $4, $5, $6, $7)"]
pub fn init_scroll_area_event_with_can_bubble_and_cancelable_and_view_and_detail_and_x_and_y(&self, type_: &str, can_bubble: bool, cancelable: bool, view: Option<&Window>, detail: i32, x: f32, y: f32) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Window")]
    #[doc = "The `initScrollAreaEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScrollAreaEvent/initScrollAreaEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScrollAreaEvent`, `Window`*"]
impl ScrollAreaEvent {
    #[js::import = "$0.initScrollAreaEvent($1, $2, $3, $4, $5, $6, $7, $8)"]
pub fn init_scroll_area_event_with_can_bubble_and_cancelable_and_view_and_detail_and_x_and_y_and_width(&self, type_: &str, can_bubble: bool, cancelable: bool, view: Option<&Window>, detail: i32, x: f32, y: f32, width: f32) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Window")]
    #[doc = "The `initScrollAreaEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScrollAreaEvent/initScrollAreaEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScrollAreaEvent`, `Window`*"]
impl ScrollAreaEvent {
    #[js::import = "$0.initScrollAreaEvent($1, $2, $3, $4, $5, $6, $7, $8, $9)"]
pub fn init_scroll_area_event_with_can_bubble_and_cancelable_and_view_and_detail_and_x_and_y_and_width_and_height(&self, type_: &str, can_bubble: bool, cancelable: bool, view: Option<&Window>, detail: i32, x: f32, y: f32, width: f32, height: f32) { panic!("web-sys intrinsic") }
}
