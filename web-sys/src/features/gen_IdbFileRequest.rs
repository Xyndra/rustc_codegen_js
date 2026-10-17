#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `IdbFileRequest` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileRequest)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbFileRequest`*"]
    #[deprecated]
#[js::opaque]
#[derive(Copy)]
pub struct IdbFileRequest(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "IdbFileHandle")]
    #[doc = "Getter for the `fileHandle` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileRequest/fileHandle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbFileHandle`, `IdbFileRequest`*"]
    #[deprecated]
impl IdbFileRequest {
    #[js::import = "$0.fileHandle"]
pub fn file_handle(&self) -> Option<IdbFileHandle> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "IdbFileHandle")]
    #[doc = "Getter for the `lockedFile` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileRequest/lockedFile)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbFileHandle`, `IdbFileRequest`*"]
    #[deprecated]
impl IdbFileRequest {
    #[js::import = "$0.lockedFile"]
pub fn locked_file(&self) -> Option<IdbFileHandle> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onprogress` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileRequest/onprogress)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbFileRequest`*"]
    #[deprecated]
impl IdbFileRequest {
    #[js::import = "$0.onprogress"]
pub fn onprogress(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onprogress` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileRequest/onprogress)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbFileRequest`*"]
    #[deprecated]
impl IdbFileRequest {
    #[js::import = "$0.onprogress = $1"]
pub fn set_onprogress(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
