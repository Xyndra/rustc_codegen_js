#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
#[cfg(web_sys_unstable_apis)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `XrInputSource` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRInputSource)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrInputSource`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::opaque]
#[derive(Copy)]
pub struct XrInputSource(pub ::wasm_bindgen::JsValue);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "XrHandedness")]
    #[doc = "Getter for the `handedness` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRInputSource/handedness)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrHandedness`, `XrInputSource`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl XrInputSource {
    #[js::import = "$0.handedness"]
pub fn handedness(&self) -> XrHandedness { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "XrTargetRayMode")]
    #[doc = "Getter for the `targetRayMode` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRInputSource/targetRayMode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrInputSource`, `XrTargetRayMode`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl XrInputSource {
    #[js::import = "$0.targetRayMode"]
pub fn target_ray_mode(&self) -> XrTargetRayMode { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "XrSpace")]
    #[doc = "Getter for the `targetRaySpace` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRInputSource/targetRaySpace)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrInputSource`, `XrSpace`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl XrInputSource {
    #[js::import = "$0.targetRaySpace"]
pub fn target_ray_space(&self) -> XrSpace { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "XrSpace")]
    #[doc = "Getter for the `gripSpace` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRInputSource/gripSpace)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrInputSource`, `XrSpace`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl XrInputSource {
    #[js::import = "$0.gripSpace"]
pub fn grip_space(&self) -> Option<XrSpace> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `profiles` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRInputSource/profiles)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrInputSource`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl XrInputSource {
    #[js::import = "$0.profiles"]
pub fn profiles(&self) -> ::js_sys::Array { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "Gamepad")]
    #[doc = "Getter for the `gamepad` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRInputSource/gamepad)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Gamepad`, `XrInputSource`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl XrInputSource {
    #[js::import = "$0.gamepad"]
pub fn gamepad(&self) -> Option<Gamepad> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "XrHand")]
    #[doc = "Getter for the `hand` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRInputSource/hand)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrHand`, `XrInputSource`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl XrInputSource {
    #[js::import = "$0.hand"]
pub fn hand(&self) -> Option<XrHand> { panic!("web-sys intrinsic") }
}
