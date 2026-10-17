#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ConvolverOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConvolverOptions`*"]
#[js::opaque]
#[derive(Copy)]
pub struct ConvolverOptions(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `channelCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConvolverOptions`*"]
#[js::import = "None"]
pub fn get_channel_count(this: &ConvolverOptions) -> Option<u32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `channelCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConvolverOptions`*"]
#[js::import = "None = $1"]
pub fn set_channel_count(this: &ConvolverOptions, val: u32) { panic!("web-sys intrinsic") }
    #[cfg(feature = "ChannelCountMode")]
    #[doc = "Get the `channelCountMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelCountMode`, `ConvolverOptions`*"]
#[js::import = "None"]
pub fn get_channel_count_mode(this: &ConvolverOptions) -> Option<ChannelCountMode> { panic!("web-sys intrinsic") }
    #[cfg(feature = "ChannelCountMode")]
    #[doc = "Change the `channelCountMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelCountMode`, `ConvolverOptions`*"]
#[js::import = "None = $1"]
pub fn set_channel_count_mode(this: &ConvolverOptions, val: ChannelCountMode) { panic!("web-sys intrinsic") }
    #[cfg(feature = "ChannelInterpretation")]
    #[doc = "Get the `channelInterpretation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelInterpretation`, `ConvolverOptions`*"]
#[js::import = "None"]
pub fn get_channel_interpretation(this: &ConvolverOptions) -> Option<ChannelInterpretation> { panic!("web-sys intrinsic") }
    #[cfg(feature = "ChannelInterpretation")]
    #[doc = "Change the `channelInterpretation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelInterpretation`, `ConvolverOptions`*"]
#[js::import = "None = $1"]
pub fn set_channel_interpretation(this: &ConvolverOptions, val: ChannelInterpretation) { panic!("web-sys intrinsic") }
    #[cfg(feature = "AudioBuffer")]
    #[doc = "Get the `buffer` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBuffer`, `ConvolverOptions`*"]
#[js::import = "None"]
pub fn get_buffer(this: &ConvolverOptions) -> Option<AudioBuffer> { panic!("web-sys intrinsic") }
    #[cfg(feature = "AudioBuffer")]
    #[doc = "Change the `buffer` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBuffer`, `ConvolverOptions`*"]
#[js::import = "None = $1"]
pub fn set_buffer(this: &ConvolverOptions, val: Option<&AudioBuffer>) { panic!("web-sys intrinsic") }
    #[doc = "Get the `disableNormalization` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConvolverOptions`*"]
#[js::import = "None"]
pub fn get_disable_normalization(this: &ConvolverOptions) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `disableNormalization` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConvolverOptions`*"]
#[js::import = "None = $1"]
pub fn set_disable_normalization(this: &ConvolverOptions, val: bool) { panic!("web-sys intrinsic") }
impl ConvolverOptions {
    #[doc = "Construct a new `ConvolverOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConvolverOptions`*"]
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
    #[cfg(feature = "AudioBuffer")]
    #[deprecated = "Use `set_buffer()` instead."]
#[js::import = "None"]
pub fn buffer(&mut self, val: Option<&AudioBuffer>) -> &mut Self { self.set_buffer(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_disable_normalization()` instead."]
#[js::import = "None"]
pub fn disable_normalization(&mut self, val: bool) -> &mut Self { self.set_disable_normalization(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for ConvolverOptions {
    fn default() -> Self {
        Self::new()
    }
}
