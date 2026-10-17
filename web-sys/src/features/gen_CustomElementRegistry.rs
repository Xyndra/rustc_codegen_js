#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CustomElementRegistry` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CustomElementRegistry)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CustomElementRegistry`*"]
#[js::opaque]
#[derive(Copy)]
pub struct CustomElementRegistry(pub ::wasm_bindgen::JsValue);
    #[doc = "The `define()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CustomElementRegistry/define)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CustomElementRegistry`*"]
impl CustomElementRegistry {
    #[js::import = "$0.define($1, $2)"]
pub fn define(&self, name: &str, function_constructor: &::js_sys::Function) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "ElementDefinitionOptions")]
    #[doc = "The `define()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CustomElementRegistry/define)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CustomElementRegistry`, `ElementDefinitionOptions`*"]
impl CustomElementRegistry {
    #[js::import = "$0.define($1, $2, $3)"]
pub fn define_with_options(&self, name: &str, function_constructor: &::js_sys::Function, options: &ElementDefinitionOptions) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `get()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CustomElementRegistry/get)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CustomElementRegistry`*"]
impl CustomElementRegistry {
    #[js::import = "$0.get($1)"]
pub fn get(&self, name: &str) -> ::wasm_bindgen::JsValue { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Node")]
    #[doc = "The `upgrade()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CustomElementRegistry/upgrade)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CustomElementRegistry`, `Node`*"]
impl CustomElementRegistry {
    #[js::import = "$0.upgrade($1)"]
pub fn upgrade(&self, root: &Node) { panic!("web-sys intrinsic") }
}
    #[doc = "The `whenDefined()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CustomElementRegistry/whenDefined)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CustomElementRegistry`*"]
impl CustomElementRegistry {
    #[js::import = "$0.whenDefined($1)"]
pub fn when_defined(&self, name: &str) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
