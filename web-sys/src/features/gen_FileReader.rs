#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `FileReader` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReader)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileReader`*"]
#[js::opaque]
#[derive(Copy)]
pub struct FileReader(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `readyState` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/readyState)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileReader`*"]
impl FileReader {
    #[js::import = "$0.readyState"]
pub fn ready_state(&self) -> u16 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `result` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/result)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileReader`*"]
impl FileReader {
    #[js::import = "$0.result"]
pub fn result(&self) -> Result<::wasm_bindgen::JsValue, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "DomException")]
    #[doc = "Getter for the `error` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/error)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomException`, `FileReader`*"]
impl FileReader {
    #[js::import = "$0.error"]
pub fn error(&self) -> Option<DomException> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onloadstart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/onloadstart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileReader`*"]
impl FileReader {
    #[js::import = "$0.onloadstart"]
pub fn onloadstart(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onloadstart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/onloadstart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileReader`*"]
impl FileReader {
    #[js::import = "$0.onloadstart = $1"]
pub fn set_onloadstart(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onprogress` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/onprogress)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileReader`*"]
impl FileReader {
    #[js::import = "$0.onprogress"]
pub fn onprogress(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onprogress` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/onprogress)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileReader`*"]
impl FileReader {
    #[js::import = "$0.onprogress = $1"]
pub fn set_onprogress(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onload` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/onload)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileReader`*"]
impl FileReader {
    #[js::import = "$0.onload"]
pub fn onload(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onload` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/onload)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileReader`*"]
impl FileReader {
    #[js::import = "$0.onload = $1"]
pub fn set_onload(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onabort` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/onabort)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileReader`*"]
impl FileReader {
    #[js::import = "$0.onabort"]
pub fn onabort(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onabort` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/onabort)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileReader`*"]
impl FileReader {
    #[js::import = "$0.onabort = $1"]
pub fn set_onabort(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/onerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileReader`*"]
impl FileReader {
    #[js::import = "$0.onerror"]
pub fn onerror(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/onerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileReader`*"]
impl FileReader {
    #[js::import = "$0.onerror = $1"]
pub fn set_onerror(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onloadend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/onloadend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileReader`*"]
impl FileReader {
    #[js::import = "$0.onloadend"]
pub fn onloadend(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onloadend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/onloadend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileReader`*"]
impl FileReader {
    #[js::import = "$0.onloadend = $1"]
pub fn set_onloadend(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "The `new FileReader(..)` constructor, creating a new instance of `FileReader`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/FileReader)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileReader`*"]
impl FileReader {
    #[js::import = "new FileReader()"]
pub fn new() -> Result<FileReader, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `abort()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/abort)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileReader`*"]
impl FileReader {
    #[js::import = "$0.abort()"]
pub fn abort(&self) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Blob")]
    #[doc = "The `readAsArrayBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/readAsArrayBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`, `FileReader`*"]
impl FileReader {
    #[js::import = "$0.readAsArrayBuffer($1)"]
pub fn read_as_array_buffer(&self, blob: &Blob) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Blob")]
    #[doc = "The `readAsBinaryString()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/readAsBinaryString)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`, `FileReader`*"]
impl FileReader {
    #[js::import = "$0.readAsBinaryString($1)"]
pub fn read_as_binary_string(&self, filedata: &Blob) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Blob")]
    #[doc = "The `readAsDataURL()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/readAsDataURL)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`, `FileReader`*"]
impl FileReader {
    #[js::import = "$0.readAsDataURL($1)"]
pub fn read_as_data_url(&self, blob: &Blob) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Blob")]
    #[doc = "The `readAsText()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/readAsText)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`, `FileReader`*"]
impl FileReader {
    #[js::import = "$0.readAsText($1)"]
pub fn read_as_text(&self, blob: &Blob) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Blob")]
    #[doc = "The `readAsText()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/readAsText)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`, `FileReader`*"]
impl FileReader {
    #[js::import = "$0.readAsText($1, $2)"]
pub fn read_as_text_with_label(&self, blob: &Blob, label: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
impl FileReader {
    #[doc = "The `FileReader.EMPTY` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileReader`*"]
    pub const EMPTY: u16 = 0i64 as u16;
    #[doc = "The `FileReader.LOADING` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileReader`*"]
    pub const LOADING: u16 = 1u64 as u16;
    #[doc = "The `FileReader.DONE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileReader`*"]
    pub const DONE: u16 = 2u64 as u16;
}
