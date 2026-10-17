#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AudioContextOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioContextOptions`*"]
#[js::opaque]
#[derive(Copy)]
pub struct AudioContextOptions(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `latencyHint` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioContextOptions`*"]
#[js::import = "None"]
pub fn get_latency_hint(this: &AudioContextOptions) -> ::wasm_bindgen::JsValue { panic!("web-sys intrinsic") }
    #[doc = "Change the `latencyHint` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioContextOptions`*"]
#[js::import = "None = $1"]
pub fn set_latency_hint(this: &AudioContextOptions, val: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
    #[doc = "Get the `sampleRate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioContextOptions`*"]
#[js::import = "None"]
pub fn get_sample_rate(this: &AudioContextOptions) -> Option<f32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `sampleRate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioContextOptions`*"]
#[js::import = "None = $1"]
pub fn set_sample_rate(this: &AudioContextOptions, val: f32) { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `sinkId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioContextOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None"]
pub fn get_sink_id(this: &AudioContextOptions) -> ::wasm_bindgen::JsValue { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `sinkId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioContextOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None = $1"]
pub fn set_sink_id(this: &AudioContextOptions, val: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
impl AudioContextOptions {
    #[doc = "Construct a new `AudioContextOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioContextOptions`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_latency_hint()` instead."]
#[js::import = "None"]
pub fn latency_hint(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self { self.set_latency_hint(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_sample_rate()` instead."]
#[js::import = "None"]
pub fn sample_rate(&mut self, val: f32) -> &mut Self { self.set_sample_rate(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_sink_id()` instead."]
#[js::import = "None"]
pub fn sink_id(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self { self.set_sink_id(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for AudioContextOptions {
    fn default() -> Self {
        Self::new()
    }
}
