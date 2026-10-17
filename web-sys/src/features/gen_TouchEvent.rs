#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `TouchEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TouchEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchEvent`*"]
#[js::opaque]
#[derive(Copy)]
pub struct TouchEvent(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "TouchList")]
    #[doc = "Getter for the `touches` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TouchEvent/touches)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchEvent`, `TouchList`*"]
impl TouchEvent {
    #[js::import = "$0.touches"]
pub fn touches(&self) -> TouchList { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "TouchList")]
    #[doc = "Getter for the `targetTouches` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TouchEvent/targetTouches)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchEvent`, `TouchList`*"]
impl TouchEvent {
    #[js::import = "$0.targetTouches"]
pub fn target_touches(&self) -> TouchList { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "TouchList")]
    #[doc = "Getter for the `changedTouches` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TouchEvent/changedTouches)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchEvent`, `TouchList`*"]
impl TouchEvent {
    #[js::import = "$0.changedTouches"]
pub fn changed_touches(&self) -> TouchList { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `altKey` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TouchEvent/altKey)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchEvent`*"]
impl TouchEvent {
    #[js::import = "$0.altKey"]
pub fn alt_key(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `metaKey` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TouchEvent/metaKey)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchEvent`*"]
impl TouchEvent {
    #[js::import = "$0.metaKey"]
pub fn meta_key(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `ctrlKey` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TouchEvent/ctrlKey)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchEvent`*"]
impl TouchEvent {
    #[js::import = "$0.ctrlKey"]
pub fn ctrl_key(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `shiftKey` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TouchEvent/shiftKey)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchEvent`*"]
impl TouchEvent {
    #[js::import = "$0.shiftKey"]
pub fn shift_key(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "The `new TouchEvent(..)` constructor, creating a new instance of `TouchEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TouchEvent/TouchEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchEvent`*"]
impl TouchEvent {
    #[js::import = "new TouchEvent($0)"]
pub fn new(type_: &str) -> Result<TouchEvent, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "TouchEventInit")]
    #[doc = "The `new TouchEvent(..)` constructor, creating a new instance of `TouchEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TouchEvent/TouchEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchEvent`, `TouchEventInit`*"]
impl TouchEvent {
    #[js::import = "new TouchEvent($0, $1)"]
pub fn new_with_event_init_dict(type_: &str, event_init_dict: &TouchEventInit,) -> Result<TouchEvent, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `initTouchEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TouchEvent/initTouchEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchEvent`*"]
impl TouchEvent {
    #[js::import = "$0.initTouchEvent($1)"]
pub fn init_touch_event(&self, type_: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "The `initTouchEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TouchEvent/initTouchEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchEvent`*"]
impl TouchEvent {
    #[js::import = "$0.initTouchEvent($1, $2)"]
pub fn init_touch_event_with_can_bubble(&self, type_: &str, can_bubble: bool) { panic!("web-sys intrinsic") }
}
    #[doc = "The `initTouchEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TouchEvent/initTouchEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchEvent`*"]
impl TouchEvent {
    #[js::import = "$0.initTouchEvent($1, $2, $3)"]
pub fn init_touch_event_with_can_bubble_and_cancelable(&self, type_: &str, can_bubble: bool, cancelable: bool) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Window")]
    #[doc = "The `initTouchEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TouchEvent/initTouchEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchEvent`, `Window`*"]
impl TouchEvent {
    #[js::import = "$0.initTouchEvent($1, $2, $3, $4)"]
pub fn init_touch_event_with_can_bubble_and_cancelable_and_view(&self, type_: &str, can_bubble: bool, cancelable: bool, view: Option<&Window>) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Window")]
    #[doc = "The `initTouchEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TouchEvent/initTouchEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchEvent`, `Window`*"]
impl TouchEvent {
    #[js::import = "$0.initTouchEvent($1, $2, $3, $4, $5)"]
pub fn init_touch_event_with_can_bubble_and_cancelable_and_view_and_detail(&self, type_: &str, can_bubble: bool, cancelable: bool, view: Option<&Window>, detail: i32) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Window")]
    #[doc = "The `initTouchEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TouchEvent/initTouchEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchEvent`, `Window`*"]
impl TouchEvent {
    #[js::import = "$0.initTouchEvent($1, $2, $3, $4, $5, $6)"]
pub fn init_touch_event_with_can_bubble_and_cancelable_and_view_and_detail_and_ctrl_key(&self, type_: &str, can_bubble: bool, cancelable: bool, view: Option<&Window>, detail: i32, ctrl_key: bool) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Window")]
    #[doc = "The `initTouchEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TouchEvent/initTouchEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchEvent`, `Window`*"]
impl TouchEvent {
    #[js::import = "$0.initTouchEvent($1, $2, $3, $4, $5, $6, $7)"]
pub fn init_touch_event_with_can_bubble_and_cancelable_and_view_and_detail_and_ctrl_key_and_alt_key(&self, type_: &str, can_bubble: bool, cancelable: bool, view: Option<&Window>, detail: i32, ctrl_key: bool, alt_key: bool) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Window")]
    #[doc = "The `initTouchEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TouchEvent/initTouchEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchEvent`, `Window`*"]
impl TouchEvent {
    #[js::import = "$0.initTouchEvent($1, $2, $3, $4, $5, $6, $7, $8)"]
pub fn init_touch_event_with_can_bubble_and_cancelable_and_view_and_detail_and_ctrl_key_and_alt_key_and_shift_key(&self, type_: &str, can_bubble: bool, cancelable: bool, view: Option<&Window>, detail: i32, ctrl_key: bool, alt_key: bool, shift_key: bool) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Window")]
    #[doc = "The `initTouchEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TouchEvent/initTouchEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchEvent`, `Window`*"]
impl TouchEvent {
    #[js::import = "$0.initTouchEvent($1, $2, $3, $4, $5, $6, $7, $8, $9)"]
pub fn init_touch_event_with_can_bubble_and_cancelable_and_view_and_detail_and_ctrl_key_and_alt_key_and_shift_key_and_meta_key(&self, type_: &str, can_bubble: bool, cancelable: bool, view: Option<&Window>, detail: i32, ctrl_key: bool, alt_key: bool, shift_key: bool, meta_key: bool) { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "TouchList", feature = "Window",))]
    #[doc = "The `initTouchEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TouchEvent/initTouchEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchEvent`, `TouchList`, `Window`*"]
impl TouchEvent {
    #[js::import = "$0.initTouchEvent($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)"]
pub fn init_touch_event_with_can_bubble_and_cancelable_and_view_and_detail_and_ctrl_key_and_alt_key_and_shift_key_and_meta_key_and_touches(&self, type_: &str, can_bubble: bool, cancelable: bool, view: Option<&Window>, detail: i32, ctrl_key: bool, alt_key: bool, shift_key: bool, meta_key: bool, touches: Option<&TouchList>) { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "TouchList", feature = "Window",))]
    #[doc = "The `initTouchEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TouchEvent/initTouchEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchEvent`, `TouchList`, `Window`*"]
impl TouchEvent {
    #[js::import = "$0.initTouchEvent($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)"]
pub fn init_touch_event_with_can_bubble_and_cancelable_and_view_and_detail_and_ctrl_key_and_alt_key_and_shift_key_and_meta_key_and_touches_and_target_touches(&self, type_: &str, can_bubble: bool, cancelable: bool, view: Option<&Window>, detail: i32, ctrl_key: bool, alt_key: bool, shift_key: bool, meta_key: bool, touches: Option<&TouchList>, target_touches: Option<&TouchList>) { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "TouchList", feature = "Window",))]
    #[doc = "The `initTouchEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TouchEvent/initTouchEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchEvent`, `TouchList`, `Window`*"]
impl TouchEvent {
    #[js::import = "$0.initTouchEvent($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)"]
pub fn init_touch_event_with_can_bubble_and_cancelable_and_view_and_detail_and_ctrl_key_and_alt_key_and_shift_key_and_meta_key_and_touches_and_target_touches_and_changed_touches(&self, type_: &str, can_bubble: bool, cancelable: bool, view: Option<&Window>, detail: i32, ctrl_key: bool, alt_key: bool, shift_key: bool, meta_key: bool, touches: Option<&TouchList>, target_touches: Option<&TouchList>, changed_touches: Option<&TouchList>) { panic!("web-sys intrinsic") }
}
