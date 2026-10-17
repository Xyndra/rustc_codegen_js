#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `TextEncoder` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextEncoder)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextEncoder`*"]
#[js::opaque]
#[derive(Copy)]
pub struct TextEncoder(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `encoding` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextEncoder/encoding)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextEncoder`*"]
impl TextEncoder {
    #[js::import = "$0.encoding"]
pub fn encoding(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "The `new TextEncoder(..)` constructor, creating a new instance of `TextEncoder`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextEncoder/TextEncoder)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextEncoder`*"]
impl TextEncoder {
    #[js::import = "new TextEncoder()"]
pub fn new() -> Result<TextEncoder, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `encode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextEncoder/encode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextEncoder`*"]
impl TextEncoder {
    #[js::import = "$0.encode()"]
pub fn encode(&self) -> ::alloc::vec::Vec<u8> { panic!("web-sys intrinsic") }
}
    #[doc = "The `encode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextEncoder/encode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextEncoder`*"]
impl TextEncoder {
    #[js::import = "$0.encode($1)"]
pub fn encode_with_input(&self, input: &str) -> ::alloc::vec::Vec<u8> { panic!("web-sys intrinsic") }
}
