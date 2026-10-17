#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ClipboardEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ClipboardEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ClipboardEvent`*"]
#[js::opaque]
#[derive(Copy)]
pub struct ClipboardEvent(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "DataTransfer")]
    #[doc = "Getter for the `clipboardData` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ClipboardEvent/clipboardData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ClipboardEvent`, `DataTransfer`*"]
impl ClipboardEvent {
    #[js::import = "$0.clipboardData"]
pub fn clipboard_data(&self) -> Option<DataTransfer> { panic!("web-sys intrinsic") }
}
    #[doc = "The `new ClipboardEvent(..)` constructor, creating a new instance of `ClipboardEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ClipboardEvent/ClipboardEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ClipboardEvent`*"]
impl ClipboardEvent {
    #[js::import = "new ClipboardEvent($0)"]
pub fn new(type_: &str) -> Result<ClipboardEvent, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "ClipboardEventInit")]
    #[doc = "The `new ClipboardEvent(..)` constructor, creating a new instance of `ClipboardEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ClipboardEvent/ClipboardEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ClipboardEvent`, `ClipboardEventInit`*"]
impl ClipboardEvent {
    #[js::import = "new ClipboardEvent($0, $1)"]
pub fn new_with_event_init_dict(type_: &str, event_init_dict: &ClipboardEventInit,) -> Result<ClipboardEvent, JsValue> { panic!("web-sys intrinsic") }
}
