#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `OscillatorOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OscillatorOptions`*"]
#[js::opaque]
#[derive(Copy)]
pub struct OscillatorOptions(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `channelCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OscillatorOptions`*"]
#[js::import = "None"]
pub fn get_channel_count(this: &OscillatorOptions) -> Option<u32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `channelCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OscillatorOptions`*"]
#[js::import = "None = $1"]
pub fn set_channel_count(this: &OscillatorOptions, val: u32) { panic!("web-sys intrinsic") }
    #[cfg(feature = "ChannelCountMode")]
    #[doc = "Get the `channelCountMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelCountMode`, `OscillatorOptions`*"]
#[js::import = "None"]
pub fn get_channel_count_mode(this: &OscillatorOptions) -> Option<ChannelCountMode> { panic!("web-sys intrinsic") }
    #[cfg(feature = "ChannelCountMode")]
    #[doc = "Change the `channelCountMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelCountMode`, `OscillatorOptions`*"]
#[js::import = "None = $1"]
pub fn set_channel_count_mode(this: &OscillatorOptions, val: ChannelCountMode) { panic!("web-sys intrinsic") }
    #[cfg(feature = "ChannelInterpretation")]
    #[doc = "Get the `channelInterpretation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelInterpretation`, `OscillatorOptions`*"]
#[js::import = "None"]
pub fn get_channel_interpretation(this: &OscillatorOptions) -> Option<ChannelInterpretation> { panic!("web-sys intrinsic") }
    #[cfg(feature = "ChannelInterpretation")]
    #[doc = "Change the `channelInterpretation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelInterpretation`, `OscillatorOptions`*"]
#[js::import = "None = $1"]
pub fn set_channel_interpretation(this: &OscillatorOptions, val: ChannelInterpretation) { panic!("web-sys intrinsic") }
    #[doc = "Get the `detune` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OscillatorOptions`*"]
#[js::import = "None"]
pub fn get_detune(this: &OscillatorOptions) -> Option<f32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `detune` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OscillatorOptions`*"]
#[js::import = "None = $1"]
pub fn set_detune(this: &OscillatorOptions, val: f32) { panic!("web-sys intrinsic") }
    #[doc = "Get the `frequency` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OscillatorOptions`*"]
#[js::import = "None"]
pub fn get_frequency(this: &OscillatorOptions) -> Option<f32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `frequency` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OscillatorOptions`*"]
#[js::import = "None = $1"]
pub fn set_frequency(this: &OscillatorOptions, val: f32) { panic!("web-sys intrinsic") }
    #[cfg(feature = "PeriodicWave")]
    #[doc = "Get the `periodicWave` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OscillatorOptions`, `PeriodicWave`*"]
#[js::import = "None"]
pub fn get_periodic_wave(this: &OscillatorOptions) -> Option<PeriodicWave> { panic!("web-sys intrinsic") }
    #[cfg(feature = "PeriodicWave")]
    #[doc = "Change the `periodicWave` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OscillatorOptions`, `PeriodicWave`*"]
#[js::import = "None = $1"]
pub fn set_periodic_wave(this: &OscillatorOptions, val: &PeriodicWave) { panic!("web-sys intrinsic") }
    #[cfg(feature = "OscillatorType")]
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OscillatorOptions`, `OscillatorType`*"]
#[js::import = "None"]
pub fn get_type(this: &OscillatorOptions) -> Option<OscillatorType> { panic!("web-sys intrinsic") }
    #[cfg(feature = "OscillatorType")]
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OscillatorOptions`, `OscillatorType`*"]
#[js::import = "None = $1"]
pub fn set_type(this: &OscillatorOptions, val: OscillatorType) { panic!("web-sys intrinsic") }
impl OscillatorOptions {
    #[doc = "Construct a new `OscillatorOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OscillatorOptions`*"]
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
    #[cfg(feature = "PeriodicWave")]
    #[deprecated = "Use `set_periodic_wave()` instead."]
#[js::import = "None"]
pub fn periodic_wave(&mut self, val: &PeriodicWave) -> &mut Self { self.set_periodic_wave(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(feature = "OscillatorType")]
    #[deprecated = "Use `set_type()` instead."]
#[js::import = "None"]
pub fn type_(&mut self, val: OscillatorType) -> &mut Self { self.set_type(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for OscillatorOptions {
    fn default() -> Self {
        Self::new()
    }
}
