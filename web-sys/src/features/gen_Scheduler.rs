#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
#[cfg(web_sys_unstable_apis)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Scheduler` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Scheduler)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Scheduler`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::opaque]
#[derive(Copy)]
pub struct Scheduler(pub ::wasm_bindgen::JsValue);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `postTask()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Scheduler/postTask)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Scheduler`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl Scheduler {
    #[js::import = "$0.postTask($1)"]
pub fn post_task(&self, callback: &::js_sys::Function) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "SchedulerPostTaskOptions")]
    #[doc = "The `postTask()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Scheduler/postTask)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Scheduler`, `SchedulerPostTaskOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl Scheduler {
    #[js::import = "$0.postTask($1, $2)"]
pub fn post_task_with_options(&self, callback: &::js_sys::Function, options: &SchedulerPostTaskOptions) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
