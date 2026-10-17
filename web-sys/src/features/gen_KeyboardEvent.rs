#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `KeyboardEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEvent`*"]
#[js::opaque]
#[derive(Copy)]
pub struct KeyboardEvent(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `charCode` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/charCode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEvent`*"]
impl KeyboardEvent {
    #[js::import = "$0.charCode"]
pub fn char_code(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `keyCode` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/keyCode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEvent`*"]
impl KeyboardEvent {
    #[js::import = "$0.keyCode"]
pub fn key_code(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `altKey` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/altKey)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEvent`*"]
impl KeyboardEvent {
    #[js::import = "$0.altKey"]
pub fn alt_key(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `ctrlKey` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/ctrlKey)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEvent`*"]
impl KeyboardEvent {
    #[js::import = "$0.ctrlKey"]
pub fn ctrl_key(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `shiftKey` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/shiftKey)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEvent`*"]
impl KeyboardEvent {
    #[js::import = "$0.shiftKey"]
pub fn shift_key(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `metaKey` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/metaKey)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEvent`*"]
impl KeyboardEvent {
    #[js::import = "$0.metaKey"]
pub fn meta_key(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `location` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/location)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEvent`*"]
impl KeyboardEvent {
    #[js::import = "$0.location"]
pub fn location(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `repeat` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/repeat)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEvent`*"]
impl KeyboardEvent {
    #[js::import = "$0.repeat"]
pub fn repeat(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `isComposing` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/isComposing)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEvent`*"]
impl KeyboardEvent {
    #[js::import = "$0.isComposing"]
pub fn is_composing(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `key` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/key)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEvent`*"]
impl KeyboardEvent {
    #[js::import = "$0.key"]
pub fn key(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `code` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/code)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEvent`*"]
impl KeyboardEvent {
    #[js::import = "$0.code"]
pub fn code(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "The `new KeyboardEvent(..)` constructor, creating a new instance of `KeyboardEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/KeyboardEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEvent`*"]
impl KeyboardEvent {
    #[js::import = "new KeyboardEvent($0)"]
pub fn new(type_arg: &str) -> Result<KeyboardEvent, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "KeyboardEventInit")]
    #[doc = "The `new KeyboardEvent(..)` constructor, creating a new instance of `KeyboardEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/KeyboardEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEvent`, `KeyboardEventInit`*"]
impl KeyboardEvent {
    #[js::import = "new KeyboardEvent($0, $1)"]
pub fn new_with_keyboard_event_init_dict(type_arg: &str, keyboard_event_init_dict: &KeyboardEventInit,) -> Result<KeyboardEvent, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `getModifierState()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/getModifierState)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEvent`*"]
impl KeyboardEvent {
    #[js::import = "$0.getModifierState($1)"]
pub fn get_modifier_state(&self, key: &str) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "The `initKeyboardEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/initKeyboardEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEvent`*"]
impl KeyboardEvent {
    #[js::import = "$0.initKeyboardEvent($1)"]
pub fn init_keyboard_event(&self, type_arg: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `initKeyboardEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/initKeyboardEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEvent`*"]
impl KeyboardEvent {
    #[js::import = "$0.initKeyboardEvent($1, $2)"]
pub fn init_keyboard_event_with_bubbles_arg(&self, type_arg: &str, bubbles_arg: bool) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `initKeyboardEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/initKeyboardEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEvent`*"]
impl KeyboardEvent {
    #[js::import = "$0.initKeyboardEvent($1, $2, $3)"]
pub fn init_keyboard_event_with_bubbles_arg_and_cancelable_arg(&self, type_arg: &str, bubbles_arg: bool, cancelable_arg: bool) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Window")]
    #[doc = "The `initKeyboardEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/initKeyboardEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEvent`, `Window`*"]
impl KeyboardEvent {
    #[js::import = "$0.initKeyboardEvent($1, $2, $3, $4)"]
pub fn init_keyboard_event_with_bubbles_arg_and_cancelable_arg_and_view_arg(&self, type_arg: &str, bubbles_arg: bool, cancelable_arg: bool, view_arg: Option<&Window>) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Window")]
    #[doc = "The `initKeyboardEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/initKeyboardEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEvent`, `Window`*"]
impl KeyboardEvent {
    #[js::import = "$0.initKeyboardEvent($1, $2, $3, $4, $5)"]
pub fn init_keyboard_event_with_bubbles_arg_and_cancelable_arg_and_view_arg_and_key_arg(&self, type_arg: &str, bubbles_arg: bool, cancelable_arg: bool, view_arg: Option<&Window>, key_arg: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Window")]
    #[doc = "The `initKeyboardEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/initKeyboardEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEvent`, `Window`*"]
impl KeyboardEvent {
    #[js::import = "$0.initKeyboardEvent($1, $2, $3, $4, $5, $6)"]
pub fn init_keyboard_event_with_bubbles_arg_and_cancelable_arg_and_view_arg_and_key_arg_and_location_arg(&self, type_arg: &str, bubbles_arg: bool, cancelable_arg: bool, view_arg: Option<&Window>, key_arg: &str, location_arg: u32) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Window")]
    #[doc = "The `initKeyboardEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/initKeyboardEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEvent`, `Window`*"]
impl KeyboardEvent {
    #[js::import = "$0.initKeyboardEvent($1, $2, $3, $4, $5, $6, $7)"]
pub fn init_keyboard_event_with_bubbles_arg_and_cancelable_arg_and_view_arg_and_key_arg_and_location_arg_and_ctrl_key(&self, type_arg: &str, bubbles_arg: bool, cancelable_arg: bool, view_arg: Option<&Window>, key_arg: &str, location_arg: u32, ctrl_key: bool) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Window")]
    #[doc = "The `initKeyboardEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/initKeyboardEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEvent`, `Window`*"]
impl KeyboardEvent {
    #[js::import = "$0.initKeyboardEvent($1, $2, $3, $4, $5, $6, $7, $8)"]
pub fn init_keyboard_event_with_bubbles_arg_and_cancelable_arg_and_view_arg_and_key_arg_and_location_arg_and_ctrl_key_and_alt_key(&self, type_arg: &str, bubbles_arg: bool, cancelable_arg: bool, view_arg: Option<&Window>, key_arg: &str, location_arg: u32, ctrl_key: bool, alt_key: bool) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Window")]
    #[doc = "The `initKeyboardEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/initKeyboardEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEvent`, `Window`*"]
impl KeyboardEvent {
    #[js::import = "$0.initKeyboardEvent($1, $2, $3, $4, $5, $6, $7, $8, $9)"]
pub fn init_keyboard_event_with_bubbles_arg_and_cancelable_arg_and_view_arg_and_key_arg_and_location_arg_and_ctrl_key_and_alt_key_and_shift_key(&self, type_arg: &str, bubbles_arg: bool, cancelable_arg: bool, view_arg: Option<&Window>, key_arg: &str, location_arg: u32, ctrl_key: bool, alt_key: bool, shift_key: bool) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Window")]
    #[doc = "The `initKeyboardEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/initKeyboardEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEvent`, `Window`*"]
impl KeyboardEvent {
    #[js::import = "$0.initKeyboardEvent($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)"]
pub fn init_keyboard_event_with_bubbles_arg_and_cancelable_arg_and_view_arg_and_key_arg_and_location_arg_and_ctrl_key_and_alt_key_and_shift_key_and_meta_key(&self, type_arg: &str, bubbles_arg: bool, cancelable_arg: bool, view_arg: Option<&Window>, key_arg: &str, location_arg: u32, ctrl_key: bool, alt_key: bool, shift_key: bool, meta_key: bool) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
impl KeyboardEvent {
    #[doc = "The `KeyboardEvent.DOM_KEY_LOCATION_STANDARD` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEvent`*"]
    pub const DOM_KEY_LOCATION_STANDARD: u32 = 0u64 as u32;
    #[doc = "The `KeyboardEvent.DOM_KEY_LOCATION_LEFT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEvent`*"]
    pub const DOM_KEY_LOCATION_LEFT: u32 = 1u64 as u32;
    #[doc = "The `KeyboardEvent.DOM_KEY_LOCATION_RIGHT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEvent`*"]
    pub const DOM_KEY_LOCATION_RIGHT: u32 = 2u64 as u32;
    #[doc = "The `KeyboardEvent.DOM_KEY_LOCATION_NUMPAD` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyboardEvent`*"]
    pub const DOM_KEY_LOCATION_NUMPAD: u32 = 3u64 as u32;
}
