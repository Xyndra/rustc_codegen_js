#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `BlobEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BlobEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BlobEvent`*"]
#[js::opaque]
#[derive(Copy)]
pub struct BlobEvent(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "Blob")]
    #[doc = "Getter for the `data` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BlobEvent/data)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`, `BlobEvent`*"]
impl BlobEvent {
    #[js::import = "$0.data"]
pub fn data(&self) -> Option<Blob> { panic!("web-sys intrinsic") }
}
    #[doc = "The `new BlobEvent(..)` constructor, creating a new instance of `BlobEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BlobEvent/BlobEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BlobEvent`*"]
impl BlobEvent {
    #[js::import = "new BlobEvent($0)"]
pub fn new(type_: &str) -> Result<BlobEvent, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "BlobEventInit")]
    #[doc = "The `new BlobEvent(..)` constructor, creating a new instance of `BlobEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BlobEvent/BlobEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BlobEvent`, `BlobEventInit`*"]
impl BlobEvent {
    #[js::import = "new BlobEvent($0, $1)"]
pub fn new_with_event_init_dict(type_: &str, event_init_dict: &BlobEventInit,) -> Result<BlobEvent, JsValue> { panic!("web-sys intrinsic") }
}
