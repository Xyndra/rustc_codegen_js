#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
#[cfg(web_sys_unstable_apis)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `XrReferenceSpaceEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRReferenceSpaceEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrReferenceSpaceEvent`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::opaque]
#[derive(Copy)]
pub struct XrReferenceSpaceEvent(pub ::wasm_bindgen::JsValue);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "XrReferenceSpace")]
    #[doc = "Getter for the `referenceSpace` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRReferenceSpaceEvent/referenceSpace)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrReferenceSpace`, `XrReferenceSpaceEvent`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl XrReferenceSpaceEvent {
    #[js::import = "$0.referenceSpace"]
pub fn reference_space(&self) -> XrReferenceSpace { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "XrRigidTransform")]
    #[doc = "Getter for the `transform` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRReferenceSpaceEvent/transform)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrReferenceSpaceEvent`, `XrRigidTransform`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl XrReferenceSpaceEvent {
    #[js::import = "$0.transform"]
pub fn transform(&self) -> Option<XrRigidTransform> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "XrReferenceSpaceEventInit")]
    #[doc = "The `new XrReferenceSpaceEvent(..)` constructor, creating a new instance of `XrReferenceSpaceEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRReferenceSpaceEvent/XRReferenceSpaceEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrReferenceSpaceEvent`, `XrReferenceSpaceEventInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl XrReferenceSpaceEvent {
    #[js::import = "new XRReferenceSpaceEvent($0, $1)"]
pub fn new(type_: &str, event_init_dict: &XrReferenceSpaceEventInit,) -> Result<XrReferenceSpaceEvent, JsValue> { panic!("web-sys intrinsic") }
}
