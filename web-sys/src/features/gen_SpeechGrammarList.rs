#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SpeechGrammarList` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechGrammarList)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechGrammarList`*"]
#[js::opaque]
#[derive(Copy)]
pub struct SpeechGrammarList(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `length` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechGrammarList/length)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechGrammarList`*"]
impl SpeechGrammarList {
    #[js::import = "$0.length"]
pub fn length(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[doc = "The `new SpeechGrammarList(..)` constructor, creating a new instance of `SpeechGrammarList`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechGrammarList/SpeechGrammarList)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechGrammarList`*"]
impl SpeechGrammarList {
    #[js::import = "new SpeechGrammarList()"]
pub fn new() -> Result<SpeechGrammarList, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `addFromString()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechGrammarList/addFromString)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechGrammarList`*"]
impl SpeechGrammarList {
    #[js::import = "$0.addFromString($1)"]
pub fn add_from_string(&self, string: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `addFromString()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechGrammarList/addFromString)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechGrammarList`*"]
impl SpeechGrammarList {
    #[js::import = "$0.addFromString($1, $2)"]
pub fn add_from_string_with_weight(&self, string: &str, weight: f32) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `addFromURI()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechGrammarList/addFromURI)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechGrammarList`*"]
impl SpeechGrammarList {
    #[js::import = "$0.addFromURI($1)"]
pub fn add_from_uri(&self, src: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `addFromURI()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechGrammarList/addFromURI)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechGrammarList`*"]
impl SpeechGrammarList {
    #[js::import = "$0.addFromURI($1, $2)"]
pub fn add_from_uri_with_weight(&self, src: &str, weight: f32) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SpeechGrammar")]
    #[doc = "The `item()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SpeechGrammarList/item)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechGrammar`, `SpeechGrammarList`*"]
impl SpeechGrammarList {
    #[js::import = "$0.item($1)"]
pub fn item(&self, index: u32) -> Result<SpeechGrammar, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SpeechGrammar")]
        catch,
        method,
        structural,
        js_class = "SpeechGrammarList",
        indexing_getter
    )]
    #[doc = "Indexing getter. As in the literal Javascript `this[key]`."]
    #[doc = ""]
    #[doc = ""]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechGrammar`, `SpeechGrammarList`*"]
#[js::import = "None"]
pub fn get(this: &SpeechGrammarList, index: u32) -> Result<SpeechGrammar, JsValue> { panic!("web-sys intrinsic") }
