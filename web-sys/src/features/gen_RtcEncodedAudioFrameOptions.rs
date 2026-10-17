#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
#[cfg(web_sys_unstable_apis)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcEncodedAudioFrameOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcEncodedAudioFrameOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::opaque]
#[derive(Copy)]
pub struct RtcEncodedAudioFrameOptions(pub ::wasm_bindgen::JsValue);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "RtcEncodedAudioFrameMetadata")]
    #[doc = "Get the `metadata` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcEncodedAudioFrameMetadata`, `RtcEncodedAudioFrameOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None"]
pub fn get_metadata(this: &RtcEncodedAudioFrameOptions) -> Option<RtcEncodedAudioFrameMetadata> { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "RtcEncodedAudioFrameMetadata")]
    #[doc = "Change the `metadata` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcEncodedAudioFrameMetadata`, `RtcEncodedAudioFrameOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None = $1"]
pub fn set_metadata(this: &RtcEncodedAudioFrameOptions, val: &RtcEncodedAudioFrameMetadata) { panic!("web-sys intrinsic") }
#[cfg(web_sys_unstable_apis)]
impl RtcEncodedAudioFrameOptions {
    #[doc = "Construct a new `RtcEncodedAudioFrameOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcEncodedAudioFrameOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "RtcEncodedAudioFrameMetadata")]
    #[deprecated = "Use `set_metadata()` instead."]
#[js::import = "None"]
pub fn metadata(&mut self, val: &RtcEncodedAudioFrameMetadata) -> &mut Self { self.set_metadata(val) { panic!("web-sys intrinsic") }
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for RtcEncodedAudioFrameOptions {
    fn default() -> Self {
        Self::new()
    }
}
