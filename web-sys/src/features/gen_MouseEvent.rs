#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MouseEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEvent`*"]
#[js::opaque]
#[derive(Copy)]
pub struct MouseEvent(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `screenX` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/screenX)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEvent`*"]
impl MouseEvent {
    #[js::import = "$0.screenX"]
pub fn screen_x(&self) -> i32 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `screenY` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/screenY)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEvent`*"]
impl MouseEvent {
    #[js::import = "$0.screenY"]
pub fn screen_y(&self) -> i32 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `clientX` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/clientX)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEvent`*"]
impl MouseEvent {
    #[js::import = "$0.clientX"]
pub fn client_x(&self) -> i32 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `clientY` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/clientY)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEvent`*"]
impl MouseEvent {
    #[js::import = "$0.clientY"]
pub fn client_y(&self) -> i32 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `x` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/x)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEvent`*"]
impl MouseEvent {
    #[js::import = "$0.x"]
pub fn x(&self) -> i32 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `y` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/y)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEvent`*"]
impl MouseEvent {
    #[js::import = "$0.y"]
pub fn y(&self) -> i32 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `offsetX` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/offsetX)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEvent`*"]
impl MouseEvent {
    #[js::import = "$0.offsetX"]
pub fn offset_x(&self) -> i32 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `offsetY` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/offsetY)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEvent`*"]
impl MouseEvent {
    #[js::import = "$0.offsetY"]
pub fn offset_y(&self) -> i32 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `ctrlKey` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/ctrlKey)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEvent`*"]
impl MouseEvent {
    #[js::import = "$0.ctrlKey"]
pub fn ctrl_key(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `shiftKey` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/shiftKey)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEvent`*"]
impl MouseEvent {
    #[js::import = "$0.shiftKey"]
pub fn shift_key(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `altKey` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/altKey)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEvent`*"]
impl MouseEvent {
    #[js::import = "$0.altKey"]
pub fn alt_key(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `metaKey` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/metaKey)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEvent`*"]
impl MouseEvent {
    #[js::import = "$0.metaKey"]
pub fn meta_key(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `button` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/button)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEvent`*"]
impl MouseEvent {
    #[js::import = "$0.button"]
pub fn button(&self) -> i16 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `buttons` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/buttons)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEvent`*"]
impl MouseEvent {
    #[js::import = "$0.buttons"]
pub fn buttons(&self) -> u16 { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "EventTarget")]
    #[doc = "Getter for the `relatedTarget` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/relatedTarget)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventTarget`, `MouseEvent`*"]
impl MouseEvent {
    #[js::import = "$0.relatedTarget"]
pub fn related_target(&self) -> Option<EventTarget> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `region` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/region)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEvent`*"]
impl MouseEvent {
    #[js::import = "$0.region"]
pub fn region(&self) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `movementX` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/movementX)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEvent`*"]
impl MouseEvent {
    #[js::import = "$0.movementX"]
pub fn movement_x(&self) -> i32 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `movementY` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/movementY)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEvent`*"]
impl MouseEvent {
    #[js::import = "$0.movementY"]
pub fn movement_y(&self) -> i32 { panic!("web-sys intrinsic") }
}
    #[doc = "The `new MouseEvent(..)` constructor, creating a new instance of `MouseEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/MouseEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEvent`*"]
impl MouseEvent {
    #[js::import = "new MouseEvent($0)"]
pub fn new(type_arg: &str) -> Result<MouseEvent, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "MouseEventInit")]
    #[doc = "The `new MouseEvent(..)` constructor, creating a new instance of `MouseEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/MouseEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEvent`, `MouseEventInit`*"]
impl MouseEvent {
    #[js::import = "new MouseEvent($0, $1)"]
pub fn new_with_mouse_event_init_dict(type_arg: &str, mouse_event_init_dict: &MouseEventInit,) -> Result<MouseEvent, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `getModifierState()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/getModifierState)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEvent`*"]
impl MouseEvent {
    #[js::import = "$0.getModifierState($1)"]
pub fn get_modifier_state(&self, key_arg: &str) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "The `initMouseEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/initMouseEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEvent`*"]
impl MouseEvent {
    #[js::import = "$0.initMouseEvent($1)"]
pub fn init_mouse_event(&self, type_arg: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "The `initMouseEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/initMouseEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEvent`*"]
impl MouseEvent {
    #[js::import = "$0.initMouseEvent($1, $2)"]
pub fn init_mouse_event_with_can_bubble_arg(&self, type_arg: &str, can_bubble_arg: bool) { panic!("web-sys intrinsic") }
}
    #[doc = "The `initMouseEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/initMouseEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEvent`*"]
impl MouseEvent {
    #[js::import = "$0.initMouseEvent($1, $2, $3)"]
pub fn init_mouse_event_with_can_bubble_arg_and_cancelable_arg(&self, type_arg: &str, can_bubble_arg: bool, cancelable_arg: bool) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Window")]
    #[doc = "The `initMouseEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/initMouseEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEvent`, `Window`*"]
impl MouseEvent {
    #[js::import = "$0.initMouseEvent($1, $2, $3, $4)"]
pub fn init_mouse_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg(&self, type_arg: &str, can_bubble_arg: bool, cancelable_arg: bool, view_arg: Option<&Window>) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Window")]
    #[doc = "The `initMouseEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/initMouseEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEvent`, `Window`*"]
impl MouseEvent {
    #[js::import = "$0.initMouseEvent($1, $2, $3, $4, $5)"]
pub fn init_mouse_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg_and_detail_arg(&self, type_arg: &str, can_bubble_arg: bool, cancelable_arg: bool, view_arg: Option<&Window>, detail_arg: i32) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Window")]
    #[doc = "The `initMouseEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/initMouseEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEvent`, `Window`*"]
impl MouseEvent {
    #[js::import = "$0.initMouseEvent($1, $2, $3, $4, $5, $6)"]
pub fn init_mouse_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg_and_detail_arg_and_screen_x_arg(&self, type_arg: &str, can_bubble_arg: bool, cancelable_arg: bool, view_arg: Option<&Window>, detail_arg: i32, screen_x_arg: i32) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Window")]
    #[doc = "The `initMouseEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/initMouseEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEvent`, `Window`*"]
impl MouseEvent {
    #[js::import = "$0.initMouseEvent($1, $2, $3, $4, $5, $6, $7)"]
pub fn init_mouse_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg_and_detail_arg_and_screen_x_arg_and_screen_y_arg(&self, type_arg: &str, can_bubble_arg: bool, cancelable_arg: bool, view_arg: Option<&Window>, detail_arg: i32, screen_x_arg: i32, screen_y_arg: i32) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Window")]
    #[doc = "The `initMouseEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/initMouseEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEvent`, `Window`*"]
impl MouseEvent {
    #[js::import = "$0.initMouseEvent($1, $2, $3, $4, $5, $6, $7, $8)"]
pub fn init_mouse_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg_and_detail_arg_and_screen_x_arg_and_screen_y_arg_and_client_x_arg(&self, type_arg: &str, can_bubble_arg: bool, cancelable_arg: bool, view_arg: Option<&Window>, detail_arg: i32, screen_x_arg: i32, screen_y_arg: i32, client_x_arg: i32) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Window")]
    #[doc = "The `initMouseEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/initMouseEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEvent`, `Window`*"]
impl MouseEvent {
    #[js::import = "$0.initMouseEvent($1, $2, $3, $4, $5, $6, $7, $8, $9)"]
pub fn init_mouse_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg_and_detail_arg_and_screen_x_arg_and_screen_y_arg_and_client_x_arg_and_client_y_arg(&self, type_arg: &str, can_bubble_arg: bool, cancelable_arg: bool, view_arg: Option<&Window>, detail_arg: i32, screen_x_arg: i32, screen_y_arg: i32, client_x_arg: i32, client_y_arg: i32) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Window")]
    #[doc = "The `initMouseEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/initMouseEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEvent`, `Window`*"]
impl MouseEvent {
    #[js::import = "$0.initMouseEvent($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)"]
pub fn init_mouse_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg_and_detail_arg_and_screen_x_arg_and_screen_y_arg_and_client_x_arg_and_client_y_arg_and_ctrl_key_arg(&self, type_arg: &str, can_bubble_arg: bool, cancelable_arg: bool, view_arg: Option<&Window>, detail_arg: i32, screen_x_arg: i32, screen_y_arg: i32, client_x_arg: i32, client_y_arg: i32, ctrl_key_arg: bool) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Window")]
    #[doc = "The `initMouseEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/initMouseEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEvent`, `Window`*"]
impl MouseEvent {
    #[js::import = "$0.initMouseEvent($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)"]
pub fn init_mouse_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg_and_detail_arg_and_screen_x_arg_and_screen_y_arg_and_client_x_arg_and_client_y_arg_and_ctrl_key_arg_and_alt_key_arg(&self, type_arg: &str, can_bubble_arg: bool, cancelable_arg: bool, view_arg: Option<&Window>, detail_arg: i32, screen_x_arg: i32, screen_y_arg: i32, client_x_arg: i32, client_y_arg: i32, ctrl_key_arg: bool, alt_key_arg: bool) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Window")]
    #[doc = "The `initMouseEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/initMouseEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEvent`, `Window`*"]
impl MouseEvent {
    #[js::import = "$0.initMouseEvent($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)"]
pub fn init_mouse_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg_and_detail_arg_and_screen_x_arg_and_screen_y_arg_and_client_x_arg_and_client_y_arg_and_ctrl_key_arg_and_alt_key_arg_and_shift_key_arg(&self, type_arg: &str, can_bubble_arg: bool, cancelable_arg: bool, view_arg: Option<&Window>, detail_arg: i32, screen_x_arg: i32, screen_y_arg: i32, client_x_arg: i32, client_y_arg: i32, ctrl_key_arg: bool, alt_key_arg: bool, shift_key_arg: bool) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Window")]
    #[doc = "The `initMouseEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/initMouseEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEvent`, `Window`*"]
impl MouseEvent {
    #[js::import = "$0.initMouseEvent($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)"]
pub fn init_mouse_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg_and_detail_arg_and_screen_x_arg_and_screen_y_arg_and_client_x_arg_and_client_y_arg_and_ctrl_key_arg_and_alt_key_arg_and_shift_key_arg_and_meta_key_arg(&self, type_arg: &str, can_bubble_arg: bool, cancelable_arg: bool, view_arg: Option<&Window>, detail_arg: i32, screen_x_arg: i32, screen_y_arg: i32, client_x_arg: i32, client_y_arg: i32, ctrl_key_arg: bool, alt_key_arg: bool, shift_key_arg: bool, meta_key_arg: bool) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Window")]
    #[doc = "The `initMouseEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/initMouseEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MouseEvent`, `Window`*"]
impl MouseEvent {
    #[js::import = "$0.initMouseEvent($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)"]
pub fn init_mouse_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg_and_detail_arg_and_screen_x_arg_and_screen_y_arg_and_client_x_arg_and_client_y_arg_and_ctrl_key_arg_and_alt_key_arg_and_shift_key_arg_and_meta_key_arg_and_button_arg(&self, type_arg: &str, can_bubble_arg: bool, cancelable_arg: bool, view_arg: Option<&Window>, detail_arg: i32, screen_x_arg: i32, screen_y_arg: i32, client_x_arg: i32, client_y_arg: i32, ctrl_key_arg: bool, alt_key_arg: bool, shift_key_arg: bool, meta_key_arg: bool, button_arg: i16) { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "EventTarget", feature = "Window",))]
    #[doc = "The `initMouseEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/initMouseEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventTarget`, `MouseEvent`, `Window`*"]
impl MouseEvent {
    #[js::import = "$0.initMouseEvent($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15)"]
pub fn init_mouse_event_with_can_bubble_arg_and_cancelable_arg_and_view_arg_and_detail_arg_and_screen_x_arg_and_screen_y_arg_and_client_x_arg_and_client_y_arg_and_ctrl_key_arg_and_alt_key_arg_and_shift_key_arg_and_meta_key_arg_and_button_arg_and_related_target_arg(&self, type_arg: &str, can_bubble_arg: bool, cancelable_arg: bool, view_arg: Option<&Window>, detail_arg: i32, screen_x_arg: i32, screen_y_arg: i32, client_x_arg: i32, client_y_arg: i32, ctrl_key_arg: bool, alt_key_arg: bool, shift_key_arg: bool, meta_key_arg: bool, button_arg: i16, related_target_arg: Option<&EventTarget>) { panic!("web-sys intrinsic") }
}
