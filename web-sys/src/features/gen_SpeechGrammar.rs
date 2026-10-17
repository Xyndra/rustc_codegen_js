#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SpeechGrammar` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechGrammar)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechGrammar`*"]
#[js::opaque]
#[derive(Copy)]
pub struct SpeechGrammar(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `src` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechGrammar/src)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechGrammar`*"]
impl SpeechGrammar {
    #[js::import = "$0.src"]
pub fn src(&self) -> Result<::alloc::string::String, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `src` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechGrammar/src)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechGrammar`*"]
impl SpeechGrammar {
    #[js::import = "$0.src = $1"]
pub fn set_src(&self, value: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `weight` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechGrammar/weight)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechGrammar`*"]
impl SpeechGrammar {
    #[js::import = "$0.weight"]
pub fn weight(&self) -> Result<f32, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `weight` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechGrammar/weight)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechGrammar`*"]
impl SpeechGrammar {
    #[js::import = "$0.weight = $1"]
pub fn set_weight(&self, value: f32) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `new SpeechGrammar(..)` constructor, creating a new instance of `SpeechGrammar`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechGrammar/SpeechGrammar)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechGrammar`*"]
impl SpeechGrammar {
    #[js::import = "new SpeechGrammar()"]
pub fn new() -> Result<SpeechGrammar, JsValue> { panic!("web-sys intrinsic") }
}
