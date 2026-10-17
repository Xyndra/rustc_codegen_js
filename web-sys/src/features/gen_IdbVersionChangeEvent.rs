#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `IdbVersionChangeEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBVersionChangeEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbVersionChangeEvent`*"]
#[js::opaque]
#[derive(Copy)]
pub struct IdbVersionChangeEvent(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `oldVersion` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBVersionChangeEvent/oldVersion)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbVersionChangeEvent`*"]
impl IdbVersionChangeEvent {
    #[js::import = "$0.oldVersion"]
pub fn old_version(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `newVersion` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBVersionChangeEvent/newVersion)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbVersionChangeEvent`*"]
impl IdbVersionChangeEvent {
    #[js::import = "$0.newVersion"]
pub fn new_version(&self) -> Option<f64> { panic!("web-sys intrinsic") }
}
    #[doc = "The `new IdbVersionChangeEvent(..)` constructor, creating a new instance of `IdbVersionChangeEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBVersionChangeEvent/IDBVersionChangeEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbVersionChangeEvent`*"]
impl IdbVersionChangeEvent {
    #[js::import = "new IDBVersionChangeEvent($0)"]
pub fn new(type_: &str) -> Result<IdbVersionChangeEvent, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "IdbVersionChangeEventInit")]
    #[doc = "The `new IdbVersionChangeEvent(..)` constructor, creating a new instance of `IdbVersionChangeEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBVersionChangeEvent/IDBVersionChangeEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbVersionChangeEvent`, `IdbVersionChangeEventInit`*"]
impl IdbVersionChangeEvent {
    #[js::import = "new IDBVersionChangeEvent($0, $1)"]
pub fn new_with_event_init_dict(type_: &str, event_init_dict: &IdbVersionChangeEventInit,) -> Result<IdbVersionChangeEvent, JsValue> { panic!("web-sys intrinsic") }
}
