#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HashChangeEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HashChangeEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HashChangeEvent`*"]
#[js::opaque]
#[derive(Copy)]
pub struct HashChangeEvent(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `oldURL` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HashChangeEvent/oldURL)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HashChangeEvent`*"]
impl HashChangeEvent {
    #[js::import = "$0.oldURL"]
pub fn old_url(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `newURL` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HashChangeEvent/newURL)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HashChangeEvent`*"]
impl HashChangeEvent {
    #[js::import = "$0.newURL"]
pub fn new_url(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "The `new HashChangeEvent(..)` constructor, creating a new instance of `HashChangeEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HashChangeEvent/HashChangeEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HashChangeEvent`*"]
impl HashChangeEvent {
    #[js::import = "new HashChangeEvent($0)"]
pub fn new(type_: &str) -> Result<HashChangeEvent, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "HashChangeEventInit")]
    #[doc = "The `new HashChangeEvent(..)` constructor, creating a new instance of `HashChangeEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HashChangeEvent/HashChangeEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HashChangeEvent`, `HashChangeEventInit`*"]
impl HashChangeEvent {
    #[js::import = "new HashChangeEvent($0, $1)"]
pub fn new_with_event_init_dict(type_: &str, event_init_dict: &HashChangeEventInit,) -> Result<HashChangeEvent, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `initHashChangeEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HashChangeEvent/initHashChangeEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HashChangeEvent`*"]
impl HashChangeEvent {
    #[js::import = "$0.initHashChangeEvent($1)"]
pub fn init_hash_change_event(&self, type_arg: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "The `initHashChangeEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HashChangeEvent/initHashChangeEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HashChangeEvent`*"]
impl HashChangeEvent {
    #[js::import = "$0.initHashChangeEvent($1, $2)"]
pub fn init_hash_change_event_with_can_bubble_arg(&self, type_arg: &str, can_bubble_arg: bool) { panic!("web-sys intrinsic") }
}
    #[doc = "The `initHashChangeEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HashChangeEvent/initHashChangeEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HashChangeEvent`*"]
impl HashChangeEvent {
    #[js::import = "$0.initHashChangeEvent($1, $2, $3)"]
pub fn init_hash_change_event_with_can_bubble_arg_and_cancelable_arg(&self, type_arg: &str, can_bubble_arg: bool, cancelable_arg: bool) { panic!("web-sys intrinsic") }
}
    #[doc = "The `initHashChangeEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HashChangeEvent/initHashChangeEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HashChangeEvent`*"]
impl HashChangeEvent {
    #[js::import = "$0.initHashChangeEvent($1, $2, $3, $4)"]
pub fn init_hash_change_event_with_can_bubble_arg_and_cancelable_arg_and_old_url_arg(&self, type_arg: &str, can_bubble_arg: bool, cancelable_arg: bool, old_url_arg: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "The `initHashChangeEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HashChangeEvent/initHashChangeEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HashChangeEvent`*"]
impl HashChangeEvent {
    #[js::import = "$0.initHashChangeEvent($1, $2, $3, $4, $5)"]
pub fn init_hash_change_event_with_can_bubble_arg_and_cancelable_arg_and_old_url_arg_and_new_url_arg(&self, type_arg: &str, can_bubble_arg: bool, cancelable_arg: bool, old_url_arg: &str, new_url_arg: &str) { panic!("web-sys intrinsic") }
}
