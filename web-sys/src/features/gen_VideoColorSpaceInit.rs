#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
#[cfg(web_sys_unstable_apis)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `VideoColorSpaceInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoColorSpaceInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::opaque]
#[derive(Copy)]
pub struct VideoColorSpaceInit(pub ::wasm_bindgen::JsValue);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `fullRange` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoColorSpaceInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None"]
pub fn get_full_range(this: &VideoColorSpaceInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `fullRange` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoColorSpaceInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None = $1"]
pub fn set_full_range(this: &VideoColorSpaceInit, val: bool) { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "VideoMatrixCoefficients")]
    #[doc = "Get the `matrix` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoColorSpaceInit`, `VideoMatrixCoefficients`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None"]
pub fn get_matrix(this: &VideoColorSpaceInit) -> Option<VideoMatrixCoefficients> { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "VideoMatrixCoefficients")]
    #[doc = "Change the `matrix` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoColorSpaceInit`, `VideoMatrixCoefficients`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None = $1"]
pub fn set_matrix(this: &VideoColorSpaceInit, val: VideoMatrixCoefficients) { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "VideoColorPrimaries")]
    #[doc = "Get the `primaries` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoColorPrimaries`, `VideoColorSpaceInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None"]
pub fn get_primaries(this: &VideoColorSpaceInit) -> Option<VideoColorPrimaries> { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "VideoColorPrimaries")]
    #[doc = "Change the `primaries` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoColorPrimaries`, `VideoColorSpaceInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None = $1"]
pub fn set_primaries(this: &VideoColorSpaceInit, val: VideoColorPrimaries) { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "VideoTransferCharacteristics")]
    #[doc = "Get the `transfer` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoColorSpaceInit`, `VideoTransferCharacteristics`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None"]
pub fn get_transfer(this: &VideoColorSpaceInit) -> Option<VideoTransferCharacteristics> { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "VideoTransferCharacteristics")]
    #[doc = "Change the `transfer` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoColorSpaceInit`, `VideoTransferCharacteristics`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None = $1"]
pub fn set_transfer(this: &VideoColorSpaceInit, val: VideoTransferCharacteristics) { panic!("web-sys intrinsic") }
#[cfg(web_sys_unstable_apis)]
impl VideoColorSpaceInit {
    #[doc = "Construct a new `VideoColorSpaceInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoColorSpaceInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_full_range()` instead."]
#[js::import = "None"]
pub fn full_range(&mut self, val: bool) -> &mut Self { self.set_full_range(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "VideoMatrixCoefficients")]
    #[deprecated = "Use `set_matrix()` instead."]
#[js::import = "None"]
pub fn matrix(&mut self, val: VideoMatrixCoefficients) -> &mut Self { self.set_matrix(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "VideoColorPrimaries")]
    #[deprecated = "Use `set_primaries()` instead."]
#[js::import = "None"]
pub fn primaries(&mut self, val: VideoColorPrimaries) -> &mut Self { self.set_primaries(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "VideoTransferCharacteristics")]
    #[deprecated = "Use `set_transfer()` instead."]
#[js::import = "None"]
pub fn transfer(&mut self, val: VideoTransferCharacteristics) -> &mut Self { self.set_transfer(val) { panic!("web-sys intrinsic") }
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for VideoColorSpaceInit {
    fn default() -> Self {
        Self::new()
    }
}
