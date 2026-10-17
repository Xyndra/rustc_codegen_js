#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
#[cfg(web_sys_unstable_apis)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `IsInputPendingOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IsInputPendingOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::opaque]
#[derive(Copy)]
pub struct IsInputPendingOptions(pub ::wasm_bindgen::JsValue);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `includeContinuous` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IsInputPendingOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None"]
pub fn get_include_continuous(this: &IsInputPendingOptions) -> Option<bool> { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `includeContinuous` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IsInputPendingOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None = $1"]
pub fn set_include_continuous(this: &IsInputPendingOptions, val: bool) { panic!("web-sys intrinsic") }
#[cfg(web_sys_unstable_apis)]
impl IsInputPendingOptions {
    #[doc = "Construct a new `IsInputPendingOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IsInputPendingOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_include_continuous()` instead."]
#[js::import = "None"]
pub fn include_continuous(&mut self, val: bool) -> &mut Self { self.set_include_continuous(val) { panic!("web-sys intrinsic") }
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for IsInputPendingOptions {
    fn default() -> Self {
        Self::new()
    }
}
