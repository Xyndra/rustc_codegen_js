#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
#[cfg(web_sys_unstable_apis)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SFrameTransformOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SFrameTransformOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::opaque]
#[derive(Copy)]
pub struct SFrameTransformOptions(pub ::wasm_bindgen::JsValue);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "SFrameTransformRole")]
    #[doc = "Get the `role` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SFrameTransformOptions`, `SFrameTransformRole`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None"]
pub fn get_role(this: &SFrameTransformOptions) -> Option<SFrameTransformRole> { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "SFrameTransformRole")]
    #[doc = "Change the `role` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SFrameTransformOptions`, `SFrameTransformRole`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None = $1"]
pub fn set_role(this: &SFrameTransformOptions, val: SFrameTransformRole) { panic!("web-sys intrinsic") }
#[cfg(web_sys_unstable_apis)]
impl SFrameTransformOptions {
    #[doc = "Construct a new `SFrameTransformOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SFrameTransformOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "SFrameTransformRole")]
    #[deprecated = "Use `set_role()` instead."]
#[js::import = "None"]
pub fn role(&mut self, val: SFrameTransformRole) -> &mut Self { self.set_role(val) { panic!("web-sys intrinsic") }
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for SFrameTransformOptions {
    fn default() -> Self {
        Self::new()
    }
}
