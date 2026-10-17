#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
#[cfg(web_sys_unstable_apis)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AudioEncoderSupport` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioEncoderSupport`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::opaque]
#[derive(Copy)]
pub struct AudioEncoderSupport(pub ::wasm_bindgen::JsValue);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "AudioEncoderConfig")]
    #[doc = "Get the `config` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioEncoderConfig`, `AudioEncoderSupport`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None"]
pub fn get_config(this: &AudioEncoderSupport) -> Option<AudioEncoderConfig> { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "AudioEncoderConfig")]
    #[doc = "Change the `config` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioEncoderConfig`, `AudioEncoderSupport`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None = $1"]
pub fn set_config(this: &AudioEncoderSupport, val: &AudioEncoderConfig) { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `supported` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioEncoderSupport`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None"]
pub fn get_supported(this: &AudioEncoderSupport) -> Option<bool> { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `supported` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioEncoderSupport`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None = $1"]
pub fn set_supported(this: &AudioEncoderSupport, val: bool) { panic!("web-sys intrinsic") }
#[cfg(web_sys_unstable_apis)]
impl AudioEncoderSupport {
    #[doc = "Construct a new `AudioEncoderSupport`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioEncoderSupport`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "AudioEncoderConfig")]
    #[deprecated = "Use `set_config()` instead."]
#[js::import = "None"]
pub fn config(&mut self, val: &AudioEncoderConfig) -> &mut Self { self.set_config(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_supported()` instead."]
#[js::import = "None"]
pub fn supported(&mut self, val: bool) -> &mut Self { self.set_supported(val) { panic!("web-sys intrinsic") }
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for AudioEncoderSupport {
    fn default() -> Self {
        Self::new()
    }
}
