#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
#[cfg(web_sys_unstable_apis)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Scheduling` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Scheduling)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Scheduling`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::opaque]
#[derive(Copy)]
pub struct Scheduling(pub ::wasm_bindgen::JsValue);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `isInputPending()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Scheduling/isInputPending)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Scheduling`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl Scheduling {
    #[js::import = "$0.isInputPending()"]
pub fn is_input_pending(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "IsInputPendingOptions")]
    #[doc = "The `isInputPending()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Scheduling/isInputPending)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IsInputPendingOptions`, `Scheduling`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl Scheduling {
    #[js::import = "$0.isInputPending($1)"]
pub fn is_input_pending_with_is_input_pending_options(&self, is_input_pending_options: &IsInputPendingOptions) -> bool { panic!("web-sys intrinsic") }
}
