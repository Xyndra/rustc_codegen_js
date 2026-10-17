#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WaveShaperOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WaveShaperOptions`*"]
#[js::opaque]
#[derive(Copy)]
pub struct WaveShaperOptions(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `channelCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WaveShaperOptions`*"]
#[js::import = "None"]
pub fn get_channel_count(this: &WaveShaperOptions) -> Option<u32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `channelCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WaveShaperOptions`*"]
#[js::import = "None = $1"]
pub fn set_channel_count(this: &WaveShaperOptions, val: u32) { panic!("web-sys intrinsic") }
    #[cfg(feature = "ChannelCountMode")]
    #[doc = "Get the `channelCountMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelCountMode`, `WaveShaperOptions`*"]
#[js::import = "None"]
pub fn get_channel_count_mode(this: &WaveShaperOptions) -> Option<ChannelCountMode> { panic!("web-sys intrinsic") }
    #[cfg(feature = "ChannelCountMode")]
    #[doc = "Change the `channelCountMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelCountMode`, `WaveShaperOptions`*"]
#[js::import = "None = $1"]
pub fn set_channel_count_mode(this: &WaveShaperOptions, val: ChannelCountMode) { panic!("web-sys intrinsic") }
    #[cfg(feature = "ChannelInterpretation")]
    #[doc = "Get the `channelInterpretation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelInterpretation`, `WaveShaperOptions`*"]
#[js::import = "None"]
pub fn get_channel_interpretation(this: &WaveShaperOptions) -> Option<ChannelInterpretation> { panic!("web-sys intrinsic") }
    #[cfg(feature = "ChannelInterpretation")]
    #[doc = "Change the `channelInterpretation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelInterpretation`, `WaveShaperOptions`*"]
#[js::import = "None = $1"]
pub fn set_channel_interpretation(this: &WaveShaperOptions, val: ChannelInterpretation) { panic!("web-sys intrinsic") }
    #[doc = "Get the `curve` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WaveShaperOptions`*"]
#[js::import = "None"]
pub fn get_curve(this: &WaveShaperOptions) -> Option<::js_sys::Array> { panic!("web-sys intrinsic") }
    #[doc = "Change the `curve` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WaveShaperOptions`*"]
#[js::import = "None = $1"]
pub fn set_curve(this: &WaveShaperOptions, val: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
    #[cfg(feature = "OverSampleType")]
    #[doc = "Get the `oversample` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OverSampleType`, `WaveShaperOptions`*"]
#[js::import = "None"]
pub fn get_oversample(this: &WaveShaperOptions) -> Option<OverSampleType> { panic!("web-sys intrinsic") }
    #[cfg(feature = "OverSampleType")]
    #[doc = "Change the `oversample` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OverSampleType`, `WaveShaperOptions`*"]
#[js::import = "None = $1"]
pub fn set_oversample(this: &WaveShaperOptions, val: OverSampleType) { panic!("web-sys intrinsic") }
impl WaveShaperOptions {
    #[doc = "Construct a new `WaveShaperOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WaveShaperOptions`*"]
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
    #[deprecated = "Use `set_curve()` instead."]
#[js::import = "None"]
pub fn curve(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self { self.set_curve(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(feature = "OverSampleType")]
    #[deprecated = "Use `set_oversample()` instead."]
#[js::import = "None"]
pub fn oversample(&mut self, val: OverSampleType) -> &mut Self { self.set_oversample(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for WaveShaperOptions {
    fn default() -> Self {
        Self::new()
    }
}
