#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
#[cfg(web_sys_unstable_apis)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ViewTransition` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ViewTransition`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::opaque]
#[derive(Copy)]
pub struct ViewTransition(pub ::wasm_bindgen::JsValue);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `updateCallbackDone` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition/updateCallbackDone)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ViewTransition`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl ViewTransition {
    #[js::import = "$0.updateCallbackDone"]
pub fn update_callback_done(&self) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `ready` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition/ready)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ViewTransition`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl ViewTransition {
    #[js::import = "$0.ready"]
pub fn ready(&self) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `finished` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition/finished)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ViewTransition`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl ViewTransition {
    #[js::import = "$0.finished"]
pub fn finished(&self) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `skipTransition()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ViewTransition/skipTransition)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ViewTransition`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl ViewTransition {
    #[js::import = "$0.skipTransition()"]
pub fn skip_transition(&self) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
