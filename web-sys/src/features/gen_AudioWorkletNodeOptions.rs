#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AudioWorkletNodeOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioWorkletNodeOptions`*"]
#[js::opaque]
#[derive(Copy)]
pub struct AudioWorkletNodeOptions(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `channelCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioWorkletNodeOptions`*"]
#[js::import = "None"]
pub fn get_channel_count(this: &AudioWorkletNodeOptions) -> Option<u32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `channelCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioWorkletNodeOptions`*"]
#[js::import = "None = $1"]
pub fn set_channel_count(this: &AudioWorkletNodeOptions, val: u32) { panic!("web-sys intrinsic") }
    #[cfg(feature = "ChannelCountMode")]
    #[doc = "Get the `channelCountMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioWorkletNodeOptions`, `ChannelCountMode`*"]
#[js::import = "None"]
pub fn get_channel_count_mode(this: &AudioWorkletNodeOptions) -> Option<ChannelCountMode> { panic!("web-sys intrinsic") }
    #[cfg(feature = "ChannelCountMode")]
    #[doc = "Change the `channelCountMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioWorkletNodeOptions`, `ChannelCountMode`*"]
#[js::import = "None = $1"]
pub fn set_channel_count_mode(this: &AudioWorkletNodeOptions, val: ChannelCountMode) { panic!("web-sys intrinsic") }
    #[cfg(feature = "ChannelInterpretation")]
    #[doc = "Get the `channelInterpretation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioWorkletNodeOptions`, `ChannelInterpretation`*"]
#[js::import = "None"]
pub fn get_channel_interpretation(this: &AudioWorkletNodeOptions,) -> Option<ChannelInterpretation> { panic!("web-sys intrinsic") }
    #[cfg(feature = "ChannelInterpretation")]
    #[doc = "Change the `channelInterpretation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioWorkletNodeOptions`, `ChannelInterpretation`*"]
#[js::import = "None = $1"]
pub fn set_channel_interpretation(this: &AudioWorkletNodeOptions, val: ChannelInterpretation) { panic!("web-sys intrinsic") }
    #[doc = "Get the `numberOfInputs` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioWorkletNodeOptions`*"]
#[js::import = "None"]
pub fn get_number_of_inputs(this: &AudioWorkletNodeOptions) -> Option<u32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `numberOfInputs` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioWorkletNodeOptions`*"]
#[js::import = "None = $1"]
pub fn set_number_of_inputs(this: &AudioWorkletNodeOptions, val: u32) { panic!("web-sys intrinsic") }
    #[doc = "Get the `numberOfOutputs` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioWorkletNodeOptions`*"]
#[js::import = "None"]
pub fn get_number_of_outputs(this: &AudioWorkletNodeOptions) -> Option<u32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `numberOfOutputs` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioWorkletNodeOptions`*"]
#[js::import = "None = $1"]
pub fn set_number_of_outputs(this: &AudioWorkletNodeOptions, val: u32) { panic!("web-sys intrinsic") }
    #[doc = "Get the `outputChannelCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioWorkletNodeOptions`*"]
#[js::import = "None"]
pub fn get_output_channel_count(this: &AudioWorkletNodeOptions) -> Option<::js_sys::Array> { panic!("web-sys intrinsic") }
    #[doc = "Change the `outputChannelCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioWorkletNodeOptions`*"]
#[js::import = "None = $1"]
pub fn set_output_channel_count(this: &AudioWorkletNodeOptions, val: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
    #[doc = "Get the `parameterData` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioWorkletNodeOptions`*"]
#[js::import = "None"]
pub fn get_parameter_data(this: &AudioWorkletNodeOptions) -> Option<::js_sys::Object> { panic!("web-sys intrinsic") }
    #[doc = "Change the `parameterData` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioWorkletNodeOptions`*"]
#[js::import = "None = $1"]
pub fn set_parameter_data(this: &AudioWorkletNodeOptions, val: &::js_sys::Object) { panic!("web-sys intrinsic") }
    #[doc = "Get the `processorOptions` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioWorkletNodeOptions`*"]
#[js::import = "None"]
pub fn get_processor_options(this: &AudioWorkletNodeOptions) -> Option<::js_sys::Object> { panic!("web-sys intrinsic") }
    #[doc = "Change the `processorOptions` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioWorkletNodeOptions`*"]
#[js::import = "None = $1"]
pub fn set_processor_options(this: &AudioWorkletNodeOptions, val: Option<&::js_sys::Object>) { panic!("web-sys intrinsic") }
impl AudioWorkletNodeOptions {
    #[doc = "Construct a new `AudioWorkletNodeOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioWorkletNodeOptions`*"]
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
    #[deprecated = "Use `set_number_of_inputs()` instead."]
#[js::import = "None"]
pub fn number_of_inputs(&mut self, val: u32) -> &mut Self { self.set_number_of_inputs(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_number_of_outputs()` instead."]
#[js::import = "None"]
pub fn number_of_outputs(&mut self, val: u32) -> &mut Self { self.set_number_of_outputs(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_output_channel_count()` instead."]
#[js::import = "None"]
pub fn output_channel_count(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self { self.set_output_channel_count(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_parameter_data()` instead."]
#[js::import = "None"]
pub fn parameter_data(&mut self, val: &::js_sys::Object) -> &mut Self { self.set_parameter_data(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_processor_options()` instead."]
#[js::import = "None"]
pub fn processor_options(&mut self, val: Option<&::js_sys::Object>) -> &mut Self { self.set_processor_options(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for AudioWorkletNodeOptions {
    fn default() -> Self {
        Self::new()
    }
}
