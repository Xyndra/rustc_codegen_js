#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Credential` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Credential)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Credential`*"]
#[js::opaque]
#[derive(Copy)]
pub struct Credential(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `id` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Credential/id)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Credential`*"]
impl Credential {
    #[js::import = "$0.id"]
pub fn id(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Credential/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Credential`*"]
impl Credential {
    #[js::import = "$0.type"]
pub fn type_(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
