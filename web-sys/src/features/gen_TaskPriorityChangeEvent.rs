#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
#[cfg(web_sys_unstable_apis)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `TaskPriorityChangeEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TaskPriorityChangeEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TaskPriorityChangeEvent`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::opaque]
#[derive(Copy)]
pub struct TaskPriorityChangeEvent(pub ::wasm_bindgen::JsValue);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "TaskPriority")]
    #[doc = "Getter for the `previousPriority` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TaskPriorityChangeEvent/previousPriority)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TaskPriority`, `TaskPriorityChangeEvent`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl TaskPriorityChangeEvent {
    #[js::import = "$0.previousPriority"]
pub fn previous_priority(&self) -> TaskPriority { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "TaskPriorityChangeEventInit")]
    #[doc = "The `new TaskPriorityChangeEvent(..)` constructor, creating a new instance of `TaskPriorityChangeEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TaskPriorityChangeEvent/TaskPriorityChangeEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TaskPriorityChangeEvent`, `TaskPriorityChangeEventInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl TaskPriorityChangeEvent {
    #[js::import = "new TaskPriorityChangeEvent($0, $1)"]
pub fn new(type_: &str, priority_change_event_init_dict: &TaskPriorityChangeEventInit,) -> Result<TaskPriorityChangeEvent, JsValue> { panic!("web-sys intrinsic") }
}
