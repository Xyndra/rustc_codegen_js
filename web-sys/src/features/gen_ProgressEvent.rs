#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ProgressEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ProgressEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProgressEvent`*"]
#[js::opaque]
#[derive(Copy)]
pub struct ProgressEvent(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `lengthComputable` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ProgressEvent/lengthComputable)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProgressEvent`*"]
impl ProgressEvent {
    #[js::import = "$0.lengthComputable"]
pub fn length_computable(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `loaded` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ProgressEvent/loaded)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProgressEvent`*"]
impl ProgressEvent {
    #[js::import = "$0.loaded"]
pub fn loaded(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `total` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ProgressEvent/total)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProgressEvent`*"]
impl ProgressEvent {
    #[js::import = "$0.total"]
pub fn total(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "The `new ProgressEvent(..)` constructor, creating a new instance of `ProgressEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ProgressEvent/ProgressEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProgressEvent`*"]
impl ProgressEvent {
    #[js::import = "new ProgressEvent($0)"]
pub fn new(type_: &str) -> Result<ProgressEvent, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "ProgressEventInit")]
    #[doc = "The `new ProgressEvent(..)` constructor, creating a new instance of `ProgressEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ProgressEvent/ProgressEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProgressEvent`, `ProgressEventInit`*"]
impl ProgressEvent {
    #[js::import = "new ProgressEvent($0, $1)"]
pub fn new_with_event_init_dict(type_: &str, event_init_dict: &ProgressEventInit,) -> Result<ProgressEvent, JsValue> { panic!("web-sys intrinsic") }
}
