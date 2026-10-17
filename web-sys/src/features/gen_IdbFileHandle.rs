#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `IdbFileHandle` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbFileHandle`*"]
    #[deprecated]
#[js::opaque]
#[derive(Copy)]
pub struct IdbFileHandle(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "IdbMutableFile")]
    #[doc = "Getter for the `mutableFile` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/mutableFile)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbFileHandle`, `IdbMutableFile`*"]
    #[deprecated]
impl IdbFileHandle {
    #[js::import = "$0.mutableFile"]
pub fn mutable_file(&self) -> Option<IdbMutableFile> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "IdbMutableFile")]
    #[doc = "Getter for the `fileHandle` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/fileHandle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbFileHandle`, `IdbMutableFile`*"]
    #[deprecated]
impl IdbFileHandle {
    #[js::import = "$0.fileHandle"]
pub fn file_handle(&self) -> Option<IdbMutableFile> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `active` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/active)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbFileHandle`*"]
    #[deprecated]
impl IdbFileHandle {
    #[js::import = "$0.active"]
pub fn active(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `location` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/location)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbFileHandle`*"]
    #[deprecated]
impl IdbFileHandle {
    #[js::import = "$0.location"]
pub fn location(&self) -> Option<f64> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `location` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/location)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbFileHandle`*"]
    #[deprecated]
impl IdbFileHandle {
    #[js::import = "$0.location = $1"]
pub fn set_location(&self, value: Option<f64>) { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `location` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/location)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbFileHandle`*"]
    #[deprecated]
impl IdbFileHandle {
    #[js::import = "$0.location = $1"]
pub fn set_location_opt_u32(&self, value: Option<u32>) { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `location` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/location)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbFileHandle`*"]
    #[deprecated]
impl IdbFileHandle {
    #[js::import = "$0.location = $1"]
pub fn set_location_opt_f64(&self, value: Option<f64>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `oncomplete` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/oncomplete)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbFileHandle`*"]
    #[deprecated]
impl IdbFileHandle {
    #[js::import = "$0.oncomplete"]
pub fn oncomplete(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `oncomplete` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/oncomplete)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbFileHandle`*"]
    #[deprecated]
impl IdbFileHandle {
    #[js::import = "$0.oncomplete = $1"]
pub fn set_oncomplete(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onabort` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/onabort)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbFileHandle`*"]
    #[deprecated]
impl IdbFileHandle {
    #[js::import = "$0.onabort"]
pub fn onabort(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onabort` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/onabort)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbFileHandle`*"]
    #[deprecated]
impl IdbFileHandle {
    #[js::import = "$0.onabort = $1"]
pub fn set_onabort(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/onerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbFileHandle`*"]
    #[deprecated]
impl IdbFileHandle {
    #[js::import = "$0.onerror"]
pub fn onerror(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/onerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbFileHandle`*"]
    #[deprecated]
impl IdbFileHandle {
    #[js::import = "$0.onerror = $1"]
pub fn set_onerror(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "The `abort()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/abort)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbFileHandle`*"]
    #[deprecated]
impl IdbFileHandle {
    #[js::import = "$0.abort()"]
pub fn abort(&self) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "IdbFileRequest")]
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbFileHandle`, `IdbFileRequest`*"]
    #[deprecated]
impl IdbFileHandle {
    #[js::import = "$0.append($1)"]
pub fn append_with_str(&self, value: &str) -> Result<Option<IdbFileRequest>, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "IdbFileRequest")]
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbFileHandle`, `IdbFileRequest`*"]
    #[deprecated]
impl IdbFileHandle {
    #[js::import = "$0.append($1)"]
pub fn append_with_array_buffer(&self, value: &::js_sys::ArrayBuffer) -> Result<Option<IdbFileRequest>, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "IdbFileRequest")]
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbFileHandle`, `IdbFileRequest`*"]
    #[deprecated]
impl IdbFileHandle {
    #[js::import = "$0.append($1)"]
pub fn append_with_array_buffer_view(&self, value: &::js_sys::Object) -> Result<Option<IdbFileRequest>, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "IdbFileRequest")]
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbFileHandle`, `IdbFileRequest`*"]
    #[deprecated]
impl IdbFileHandle {
    #[js::import = "$0.append($1)"]
pub fn append_with_u8_array(&self, value: &mut [u8]) -> Result<Option<IdbFileRequest>, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "IdbFileRequest")]
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbFileHandle`, `IdbFileRequest`*"]
    #[deprecated]
impl IdbFileHandle {
    #[js::import = "$0.append($1)"]
pub fn append_with_js_u8_array(&self, value: &::js_sys::Uint8Array) -> Result<Option<IdbFileRequest>, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "Blob", feature = "IdbFileRequest",))]
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`, `IdbFileHandle`, `IdbFileRequest`*"]
    #[deprecated]
impl IdbFileHandle {
    #[js::import = "$0.append($1)"]
pub fn append_with_blob(&self, value: &Blob) -> Result<Option<IdbFileRequest>, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "IdbFileRequest")]
    #[doc = "The `flush()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/flush)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbFileHandle`, `IdbFileRequest`*"]
    #[deprecated]
impl IdbFileHandle {
    #[js::import = "$0.flush()"]
pub fn flush(&self) -> Result<Option<IdbFileRequest>, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "IdbFileRequest")]
    #[doc = "The `getMetadata()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/getMetadata)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbFileHandle`, `IdbFileRequest`*"]
    #[deprecated]
impl IdbFileHandle {
    #[js::import = "$0.getMetadata()"]
pub fn get_metadata(&self) -> Result<Option<IdbFileRequest>, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "IdbFileMetadataParameters", feature = "IdbFileRequest",))]
    #[doc = "The `getMetadata()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/getMetadata)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbFileHandle`, `IdbFileMetadataParameters`, `IdbFileRequest`*"]
    #[deprecated]
impl IdbFileHandle {
    #[js::import = "$0.getMetadata($1)"]
pub fn get_metadata_with_parameters(&self, parameters: &IdbFileMetadataParameters) -> Result<Option<IdbFileRequest>, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "IdbFileRequest")]
    #[doc = "The `readAsArrayBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/readAsArrayBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbFileHandle`, `IdbFileRequest`*"]
    #[deprecated]
impl IdbFileHandle {
    #[js::import = "$0.readAsArrayBuffer($1)"]
pub fn read_as_array_buffer_with_u32(&self, size: u32) -> Result<Option<IdbFileRequest>, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "IdbFileRequest")]
    #[doc = "The `readAsArrayBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/readAsArrayBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbFileHandle`, `IdbFileRequest`*"]
    #[deprecated]
impl IdbFileHandle {
    #[js::import = "$0.readAsArrayBuffer($1)"]
pub fn read_as_array_buffer_with_f64(&self, size: f64) -> Result<Option<IdbFileRequest>, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "IdbFileRequest")]
    #[doc = "The `readAsText()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/readAsText)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbFileHandle`, `IdbFileRequest`*"]
    #[deprecated]
impl IdbFileHandle {
    #[js::import = "$0.readAsText($1)"]
pub fn read_as_text_with_u32(&self, size: u32) -> Result<Option<IdbFileRequest>, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "IdbFileRequest")]
    #[doc = "The `readAsText()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/readAsText)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbFileHandle`, `IdbFileRequest`*"]
    #[deprecated]
impl IdbFileHandle {
    #[js::import = "$0.readAsText($1)"]
pub fn read_as_text_with_f64(&self, size: f64) -> Result<Option<IdbFileRequest>, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "IdbFileRequest")]
    #[doc = "The `readAsText()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/readAsText)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbFileHandle`, `IdbFileRequest`*"]
    #[deprecated]
impl IdbFileHandle {
    #[js::import = "$0.readAsText($1, $2)"]
pub fn read_as_text_with_u32_and_encoding(&self, size: u32, encoding: Option<&str>) -> Result<Option<IdbFileRequest>, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "IdbFileRequest")]
    #[doc = "The `readAsText()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/readAsText)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbFileHandle`, `IdbFileRequest`*"]
    #[deprecated]
impl IdbFileHandle {
    #[js::import = "$0.readAsText($1, $2)"]
pub fn read_as_text_with_f64_and_encoding(&self, size: f64, encoding: Option<&str>) -> Result<Option<IdbFileRequest>, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "IdbFileRequest")]
    #[doc = "The `truncate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/truncate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbFileHandle`, `IdbFileRequest`*"]
    #[deprecated]
impl IdbFileHandle {
    #[js::import = "$0.truncate()"]
pub fn truncate(&self) -> Result<Option<IdbFileRequest>, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "IdbFileRequest")]
    #[doc = "The `truncate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/truncate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbFileHandle`, `IdbFileRequest`*"]
    #[deprecated]
impl IdbFileHandle {
    #[js::import = "$0.truncate($1)"]
pub fn truncate_with_u32(&self, size: u32) -> Result<Option<IdbFileRequest>, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "IdbFileRequest")]
    #[doc = "The `truncate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/truncate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbFileHandle`, `IdbFileRequest`*"]
    #[deprecated]
impl IdbFileHandle {
    #[js::import = "$0.truncate($1)"]
pub fn truncate_with_f64(&self, size: f64) -> Result<Option<IdbFileRequest>, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "IdbFileRequest")]
    #[doc = "The `write()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/write)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbFileHandle`, `IdbFileRequest`*"]
    #[deprecated]
impl IdbFileHandle {
    #[js::import = "$0.write($1)"]
pub fn write_with_str(&self, value: &str) -> Result<Option<IdbFileRequest>, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "IdbFileRequest")]
    #[doc = "The `write()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/write)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbFileHandle`, `IdbFileRequest`*"]
    #[deprecated]
impl IdbFileHandle {
    #[js::import = "$0.write($1)"]
pub fn write_with_array_buffer(&self, value: &::js_sys::ArrayBuffer) -> Result<Option<IdbFileRequest>, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "IdbFileRequest")]
    #[doc = "The `write()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/write)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbFileHandle`, `IdbFileRequest`*"]
    #[deprecated]
impl IdbFileHandle {
    #[js::import = "$0.write($1)"]
pub fn write_with_array_buffer_view(&self, value: &::js_sys::Object) -> Result<Option<IdbFileRequest>, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "IdbFileRequest")]
    #[doc = "The `write()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/write)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbFileHandle`, `IdbFileRequest`*"]
    #[deprecated]
impl IdbFileHandle {
    #[js::import = "$0.write($1)"]
pub fn write_with_u8_array(&self, value: &[u8]) -> Result<Option<IdbFileRequest>, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "IdbFileRequest")]
    #[doc = "The `write()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/write)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbFileHandle`, `IdbFileRequest`*"]
    #[deprecated]
impl IdbFileHandle {
    #[js::import = "$0.write($1)"]
pub fn write_with_js_u8_array(&self, value: &::js_sys::Uint8Array) -> Result<Option<IdbFileRequest>, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "Blob", feature = "IdbFileRequest",))]
    #[doc = "The `write()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/write)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`, `IdbFileHandle`, `IdbFileRequest`*"]
    #[deprecated]
impl IdbFileHandle {
    #[js::import = "$0.write($1)"]
pub fn write_with_blob(&self, value: &Blob) -> Result<Option<IdbFileRequest>, JsValue> { panic!("web-sys intrinsic") }
}
