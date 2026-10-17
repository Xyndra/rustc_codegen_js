#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
#[cfg(web_sys_unstable_apis)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcEncodedVideoFrameOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcEncodedVideoFrameOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::opaque]
#[derive(Copy)]
pub struct RtcEncodedVideoFrameOptions(pub ::wasm_bindgen::JsValue);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "RtcEncodedVideoFrameMetadata")]
    #[doc = "Get the `metadata` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcEncodedVideoFrameMetadata`, `RtcEncodedVideoFrameOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None"]
pub fn get_metadata(this: &RtcEncodedVideoFrameOptions) -> Option<RtcEncodedVideoFrameMetadata> { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "RtcEncodedVideoFrameMetadata")]
    #[doc = "Change the `metadata` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcEncodedVideoFrameMetadata`, `RtcEncodedVideoFrameOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None = $1"]
pub fn set_metadata(this: &RtcEncodedVideoFrameOptions, val: &RtcEncodedVideoFrameMetadata) { panic!("web-sys intrinsic") }
#[cfg(web_sys_unstable_apis)]
impl RtcEncodedVideoFrameOptions {
    #[doc = "Construct a new `RtcEncodedVideoFrameOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcEncodedVideoFrameOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "RtcEncodedVideoFrameMetadata")]
    #[deprecated = "Use `set_metadata()` instead."]
#[js::import = "None"]
pub fn metadata(&mut self, val: &RtcEncodedVideoFrameMetadata) -> &mut Self { self.set_metadata(val) { panic!("web-sys intrinsic") }
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for RtcEncodedVideoFrameOptions {
    fn default() -> Self {
        Self::new()
    }
}
