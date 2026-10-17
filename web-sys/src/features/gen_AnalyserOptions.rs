#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AnalyserOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnalyserOptions`*"]
#[js::opaque]
#[derive(Copy)]
pub struct AnalyserOptions(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `channelCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnalyserOptions`*"]
#[js::import = "None"]
pub fn get_channel_count(this: &AnalyserOptions) -> Option<u32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `channelCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnalyserOptions`*"]
#[js::import = "None = $1"]
pub fn set_channel_count(this: &AnalyserOptions, val: u32) { panic!("web-sys intrinsic") }
    #[cfg(feature = "ChannelCountMode")]
    #[doc = "Get the `channelCountMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnalyserOptions`, `ChannelCountMode`*"]
#[js::import = "None"]
pub fn get_channel_count_mode(this: &AnalyserOptions) -> Option<ChannelCountMode> { panic!("web-sys intrinsic") }
    #[cfg(feature = "ChannelCountMode")]
    #[doc = "Change the `channelCountMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnalyserOptions`, `ChannelCountMode`*"]
#[js::import = "None = $1"]
pub fn set_channel_count_mode(this: &AnalyserOptions, val: ChannelCountMode) { panic!("web-sys intrinsic") }
    #[cfg(feature = "ChannelInterpretation")]
    #[doc = "Get the `channelInterpretation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnalyserOptions`, `ChannelInterpretation`*"]
#[js::import = "None"]
pub fn get_channel_interpretation(this: &AnalyserOptions) -> Option<ChannelInterpretation> { panic!("web-sys intrinsic") }
    #[cfg(feature = "ChannelInterpretation")]
    #[doc = "Change the `channelInterpretation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnalyserOptions`, `ChannelInterpretation`*"]
#[js::import = "None = $1"]
pub fn set_channel_interpretation(this: &AnalyserOptions, val: ChannelInterpretation) { panic!("web-sys intrinsic") }
    #[doc = "Get the `fftSize` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnalyserOptions`*"]
#[js::import = "None"]
pub fn get_fft_size(this: &AnalyserOptions) -> Option<u32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `fftSize` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnalyserOptions`*"]
#[js::import = "None = $1"]
pub fn set_fft_size(this: &AnalyserOptions, val: u32) { panic!("web-sys intrinsic") }
    #[doc = "Get the `maxDecibels` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnalyserOptions`*"]
#[js::import = "None"]
pub fn get_max_decibels(this: &AnalyserOptions) -> Option<f64> { panic!("web-sys intrinsic") }
    #[doc = "Change the `maxDecibels` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnalyserOptions`*"]
#[js::import = "None = $1"]
pub fn set_max_decibels(this: &AnalyserOptions, val: f64) { panic!("web-sys intrinsic") }
    #[doc = "Get the `minDecibels` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnalyserOptions`*"]
#[js::import = "None"]
pub fn get_min_decibels(this: &AnalyserOptions) -> Option<f64> { panic!("web-sys intrinsic") }
    #[doc = "Change the `minDecibels` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnalyserOptions`*"]
#[js::import = "None = $1"]
pub fn set_min_decibels(this: &AnalyserOptions, val: f64) { panic!("web-sys intrinsic") }
    #[doc = "Get the `smoothingTimeConstant` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnalyserOptions`*"]
#[js::import = "None"]
pub fn get_smoothing_time_constant(this: &AnalyserOptions) -> Option<f64> { panic!("web-sys intrinsic") }
    #[doc = "Change the `smoothingTimeConstant` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnalyserOptions`*"]
#[js::import = "None = $1"]
pub fn set_smoothing_time_constant(this: &AnalyserOptions, val: f64) { panic!("web-sys intrinsic") }
impl AnalyserOptions {
    #[doc = "Construct a new `AnalyserOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnalyserOptions`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_channel_count()` instead."]
#[js::import = "None"]
pub fn channel_count(&mut self, val: u32) -> &mut Self { self.set_channel_count(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(feature = "ChannelCountMode")]
    #[deprecated = "Use `set_channel_count_mode()` instead."]
#[js::import = "None"]
pub fn channel_count_mode(&mut self, val: ChannelCountMode) -> &mut Self { self.set_channel_count_mode(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(feature = "ChannelInterpretation")]
    #[deprecated = "Use `set_channel_interpretation()` instead."]
#[js::import = "None"]
pub fn channel_interpretation(&mut self, val: ChannelInterpretation) -> &mut Self { self.set_channel_interpretation(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_fft_size()` instead."]
#[js::import = "None"]
pub fn fft_size(&mut self, val: u32) -> &mut Self { self.set_fft_size(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_max_decibels()` instead."]
#[js::import = "None"]
pub fn max_decibels(&mut self, val: f64) -> &mut Self { self.set_max_decibels(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_min_decibels()` instead."]
#[js::import = "None"]
pub fn min_decibels(&mut self, val: f64) -> &mut Self { self.set_min_decibels(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_smoothing_time_constant()` instead."]
#[js::import = "None"]
pub fn smoothing_time_constant(&mut self, val: f64) -> &mut Self { self.set_smoothing_time_constant(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for AnalyserOptions {
    fn default() -> Self {
        Self::new()
    }
}
