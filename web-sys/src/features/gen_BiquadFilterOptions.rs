#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `BiquadFilterOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BiquadFilterOptions`*"]
#[js::opaque]
#[derive(Copy)]
pub struct BiquadFilterOptions(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `channelCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BiquadFilterOptions`*"]
#[js::import = "None"]
pub fn get_channel_count(this: &BiquadFilterOptions) -> Option<u32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `channelCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BiquadFilterOptions`*"]
#[js::import = "None = $1"]
pub fn set_channel_count(this: &BiquadFilterOptions, val: u32) { panic!("web-sys intrinsic") }
    #[cfg(feature = "ChannelCountMode")]
    #[doc = "Get the `channelCountMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BiquadFilterOptions`, `ChannelCountMode`*"]
#[js::import = "None"]
pub fn get_channel_count_mode(this: &BiquadFilterOptions) -> Option<ChannelCountMode> { panic!("web-sys intrinsic") }
    #[cfg(feature = "ChannelCountMode")]
    #[doc = "Change the `channelCountMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BiquadFilterOptions`, `ChannelCountMode`*"]
#[js::import = "None = $1"]
pub fn set_channel_count_mode(this: &BiquadFilterOptions, val: ChannelCountMode) { panic!("web-sys intrinsic") }
    #[cfg(feature = "ChannelInterpretation")]
    #[doc = "Get the `channelInterpretation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BiquadFilterOptions`, `ChannelInterpretation`*"]
#[js::import = "None"]
pub fn get_channel_interpretation(this: &BiquadFilterOptions) -> Option<ChannelInterpretation> { panic!("web-sys intrinsic") }
    #[cfg(feature = "ChannelInterpretation")]
    #[doc = "Change the `channelInterpretation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BiquadFilterOptions`, `ChannelInterpretation`*"]
#[js::import = "None = $1"]
pub fn set_channel_interpretation(this: &BiquadFilterOptions, val: ChannelInterpretation) { panic!("web-sys intrinsic") }
    #[doc = "Get the `Q` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BiquadFilterOptions`*"]
#[js::import = "None"]
pub fn get_q(this: &BiquadFilterOptions) -> Option<f32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `Q` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BiquadFilterOptions`*"]
#[js::import = "None = $1"]
pub fn set_q(this: &BiquadFilterOptions, val: f32) { panic!("web-sys intrinsic") }
    #[doc = "Get the `detune` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BiquadFilterOptions`*"]
#[js::import = "None"]
pub fn get_detune(this: &BiquadFilterOptions) -> Option<f32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `detune` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BiquadFilterOptions`*"]
#[js::import = "None = $1"]
pub fn set_detune(this: &BiquadFilterOptions, val: f32) { panic!("web-sys intrinsic") }
    #[doc = "Get the `frequency` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BiquadFilterOptions`*"]
#[js::import = "None"]
pub fn get_frequency(this: &BiquadFilterOptions) -> Option<f32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `frequency` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BiquadFilterOptions`*"]
#[js::import = "None = $1"]
pub fn set_frequency(this: &BiquadFilterOptions, val: f32) { panic!("web-sys intrinsic") }
    #[doc = "Get the `gain` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BiquadFilterOptions`*"]
#[js::import = "None"]
pub fn get_gain(this: &BiquadFilterOptions) -> Option<f32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `gain` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BiquadFilterOptions`*"]
#[js::import = "None = $1"]
pub fn set_gain(this: &BiquadFilterOptions, val: f32) { panic!("web-sys intrinsic") }
    #[cfg(feature = "BiquadFilterType")]
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BiquadFilterOptions`, `BiquadFilterType`*"]
#[js::import = "None"]
pub fn get_type(this: &BiquadFilterOptions) -> Option<BiquadFilterType> { panic!("web-sys intrinsic") }
    #[cfg(feature = "BiquadFilterType")]
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BiquadFilterOptions`, `BiquadFilterType`*"]
#[js::import = "None = $1"]
pub fn set_type(this: &BiquadFilterOptions, val: BiquadFilterType) { panic!("web-sys intrinsic") }
impl BiquadFilterOptions {
    #[doc = "Construct a new `BiquadFilterOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BiquadFilterOptions`*"]
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
    #[deprecated = "Use `set_q()` instead."]
#[js::import = "None"]
pub fn q(&mut self, val: f32) -> &mut Self { self.set_q(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_detune()` instead."]
#[js::import = "None"]
pub fn detune(&mut self, val: f32) -> &mut Self { self.set_detune(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_frequency()` instead."]
#[js::import = "None"]
pub fn frequency(&mut self, val: f32) -> &mut Self { self.set_frequency(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_gain()` instead."]
#[js::import = "None"]
pub fn gain(&mut self, val: f32) -> &mut Self { self.set_gain(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(feature = "BiquadFilterType")]
    #[deprecated = "Use `set_type()` instead."]
#[js::import = "None"]
pub fn type_(&mut self, val: BiquadFilterType) -> &mut Self { self.set_type(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for BiquadFilterOptions {
    fn default() -> Self {
        Self::new()
    }
}
