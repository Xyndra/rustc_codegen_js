#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `FileSystemSyncAccessHandle` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemSyncAccessHandle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemSyncAccessHandle`*"]
#[js::opaque]
#[derive(Copy)]
pub struct FileSystemSyncAccessHandle(pub ::wasm_bindgen::JsValue);
    #[doc = "The `close()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemSyncAccessHandle/close)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemSyncAccessHandle`*"]
impl FileSystemSyncAccessHandle {
    #[js::import = "$0.close()"]
pub fn close(&self) { panic!("web-sys intrinsic") }
}
    #[doc = "The `flush()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemSyncAccessHandle/flush)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemSyncAccessHandle`*"]
impl FileSystemSyncAccessHandle {
    #[js::import = "$0.flush()"]
pub fn flush(&self) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `getSize()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemSyncAccessHandle/getSize)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemSyncAccessHandle`*"]
impl FileSystemSyncAccessHandle {
    #[js::import = "$0.getSize()"]
pub fn get_size(&self) -> Result<f64, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `read()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemSyncAccessHandle/read)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemSyncAccessHandle`*"]
impl FileSystemSyncAccessHandle {
    #[js::import = "$0.read($1)"]
pub fn read_with_buffer_source(&self, buffer: &::js_sys::Object) -> Result<f64, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `read()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemSyncAccessHandle/read)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemSyncAccessHandle`*"]
impl FileSystemSyncAccessHandle {
    #[js::import = "$0.read($1)"]
pub fn read_with_u8_array(&self, buffer: &mut [u8]) -> Result<f64, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `read()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemSyncAccessHandle/read)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemSyncAccessHandle`*"]
impl FileSystemSyncAccessHandle {
    #[js::import = "$0.read($1)"]
pub fn read_with_js_u8_array(&self, buffer: &::js_sys::Uint8Array) -> Result<f64, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "FileSystemReadWriteOptions")]
    #[doc = "The `read()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemSyncAccessHandle/read)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemReadWriteOptions`, `FileSystemSyncAccessHandle`*"]
impl FileSystemSyncAccessHandle {
    #[js::import = "$0.read($1, $2)"]
pub fn read_with_buffer_source_and_options(&self, buffer: &::js_sys::Object, options: &FileSystemReadWriteOptions) -> Result<f64, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "FileSystemReadWriteOptions")]
    #[doc = "The `read()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemSyncAccessHandle/read)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemReadWriteOptions`, `FileSystemSyncAccessHandle`*"]
impl FileSystemSyncAccessHandle {
    #[js::import = "$0.read($1, $2)"]
pub fn read_with_u8_array_and_options(&self, buffer: &mut [u8], options: &FileSystemReadWriteOptions) -> Result<f64, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "FileSystemReadWriteOptions")]
    #[doc = "The `read()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemSyncAccessHandle/read)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemReadWriteOptions`, `FileSystemSyncAccessHandle`*"]
impl FileSystemSyncAccessHandle {
    #[js::import = "$0.read($1, $2)"]
pub fn read_with_js_u8_array_and_options(&self, buffer: &::js_sys::Uint8Array, options: &FileSystemReadWriteOptions) -> Result<f64, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `truncate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemSyncAccessHandle/truncate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemSyncAccessHandle`*"]
impl FileSystemSyncAccessHandle {
    #[js::import = "$0.truncate($1)"]
pub fn truncate_with_u32(&self, new_size: u32) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `truncate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemSyncAccessHandle/truncate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemSyncAccessHandle`*"]
impl FileSystemSyncAccessHandle {
    #[js::import = "$0.truncate($1)"]
pub fn truncate_with_f64(&self, new_size: f64) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `write()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemSyncAccessHandle/write)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemSyncAccessHandle`*"]
impl FileSystemSyncAccessHandle {
    #[js::import = "$0.write($1)"]
pub fn write_with_buffer_source(&self, buffer: &::js_sys::Object) -> Result<f64, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `write()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemSyncAccessHandle/write)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemSyncAccessHandle`*"]
impl FileSystemSyncAccessHandle {
    #[js::import = "$0.write($1)"]
pub fn write_with_u8_array(&self, buffer: &[u8]) -> Result<f64, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `write()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemSyncAccessHandle/write)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemSyncAccessHandle`*"]
impl FileSystemSyncAccessHandle {
    #[js::import = "$0.write($1)"]
pub fn write_with_js_u8_array(&self, buffer: &::js_sys::Uint8Array) -> Result<f64, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "FileSystemReadWriteOptions")]
    #[doc = "The `write()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemSyncAccessHandle/write)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemReadWriteOptions`, `FileSystemSyncAccessHandle`*"]
impl FileSystemSyncAccessHandle {
    #[js::import = "$0.write($1, $2)"]
pub fn write_with_buffer_source_and_options(&self, buffer: &::js_sys::Object, options: &FileSystemReadWriteOptions) -> Result<f64, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "FileSystemReadWriteOptions")]
    #[doc = "The `write()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemSyncAccessHandle/write)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemReadWriteOptions`, `FileSystemSyncAccessHandle`*"]
impl FileSystemSyncAccessHandle {
    #[js::import = "$0.write($1, $2)"]
pub fn write_with_u8_array_and_options(&self, buffer: &[u8], options: &FileSystemReadWriteOptions) -> Result<f64, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "FileSystemReadWriteOptions")]
    #[doc = "The `write()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemSyncAccessHandle/write)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemReadWriteOptions`, `FileSystemSyncAccessHandle`*"]
impl FileSystemSyncAccessHandle {
    #[js::import = "$0.write($1, $2)"]
pub fn write_with_js_u8_array_and_options(&self, buffer: &::js_sys::Uint8Array, options: &FileSystemReadWriteOptions) -> Result<f64, JsValue> { panic!("web-sys intrinsic") }
}
