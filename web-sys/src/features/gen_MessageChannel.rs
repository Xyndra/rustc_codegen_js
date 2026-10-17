#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MessageChannel` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessageChannel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessageChannel`*"]
#[js::opaque]
#[derive(Copy)]
pub struct MessageChannel(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "MessagePort")]
    #[doc = "Getter for the `port1` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessageChannel/port1)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessageChannel`, `MessagePort`*"]
impl MessageChannel {
    #[js::import = "$0.port1"]
pub fn port1(&self) -> MessagePort { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "MessagePort")]
    #[doc = "Getter for the `port2` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessageChannel/port2)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessageChannel`, `MessagePort`*"]
impl MessageChannel {
    #[js::import = "$0.port2"]
pub fn port2(&self) -> MessagePort { panic!("web-sys intrinsic") }
}
    #[doc = "The `new MessageChannel(..)` constructor, creating a new instance of `MessageChannel`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessageChannel/MessageChannel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessageChannel`*"]
impl MessageChannel {
    #[js::import = "new MessageChannel()"]
pub fn new() -> Result<MessageChannel, JsValue> { panic!("web-sys intrinsic") }
}
