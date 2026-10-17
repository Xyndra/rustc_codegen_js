#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ChannelSplitterOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelSplitterOptions`*"]
#[js::opaque]
#[derive(Copy)]
pub struct ChannelSplitterOptions(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `channelCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelSplitterOptions`*"]
#[js::import = "None"]
pub fn get_channel_count(this: &ChannelSplitterOptions) -> Option<u32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `channelCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelSplitterOptions`*"]
#[js::import = "None = $1"]
pub fn set_channel_count(this: &ChannelSplitterOptions, val: u32) { panic!("web-sys intrinsic") }
    #[cfg(feature = "ChannelCountMode")]
    #[doc = "Get the `channelCountMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelCountMode`, `ChannelSplitterOptions`*"]
#[js::import = "None"]
pub fn get_channel_count_mode(this: &ChannelSplitterOptions) -> Option<ChannelCountMode> { panic!("web-sys intrinsic") }
    #[cfg(feature = "ChannelCountMode")]
    #[doc = "Change the `channelCountMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelCountMode`, `ChannelSplitterOptions`*"]
#[js::import = "None = $1"]
pub fn set_channel_count_mode(this: &ChannelSplitterOptions, val: ChannelCountMode) { panic!("web-sys intrinsic") }
    #[cfg(feature = "ChannelInterpretation")]
    #[doc = "Get the `channelInterpretation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelInterpretation`, `ChannelSplitterOptions`*"]
#[js::import = "None"]
pub fn get_channel_interpretation(this: &ChannelSplitterOptions,) -> Option<ChannelInterpretation> { panic!("web-sys intrinsic") }
    #[cfg(feature = "ChannelInterpretation")]
    #[doc = "Change the `channelInterpretation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelInterpretation`, `ChannelSplitterOptions`*"]
#[js::import = "None = $1"]
pub fn set_channel_interpretation(this: &ChannelSplitterOptions, val: ChannelInterpretation) { panic!("web-sys intrinsic") }
    #[doc = "Get the `numberOfOutputs` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelSplitterOptions`*"]
#[js::import = "None"]
pub fn get_number_of_outputs(this: &ChannelSplitterOptions) -> Option<u32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `numberOfOutputs` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelSplitterOptions`*"]
#[js::import = "None = $1"]
pub fn set_number_of_outputs(this: &ChannelSplitterOptions, val: u32) { panic!("web-sys intrinsic") }
impl ChannelSplitterOptions {
    #[doc = "Construct a new `ChannelSplitterOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelSplitterOptions`*"]
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
    #[deprecated = "Use `set_number_of_outputs()` instead."]
#[js::import = "None"]
pub fn number_of_outputs(&mut self, val: u32) -> &mut Self { self.set_number_of_outputs(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for ChannelSplitterOptions {
    fn default() -> Self {
        Self::new()
    }
}
