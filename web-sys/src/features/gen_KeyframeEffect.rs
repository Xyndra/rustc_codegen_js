#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `KeyframeEffect` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyframeEffect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyframeEffect`*"]
#[js::opaque]
#[derive(Copy)]
pub struct KeyframeEffect(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `target` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyframeEffect/target)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyframeEffect`*"]
impl KeyframeEffect {
    #[js::import = "$0.target"]
pub fn target(&self) -> Option<::js_sys::Object> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `target` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyframeEffect/target)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyframeEffect`*"]
    #[deprecated]
impl KeyframeEffect {
    #[js::import = "$0.target = $1"]
pub fn set_target(&self, value: Option<&::js_sys::Object>) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Element")]
    #[doc = "Setter for the `target` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyframeEffect/target)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`, `KeyframeEffect`*"]
impl KeyframeEffect {
    #[js::import = "$0.target = $1"]
pub fn set_target_opt_element(&self, value: Option<&Element>) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "CssPseudoElement")]
    #[doc = "Setter for the `target` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyframeEffect/target)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssPseudoElement`, `KeyframeEffect`*"]
impl KeyframeEffect {
    #[js::import = "$0.target = $1"]
pub fn set_target_opt_css_pseudo_element(&self, value: Option<&CssPseudoElement>) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "IterationCompositeOperation")]
    #[doc = "Getter for the `iterationComposite` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyframeEffect/iterationComposite)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IterationCompositeOperation`, `KeyframeEffect`*"]
impl KeyframeEffect {
    #[js::import = "$0.iterationComposite"]
pub fn iteration_composite(&self) -> IterationCompositeOperation { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "IterationCompositeOperation")]
    #[doc = "Setter for the `iterationComposite` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyframeEffect/iterationComposite)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IterationCompositeOperation`, `KeyframeEffect`*"]
impl KeyframeEffect {
    #[js::import = "$0.iterationComposite = $1"]
pub fn set_iteration_composite(&self, value: IterationCompositeOperation) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "CompositeOperation")]
    #[doc = "Getter for the `composite` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyframeEffect/composite)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CompositeOperation`, `KeyframeEffect`*"]
impl KeyframeEffect {
    #[js::import = "$0.composite"]
pub fn composite(&self) -> CompositeOperation { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "CompositeOperation")]
    #[doc = "Setter for the `composite` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyframeEffect/composite)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CompositeOperation`, `KeyframeEffect`*"]
impl KeyframeEffect {
    #[js::import = "$0.composite = $1"]
pub fn set_composite(&self, value: CompositeOperation) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Element")]
    #[doc = "The `new KeyframeEffect(..)` constructor, creating a new instance of `KeyframeEffect`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyframeEffect/KeyframeEffect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`, `KeyframeEffect`*"]
impl KeyframeEffect {
    #[js::import = "new KeyframeEffect($0, $1)"]
pub fn new_with_opt_element_and_keyframes(target: Option<&Element>, keyframes: Option<&::js_sys::Object>,) -> Result<KeyframeEffect, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "CssPseudoElement")]
    #[doc = "The `new KeyframeEffect(..)` constructor, creating a new instance of `KeyframeEffect`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyframeEffect/KeyframeEffect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssPseudoElement`, `KeyframeEffect`*"]
impl KeyframeEffect {
    #[js::import = "new KeyframeEffect($0, $1)"]
pub fn new_with_opt_css_pseudo_element_and_keyframes(target: Option<&CssPseudoElement>, keyframes: Option<&::js_sys::Object>,) -> Result<KeyframeEffect, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Element")]
    #[doc = "The `new KeyframeEffect(..)` constructor, creating a new instance of `KeyframeEffect`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyframeEffect/KeyframeEffect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`, `KeyframeEffect`*"]
impl KeyframeEffect {
    #[js::import = "new KeyframeEffect($0, $1, $2)"]
pub fn new_with_opt_element_and_keyframes_and_f64(target: Option<&Element>, keyframes: Option<&::js_sys::Object>, options: f64,) -> Result<KeyframeEffect, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "CssPseudoElement")]
    #[doc = "The `new KeyframeEffect(..)` constructor, creating a new instance of `KeyframeEffect`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyframeEffect/KeyframeEffect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssPseudoElement`, `KeyframeEffect`*"]
impl KeyframeEffect {
    #[js::import = "new KeyframeEffect($0, $1, $2)"]
pub fn new_with_opt_css_pseudo_element_and_keyframes_and_f64(target: Option<&CssPseudoElement>, keyframes: Option<&::js_sys::Object>, options: f64,) -> Result<KeyframeEffect, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "Element", feature = "KeyframeEffectOptions",))]
    #[doc = "The `new KeyframeEffect(..)` constructor, creating a new instance of `KeyframeEffect`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyframeEffect/KeyframeEffect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`, `KeyframeEffect`, `KeyframeEffectOptions`*"]
impl KeyframeEffect {
    #[js::import = "new KeyframeEffect($0, $1, $2)"]
pub fn new_with_opt_element_and_keyframes_and_keyframe_effect_options(target: Option<&Element>, keyframes: Option<&::js_sys::Object>, options: &KeyframeEffectOptions,) -> Result<KeyframeEffect, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "CssPseudoElement", feature = "KeyframeEffectOptions",))]
    #[doc = "The `new KeyframeEffect(..)` constructor, creating a new instance of `KeyframeEffect`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyframeEffect/KeyframeEffect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssPseudoElement`, `KeyframeEffect`, `KeyframeEffectOptions`*"]
impl KeyframeEffect {
    #[js::import = "new KeyframeEffect($0, $1, $2)"]
pub fn new_with_opt_css_pseudo_element_and_keyframes_and_keyframe_effect_options(target: Option<&CssPseudoElement>, keyframes: Option<&::js_sys::Object>, options: &KeyframeEffectOptions,) -> Result<KeyframeEffect, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `new KeyframeEffect(..)` constructor, creating a new instance of `KeyframeEffect`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyframeEffect/KeyframeEffect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyframeEffect`*"]
impl KeyframeEffect {
    #[js::import = "new KeyframeEffect($0)"]
pub fn new_with_source(source: &KeyframeEffect) -> Result<KeyframeEffect, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `getKeyframes()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyframeEffect/getKeyframes)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyframeEffect`*"]
impl KeyframeEffect {
    #[js::import = "$0.getKeyframes()"]
pub fn get_keyframes(&self) -> Result<::js_sys::Array, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `setKeyframes()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyframeEffect/setKeyframes)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyframeEffect`*"]
impl KeyframeEffect {
    #[js::import = "$0.setKeyframes($1)"]
pub fn set_keyframes(&self, keyframes: Option<&::js_sys::Object>) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
