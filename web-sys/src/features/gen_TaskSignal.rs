#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
#[cfg(web_sys_unstable_apis)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `TaskSignal` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TaskSignal)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TaskSignal`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::opaque]
#[derive(Copy)]
pub struct TaskSignal(pub ::wasm_bindgen::JsValue);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "TaskPriority")]
    #[doc = "Getter for the `priority` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TaskSignal/priority)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TaskPriority`, `TaskSignal`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl TaskSignal {
    #[js::import = "$0.priority"]
pub fn priority(&self) -> TaskPriority { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `onprioritychange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TaskSignal/onprioritychange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TaskSignal`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl TaskSignal {
    #[js::import = "$0.onprioritychange"]
pub fn onprioritychange(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Setter for the `onprioritychange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TaskSignal/onprioritychange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TaskSignal`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl TaskSignal {
    #[js::import = "$0.onprioritychange = $1"]
pub fn set_onprioritychange(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `any()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TaskSignal/any_static)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TaskSignal`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl TaskSignal {
    #[js::import = "TaskSignal.any"]
pub fn any(signals: &::wasm_bindgen::JsValue) -> TaskSignal { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "TaskSignalAnyInit")]
    #[doc = "The `any()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TaskSignal/any_static)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TaskSignal`, `TaskSignalAnyInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl TaskSignal {
    #[js::import = "TaskSignal.any"]
pub fn any_with_init(signals: &::wasm_bindgen::JsValue, init: &TaskSignalAnyInit) -> TaskSignal { panic!("web-sys intrinsic") }
}
