#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Worklet` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Worklet)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Worklet`*"]
#[js::opaque]
#[derive(Copy)]
pub struct Worklet(pub ::wasm_bindgen::JsValue);
    #[doc = "The `addModule()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Worklet/addModule)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Worklet`*"]
impl Worklet {
    #[js::import = "$0.addModule($1)"]
pub fn add_module(&self, module_url: &str) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "WorkletOptions")]
    #[doc = "The `addModule()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Worklet/addModule)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Worklet`, `WorkletOptions`*"]
impl Worklet {
    #[js::import = "$0.addModule($1, $2)"]
pub fn add_module_with_options(&self, module_url: &str, options: &WorkletOptions) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
