#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
#[cfg(web_sys_unstable_apis)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuPipelineErrorInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuPipelineErrorInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::opaque]
#[derive(Copy)]
pub struct GpuPipelineErrorInit(pub ::wasm_bindgen::JsValue);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuPipelineErrorReason")]
    #[doc = "Get the `reason` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuPipelineErrorInit`, `GpuPipelineErrorReason`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None"]
pub fn get_reason(this: &GpuPipelineErrorInit) -> GpuPipelineErrorReason { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuPipelineErrorReason")]
    #[doc = "Change the `reason` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuPipelineErrorInit`, `GpuPipelineErrorReason`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None = $1"]
pub fn set_reason(this: &GpuPipelineErrorInit, val: GpuPipelineErrorReason) { panic!("web-sys intrinsic") }
#[cfg(web_sys_unstable_apis)]
impl GpuPipelineErrorInit {
    #[cfg(feature = "GpuPipelineErrorReason")]
    #[doc = "Construct a new `GpuPipelineErrorInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuPipelineErrorInit`, `GpuPipelineErrorReason`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None"]
pub fn new(reason: GpuPipelineErrorReason) -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret.set_reason(reason);
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuPipelineErrorReason")]
    #[deprecated = "Use `set_reason()` instead."]
#[js::import = "None"]
pub fn reason(&mut self, val: GpuPipelineErrorReason) -> &mut Self { self.set_reason(val) { panic!("web-sys intrinsic") }
        self
    }
}
