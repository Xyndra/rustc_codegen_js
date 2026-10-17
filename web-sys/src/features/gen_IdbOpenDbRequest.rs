#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `IdbOpenDbRequest` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBOpenDBRequest)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbOpenDbRequest`*"]
#[js::opaque]
#[derive(Copy)]
pub struct IdbOpenDbRequest(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `onblocked` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBOpenDBRequest/onblocked)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbOpenDbRequest`*"]
impl IdbOpenDbRequest {
    #[js::import = "$0.onblocked"]
pub fn onblocked(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onblocked` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBOpenDBRequest/onblocked)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbOpenDbRequest`*"]
impl IdbOpenDbRequest {
    #[js::import = "$0.onblocked = $1"]
pub fn set_onblocked(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onupgradeneeded` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBOpenDBRequest/onupgradeneeded)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbOpenDbRequest`*"]
impl IdbOpenDbRequest {
    #[js::import = "$0.onupgradeneeded"]
pub fn onupgradeneeded(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onupgradeneeded` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBOpenDBRequest/onupgradeneeded)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbOpenDbRequest`*"]
impl IdbOpenDbRequest {
    #[js::import = "$0.onupgradeneeded = $1"]
pub fn set_onupgradeneeded(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
