#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `FontFaceSetIterator` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSetIterator)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceSetIterator`*"]
#[js::opaque]
#[derive(Copy)]
pub struct FontFaceSetIterator(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "FontFaceSetIteratorResult")]
    #[doc = "The `next()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSetIterator/next)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceSetIterator`, `FontFaceSetIteratorResult`*"]
impl FontFaceSetIterator {
    #[js::import = "$0.next()"]
pub fn next(&self) -> Result<FontFaceSetIteratorResult, JsValue> { panic!("web-sys intrinsic") }
}
