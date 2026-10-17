#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `StereoPannerOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StereoPannerOptions`*"]
#[js::opaque]
#[derive(Copy)]
pub struct StereoPannerOptions(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `channelCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StereoPannerOptions`*"]
#[js::import = "None"]
pub fn get_channel_count(this: &StereoPannerOptions) -> Option<u32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `channelCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StereoPannerOptions`*"]
#[js::import = "None = $1"]
pub fn set_channel_count(this: &StereoPannerOptions, val: u32) { panic!("web-sys intrinsic") }
    #[cfg(feature = "ChannelCountMode")]
    #[doc = "Get the `channelCountMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelCountMode`, `StereoPannerOptions`*"]
#[js::import = "None"]
pub fn get_channel_count_mode(this: &StereoPannerOptions) -> Option<ChannelCountMode> { panic!("web-sys intrinsic") }
    #[cfg(feature = "ChannelCountMode")]
    #[doc = "Change the `channelCountMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelCountMode`, `StereoPannerOptions`*"]
#[js::import = "None = $1"]
pub fn set_channel_count_mode(this: &StereoPannerOptions, val: ChannelCountMode) { panic!("web-sys intrinsic") }
    #[cfg(feature = "ChannelInterpretation")]
    #[doc = "Get the `channelInterpretation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelInterpretation`, `StereoPannerOptions`*"]
#[js::import = "None"]
pub fn get_channel_interpretation(this: &StereoPannerOptions) -> Option<ChannelInterpretation> { panic!("web-sys intrinsic") }
    #[cfg(feature = "ChannelInterpretation")]
    #[doc = "Change the `channelInterpretation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelInterpretation`, `StereoPannerOptions`*"]
#[js::import = "None = $1"]
pub fn set_channel_interpretation(this: &StereoPannerOptions, val: ChannelInterpretation) { panic!("web-sys intrinsic") }
    #[doc = "Get the `pan` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StereoPannerOptions`*"]
#[js::import = "None"]
pub fn get_pan(this: &StereoPannerOptions) -> Option<f32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `pan` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StereoPannerOptions`*"]
#[js::import = "None = $1"]
pub fn set_pan(this: &StereoPannerOptions, val: f32) { panic!("web-sys intrinsic") }
impl StereoPannerOptions {
    #[doc = "Construct a new `StereoPannerOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StereoPannerOptions`*"]
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
    #[deprecated = "Use `set_pan()` instead."]
#[js::import = "None"]
pub fn pan(&mut self, val: f32) -> &mut Self { self.set_pan(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for StereoPannerOptions {
    fn default() -> Self {
        Self::new()
    }
}
