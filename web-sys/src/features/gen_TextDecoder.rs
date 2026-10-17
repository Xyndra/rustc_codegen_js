#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `TextDecoder` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextDecoder)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextDecoder`*"]
#[js::opaque]
#[derive(Copy)]
pub struct TextDecoder(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `encoding` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextDecoder/encoding)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextDecoder`*"]
impl TextDecoder {
    #[js::import = "$0.encoding"]
pub fn encoding(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `fatal` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextDecoder/fatal)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextDecoder`*"]
impl TextDecoder {
    #[js::import = "$0.fatal"]
pub fn fatal(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "The `new TextDecoder(..)` constructor, creating a new instance of `TextDecoder`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextDecoder/TextDecoder)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextDecoder`*"]
impl TextDecoder {
    #[js::import = "new TextDecoder()"]
pub fn new() -> Result<TextDecoder, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `new TextDecoder(..)` constructor, creating a new instance of `TextDecoder`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextDecoder/TextDecoder)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextDecoder`*"]
impl TextDecoder {
    #[js::import = "new TextDecoder($0)"]
pub fn new_with_label(label: &str) -> Result<TextDecoder, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "TextDecoderOptions")]
    #[doc = "The `new TextDecoder(..)` constructor, creating a new instance of `TextDecoder`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextDecoder/TextDecoder)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextDecoder`, `TextDecoderOptions`*"]
impl TextDecoder {
    #[js::import = "new TextDecoder($0, $1)"]
pub fn new_with_label_and_options(label: &str, options: &TextDecoderOptions,) -> Result<TextDecoder, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `decode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextDecoder/decode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextDecoder`*"]
impl TextDecoder {
    #[js::import = "$0.decode()"]
pub fn decode(&self) -> Result<::alloc::string::String, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `decode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextDecoder/decode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextDecoder`*"]
impl TextDecoder {
    #[js::import = "$0.decode($1)"]
pub fn decode_with_buffer_source(&self, input: &::js_sys::Object) -> Result<::alloc::string::String, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `decode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextDecoder/decode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextDecoder`*"]
impl TextDecoder {
    #[js::import = "$0.decode($1)"]
pub fn decode_with_u8_array(&self, input: &[u8]) -> Result<::alloc::string::String, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `decode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextDecoder/decode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextDecoder`*"]
impl TextDecoder {
    #[js::import = "$0.decode($1)"]
pub fn decode_with_js_u8_array(&self, input: &::js_sys::Uint8Array) -> Result<::alloc::string::String, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "TextDecodeOptions")]
    #[doc = "The `decode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextDecoder/decode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextDecodeOptions`, `TextDecoder`*"]
impl TextDecoder {
    #[js::import = "$0.decode($1, $2)"]
pub fn decode_with_buffer_source_and_options(&self, input: &::js_sys::Object, options: &TextDecodeOptions) -> Result<::alloc::string::String, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "TextDecodeOptions")]
    #[doc = "The `decode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextDecoder/decode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextDecodeOptions`, `TextDecoder`*"]
impl TextDecoder {
    #[js::import = "$0.decode($1, $2)"]
pub fn decode_with_u8_array_and_options(&self, input: &[u8], options: &TextDecodeOptions) -> Result<::alloc::string::String, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "TextDecodeOptions")]
    #[doc = "The `decode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextDecoder/decode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextDecodeOptions`, `TextDecoder`*"]
impl TextDecoder {
    #[js::import = "$0.decode($1, $2)"]
pub fn decode_with_js_u8_array_and_options(&self, input: &::js_sys::Uint8Array, options: &TextDecodeOptions) -> Result<::alloc::string::String, JsValue> { panic!("web-sys intrinsic") }
}
