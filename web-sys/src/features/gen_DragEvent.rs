#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DragEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DragEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DragEvent`*"]
#[js::opaque]
#[derive(Copy)]
pub struct DragEvent(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "DataTransfer")]
    #[doc = "Getter for the `dataTransfer` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DragEvent/dataTransfer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DataTransfer`, `DragEvent`*"]
impl DragEvent {
    #[js::import = "$0.dataTransfer"]
pub fn data_transfer(&self) -> Option<DataTransfer> { panic!("web-sys intrinsic") }
}
    #[doc = "The `new DragEvent(..)` constructor, creating a new instance of `DragEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DragEvent/DragEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DragEvent`*"]
impl DragEvent {
    #[js::import = "new DragEvent($0)"]
pub fn new(type_: &str) -> Result<DragEvent, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "DragEventInit")]
    #[doc = "The `new DragEvent(..)` constructor, creating a new instance of `DragEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DragEvent/DragEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DragEvent`, `DragEventInit`*"]
impl DragEvent {
    #[js::import = "new DragEvent($0, $1)"]
pub fn new_with_event_init_dict(type_: &str, event_init_dict: &DragEventInit,) -> Result<DragEvent, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `initDragEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DragEvent/initDragEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DragEvent`*"]
impl DragEvent {
    #[js::import = "$0.initDragEvent($1)"]
pub fn init_drag_event(&self, type_: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "The `initDragEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DragEvent/initDragEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DragEvent`*"]
impl DragEvent {
    #[js::import = "$0.initDragEvent($1, $2)"]
pub fn init_drag_event_with_can_bubble(&self, type_: &str, can_bubble: bool) { panic!("web-sys intrinsic") }
}
    #[doc = "The `initDragEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DragEvent/initDragEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DragEvent`*"]
impl DragEvent {
    #[js::import = "$0.initDragEvent($1, $2, $3)"]
pub fn init_drag_event_with_can_bubble_and_cancelable(&self, type_: &str, can_bubble: bool, cancelable: bool) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Window")]
    #[doc = "The `initDragEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DragEvent/initDragEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DragEvent`, `Window`*"]
impl DragEvent {
    #[js::import = "$0.initDragEvent($1, $2, $3, $4)"]
pub fn init_drag_event_with_can_bubble_and_cancelable_and_a_view(&self, type_: &str, can_bubble: bool, cancelable: bool, a_view: Option<&Window>) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Window")]
    #[doc = "The `initDragEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DragEvent/initDragEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DragEvent`, `Window`*"]
impl DragEvent {
    #[js::import = "$0.initDragEvent($1, $2, $3, $4, $5)"]
pub fn init_drag_event_with_can_bubble_and_cancelable_and_a_view_and_a_detail(&self, type_: &str, can_bubble: bool, cancelable: bool, a_view: Option<&Window>, a_detail: i32) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Window")]
    #[doc = "The `initDragEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DragEvent/initDragEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DragEvent`, `Window`*"]
impl DragEvent {
    #[js::import = "$0.initDragEvent($1, $2, $3, $4, $5, $6)"]
pub fn init_drag_event_with_can_bubble_and_cancelable_and_a_view_and_a_detail_and_a_screen_x(&self, type_: &str, can_bubble: bool, cancelable: bool, a_view: Option<&Window>, a_detail: i32, a_screen_x: i32) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Window")]
    #[doc = "The `initDragEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DragEvent/initDragEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DragEvent`, `Window`*"]
impl DragEvent {
    #[js::import = "$0.initDragEvent($1, $2, $3, $4, $5, $6, $7)"]
pub fn init_drag_event_with_can_bubble_and_cancelable_and_a_view_and_a_detail_and_a_screen_x_and_a_screen_y(&self, type_: &str, can_bubble: bool, cancelable: bool, a_view: Option<&Window>, a_detail: i32, a_screen_x: i32, a_screen_y: i32) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Window")]
    #[doc = "The `initDragEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DragEvent/initDragEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DragEvent`, `Window`*"]
impl DragEvent {
    #[js::import = "$0.initDragEvent($1, $2, $3, $4, $5, $6, $7, $8)"]
pub fn init_drag_event_with_can_bubble_and_cancelable_and_a_view_and_a_detail_and_a_screen_x_and_a_screen_y_and_a_client_x(&self, type_: &str, can_bubble: bool, cancelable: bool, a_view: Option<&Window>, a_detail: i32, a_screen_x: i32, a_screen_y: i32, a_client_x: i32) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Window")]
    #[doc = "The `initDragEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DragEvent/initDragEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DragEvent`, `Window`*"]
impl DragEvent {
    #[js::import = "$0.initDragEvent($1, $2, $3, $4, $5, $6, $7, $8, $9)"]
pub fn init_drag_event_with_can_bubble_and_cancelable_and_a_view_and_a_detail_and_a_screen_x_and_a_screen_y_and_a_client_x_and_a_client_y(&self, type_: &str, can_bubble: bool, cancelable: bool, a_view: Option<&Window>, a_detail: i32, a_screen_x: i32, a_screen_y: i32, a_client_x: i32, a_client_y: i32) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Window")]
    #[doc = "The `initDragEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DragEvent/initDragEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DragEvent`, `Window`*"]
impl DragEvent {
    #[js::import = "$0.initDragEvent($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)"]
pub fn init_drag_event_with_can_bubble_and_cancelable_and_a_view_and_a_detail_and_a_screen_x_and_a_screen_y_and_a_client_x_and_a_client_y_and_a_ctrl_key(&self, type_: &str, can_bubble: bool, cancelable: bool, a_view: Option<&Window>, a_detail: i32, a_screen_x: i32, a_screen_y: i32, a_client_x: i32, a_client_y: i32, a_ctrl_key: bool) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Window")]
    #[doc = "The `initDragEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DragEvent/initDragEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DragEvent`, `Window`*"]
impl DragEvent {
    #[js::import = "$0.initDragEvent($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)"]
pub fn init_drag_event_with_can_bubble_and_cancelable_and_a_view_and_a_detail_and_a_screen_x_and_a_screen_y_and_a_client_x_and_a_client_y_and_a_ctrl_key_and_a_alt_key(&self, type_: &str, can_bubble: bool, cancelable: bool, a_view: Option<&Window>, a_detail: i32, a_screen_x: i32, a_screen_y: i32, a_client_x: i32, a_client_y: i32, a_ctrl_key: bool, a_alt_key: bool) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Window")]
    #[doc = "The `initDragEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DragEvent/initDragEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DragEvent`, `Window`*"]
impl DragEvent {
    #[js::import = "$0.initDragEvent($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)"]
pub fn init_drag_event_with_can_bubble_and_cancelable_and_a_view_and_a_detail_and_a_screen_x_and_a_screen_y_and_a_client_x_and_a_client_y_and_a_ctrl_key_and_a_alt_key_and_a_shift_key(&self, type_: &str, can_bubble: bool, cancelable: bool, a_view: Option<&Window>, a_detail: i32, a_screen_x: i32, a_screen_y: i32, a_client_x: i32, a_client_y: i32, a_ctrl_key: bool, a_alt_key: bool, a_shift_key: bool) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Window")]
    #[doc = "The `initDragEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DragEvent/initDragEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DragEvent`, `Window`*"]
impl DragEvent {
    #[js::import = "$0.initDragEvent($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)"]
pub fn init_drag_event_with_can_bubble_and_cancelable_and_a_view_and_a_detail_and_a_screen_x_and_a_screen_y_and_a_client_x_and_a_client_y_and_a_ctrl_key_and_a_alt_key_and_a_shift_key_and_a_meta_key(&self, type_: &str, can_bubble: bool, cancelable: bool, a_view: Option<&Window>, a_detail: i32, a_screen_x: i32, a_screen_y: i32, a_client_x: i32, a_client_y: i32, a_ctrl_key: bool, a_alt_key: bool, a_shift_key: bool, a_meta_key: bool) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Window")]
    #[doc = "The `initDragEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DragEvent/initDragEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DragEvent`, `Window`*"]
impl DragEvent {
    #[js::import = "$0.initDragEvent($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)"]
pub fn init_drag_event_with_can_bubble_and_cancelable_and_a_view_and_a_detail_and_a_screen_x_and_a_screen_y_and_a_client_x_and_a_client_y_and_a_ctrl_key_and_a_alt_key_and_a_shift_key_and_a_meta_key_and_a_button(&self, type_: &str, can_bubble: bool, cancelable: bool, a_view: Option<&Window>, a_detail: i32, a_screen_x: i32, a_screen_y: i32, a_client_x: i32, a_client_y: i32, a_ctrl_key: bool, a_alt_key: bool, a_shift_key: bool, a_meta_key: bool, a_button: u16) { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "EventTarget", feature = "Window",))]
    #[doc = "The `initDragEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DragEvent/initDragEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DragEvent`, `EventTarget`, `Window`*"]
impl DragEvent {
    #[js::import = "$0.initDragEvent($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15)"]
pub fn init_drag_event_with_can_bubble_and_cancelable_and_a_view_and_a_detail_and_a_screen_x_and_a_screen_y_and_a_client_x_and_a_client_y_and_a_ctrl_key_and_a_alt_key_and_a_shift_key_and_a_meta_key_and_a_button_and_a_related_target(&self, type_: &str, can_bubble: bool, cancelable: bool, a_view: Option<&Window>, a_detail: i32, a_screen_x: i32, a_screen_y: i32, a_client_x: i32, a_client_y: i32, a_ctrl_key: bool, a_alt_key: bool, a_shift_key: bool, a_meta_key: bool, a_button: u16, a_related_target: Option<&EventTarget>) { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "DataTransfer", feature = "EventTarget", feature = "Window",))]
    #[doc = "The `initDragEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DragEvent/initDragEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DataTransfer`, `DragEvent`, `EventTarget`, `Window`*"]
impl DragEvent {
    #[js::import = "$0.initDragEvent($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16)"]
pub fn init_drag_event_with_can_bubble_and_cancelable_and_a_view_and_a_detail_and_a_screen_x_and_a_screen_y_and_a_client_x_and_a_client_y_and_a_ctrl_key_and_a_alt_key_and_a_shift_key_and_a_meta_key_and_a_button_and_a_related_target_and_a_data_transfer(&self, type_: &str, can_bubble: bool, cancelable: bool, a_view: Option<&Window>, a_detail: i32, a_screen_x: i32, a_screen_y: i32, a_client_x: i32, a_client_y: i32, a_ctrl_key: bool, a_alt_key: bool, a_shift_key: bool, a_meta_key: bool, a_button: u16, a_related_target: Option<&EventTarget>, a_data_transfer: Option<&DataTransfer>) { panic!("web-sys intrinsic") }
}
