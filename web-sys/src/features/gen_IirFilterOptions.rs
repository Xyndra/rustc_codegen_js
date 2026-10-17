#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `IirFilterOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IirFilterOptions`*"]
#[js::opaque]
#[derive(Copy)]
pub struct IirFilterOptions(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `channelCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IirFilterOptions`*"]
#[js::import = "None"]
pub fn get_channel_count(this: &IirFilterOptions) -> Option<u32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `channelCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IirFilterOptions`*"]
#[js::import = "None = $1"]
pub fn set_channel_count(this: &IirFilterOptions, val: u32) { panic!("web-sys intrinsic") }
    #[cfg(feature = "ChannelCountMode")]
    #[doc = "Get the `channelCountMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelCountMode`, `IirFilterOptions`*"]
#[js::import = "None"]
pub fn get_channel_count_mode(this: &IirFilterOptions) -> Option<ChannelCountMode> { panic!("web-sys intrinsic") }
    #[cfg(feature = "ChannelCountMode")]
    #[doc = "Change the `channelCountMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelCountMode`, `IirFilterOptions`*"]
#[js::import = "None = $1"]
pub fn set_channel_count_mode(this: &IirFilterOptions, val: ChannelCountMode) { panic!("web-sys intrinsic") }
    #[cfg(feature = "ChannelInterpretation")]
    #[doc = "Get the `channelInterpretation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelInterpretation`, `IirFilterOptions`*"]
#[js::import = "None"]
pub fn get_channel_interpretation(this: &IirFilterOptions) -> Option<ChannelInterpretation> { panic!("web-sys intrinsic") }
    #[cfg(feature = "ChannelInterpretation")]
    #[doc = "Change the `channelInterpretation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelInterpretation`, `IirFilterOptions`*"]
#[js::import = "None = $1"]
pub fn set_channel_interpretation(this: &IirFilterOptions, val: ChannelInterpretation) { panic!("web-sys intrinsic") }
    #[doc = "Get the `feedback` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IirFilterOptions`*"]
#[js::import = "None"]
pub fn get_feedback(this: &IirFilterOptions) -> ::js_sys::Array { panic!("web-sys intrinsic") }
    #[doc = "Change the `feedback` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IirFilterOptions`*"]
#[js::import = "None = $1"]
pub fn set_feedback(this: &IirFilterOptions, val: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
    #[doc = "Get the `feedforward` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IirFilterOptions`*"]
#[js::import = "None"]
pub fn get_feedforward(this: &IirFilterOptions) -> ::js_sys::Array { panic!("web-sys intrinsic") }
    #[doc = "Change the `feedforward` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IirFilterOptions`*"]
#[js::import = "None = $1"]
pub fn set_feedforward(this: &IirFilterOptions, val: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
impl IirFilterOptions {
    #[doc = "Construct a new `IirFilterOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IirFilterOptions`*"]
#[js::import = "None"]
pub fn new(feedback: &::wasm_bindgen::JsValue, feedforward: &::wasm_bindgen::JsValue) -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret.set_feedback(feedback);
        ret.set_feedforward(feedforward);
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
    #[deprecated = "Use `set_feedback()` instead."]
#[js::import = "None"]
pub fn feedback(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self { self.set_feedback(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_feedforward()` instead."]
#[js::import = "None"]
pub fn feedforward(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self { self.set_feedforward(val) { panic!("web-sys intrinsic") }
        self
    }
}
