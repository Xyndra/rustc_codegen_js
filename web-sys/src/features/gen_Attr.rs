#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Attr` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Attr)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Attr`*"]
#[js::opaque]
#[derive(Copy)]
pub struct Attr(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `localName` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Attr/localName)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Attr`*"]
impl Attr {
    #[js::import = "$0.localName"]
pub fn local_name(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `value` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Attr/value)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Attr`*"]
impl Attr {
    #[js::import = "$0.value"]
pub fn value(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `value` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Attr/value)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Attr`*"]
impl Attr {
    #[js::import = "$0.value = $1"]
pub fn set_value(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `name` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Attr/name)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Attr`*"]
impl Attr {
    #[js::import = "$0.name"]
pub fn name(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `namespaceURI` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Attr/namespaceURI)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Attr`*"]
impl Attr {
    #[js::import = "$0.namespaceURI"]
pub fn namespace_uri(&self) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `prefix` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Attr/prefix)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Attr`*"]
impl Attr {
    #[js::import = "$0.prefix"]
pub fn prefix(&self) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `specified` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Attr/specified)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Attr`*"]
impl Attr {
    #[js::import = "$0.specified"]
pub fn specified(&self) -> bool { panic!("web-sys intrinsic") }
}
