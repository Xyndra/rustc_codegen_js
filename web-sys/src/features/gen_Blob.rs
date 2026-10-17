#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Blob` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Blob)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`*"]
#[js::opaque]
#[derive(Copy)]
pub struct Blob(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `size` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Blob/size)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`*"]
impl Blob {
    #[js::import = "$0.size"]
pub fn size(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Blob/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`*"]
impl Blob {
    #[js::import = "$0.type"]
pub fn type_(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "The `new Blob(..)` constructor, creating a new instance of `Blob`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Blob/Blob)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`*"]
impl Blob {
    #[js::import = "new Blob()"]
pub fn new() -> Result<Blob, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `new Blob(..)` constructor, creating a new instance of `Blob`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Blob/Blob)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`*"]
impl Blob {
    #[js::import = "new Blob($0)"]
pub fn new_with_buffer_source_sequence(blob_parts: &::wasm_bindgen::JsValue,) -> Result<Blob, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `new Blob(..)` constructor, creating a new instance of `Blob`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Blob/Blob)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`*"]
impl Blob {
    #[js::import = "new Blob($0)"]
pub fn new_with_u8_slice_sequence(blob_parts: &::wasm_bindgen::JsValue,) -> Result<Blob, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `new Blob(..)` constructor, creating a new instance of `Blob`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Blob/Blob)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`*"]
impl Blob {
    #[js::import = "new Blob($0)"]
pub fn new_with_u8_array_sequence(blob_parts: &::wasm_bindgen::JsValue,) -> Result<Blob, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `new Blob(..)` constructor, creating a new instance of `Blob`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Blob/Blob)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`*"]
impl Blob {
    #[js::import = "new Blob($0)"]
pub fn new_with_blob_sequence(blob_parts: &::wasm_bindgen::JsValue) -> Result<Blob, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `new Blob(..)` constructor, creating a new instance of `Blob`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Blob/Blob)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`*"]
impl Blob {
    #[js::import = "new Blob($0)"]
pub fn new_with_str_sequence(blob_parts: &::wasm_bindgen::JsValue) -> Result<Blob, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "BlobPropertyBag")]
    #[doc = "The `new Blob(..)` constructor, creating a new instance of `Blob`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Blob/Blob)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`, `BlobPropertyBag`*"]
impl Blob {
    #[js::import = "new Blob($0, $1)"]
pub fn new_with_buffer_source_sequence_and_options(blob_parts: &::wasm_bindgen::JsValue, options: &BlobPropertyBag,) -> Result<Blob, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "BlobPropertyBag")]
    #[doc = "The `new Blob(..)` constructor, creating a new instance of `Blob`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Blob/Blob)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`, `BlobPropertyBag`*"]
impl Blob {
    #[js::import = "new Blob($0, $1)"]
pub fn new_with_u8_slice_sequence_and_options(blob_parts: &::wasm_bindgen::JsValue, options: &BlobPropertyBag,) -> Result<Blob, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "BlobPropertyBag")]
    #[doc = "The `new Blob(..)` constructor, creating a new instance of `Blob`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Blob/Blob)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`, `BlobPropertyBag`*"]
impl Blob {
    #[js::import = "new Blob($0, $1)"]
pub fn new_with_u8_array_sequence_and_options(blob_parts: &::wasm_bindgen::JsValue, options: &BlobPropertyBag,) -> Result<Blob, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "BlobPropertyBag")]
    #[doc = "The `new Blob(..)` constructor, creating a new instance of `Blob`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Blob/Blob)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`, `BlobPropertyBag`*"]
impl Blob {
    #[js::import = "new Blob($0, $1)"]
pub fn new_with_blob_sequence_and_options(blob_parts: &::wasm_bindgen::JsValue, options: &BlobPropertyBag,) -> Result<Blob, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "BlobPropertyBag")]
    #[doc = "The `new Blob(..)` constructor, creating a new instance of `Blob`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Blob/Blob)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`, `BlobPropertyBag`*"]
impl Blob {
    #[js::import = "new Blob($0, $1)"]
pub fn new_with_str_sequence_and_options(blob_parts: &::wasm_bindgen::JsValue, options: &BlobPropertyBag,) -> Result<Blob, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `arrayBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Blob/arrayBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`*"]
impl Blob {
    #[js::import = "$0.arrayBuffer()"]
pub fn array_buffer(&self) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[doc = "The `bytes()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Blob/bytes)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`*"]
impl Blob {
    #[js::import = "$0.bytes()"]
pub fn bytes(&self) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[doc = "The `slice()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Blob/slice)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`*"]
impl Blob {
    #[js::import = "$0.slice()"]
pub fn slice(&self) -> Result<Blob, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `slice()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Blob/slice)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`*"]
impl Blob {
    #[js::import = "$0.slice($1)"]
pub fn slice_with_i32(&self, start: i32) -> Result<Blob, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `slice()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Blob/slice)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`*"]
impl Blob {
    #[js::import = "$0.slice($1)"]
pub fn slice_with_f64(&self, start: f64) -> Result<Blob, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `slice()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Blob/slice)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`*"]
impl Blob {
    #[js::import = "$0.slice($1, $2)"]
pub fn slice_with_i32_and_i32(&self, start: i32, end: i32) -> Result<Blob, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `slice()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Blob/slice)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`*"]
impl Blob {
    #[js::import = "$0.slice($1, $2)"]
pub fn slice_with_f64_and_i32(&self, start: f64, end: i32) -> Result<Blob, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `slice()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Blob/slice)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`*"]
impl Blob {
    #[js::import = "$0.slice($1, $2)"]
pub fn slice_with_i32_and_f64(&self, start: i32, end: f64) -> Result<Blob, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `slice()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Blob/slice)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`*"]
impl Blob {
    #[js::import = "$0.slice($1, $2)"]
pub fn slice_with_f64_and_f64(&self, start: f64, end: f64) -> Result<Blob, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `slice()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Blob/slice)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`*"]
impl Blob {
    #[js::import = "$0.slice($1, $2, $3)"]
pub fn slice_with_i32_and_i32_and_content_type(&self, start: i32, end: i32, content_type: &str) -> Result<Blob, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `slice()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Blob/slice)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`*"]
impl Blob {
    #[js::import = "$0.slice($1, $2, $3)"]
pub fn slice_with_f64_and_i32_and_content_type(&self, start: f64, end: i32, content_type: &str) -> Result<Blob, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `slice()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Blob/slice)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`*"]
impl Blob {
    #[js::import = "$0.slice($1, $2, $3)"]
pub fn slice_with_i32_and_f64_and_content_type(&self, start: i32, end: f64, content_type: &str) -> Result<Blob, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `slice()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Blob/slice)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`*"]
impl Blob {
    #[js::import = "$0.slice($1, $2, $3)"]
pub fn slice_with_f64_and_f64_and_content_type(&self, start: f64, end: f64, content_type: &str) -> Result<Blob, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "ReadableStream")]
    #[doc = "The `stream()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Blob/stream)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`, `ReadableStream`*"]
impl Blob {
    #[js::import = "$0.stream()"]
pub fn stream(&self) -> ReadableStream { panic!("web-sys intrinsic") }
}
    #[doc = "The `text()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Blob/text)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`*"]
impl Blob {
    #[js::import = "$0.text()"]
pub fn text(&self) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
