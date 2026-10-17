#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `FileSystemWritableFileStream` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemWritableFileStream)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemWritableFileStream`*"]
#[js::opaque]
#[derive(Copy)]
pub struct FileSystemWritableFileStream(pub ::wasm_bindgen::JsValue);
    #[doc = "The `seek()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemWritableFileStream/seek)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemWritableFileStream`*"]
impl FileSystemWritableFileStream {
    #[js::import = "$0.seek($1)"]
pub fn seek_with_u32(&self, position: u32) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `seek()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemWritableFileStream/seek)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemWritableFileStream`*"]
impl FileSystemWritableFileStream {
    #[js::import = "$0.seek($1)"]
pub fn seek_with_f64(&self, position: f64) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `truncate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemWritableFileStream/truncate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemWritableFileStream`*"]
impl FileSystemWritableFileStream {
    #[js::import = "$0.truncate($1)"]
pub fn truncate_with_u32(&self, size: u32) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `truncate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemWritableFileStream/truncate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemWritableFileStream`*"]
impl FileSystemWritableFileStream {
    #[js::import = "$0.truncate($1)"]
pub fn truncate_with_f64(&self, size: f64) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `write()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemWritableFileStream/write)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemWritableFileStream`*"]
impl FileSystemWritableFileStream {
    #[js::import = "$0.write($1)"]
pub fn write_with_buffer_source(&self, data: &::js_sys::Object) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `write()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemWritableFileStream/write)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemWritableFileStream`*"]
impl FileSystemWritableFileStream {
    #[js::import = "$0.write($1)"]
pub fn write_with_u8_array(&self, data: &[u8]) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `write()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemWritableFileStream/write)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemWritableFileStream`*"]
impl FileSystemWritableFileStream {
    #[js::import = "$0.write($1)"]
pub fn write_with_js_u8_array(&self, data: &::js_sys::Uint8Array) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Blob")]
    #[doc = "The `write()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemWritableFileStream/write)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`, `FileSystemWritableFileStream`*"]
impl FileSystemWritableFileStream {
    #[js::import = "$0.write($1)"]
pub fn write_with_blob(&self, data: &Blob) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `write()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemWritableFileStream/write)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemWritableFileStream`*"]
impl FileSystemWritableFileStream {
    #[js::import = "$0.write($1)"]
pub fn write_with_str(&self, data: &str) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "WriteParams")]
    #[doc = "The `write()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemWritableFileStream/write)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemWritableFileStream`, `WriteParams`*"]
impl FileSystemWritableFileStream {
    #[js::import = "$0.write($1)"]
pub fn write_with_write_params(&self, data: &WriteParams) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
