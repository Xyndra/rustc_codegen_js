#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
#[cfg(web_sys_unstable_apis)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `TaskController` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TaskController)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TaskController`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::opaque]
#[derive(Copy)]
pub struct TaskController(pub ::wasm_bindgen::JsValue);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `new TaskController(..)` constructor, creating a new instance of `TaskController`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TaskController/TaskController)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TaskController`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl TaskController {
    #[js::import = "new TaskController()"]
pub fn new() -> Result<TaskController, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "TaskControllerInit")]
    #[doc = "The `new TaskController(..)` constructor, creating a new instance of `TaskController`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TaskController/TaskController)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TaskController`, `TaskControllerInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl TaskController {
    #[js::import = "new TaskController($0)"]
pub fn new_with_init(init: &TaskControllerInit) -> Result<TaskController, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "TaskPriority")]
    #[doc = "The `setPriority()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TaskController/setPriority)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TaskController`, `TaskPriority`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl TaskController {
    #[js::import = "$0.setPriority($1)"]
pub fn set_priority(&self, priority: TaskPriority) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
