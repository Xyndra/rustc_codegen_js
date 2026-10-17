#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RadioNodeList` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RadioNodeList)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RadioNodeList`*"]
#[js::opaque]
#[derive(Copy)]
pub struct RadioNodeList(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `value` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RadioNodeList/value)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RadioNodeList`*"]
impl RadioNodeList {
    #[js::import = "$0.value"]
pub fn value(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `value` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RadioNodeList/value)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RadioNodeList`*"]
impl RadioNodeList {
    #[js::import = "$0.value = $1"]
pub fn set_value(&self, value: &str) { panic!("web-sys intrinsic") }
}
