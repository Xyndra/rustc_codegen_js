#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `OfflineAudioCompletionEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineAudioCompletionEventInit`*"]
#[js::opaque]
#[derive(Copy)]
pub struct OfflineAudioCompletionEventInit(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineAudioCompletionEventInit`*"]
#[js::import = "None"]
pub fn get_bubbles(this: &OfflineAudioCompletionEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineAudioCompletionEventInit`*"]
#[js::import = "None = $1"]
pub fn set_bubbles(this: &OfflineAudioCompletionEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineAudioCompletionEventInit`*"]
#[js::import = "None"]
pub fn get_cancelable(this: &OfflineAudioCompletionEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineAudioCompletionEventInit`*"]
#[js::import = "None = $1"]
pub fn set_cancelable(this: &OfflineAudioCompletionEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineAudioCompletionEventInit`*"]
#[js::import = "None"]
pub fn get_composed(this: &OfflineAudioCompletionEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineAudioCompletionEventInit`*"]
#[js::import = "None = $1"]
pub fn set_composed(this: &OfflineAudioCompletionEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[cfg(feature = "AudioBuffer")]
    #[doc = "Get the `renderedBuffer` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBuffer`, `OfflineAudioCompletionEventInit`*"]
#[js::import = "None"]
pub fn get_rendered_buffer(this: &OfflineAudioCompletionEventInit) -> AudioBuffer { panic!("web-sys intrinsic") }
    #[cfg(feature = "AudioBuffer")]
    #[doc = "Change the `renderedBuffer` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBuffer`, `OfflineAudioCompletionEventInit`*"]
#[js::import = "None = $1"]
pub fn set_rendered_buffer(this: &OfflineAudioCompletionEventInit, val: &AudioBuffer) { panic!("web-sys intrinsic") }
impl OfflineAudioCompletionEventInit {
    #[cfg(feature = "AudioBuffer")]
    #[doc = "Construct a new `OfflineAudioCompletionEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBuffer`, `OfflineAudioCompletionEventInit`*"]
#[js::import = "None"]
pub fn new(rendered_buffer: &AudioBuffer) -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret.set_rendered_buffer(rendered_buffer);
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
    #[cfg(feature = "AudioBuffer")]
    #[deprecated = "Use `set_rendered_buffer()` instead."]
#[js::import = "None"]
pub fn rendered_buffer(&mut self, val: &AudioBuffer) -> &mut Self { self.set_rendered_buffer(val) { panic!("web-sys intrinsic") }
        self
    }
}
