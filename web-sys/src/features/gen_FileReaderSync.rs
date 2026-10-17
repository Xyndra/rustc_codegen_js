#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `FileReaderSync` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReaderSync)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileReaderSync`*"]
#[js::opaque]
#[derive(Copy)]
pub struct FileReaderSync(pub ::wasm_bindgen::JsValue);
    #[doc = "The `new FileReaderSync(..)` constructor, creating a new instance of `FileReaderSync`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReaderSync/FileReaderSync)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileReaderSync`*"]
impl FileReaderSync {
    #[js::import = "new FileReaderSync()"]
pub fn new() -> Result<FileReaderSync, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Blob")]
    #[doc = "The `readAsArrayBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReaderSync/readAsArrayBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`, `FileReaderSync`*"]
impl FileReaderSync {
    #[js::import = "$0.readAsArrayBuffer($1)"]
pub fn read_as_array_buffer(&self, blob: &Blob) -> Result<::js_sys::ArrayBuffer, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Blob")]
    #[doc = "The `readAsBinaryString()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReaderSync/readAsBinaryString)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`, `FileReaderSync`*"]
impl FileReaderSync {
    #[js::import = "$0.readAsBinaryString($1)"]
pub fn read_as_binary_string(&self, blob: &Blob) -> Result<::alloc::string::String, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Blob")]
    #[doc = "The `readAsDataURL()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReaderSync/readAsDataURL)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`, `FileReaderSync`*"]
impl FileReaderSync {
    #[js::import = "$0.readAsDataURL($1)"]
pub fn read_as_data_url(&self, blob: &Blob) -> Result<::alloc::string::String, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Blob")]
    #[doc = "The `readAsText()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReaderSync/readAsText)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`, `FileReaderSync`*"]
impl FileReaderSync {
    #[js::import = "$0.readAsText($1)"]
pub fn read_as_text(&self, blob: &Blob) -> Result<::alloc::string::String, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Blob")]
    #[doc = "The `readAsText()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileReaderSync/readAsText)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`, `FileReaderSync`*"]
impl FileReaderSync {
    #[js::import = "$0.readAsText($1, $2)"]
pub fn read_as_text_with_encoding(&self, blob: &Blob, encoding: &str) -> Result<::alloc::string::String, JsValue> { panic!("web-sys intrinsic") }
}
