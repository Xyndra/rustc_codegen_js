#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CssPseudoElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSPseudoElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssPseudoElement`*"]
#[js::opaque]
#[derive(Copy)]
pub struct CssPseudoElement(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSPseudoElement/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssPseudoElement`*"]
impl CssPseudoElement {
    #[js::import = "$0.type"]
pub fn type_(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Element")]
    #[doc = "Getter for the `parentElement` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSPseudoElement/parentElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssPseudoElement`, `Element`*"]
impl CssPseudoElement {
    #[js::import = "$0.parentElement"]
pub fn parent_element(&self) -> Element { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "Animation")]
    #[doc = "The `animate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSPseudoElement/animate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Animation`, `CssPseudoElement`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl CssPseudoElement {
    #[js::import = "$0.animate($1)"]
pub fn animate(&self, keyframes: Option<&::js_sys::Object>) -> Animation { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "Animation")]
    #[doc = "The `animate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSPseudoElement/animate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Animation`, `CssPseudoElement`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl CssPseudoElement {
    #[js::import = "$0.animate($1, $2)"]
pub fn animate_with_f64(&self, keyframes: Option<&::js_sys::Object>, options: f64) -> Animation { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(feature = "Animation", feature = "KeyframeAnimationOptions",))]
    #[doc = "The `animate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSPseudoElement/animate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Animation`, `CssPseudoElement`, `KeyframeAnimationOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl CssPseudoElement {
    #[js::import = "$0.animate($1, $2)"]
pub fn animate_with_keyframe_animation_options(&self, keyframes: Option<&::js_sys::Object>, options: &KeyframeAnimationOptions) -> Animation { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `getAnimations()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSPseudoElement/getAnimations)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssPseudoElement`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl CssPseudoElement {
    #[js::import = "$0.getAnimations()"]
pub fn get_animations(&self) -> ::js_sys::Array { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GetAnimationsOptions")]
    #[doc = "The `getAnimations()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSPseudoElement/getAnimations)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssPseudoElement`, `GetAnimationsOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl CssPseudoElement {
    #[js::import = "$0.getAnimations($1)"]
pub fn get_animations_with_options(&self, options: &GetAnimationsOptions) -> ::js_sys::Array { panic!("web-sys intrinsic") }
}
