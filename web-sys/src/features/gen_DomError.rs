#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DomError` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMError)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomError`*"]
#[js::opaque]
#[derive(Copy)]
pub struct DomError(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `name` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMError/name)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomError`*"]
impl DomError {
    #[js::import = "$0.name"]
pub fn name(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `message` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMError/message)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomError`*"]
impl DomError {
    #[js::import = "$0.message"]
pub fn message(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "The `new DomError(..)` constructor, creating a new instance of `DomError`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMError/DOMError)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomError`*"]
impl DomError {
    #[js::import = "new DOMError($0)"]
pub fn new(name: &str) -> Result<DomError, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `new DomError(..)` constructor, creating a new instance of `DomError`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMError/DOMError)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomError`*"]
impl DomError {
    #[js::import = "new DOMError($0, $1)"]
pub fn new_with_message(name: &str, message: &str) -> Result<DomError, JsValue> { panic!("web-sys intrinsic") }
}
