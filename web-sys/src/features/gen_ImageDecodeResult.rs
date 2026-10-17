#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
#[cfg(web_sys_unstable_apis)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ImageDecodeResult` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageDecodeResult`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::opaque]
#[derive(Copy)]
pub struct ImageDecodeResult(pub ::wasm_bindgen::JsValue);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `complete` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageDecodeResult`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None"]
pub fn get_complete(this: &ImageDecodeResult) -> bool { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `complete` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageDecodeResult`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None = $1"]
pub fn set_complete(this: &ImageDecodeResult, val: bool) { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "VideoFrame")]
    #[doc = "Get the `image` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageDecodeResult`, `VideoFrame`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None"]
pub fn get_image(this: &ImageDecodeResult) -> VideoFrame { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "VideoFrame")]
    #[doc = "Change the `image` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageDecodeResult`, `VideoFrame`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None = $1"]
pub fn set_image(this: &ImageDecodeResult, val: &VideoFrame) { panic!("web-sys intrinsic") }
#[cfg(web_sys_unstable_apis)]
impl ImageDecodeResult {
    #[cfg(feature = "VideoFrame")]
    #[doc = "Construct a new `ImageDecodeResult`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageDecodeResult`, `VideoFrame`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None"]
pub fn new(complete: bool, image: &VideoFrame) -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret.set_complete(complete);
        ret.set_image(image);
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_complete()` instead."]
#[js::import = "None"]
pub fn complete(&mut self, val: bool) -> &mut Self { self.set_complete(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "VideoFrame")]
    #[deprecated = "Use `set_image()` instead."]
#[js::import = "None"]
pub fn image(&mut self, val: &VideoFrame) -> &mut Self { self.set_image(val) { panic!("web-sys intrinsic") }
        self
    }
}
