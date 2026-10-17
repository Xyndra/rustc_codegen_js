#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SpeechSynthesisEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisEventInit`*"]
#[js::opaque]
#[derive(Copy)]
pub struct SpeechSynthesisEventInit(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisEventInit`*"]
#[js::import = "None"]
pub fn get_bubbles(this: &SpeechSynthesisEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisEventInit`*"]
#[js::import = "None = $1"]
pub fn set_bubbles(this: &SpeechSynthesisEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisEventInit`*"]
#[js::import = "None"]
pub fn get_cancelable(this: &SpeechSynthesisEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisEventInit`*"]
#[js::import = "None = $1"]
pub fn set_cancelable(this: &SpeechSynthesisEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisEventInit`*"]
#[js::import = "None"]
pub fn get_composed(this: &SpeechSynthesisEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisEventInit`*"]
#[js::import = "None = $1"]
pub fn set_composed(this: &SpeechSynthesisEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `charIndex` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisEventInit`*"]
#[js::import = "None"]
pub fn get_char_index(this: &SpeechSynthesisEventInit) -> Option<u32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `charIndex` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisEventInit`*"]
#[js::import = "None = $1"]
pub fn set_char_index(this: &SpeechSynthesisEventInit, val: u32) { panic!("web-sys intrinsic") }
    #[doc = "Get the `charLength` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisEventInit`*"]
#[js::import = "None"]
pub fn get_char_length(this: &SpeechSynthesisEventInit) -> Option<u32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `charLength` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisEventInit`*"]
#[js::import = "None = $1"]
pub fn set_char_length(this: &SpeechSynthesisEventInit, val: Option<u32>) { panic!("web-sys intrinsic") }
    #[doc = "Get the `elapsedTime` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisEventInit`*"]
#[js::import = "None"]
pub fn get_elapsed_time(this: &SpeechSynthesisEventInit) -> Option<f32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `elapsedTime` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisEventInit`*"]
#[js::import = "None = $1"]
pub fn set_elapsed_time(this: &SpeechSynthesisEventInit, val: f32) { panic!("web-sys intrinsic") }
    #[doc = "Get the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisEventInit`*"]
#[js::import = "None"]
pub fn get_name(this: &SpeechSynthesisEventInit) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisEventInit`*"]
#[js::import = "None = $1"]
pub fn set_name(this: &SpeechSynthesisEventInit, val: &str) { panic!("web-sys intrinsic") }
    #[cfg(feature = "SpeechSynthesisUtterance")]
    #[doc = "Get the `utterance` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisEventInit`, `SpeechSynthesisUtterance`*"]
#[js::import = "None"]
pub fn get_utterance(this: &SpeechSynthesisEventInit) -> SpeechSynthesisUtterance { panic!("web-sys intrinsic") }
    #[cfg(feature = "SpeechSynthesisUtterance")]
    #[doc = "Change the `utterance` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisEventInit`, `SpeechSynthesisUtterance`*"]
#[js::import = "None = $1"]
pub fn set_utterance(this: &SpeechSynthesisEventInit, val: &SpeechSynthesisUtterance) { panic!("web-sys intrinsic") }
impl SpeechSynthesisEventInit {
    #[cfg(feature = "SpeechSynthesisUtterance")]
    #[doc = "Construct a new `SpeechSynthesisEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesisEventInit`, `SpeechSynthesisUtterance`*"]
#[js::import = "None"]
pub fn new(utterance: &SpeechSynthesisUtterance) -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret.set_utterance(utterance);
        ret
    }
    #[deprecated = "Use `set_bubbles()` instead."]
#[js::import = "None"]
pub fn bubbles(&mut self, val: bool) -> &mut Self { self.set_bubbles(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_cancelable()` instead."]
#[js::import = "None"]
pub fn cancelable(&mut self, val: bool) -> &mut Self { self.set_cancelable(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_composed()` instead."]
#[js::import = "None"]
pub fn composed(&mut self, val: bool) -> &mut Self { self.set_composed(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_char_index()` instead."]
#[js::import = "None"]
pub fn char_index(&mut self, val: u32) -> &mut Self { self.set_char_index(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_char_length()` instead."]
#[js::import = "None"]
pub fn char_length(&mut self, val: Option<u32>) -> &mut Self { self.set_char_length(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_elapsed_time()` instead."]
#[js::import = "None"]
pub fn elapsed_time(&mut self, val: f32) -> &mut Self { self.set_elapsed_time(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_name()` instead."]
#[js::import = "None"]
pub fn name(&mut self, val: &str) -> &mut Self { self.set_name(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(feature = "SpeechSynthesisUtterance")]
    #[deprecated = "Use `set_utterance()` instead."]
#[js::import = "None"]
pub fn utterance(&mut self, val: &SpeechSynthesisUtterance) -> &mut Self { self.set_utterance(val) { panic!("web-sys intrinsic") }
        self
    }
}
