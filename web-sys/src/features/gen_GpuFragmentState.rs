#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
#[cfg(web_sys_unstable_apis)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuFragmentState` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuFragmentState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::opaque]
#[derive(Copy)]
pub struct GpuFragmentState(pub ::wasm_bindgen::JsValue);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `constants` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuFragmentState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None"]
pub fn get_constants(this: &GpuFragmentState) -> Option<::js_sys::Object> { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `constants` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuFragmentState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None = $1"]
pub fn set_constants(this: &GpuFragmentState, val: &::js_sys::Object) { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `entryPoint` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuFragmentState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None"]
pub fn get_entry_point(this: &GpuFragmentState) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `entryPoint` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuFragmentState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None = $1"]
pub fn set_entry_point(this: &GpuFragmentState, val: &str) { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuShaderModule")]
    #[doc = "Get the `module` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuFragmentState`, `GpuShaderModule`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None"]
pub fn get_module(this: &GpuFragmentState) -> GpuShaderModule { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuShaderModule")]
    #[doc = "Change the `module` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuFragmentState`, `GpuShaderModule`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None = $1"]
pub fn set_module(this: &GpuFragmentState, val: &GpuShaderModule) { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `targets` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuFragmentState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None"]
pub fn get_targets(this: &GpuFragmentState) -> ::js_sys::Array { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `targets` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuFragmentState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None = $1"]
pub fn set_targets(this: &GpuFragmentState, val: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
#[cfg(web_sys_unstable_apis)]
impl GpuFragmentState {
    #[cfg(feature = "GpuShaderModule")]
    #[doc = "Construct a new `GpuFragmentState`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuFragmentState`, `GpuShaderModule`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None"]
pub fn new(module: &GpuShaderModule, targets: &::wasm_bindgen::JsValue) -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret.set_module(module);
        ret.set_targets(targets);
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_constants()` instead."]
#[js::import = "None"]
pub fn constants(&mut self, val: &::js_sys::Object) -> &mut Self { self.set_constants(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_entry_point()` instead."]
#[js::import = "None"]
pub fn entry_point(&mut self, val: &str) -> &mut Self { self.set_entry_point(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuShaderModule")]
    #[deprecated = "Use `set_module()` instead."]
#[js::import = "None"]
pub fn module(&mut self, val: &GpuShaderModule) -> &mut Self { self.set_module(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_targets()` instead."]
#[js::import = "None"]
pub fn targets(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self { self.set_targets(val) { panic!("web-sys intrinsic") }
        self
    }
}
