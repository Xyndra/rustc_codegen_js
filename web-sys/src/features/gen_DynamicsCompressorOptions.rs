#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DynamicsCompressorOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DynamicsCompressorOptions`*"]
#[js::opaque]
#[derive(Copy)]
pub struct DynamicsCompressorOptions(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `channelCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DynamicsCompressorOptions`*"]
#[js::import = "None"]
pub fn get_channel_count(this: &DynamicsCompressorOptions) -> Option<u32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `channelCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DynamicsCompressorOptions`*"]
#[js::import = "None = $1"]
pub fn set_channel_count(this: &DynamicsCompressorOptions, val: u32) { panic!("web-sys intrinsic") }
    #[cfg(feature = "ChannelCountMode")]
    #[doc = "Get the `channelCountMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelCountMode`, `DynamicsCompressorOptions`*"]
#[js::import = "None"]
pub fn get_channel_count_mode(this: &DynamicsCompressorOptions) -> Option<ChannelCountMode> { panic!("web-sys intrinsic") }
    #[cfg(feature = "ChannelCountMode")]
    #[doc = "Change the `channelCountMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelCountMode`, `DynamicsCompressorOptions`*"]
#[js::import = "None = $1"]
pub fn set_channel_count_mode(this: &DynamicsCompressorOptions, val: ChannelCountMode) { panic!("web-sys intrinsic") }
    #[cfg(feature = "ChannelInterpretation")]
    #[doc = "Get the `channelInterpretation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelInterpretation`, `DynamicsCompressorOptions`*"]
#[js::import = "None"]
pub fn get_channel_interpretation(this: &DynamicsCompressorOptions,) -> Option<ChannelInterpretation> { panic!("web-sys intrinsic") }
    #[cfg(feature = "ChannelInterpretation")]
    #[doc = "Change the `channelInterpretation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelInterpretation`, `DynamicsCompressorOptions`*"]
#[js::import = "None = $1"]
pub fn set_channel_interpretation(this: &DynamicsCompressorOptions, val: ChannelInterpretation) { panic!("web-sys intrinsic") }
    #[doc = "Get the `attack` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DynamicsCompressorOptions`*"]
#[js::import = "None"]
pub fn get_attack(this: &DynamicsCompressorOptions) -> Option<f32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `attack` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DynamicsCompressorOptions`*"]
#[js::import = "None = $1"]
pub fn set_attack(this: &DynamicsCompressorOptions, val: f32) { panic!("web-sys intrinsic") }
    #[doc = "Get the `knee` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DynamicsCompressorOptions`*"]
#[js::import = "None"]
pub fn get_knee(this: &DynamicsCompressorOptions) -> Option<f32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `knee` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DynamicsCompressorOptions`*"]
#[js::import = "None = $1"]
pub fn set_knee(this: &DynamicsCompressorOptions, val: f32) { panic!("web-sys intrinsic") }
    #[doc = "Get the `ratio` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DynamicsCompressorOptions`*"]
#[js::import = "None"]
pub fn get_ratio(this: &DynamicsCompressorOptions) -> Option<f32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `ratio` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DynamicsCompressorOptions`*"]
#[js::import = "None = $1"]
pub fn set_ratio(this: &DynamicsCompressorOptions, val: f32) { panic!("web-sys intrinsic") }
    #[doc = "Get the `release` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DynamicsCompressorOptions`*"]
#[js::import = "None"]
pub fn get_release(this: &DynamicsCompressorOptions) -> Option<f32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `release` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DynamicsCompressorOptions`*"]
#[js::import = "None = $1"]
pub fn set_release(this: &DynamicsCompressorOptions, val: f32) { panic!("web-sys intrinsic") }
    #[doc = "Get the `threshold` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DynamicsCompressorOptions`*"]
#[js::import = "None"]
pub fn get_threshold(this: &DynamicsCompressorOptions) -> Option<f32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `threshold` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DynamicsCompressorOptions`*"]
#[js::import = "None = $1"]
pub fn set_threshold(this: &DynamicsCompressorOptions, val: f32) { panic!("web-sys intrinsic") }
impl DynamicsCompressorOptions {
    #[doc = "Construct a new `DynamicsCompressorOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DynamicsCompressorOptions`*"]
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
    #[deprecated = "Use `set_attack()` instead."]
#[js::import = "None"]
pub fn attack(&mut self, val: f32) -> &mut Self { self.set_attack(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_knee()` instead."]
#[js::import = "None"]
pub fn knee(&mut self, val: f32) -> &mut Self { self.set_knee(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_ratio()` instead."]
#[js::import = "None"]
pub fn ratio(&mut self, val: f32) -> &mut Self { self.set_ratio(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_release()` instead."]
#[js::import = "None"]
pub fn release(&mut self, val: f32) -> &mut Self { self.set_release(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_threshold()` instead."]
#[js::import = "None"]
pub fn threshold(&mut self, val: f32) -> &mut Self { self.set_threshold(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for DynamicsCompressorOptions {
    fn default() -> Self {
        Self::new()
    }
}
