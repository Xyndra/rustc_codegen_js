#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AudioBufferSourceOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBufferSourceOptions`*"]
#[js::opaque]
#[derive(Copy)]
pub struct AudioBufferSourceOptions(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "AudioBuffer")]
    #[doc = "Get the `buffer` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBuffer`, `AudioBufferSourceOptions`*"]
#[js::import = "None"]
pub fn get_buffer(this: &AudioBufferSourceOptions) -> Option<AudioBuffer> { panic!("web-sys intrinsic") }
    #[cfg(feature = "AudioBuffer")]
    #[doc = "Change the `buffer` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBuffer`, `AudioBufferSourceOptions`*"]
#[js::import = "None = $1"]
pub fn set_buffer(this: &AudioBufferSourceOptions, val: Option<&AudioBuffer>) { panic!("web-sys intrinsic") }
    #[doc = "Get the `detune` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBufferSourceOptions`*"]
#[js::import = "None"]
pub fn get_detune(this: &AudioBufferSourceOptions) -> Option<f32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `detune` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBufferSourceOptions`*"]
#[js::import = "None = $1"]
pub fn set_detune(this: &AudioBufferSourceOptions, val: f32) { panic!("web-sys intrinsic") }
    #[doc = "Get the `loop` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBufferSourceOptions`*"]
#[js::import = "None"]
pub fn get_loop(this: &AudioBufferSourceOptions) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `loop` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBufferSourceOptions`*"]
#[js::import = "None = $1"]
pub fn set_loop(this: &AudioBufferSourceOptions, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `loopEnd` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBufferSourceOptions`*"]
#[js::import = "None"]
pub fn get_loop_end(this: &AudioBufferSourceOptions) -> Option<f64> { panic!("web-sys intrinsic") }
    #[doc = "Change the `loopEnd` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBufferSourceOptions`*"]
#[js::import = "None = $1"]
pub fn set_loop_end(this: &AudioBufferSourceOptions, val: f64) { panic!("web-sys intrinsic") }
    #[doc = "Get the `loopStart` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBufferSourceOptions`*"]
#[js::import = "None"]
pub fn get_loop_start(this: &AudioBufferSourceOptions) -> Option<f64> { panic!("web-sys intrinsic") }
    #[doc = "Change the `loopStart` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBufferSourceOptions`*"]
#[js::import = "None = $1"]
pub fn set_loop_start(this: &AudioBufferSourceOptions, val: f64) { panic!("web-sys intrinsic") }
    #[doc = "Get the `playbackRate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBufferSourceOptions`*"]
#[js::import = "None"]
pub fn get_playback_rate(this: &AudioBufferSourceOptions) -> Option<f32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `playbackRate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBufferSourceOptions`*"]
#[js::import = "None = $1"]
pub fn set_playback_rate(this: &AudioBufferSourceOptions, val: f32) { panic!("web-sys intrinsic") }
impl AudioBufferSourceOptions {
    #[doc = "Construct a new `AudioBufferSourceOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBufferSourceOptions`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[cfg(feature = "AudioBuffer")]
    #[deprecated = "Use `set_buffer()` instead."]
#[js::import = "None"]
pub fn buffer(&mut self, val: Option<&AudioBuffer>) -> &mut Self { self.set_buffer(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_detune()` instead."]
#[js::import = "None"]
pub fn detune(&mut self, val: f32) -> &mut Self { self.set_detune(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_loop()` instead."]
#[js::import = "None"]
pub fn loop_(&mut self, val: bool) -> &mut Self { self.set_loop(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_loop_end()` instead."]
#[js::import = "None"]
pub fn loop_end(&mut self, val: f64) -> &mut Self { self.set_loop_end(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_loop_start()` instead."]
#[js::import = "None"]
pub fn loop_start(&mut self, val: f64) -> &mut Self { self.set_loop_start(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_playback_rate()` instead."]
#[js::import = "None"]
pub fn playback_rate(&mut self, val: f32) -> &mut Self { self.set_playback_rate(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for AudioBufferSourceOptions {
    fn default() -> Self {
        Self::new()
    }
}
