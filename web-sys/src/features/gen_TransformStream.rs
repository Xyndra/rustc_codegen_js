#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `TransformStream` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TransformStream)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TransformStream`*"]
#[js::opaque]
#[derive(Copy)]
pub struct TransformStream(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "ReadableStream")]
    #[doc = "Getter for the `readable` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TransformStream/readable)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStream`, `TransformStream`*"]
impl TransformStream {
    #[js::import = "$0.readable"]
pub fn readable(&self) -> ReadableStream { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "WritableStream")]
    #[doc = "Getter for the `writable` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TransformStream/writable)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TransformStream`, `WritableStream`*"]
impl TransformStream {
    #[js::import = "$0.writable"]
pub fn writable(&self) -> WritableStream { panic!("web-sys intrinsic") }
}
    #[doc = "The `new TransformStream(..)` constructor, creating a new instance of `TransformStream`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TransformStream/TransformStream)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TransformStream`*"]
impl TransformStream {
    #[js::import = "new TransformStream()"]
pub fn new() -> Result<TransformStream, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `new TransformStream(..)` constructor, creating a new instance of `TransformStream`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TransformStream/TransformStream)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TransformStream`*"]
impl TransformStream {
    #[js::import = "new TransformStream($0)"]
pub fn new_with_transformer(transformer: &::js_sys::Object) -> Result<TransformStream, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "QueuingStrategy")]
    #[doc = "The `new TransformStream(..)` constructor, creating a new instance of `TransformStream`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TransformStream/TransformStream)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `QueuingStrategy`, `TransformStream`*"]
impl TransformStream {
    #[js::import = "new TransformStream($0, $1)"]
pub fn new_with_transformer_and_writable_strategy(transformer: &::js_sys::Object, writable_strategy: &QueuingStrategy,) -> Result<TransformStream, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "QueuingStrategy")]
    #[doc = "The `new TransformStream(..)` constructor, creating a new instance of `TransformStream`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TransformStream/TransformStream)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `QueuingStrategy`, `TransformStream`*"]
impl TransformStream {
    #[js::import = "new TransformStream($0, $1, $2)"]
pub fn new_with_transformer_and_writable_strategy_and_readable_strategy(transformer: &::js_sys::Object, writable_strategy: &QueuingStrategy, readable_strategy: &QueuingStrategy,) -> Result<TransformStream, JsValue> { panic!("web-sys intrinsic") }
}
