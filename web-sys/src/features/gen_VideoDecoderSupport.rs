#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
#[cfg(web_sys_unstable_apis)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `VideoDecoderSupport` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoDecoderSupport`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::opaque]
#[derive(Copy)]
pub struct VideoDecoderSupport(pub ::wasm_bindgen::JsValue);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "VideoDecoderConfig")]
    #[doc = "Get the `config` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoDecoderConfig`, `VideoDecoderSupport`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None"]
pub fn get_config(this: &VideoDecoderSupport) -> Option<VideoDecoderConfig> { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "VideoDecoderConfig")]
    #[doc = "Change the `config` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoDecoderConfig`, `VideoDecoderSupport`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None = $1"]
pub fn set_config(this: &VideoDecoderSupport, val: &VideoDecoderConfig) { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `supported` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoDecoderSupport`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None"]
pub fn get_supported(this: &VideoDecoderSupport) -> Option<bool> { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `supported` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoDecoderSupport`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None = $1"]
pub fn set_supported(this: &VideoDecoderSupport, val: bool) { panic!("web-sys intrinsic") }
#[cfg(web_sys_unstable_apis)]
impl VideoDecoderSupport {
    #[doc = "Construct a new `VideoDecoderSupport`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoDecoderSupport`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "VideoDecoderConfig")]
    #[deprecated = "Use `set_config()` instead."]
#[js::import = "None"]
pub fn config(&mut self, val: &VideoDecoderConfig) -> &mut Self { self.set_config(val) { panic!("web-sys intrinsic") }
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
impl Default for VideoDecoderSupport {
    fn default() -> Self {
        Self::new()
    }
}
