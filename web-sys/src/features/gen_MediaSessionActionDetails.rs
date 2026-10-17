#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
#[cfg(web_sys_unstable_apis)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaSessionActionDetails` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaSessionActionDetails`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::opaque]
#[derive(Copy)]
pub struct MediaSessionActionDetails(pub ::wasm_bindgen::JsValue);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "MediaSessionAction")]
    #[doc = "Get the `action` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaSessionAction`, `MediaSessionActionDetails`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None"]
pub fn get_action(this: &MediaSessionActionDetails) -> MediaSessionAction { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "MediaSessionAction")]
    #[doc = "Change the `action` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaSessionAction`, `MediaSessionActionDetails`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None = $1"]
pub fn set_action(this: &MediaSessionActionDetails, val: MediaSessionAction) { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `fastSeek` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaSessionActionDetails`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None"]
pub fn get_fast_seek(this: &MediaSessionActionDetails) -> Option<bool> { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `fastSeek` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaSessionActionDetails`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None = $1"]
pub fn set_fast_seek(this: &MediaSessionActionDetails, val: Option<bool>) { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `seekOffset` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaSessionActionDetails`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None"]
pub fn get_seek_offset(this: &MediaSessionActionDetails) -> Option<f64> { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `seekOffset` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaSessionActionDetails`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None = $1"]
pub fn set_seek_offset(this: &MediaSessionActionDetails, val: Option<f64>) { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `seekTime` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaSessionActionDetails`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None"]
pub fn get_seek_time(this: &MediaSessionActionDetails) -> Option<f64> { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `seekTime` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaSessionActionDetails`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None = $1"]
pub fn set_seek_time(this: &MediaSessionActionDetails, val: Option<f64>) { panic!("web-sys intrinsic") }
#[cfg(web_sys_unstable_apis)]
impl MediaSessionActionDetails {
    #[cfg(feature = "MediaSessionAction")]
    #[doc = "Construct a new `MediaSessionActionDetails`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaSessionAction`, `MediaSessionActionDetails`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None"]
pub fn new(action: MediaSessionAction) -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret.set_action(action);
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "MediaSessionAction")]
    #[deprecated = "Use `set_action()` instead."]
#[js::import = "None"]
pub fn action(&mut self, val: MediaSessionAction) -> &mut Self { self.set_action(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_fast_seek()` instead."]
#[js::import = "None"]
pub fn fast_seek(&mut self, val: Option<bool>) -> &mut Self { self.set_fast_seek(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_seek_offset()` instead."]
#[js::import = "None"]
pub fn seek_offset(&mut self, val: Option<f64>) -> &mut Self { self.set_seek_offset(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_seek_time()` instead."]
#[js::import = "None"]
pub fn seek_time(&mut self, val: Option<f64>) -> &mut Self { self.set_seek_time(val) { panic!("web-sys intrinsic") }
        self
    }
}
