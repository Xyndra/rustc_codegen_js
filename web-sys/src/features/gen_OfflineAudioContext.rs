#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `OfflineAudioContext` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineAudioContext`*"]
#[js::opaque]
#[derive(Copy)]
pub struct OfflineAudioContext(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `length` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/length)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineAudioContext`*"]
impl OfflineAudioContext {
    #[js::import = "$0.length"]
pub fn length(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `oncomplete` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/oncomplete)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineAudioContext`*"]
impl OfflineAudioContext {
    #[js::import = "$0.oncomplete"]
pub fn oncomplete(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `oncomplete` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/oncomplete)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineAudioContext`*"]
impl OfflineAudioContext {
    #[js::import = "$0.oncomplete = $1"]
pub fn set_oncomplete(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "AudioDestinationNode")]
    #[doc = "Getter for the `destination` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/destination)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioDestinationNode`, `OfflineAudioContext`*"]
impl OfflineAudioContext {
    #[js::import = "$0.destination"]
pub fn destination(&self) -> AudioDestinationNode { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `sampleRate` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/sampleRate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineAudioContext`*"]
impl OfflineAudioContext {
    #[js::import = "$0.sampleRate"]
pub fn sample_rate(&self) -> f32 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `currentTime` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/currentTime)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineAudioContext`*"]
impl OfflineAudioContext {
    #[js::import = "$0.currentTime"]
pub fn current_time(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "AudioListener")]
    #[doc = "Getter for the `listener` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/listener)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioListener`, `OfflineAudioContext`*"]
impl OfflineAudioContext {
    #[js::import = "$0.listener"]
pub fn listener(&self) -> AudioListener { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "AudioContextState")]
    #[doc = "Getter for the `state` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/state)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioContextState`, `OfflineAudioContext`*"]
impl OfflineAudioContext {
    #[js::import = "$0.state"]
pub fn state(&self) -> AudioContextState { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "AudioWorklet")]
    #[doc = "Getter for the `audioWorklet` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/audioWorklet)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioWorklet`, `OfflineAudioContext`*"]
impl OfflineAudioContext {
    #[js::import = "$0.audioWorklet"]
pub fn audio_worklet(&self) -> Result<AudioWorklet, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onstatechange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/onstatechange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineAudioContext`*"]
impl OfflineAudioContext {
    #[js::import = "$0.onstatechange"]
pub fn onstatechange(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onstatechange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/onstatechange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineAudioContext`*"]
impl OfflineAudioContext {
    #[js::import = "$0.onstatechange = $1"]
pub fn set_onstatechange(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "OfflineAudioContextOptions")]
    #[doc = "The `new OfflineAudioContext(..)` constructor, creating a new instance of `OfflineAudioContext`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/OfflineAudioContext)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineAudioContext`, `OfflineAudioContextOptions`*"]
impl OfflineAudioContext {
    #[js::import = "new OfflineAudioContext($0)"]
pub fn new_with_context_options(context_options: &OfflineAudioContextOptions,) -> Result<OfflineAudioContext, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `new OfflineAudioContext(..)` constructor, creating a new instance of `OfflineAudioContext`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/OfflineAudioContext)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineAudioContext`*"]
impl OfflineAudioContext {
    #[js::import = "new OfflineAudioContext($0, $1, $2)"]
pub fn new_with_number_of_channels_and_length_and_sample_rate(number_of_channels: u32, length: u32, sample_rate: f32,) -> Result<OfflineAudioContext, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `startRendering()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/startRendering)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineAudioContext`*"]
impl OfflineAudioContext {
    #[js::import = "$0.startRendering()"]
pub fn start_rendering(&self) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "AnalyserNode")]
    #[doc = "The `createAnalyser()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/createAnalyser)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnalyserNode`, `OfflineAudioContext`*"]
impl OfflineAudioContext {
    #[js::import = "$0.createAnalyser()"]
pub fn create_analyser(&self) -> Result<AnalyserNode, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "BiquadFilterNode")]
    #[doc = "The `createBiquadFilter()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/createBiquadFilter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BiquadFilterNode`, `OfflineAudioContext`*"]
impl OfflineAudioContext {
    #[js::import = "$0.createBiquadFilter()"]
pub fn create_biquad_filter(&self) -> Result<BiquadFilterNode, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "AudioBuffer")]
    #[doc = "The `createBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/createBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBuffer`, `OfflineAudioContext`*"]
impl OfflineAudioContext {
    #[js::import = "$0.createBuffer($1, $2, $3)"]
pub fn create_buffer(&self, number_of_channels: u32, length: u32, sample_rate: f32) -> Result<AudioBuffer, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "AudioBufferSourceNode")]
    #[doc = "The `createBufferSource()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/createBufferSource)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBufferSourceNode`, `OfflineAudioContext`*"]
impl OfflineAudioContext {
    #[js::import = "$0.createBufferSource()"]
pub fn create_buffer_source(&self) -> Result<AudioBufferSourceNode, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "ChannelMergerNode")]
    #[doc = "The `createChannelMerger()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/createChannelMerger)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelMergerNode`, `OfflineAudioContext`*"]
impl OfflineAudioContext {
    #[js::import = "$0.createChannelMerger()"]
pub fn create_channel_merger(&self) -> Result<ChannelMergerNode, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "ChannelMergerNode")]
    #[doc = "The `createChannelMerger()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/createChannelMerger)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelMergerNode`, `OfflineAudioContext`*"]
impl OfflineAudioContext {
    #[js::import = "$0.createChannelMerger($1)"]
pub fn create_channel_merger_with_number_of_inputs(&self, number_of_inputs: u32) -> Result<ChannelMergerNode, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "ChannelSplitterNode")]
    #[doc = "The `createChannelSplitter()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/createChannelSplitter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelSplitterNode`, `OfflineAudioContext`*"]
impl OfflineAudioContext {
    #[js::import = "$0.createChannelSplitter()"]
pub fn create_channel_splitter(&self) -> Result<ChannelSplitterNode, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "ChannelSplitterNode")]
    #[doc = "The `createChannelSplitter()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/createChannelSplitter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelSplitterNode`, `OfflineAudioContext`*"]
impl OfflineAudioContext {
    #[js::import = "$0.createChannelSplitter($1)"]
pub fn create_channel_splitter_with_number_of_outputs(&self, number_of_outputs: u32) -> Result<ChannelSplitterNode, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "ConstantSourceNode")]
    #[doc = "The `createConstantSource()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/createConstantSource)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstantSourceNode`, `OfflineAudioContext`*"]
impl OfflineAudioContext {
    #[js::import = "$0.createConstantSource()"]
pub fn create_constant_source(&self) -> Result<ConstantSourceNode, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "ConvolverNode")]
    #[doc = "The `createConvolver()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/createConvolver)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConvolverNode`, `OfflineAudioContext`*"]
impl OfflineAudioContext {
    #[js::import = "$0.createConvolver()"]
pub fn create_convolver(&self) -> Result<ConvolverNode, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "DelayNode")]
    #[doc = "The `createDelay()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/createDelay)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DelayNode`, `OfflineAudioContext`*"]
impl OfflineAudioContext {
    #[js::import = "$0.createDelay()"]
pub fn create_delay(&self) -> Result<DelayNode, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "DelayNode")]
    #[doc = "The `createDelay()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/createDelay)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DelayNode`, `OfflineAudioContext`*"]
impl OfflineAudioContext {
    #[js::import = "$0.createDelay($1)"]
pub fn create_delay_with_max_delay_time(&self, max_delay_time: f64) -> Result<DelayNode, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "DynamicsCompressorNode")]
    #[doc = "The `createDynamicsCompressor()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/createDynamicsCompressor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DynamicsCompressorNode`, `OfflineAudioContext`*"]
impl OfflineAudioContext {
    #[js::import = "$0.createDynamicsCompressor()"]
pub fn create_dynamics_compressor(&self) -> Result<DynamicsCompressorNode, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "GainNode")]
    #[doc = "The `createGain()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/createGain)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GainNode`, `OfflineAudioContext`*"]
impl OfflineAudioContext {
    #[js::import = "$0.createGain()"]
pub fn create_gain(&self) -> Result<GainNode, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "IirFilterNode")]
    #[doc = "The `createIIRFilter()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/createIIRFilter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IirFilterNode`, `OfflineAudioContext`*"]
impl OfflineAudioContext {
    #[js::import = "$0.createIIRFilter($1, $2)"]
pub fn create_iir_filter(&self, feedforward: &::wasm_bindgen::JsValue, feedback: &::wasm_bindgen::JsValue) -> Result<IirFilterNode, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "OscillatorNode")]
    #[doc = "The `createOscillator()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/createOscillator)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineAudioContext`, `OscillatorNode`*"]
impl OfflineAudioContext {
    #[js::import = "$0.createOscillator()"]
pub fn create_oscillator(&self) -> Result<OscillatorNode, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "PannerNode")]
    #[doc = "The `createPanner()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/createPanner)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineAudioContext`, `PannerNode`*"]
impl OfflineAudioContext {
    #[js::import = "$0.createPanner()"]
pub fn create_panner(&self) -> Result<PannerNode, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "PeriodicWave")]
    #[doc = "The `createPeriodicWave()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/createPeriodicWave)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineAudioContext`, `PeriodicWave`*"]
impl OfflineAudioContext {
    #[js::import = "$0.createPeriodicWave($1, $2)"]
pub fn create_periodic_wave(&self, real: &mut [f32], imag: &mut [f32]) -> Result<PeriodicWave, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "PeriodicWave")]
    #[doc = "The `createPeriodicWave()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/createPeriodicWave)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineAudioContext`, `PeriodicWave`*"]
impl OfflineAudioContext {
    #[js::import = "$0.createPeriodicWave($1, $2)"]
pub fn create_periodic_wave_with_f32_array_and_f32_slice(&self, real: &::js_sys::Float32Array, imag: &mut [f32]) -> Result<PeriodicWave, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "PeriodicWave")]
    #[doc = "The `createPeriodicWave()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/createPeriodicWave)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineAudioContext`, `PeriodicWave`*"]
impl OfflineAudioContext {
    #[js::import = "$0.createPeriodicWave($1, $2)"]
pub fn create_periodic_wave_with_f32_slice_and_f32_array(&self, real: &mut [f32], imag: &::js_sys::Float32Array) -> Result<PeriodicWave, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "PeriodicWave")]
    #[doc = "The `createPeriodicWave()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/createPeriodicWave)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineAudioContext`, `PeriodicWave`*"]
impl OfflineAudioContext {
    #[js::import = "$0.createPeriodicWave($1, $2)"]
pub fn create_periodic_wave_with_f32_array_and_f32_array(&self, real: &::js_sys::Float32Array, imag: &::js_sys::Float32Array) -> Result<PeriodicWave, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "PeriodicWave", feature = "PeriodicWaveConstraints",))]
    #[doc = "The `createPeriodicWave()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/createPeriodicWave)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineAudioContext`, `PeriodicWave`, `PeriodicWaveConstraints`*"]
impl OfflineAudioContext {
    #[js::import = "$0.createPeriodicWave($1, $2, $3)"]
pub fn create_periodic_wave_with_constraints(&self, real: &mut [f32], imag: &mut [f32], constraints: &PeriodicWaveConstraints) -> Result<PeriodicWave, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "PeriodicWave", feature = "PeriodicWaveConstraints",))]
    #[doc = "The `createPeriodicWave()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/createPeriodicWave)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineAudioContext`, `PeriodicWave`, `PeriodicWaveConstraints`*"]
impl OfflineAudioContext {
    #[js::import = "$0.createPeriodicWave($1, $2, $3)"]
pub fn create_periodic_wave_with_f32_array_and_f32_slice_and_constraints(&self, real: &::js_sys::Float32Array, imag: &mut [f32], constraints: &PeriodicWaveConstraints) -> Result<PeriodicWave, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "PeriodicWave", feature = "PeriodicWaveConstraints",))]
    #[doc = "The `createPeriodicWave()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/createPeriodicWave)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineAudioContext`, `PeriodicWave`, `PeriodicWaveConstraints`*"]
impl OfflineAudioContext {
    #[js::import = "$0.createPeriodicWave($1, $2, $3)"]
pub fn create_periodic_wave_with_f32_slice_and_f32_array_and_constraints(&self, real: &mut [f32], imag: &::js_sys::Float32Array, constraints: &PeriodicWaveConstraints) -> Result<PeriodicWave, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "PeriodicWave", feature = "PeriodicWaveConstraints",))]
    #[doc = "The `createPeriodicWave()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/createPeriodicWave)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineAudioContext`, `PeriodicWave`, `PeriodicWaveConstraints`*"]
impl OfflineAudioContext {
    #[js::import = "$0.createPeriodicWave($1, $2, $3)"]
pub fn create_periodic_wave_with_f32_array_and_f32_array_and_constraints(&self, real: &::js_sys::Float32Array, imag: &::js_sys::Float32Array, constraints: &PeriodicWaveConstraints) -> Result<PeriodicWave, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "ScriptProcessorNode")]
    #[doc = "The `createScriptProcessor()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/createScriptProcessor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineAudioContext`, `ScriptProcessorNode`*"]
impl OfflineAudioContext {
    #[js::import = "$0.createScriptProcessor()"]
pub fn create_script_processor(&self) -> Result<ScriptProcessorNode, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "ScriptProcessorNode")]
    #[doc = "The `createScriptProcessor()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/createScriptProcessor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineAudioContext`, `ScriptProcessorNode`*"]
impl OfflineAudioContext {
    #[js::import = "$0.createScriptProcessor($1)"]
pub fn create_script_processor_with_buffer_size(&self, buffer_size: u32) -> Result<ScriptProcessorNode, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "ScriptProcessorNode")]
    #[doc = "The `createScriptProcessor()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/createScriptProcessor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineAudioContext`, `ScriptProcessorNode`*"]
impl OfflineAudioContext {
    #[js::import = "$0.createScriptProcessor($1, $2)"]
pub fn create_script_processor_with_buffer_size_and_number_of_input_channels(&self, buffer_size: u32, number_of_input_channels: u32) -> Result<ScriptProcessorNode, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "ScriptProcessorNode")]
    #[doc = "The `createScriptProcessor()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/createScriptProcessor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineAudioContext`, `ScriptProcessorNode`*"]
impl OfflineAudioContext {
    #[js::import = "$0.createScriptProcessor($1, $2, $3)"]
pub fn create_script_processor_with_buffer_size_and_number_of_input_channels_and_number_of_output_channels(&self, buffer_size: u32, number_of_input_channels: u32, number_of_output_channels: u32) -> Result<ScriptProcessorNode, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "StereoPannerNode")]
    #[doc = "The `createStereoPanner()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/createStereoPanner)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineAudioContext`, `StereoPannerNode`*"]
impl OfflineAudioContext {
    #[js::import = "$0.createStereoPanner()"]
pub fn create_stereo_panner(&self) -> Result<StereoPannerNode, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "WaveShaperNode")]
    #[doc = "The `createWaveShaper()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/createWaveShaper)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineAudioContext`, `WaveShaperNode`*"]
impl OfflineAudioContext {
    #[js::import = "$0.createWaveShaper()"]
pub fn create_wave_shaper(&self) -> Result<WaveShaperNode, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `decodeAudioData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/decodeAudioData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineAudioContext`*"]
impl OfflineAudioContext {
    #[js::import = "$0.decodeAudioData($1)"]
pub fn decode_audio_data(&self, audio_data: &::js_sys::ArrayBuffer) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `decodeAudioData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/decodeAudioData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineAudioContext`*"]
impl OfflineAudioContext {
    #[js::import = "$0.decodeAudioData($1, $2)"]
pub fn decode_audio_data_with_success_callback(&self, audio_data: &::js_sys::ArrayBuffer, success_callback: &::js_sys::Function) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `decodeAudioData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/decodeAudioData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineAudioContext`*"]
impl OfflineAudioContext {
    #[js::import = "$0.decodeAudioData($1, $2, $3)"]
pub fn decode_audio_data_with_success_callback_and_error_callback(&self, audio_data: &::js_sys::ArrayBuffer, success_callback: &::js_sys::Function, error_callback: &::js_sys::Function) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `resume()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioContext/resume)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineAudioContext`*"]
impl OfflineAudioContext {
    #[js::import = "$0.resume()"]
pub fn resume(&self) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
