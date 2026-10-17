#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AbortSignal` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AbortSignal)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AbortSignal`*"]
#[js::opaque]
#[derive(Copy)]
pub struct AbortSignal(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `aborted` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AbortSignal/aborted)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AbortSignal`*"]
impl AbortSignal {
    #[js::import = "$0.aborted"]
pub fn aborted(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `reason` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AbortSignal/reason)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AbortSignal`*"]
impl AbortSignal {
    #[js::import = "$0.reason"]
pub fn reason(&self) -> ::wasm_bindgen::JsValue { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onabort` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AbortSignal/onabort)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AbortSignal`*"]
impl AbortSignal {
    #[js::import = "$0.onabort"]
pub fn onabort(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onabort` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AbortSignal/onabort)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AbortSignal`*"]
impl AbortSignal {
    #[js::import = "$0.onabort = $1"]
pub fn set_onabort(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "The `abort()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AbortSignal/abort_static)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AbortSignal`*"]
impl AbortSignal {
    #[js::import = "AbortSignal.abort"]
pub fn abort() -> AbortSignal { panic!("web-sys intrinsic") }
}
    #[doc = "The `abort()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AbortSignal/abort_static)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AbortSignal`*"]
impl AbortSignal {
    #[js::import = "AbortSignal.abort"]
pub fn abort_with_reason(reason: &::wasm_bindgen::JsValue) -> AbortSignal { panic!("web-sys intrinsic") }
}
    #[doc = "The `any()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AbortSignal/any_static)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AbortSignal`*"]
impl AbortSignal {
    #[js::import = "AbortSignal.any"]
pub fn any(signals: &::wasm_bindgen::JsValue) -> AbortSignal { panic!("web-sys intrinsic") }
}
    #[doc = "The `throwIfAborted()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AbortSignal/throwIfAborted)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AbortSignal`*"]
impl AbortSignal {
    #[js::import = "$0.throwIfAborted()"]
pub fn throw_if_aborted(&self) { panic!("web-sys intrinsic") }
}
    #[doc = "The `timeout()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AbortSignal/timeout_static)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AbortSignal`*"]
impl AbortSignal {
    #[js::import = "AbortSignal.timeout"]
pub fn timeout_with_u32(milliseconds: u32) -> AbortSignal { panic!("web-sys intrinsic") }
}
    #[doc = "The `timeout()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AbortSignal/timeout_static)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AbortSignal`*"]
impl AbortSignal {
    #[js::import = "AbortSignal.timeout"]
pub fn timeout_with_f64(milliseconds: f64) -> AbortSignal { panic!("web-sys intrinsic") }
}
