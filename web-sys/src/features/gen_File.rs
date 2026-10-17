#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `File` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/File)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `File`*"]
#[js::opaque]
#[derive(Copy)]
pub struct File(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `name` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/File/name)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `File`*"]
impl File {
    #[js::import = "$0.name"]
pub fn name(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `lastModified` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/File/lastModified)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `File`*"]
impl File {
    #[js::import = "$0.lastModified"]
pub fn last_modified(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "The `new File(..)` constructor, creating a new instance of `File`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/File/File)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `File`*"]
impl File {
    #[js::import = "new File($0, $1)"]
pub fn new_with_buffer_source_sequence(file_bits: &::wasm_bindgen::JsValue, file_name: &str,) -> Result<File, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `new File(..)` constructor, creating a new instance of `File`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/File/File)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `File`*"]
impl File {
    #[js::import = "new File($0, $1)"]
pub fn new_with_u8_slice_sequence(file_bits: &::wasm_bindgen::JsValue, file_name: &str,) -> Result<File, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `new File(..)` constructor, creating a new instance of `File`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/File/File)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `File`*"]
impl File {
    #[js::import = "new File($0, $1)"]
pub fn new_with_u8_array_sequence(file_bits: &::wasm_bindgen::JsValue, file_name: &str,) -> Result<File, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `new File(..)` constructor, creating a new instance of `File`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/File/File)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `File`*"]
impl File {
    #[js::import = "new File($0, $1)"]
pub fn new_with_blob_sequence(file_bits: &::wasm_bindgen::JsValue, file_name: &str,) -> Result<File, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `new File(..)` constructor, creating a new instance of `File`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/File/File)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `File`*"]
impl File {
    #[js::import = "new File($0, $1)"]
pub fn new_with_str_sequence(file_bits: &::wasm_bindgen::JsValue, file_name: &str,) -> Result<File, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "FilePropertyBag")]
    #[doc = "The `new File(..)` constructor, creating a new instance of `File`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/File/File)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `File`, `FilePropertyBag`*"]
impl File {
    #[js::import = "new File($0, $1, $2)"]
pub fn new_with_buffer_source_sequence_and_options(file_bits: &::wasm_bindgen::JsValue, file_name: &str, options: &FilePropertyBag,) -> Result<File, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "FilePropertyBag")]
    #[doc = "The `new File(..)` constructor, creating a new instance of `File`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/File/File)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `File`, `FilePropertyBag`*"]
impl File {
    #[js::import = "new File($0, $1, $2)"]
pub fn new_with_u8_slice_sequence_and_options(file_bits: &::wasm_bindgen::JsValue, file_name: &str, options: &FilePropertyBag,) -> Result<File, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "FilePropertyBag")]
    #[doc = "The `new File(..)` constructor, creating a new instance of `File`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/File/File)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `File`, `FilePropertyBag`*"]
impl File {
    #[js::import = "new File($0, $1, $2)"]
pub fn new_with_u8_array_sequence_and_options(file_bits: &::wasm_bindgen::JsValue, file_name: &str, options: &FilePropertyBag,) -> Result<File, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "FilePropertyBag")]
    #[doc = "The `new File(..)` constructor, creating a new instance of `File`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/File/File)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `File`, `FilePropertyBag`*"]
impl File {
    #[js::import = "new File($0, $1, $2)"]
pub fn new_with_blob_sequence_and_options(file_bits: &::wasm_bindgen::JsValue, file_name: &str, options: &FilePropertyBag,) -> Result<File, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "FilePropertyBag")]
    #[doc = "The `new File(..)` constructor, creating a new instance of `File`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/File/File)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `File`, `FilePropertyBag`*"]
impl File {
    #[js::import = "new File($0, $1, $2)"]
pub fn new_with_str_sequence_and_options(file_bits: &::wasm_bindgen::JsValue, file_name: &str, options: &FilePropertyBag,) -> Result<File, JsValue> { panic!("web-sys intrinsic") }
}
