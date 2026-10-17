#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AudioBufferOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBufferOptions`*"]
#[js::opaque]
#[derive(Copy)]
pub struct AudioBufferOptions(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `length` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBufferOptions`*"]
#[js::import = "None"]
pub fn get_length(this: &AudioBufferOptions) -> u32 { panic!("web-sys intrinsic") }
    #[doc = "Change the `length` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBufferOptions`*"]
#[js::import = "None = $1"]
pub fn set_length(this: &AudioBufferOptions, val: u32) { panic!("web-sys intrinsic") }
    #[doc = "Get the `numberOfChannels` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBufferOptions`*"]
#[js::import = "None"]
pub fn get_number_of_channels(this: &AudioBufferOptions) -> Option<u32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `numberOfChannels` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBufferOptions`*"]
#[js::import = "None = $1"]
pub fn set_number_of_channels(this: &AudioBufferOptions, val: u32) { panic!("web-sys intrinsic") }
    #[doc = "Get the `sampleRate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBufferOptions`*"]
#[js::import = "None"]
pub fn get_sample_rate(this: &AudioBufferOptions) -> f32 { panic!("web-sys intrinsic") }
    #[doc = "Change the `sampleRate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBufferOptions`*"]
#[js::import = "None = $1"]
pub fn set_sample_rate(this: &AudioBufferOptions, val: f32) { panic!("web-sys intrinsic") }
impl AudioBufferOptions {
    #[doc = "Construct a new `AudioBufferOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBufferOptions`*"]
#[js::import = "None"]
pub fn new(length: u32, sample_rate: f32) -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret.set_length(length);
        ret.set_sample_rate(sample_rate);
        ret
    }
    #[deprecated = "Use `set_length()` instead."]
#[js::import = "None"]
pub fn length(&mut self, val: u32) -> &mut Self { self.set_length(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_number_of_channels()` instead."]
#[js::import = "None"]
pub fn number_of_channels(&mut self, val: u32) -> &mut Self { self.set_number_of_channels(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_sample_rate()` instead."]
#[js::import = "None"]
pub fn sample_rate(&mut self, val: f32) -> &mut Self { self.set_sample_rate(val) { panic!("web-sys intrinsic") }
        self
    }
}
