#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MouseScrollEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseScrollEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseScrollEvent`*"]
#[js::opaque]
#[derive(Copy)]
pub struct MouseScrollEvent(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `axis` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseScrollEvent/axis)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseScrollEvent`*"]
impl MouseScrollEvent {
    #[js::import = "$0.axis"]
pub fn axis(&self) -> i32 { panic!("web-sys intrinsic") }
}
    #[doc = "The `initMouseScrollEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseScrollEvent/initMouseScrollEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseScrollEvent`*"]
impl MouseScrollEvent {
    #[js::import = "$0.initMouseScrollEvent($1)"]
pub fn init_mouse_scroll_event(&self, type_: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "The `initMouseScrollEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseScrollEvent/initMouseScrollEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseScrollEvent`*"]
impl MouseScrollEvent {
    #[js::import = "$0.initMouseScrollEvent($1, $2)"]
pub fn init_mouse_scroll_event_with_can_bubble(&self, type_: &str, can_bubble: bool) { panic!("web-sys intrinsic") }
}
    #[doc = "The `initMouseScrollEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseScrollEvent/initMouseScrollEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseScrollEvent`*"]
impl MouseScrollEvent {
    #[js::import = "$0.initMouseScrollEvent($1, $2, $3)"]
pub fn init_mouse_scroll_event_with_can_bubble_and_cancelable(&self, type_: &str, can_bubble: bool, cancelable: bool) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Window")]
    #[doc = "The `initMouseScrollEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseScrollEvent/initMouseScrollEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseScrollEvent`, `Window`*"]
impl MouseScrollEvent {
    #[js::import = "$0.initMouseScrollEvent($1, $2, $3, $4)"]
pub fn init_mouse_scroll_event_with_can_bubble_and_cancelable_and_view(&self, type_: &str, can_bubble: bool, cancelable: bool, view: Option<&Window>) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Window")]
    #[doc = "The `initMouseScrollEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseScrollEvent/initMouseScrollEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseScrollEvent`, `Window`*"]
impl MouseScrollEvent {
    #[js::import = "$0.initMouseScrollEvent($1, $2, $3, $4, $5)"]
pub fn init_mouse_scroll_event_with_can_bubble_and_cancelable_and_view_and_detail(&self, type_: &str, can_bubble: bool, cancelable: bool, view: Option<&Window>, detail: i32) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Window")]
    #[doc = "The `initMouseScrollEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseScrollEvent/initMouseScrollEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseScrollEvent`, `Window`*"]
impl MouseScrollEvent {
    #[js::import = "$0.initMouseScrollEvent($1, $2, $3, $4, $5, $6)"]
pub fn init_mouse_scroll_event_with_can_bubble_and_cancelable_and_view_and_detail_and_screen_x(&self, type_: &str, can_bubble: bool, cancelable: bool, view: Option<&Window>, detail: i32, screen_x: i32) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Window")]
    #[doc = "The `initMouseScrollEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseScrollEvent/initMouseScrollEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseScrollEvent`, `Window`*"]
impl MouseScrollEvent {
    #[js::import = "$0.initMouseScrollEvent($1, $2, $3, $4, $5, $6, $7)"]
pub fn init_mouse_scroll_event_with_can_bubble_and_cancelable_and_view_and_detail_and_screen_x_and_screen_y(&self, type_: &str, can_bubble: bool, cancelable: bool, view: Option<&Window>, detail: i32, screen_x: i32, screen_y: i32) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Window")]
    #[doc = "The `initMouseScrollEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseScrollEvent/initMouseScrollEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseScrollEvent`, `Window`*"]
impl MouseScrollEvent {
    #[js::import = "$0.initMouseScrollEvent($1, $2, $3, $4, $5, $6, $7, $8)"]
pub fn init_mouse_scroll_event_with_can_bubble_and_cancelable_and_view_and_detail_and_screen_x_and_screen_y_and_client_x(&self, type_: &str, can_bubble: bool, cancelable: bool, view: Option<&Window>, detail: i32, screen_x: i32, screen_y: i32, client_x: i32) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Window")]
    #[doc = "The `initMouseScrollEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseScrollEvent/initMouseScrollEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseScrollEvent`, `Window`*"]
impl MouseScrollEvent {
    #[js::import = "$0.initMouseScrollEvent($1, $2, $3, $4, $5, $6, $7, $8, $9)"]
pub fn init_mouse_scroll_event_with_can_bubble_and_cancelable_and_view_and_detail_and_screen_x_and_screen_y_and_client_x_and_client_y(&self, type_: &str, can_bubble: bool, cancelable: bool, view: Option<&Window>, detail: i32, screen_x: i32, screen_y: i32, client_x: i32, client_y: i32) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Window")]
    #[doc = "The `initMouseScrollEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseScrollEvent/initMouseScrollEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseScrollEvent`, `Window`*"]
impl MouseScrollEvent {
    #[js::import = "$0.initMouseScrollEvent($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)"]
pub fn init_mouse_scroll_event_with_can_bubble_and_cancelable_and_view_and_detail_and_screen_x_and_screen_y_and_client_x_and_client_y_and_ctrl_key(&self, type_: &str, can_bubble: bool, cancelable: bool, view: Option<&Window>, detail: i32, screen_x: i32, screen_y: i32, client_x: i32, client_y: i32, ctrl_key: bool) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Window")]
    #[doc = "The `initMouseScrollEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseScrollEvent/initMouseScrollEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseScrollEvent`, `Window`*"]
impl MouseScrollEvent {
    #[js::import = "$0.initMouseScrollEvent($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)"]
pub fn init_mouse_scroll_event_with_can_bubble_and_cancelable_and_view_and_detail_and_screen_x_and_screen_y_and_client_x_and_client_y_and_ctrl_key_and_alt_key(&self, type_: &str, can_bubble: bool, cancelable: bool, view: Option<&Window>, detail: i32, screen_x: i32, screen_y: i32, client_x: i32, client_y: i32, ctrl_key: bool, alt_key: bool) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Window")]
    #[doc = "The `initMouseScrollEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseScrollEvent/initMouseScrollEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseScrollEvent`, `Window`*"]
impl MouseScrollEvent {
    #[js::import = "$0.initMouseScrollEvent($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)"]
pub fn init_mouse_scroll_event_with_can_bubble_and_cancelable_and_view_and_detail_and_screen_x_and_screen_y_and_client_x_and_client_y_and_ctrl_key_and_alt_key_and_shift_key(&self, type_: &str, can_bubble: bool, cancelable: bool, view: Option<&Window>, detail: i32, screen_x: i32, screen_y: i32, client_x: i32, client_y: i32, ctrl_key: bool, alt_key: bool, shift_key: bool) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Window")]
    #[doc = "The `initMouseScrollEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseScrollEvent/initMouseScrollEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseScrollEvent`, `Window`*"]
impl MouseScrollEvent {
    #[js::import = "$0.initMouseScrollEvent($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)"]
pub fn init_mouse_scroll_event_with_can_bubble_and_cancelable_and_view_and_detail_and_screen_x_and_screen_y_and_client_x_and_client_y_and_ctrl_key_and_alt_key_and_shift_key_and_meta_key(&self, type_: &str, can_bubble: bool, cancelable: bool, view: Option<&Window>, detail: i32, screen_x: i32, screen_y: i32, client_x: i32, client_y: i32, ctrl_key: bool, alt_key: bool, shift_key: bool, meta_key: bool) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Window")]
    #[doc = "The `initMouseScrollEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseScrollEvent/initMouseScrollEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseScrollEvent`, `Window`*"]
impl MouseScrollEvent {
    #[js::import = "$0.initMouseScrollEvent($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)"]
pub fn init_mouse_scroll_event_with_can_bubble_and_cancelable_and_view_and_detail_and_screen_x_and_screen_y_and_client_x_and_client_y_and_ctrl_key_and_alt_key_and_shift_key_and_meta_key_and_button(&self, type_: &str, can_bubble: bool, cancelable: bool, view: Option<&Window>, detail: i32, screen_x: i32, screen_y: i32, client_x: i32, client_y: i32, ctrl_key: bool, alt_key: bool, shift_key: bool, meta_key: bool, button: i16) { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "EventTarget", feature = "Window",))]
    #[doc = "The `initMouseScrollEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseScrollEvent/initMouseScrollEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventTarget`, `MouseScrollEvent`, `Window`*"]
impl MouseScrollEvent {
    #[js::import = "$0.initMouseScrollEvent($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15)"]
pub fn init_mouse_scroll_event_with_can_bubble_and_cancelable_and_view_and_detail_and_screen_x_and_screen_y_and_client_x_and_client_y_and_ctrl_key_and_alt_key_and_shift_key_and_meta_key_and_button_and_related_target(&self, type_: &str, can_bubble: bool, cancelable: bool, view: Option<&Window>, detail: i32, screen_x: i32, screen_y: i32, client_x: i32, client_y: i32, ctrl_key: bool, alt_key: bool, shift_key: bool, meta_key: bool, button: i16, related_target: Option<&EventTarget>) { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "EventTarget", feature = "Window",))]
    #[doc = "The `initMouseScrollEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseScrollEvent/initMouseScrollEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventTarget`, `MouseScrollEvent`, `Window`*"]
impl MouseScrollEvent {
    #[js::import = "$0.initMouseScrollEvent($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16)"]
pub fn init_mouse_scroll_event_with_can_bubble_and_cancelable_and_view_and_detail_and_screen_x_and_screen_y_and_client_x_and_client_y_and_ctrl_key_and_alt_key_and_shift_key_and_meta_key_and_button_and_related_target_and_axis(&self, type_: &str, can_bubble: bool, cancelable: bool, view: Option<&Window>, detail: i32, screen_x: i32, screen_y: i32, client_x: i32, client_y: i32, ctrl_key: bool, alt_key: bool, shift_key: bool, meta_key: bool, button: i16, related_target: Option<&EventTarget>, axis: i32) { panic!("web-sys intrinsic") }
}
impl MouseScrollEvent {
    #[doc = "The `MouseScrollEvent.HORIZONTAL_AXIS` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseScrollEvent`*"]
    pub const HORIZONTAL_AXIS: i32 = 1u64 as i32;
    #[doc = "The `MouseScrollEvent.VERTICAL_AXIS` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseScrollEvent`*"]
    pub const VERTICAL_AXIS: i32 = 2u64 as i32;
}
