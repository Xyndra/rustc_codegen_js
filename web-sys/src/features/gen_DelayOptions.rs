#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DelayOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DelayOptions`*"]
#[js::opaque]
#[derive(Copy)]
pub struct DelayOptions(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `channelCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DelayOptions`*"]
#[js::import = "None"]
pub fn get_channel_count(this: &DelayOptions) -> Option<u32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `channelCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DelayOptions`*"]
#[js::import = "None = $1"]
pub fn set_channel_count(this: &DelayOptions, val: u32) { panic!("web-sys intrinsic") }
    #[cfg(feature = "ChannelCountMode")]
    #[doc = "Get the `channelCountMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelCountMode`, `DelayOptions`*"]
#[js::import = "None"]
pub fn get_channel_count_mode(this: &DelayOptions) -> Option<ChannelCountMode> { panic!("web-sys intrinsic") }
    #[cfg(feature = "ChannelCountMode")]
    #[doc = "Change the `channelCountMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelCountMode`, `DelayOptions`*"]
#[js::import = "None = $1"]
pub fn set_channel_count_mode(this: &DelayOptions, val: ChannelCountMode) { panic!("web-sys intrinsic") }
    #[cfg(feature = "ChannelInterpretation")]
    #[doc = "Get the `channelInterpretation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelInterpretation`, `DelayOptions`*"]
#[js::import = "None"]
pub fn get_channel_interpretation(this: &DelayOptions) -> Option<ChannelInterpretation> { panic!("web-sys intrinsic") }
    #[cfg(feature = "ChannelInterpretation")]
    #[doc = "Change the `channelInterpretation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelInterpretation`, `DelayOptions`*"]
#[js::import = "None = $1"]
pub fn set_channel_interpretation(this: &DelayOptions, val: ChannelInterpretation) { panic!("web-sys intrinsic") }
    #[doc = "Get the `delayTime` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DelayOptions`*"]
#[js::import = "None"]
pub fn get_delay_time(this: &DelayOptions) -> Option<f64> { panic!("web-sys intrinsic") }
    #[doc = "Change the `delayTime` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DelayOptions`*"]
#[js::import = "None = $1"]
pub fn set_delay_time(this: &DelayOptions, val: f64) { panic!("web-sys intrinsic") }
    #[doc = "Get the `maxDelayTime` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DelayOptions`*"]
#[js::import = "None"]
pub fn get_max_delay_time(this: &DelayOptions) -> Option<f64> { panic!("web-sys intrinsic") }
    #[doc = "Change the `maxDelayTime` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DelayOptions`*"]
#[js::import = "None = $1"]
pub fn set_max_delay_time(this: &DelayOptions, val: f64) { panic!("web-sys intrinsic") }
impl DelayOptions {
    #[doc = "Construct a new `DelayOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DelayOptions`*"]
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
    #[deprecated = "Use `set_delay_time()` instead."]
#[js::import = "None"]
pub fn delay_time(&mut self, val: f64) -> &mut Self { self.set_delay_time(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_max_delay_time()` instead."]
#[js::import = "None"]
pub fn max_delay_time(&mut self, val: f64) -> &mut Self { self.set_max_delay_time(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for DelayOptions {
    fn default() -> Self {
        Self::new()
    }
}
