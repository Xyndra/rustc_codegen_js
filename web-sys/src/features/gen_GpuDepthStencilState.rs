#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
#[cfg(web_sys_unstable_apis)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuDepthStencilState` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuDepthStencilState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::opaque]
#[derive(Copy)]
pub struct GpuDepthStencilState(pub ::wasm_bindgen::JsValue);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `depthBias` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuDepthStencilState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None"]
pub fn get_depth_bias(this: &GpuDepthStencilState) -> Option<i32> { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `depthBias` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuDepthStencilState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None = $1"]
pub fn set_depth_bias(this: &GpuDepthStencilState, val: i32) { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `depthBiasClamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuDepthStencilState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None"]
pub fn get_depth_bias_clamp(this: &GpuDepthStencilState) -> Option<f32> { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `depthBiasClamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuDepthStencilState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None = $1"]
pub fn set_depth_bias_clamp(this: &GpuDepthStencilState, val: f32) { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `depthBiasSlopeScale` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuDepthStencilState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None"]
pub fn get_depth_bias_slope_scale(this: &GpuDepthStencilState) -> Option<f32> { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `depthBiasSlopeScale` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuDepthStencilState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None = $1"]
pub fn set_depth_bias_slope_scale(this: &GpuDepthStencilState, val: f32) { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuCompareFunction")]
    #[doc = "Get the `depthCompare` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuCompareFunction`, `GpuDepthStencilState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None"]
pub fn get_depth_compare(this: &GpuDepthStencilState) -> Option<GpuCompareFunction> { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuCompareFunction")]
    #[doc = "Change the `depthCompare` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuCompareFunction`, `GpuDepthStencilState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None = $1"]
pub fn set_depth_compare(this: &GpuDepthStencilState, val: GpuCompareFunction) { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `depthWriteEnabled` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuDepthStencilState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None"]
pub fn get_depth_write_enabled(this: &GpuDepthStencilState) -> Option<bool> { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `depthWriteEnabled` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuDepthStencilState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None = $1"]
pub fn set_depth_write_enabled(this: &GpuDepthStencilState, val: bool) { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuTextureFormat")]
    #[doc = "Get the `format` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuDepthStencilState`, `GpuTextureFormat`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None"]
pub fn get_format(this: &GpuDepthStencilState) -> GpuTextureFormat { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuTextureFormat")]
    #[doc = "Change the `format` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuDepthStencilState`, `GpuTextureFormat`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None = $1"]
pub fn set_format(this: &GpuDepthStencilState, val: GpuTextureFormat) { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuStencilFaceState")]
    #[doc = "Get the `stencilBack` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuDepthStencilState`, `GpuStencilFaceState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None"]
pub fn get_stencil_back(this: &GpuDepthStencilState) -> Option<GpuStencilFaceState> { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuStencilFaceState")]
    #[doc = "Change the `stencilBack` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuDepthStencilState`, `GpuStencilFaceState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None = $1"]
pub fn set_stencil_back(this: &GpuDepthStencilState, val: &GpuStencilFaceState) { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuStencilFaceState")]
    #[doc = "Get the `stencilFront` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuDepthStencilState`, `GpuStencilFaceState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None"]
pub fn get_stencil_front(this: &GpuDepthStencilState) -> Option<GpuStencilFaceState> { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuStencilFaceState")]
    #[doc = "Change the `stencilFront` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuDepthStencilState`, `GpuStencilFaceState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None = $1"]
pub fn set_stencil_front(this: &GpuDepthStencilState, val: &GpuStencilFaceState) { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `stencilReadMask` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuDepthStencilState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None"]
pub fn get_stencil_read_mask(this: &GpuDepthStencilState) -> Option<u32> { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `stencilReadMask` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuDepthStencilState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None = $1"]
pub fn set_stencil_read_mask(this: &GpuDepthStencilState, val: u32) { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `stencilWriteMask` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuDepthStencilState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None"]
pub fn get_stencil_write_mask(this: &GpuDepthStencilState) -> Option<u32> { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `stencilWriteMask` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuDepthStencilState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None = $1"]
pub fn set_stencil_write_mask(this: &GpuDepthStencilState, val: u32) { panic!("web-sys intrinsic") }
#[cfg(web_sys_unstable_apis)]
impl GpuDepthStencilState {
    #[cfg(feature = "GpuTextureFormat")]
    #[doc = "Construct a new `GpuDepthStencilState`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuDepthStencilState`, `GpuTextureFormat`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None"]
pub fn new(format: GpuTextureFormat) -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret.set_format(format);
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_depth_bias()` instead."]
#[js::import = "None"]
pub fn depth_bias(&mut self, val: i32) -> &mut Self { self.set_depth_bias(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_depth_bias_clamp()` instead."]
#[js::import = "None"]
pub fn depth_bias_clamp(&mut self, val: f32) -> &mut Self { self.set_depth_bias_clamp(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_depth_bias_slope_scale()` instead."]
#[js::import = "None"]
pub fn depth_bias_slope_scale(&mut self, val: f32) -> &mut Self { self.set_depth_bias_slope_scale(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuCompareFunction")]
    #[deprecated = "Use `set_depth_compare()` instead."]
#[js::import = "None"]
pub fn depth_compare(&mut self, val: GpuCompareFunction) -> &mut Self { self.set_depth_compare(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_depth_write_enabled()` instead."]
#[js::import = "None"]
pub fn depth_write_enabled(&mut self, val: bool) -> &mut Self { self.set_depth_write_enabled(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuTextureFormat")]
    #[deprecated = "Use `set_format()` instead."]
#[js::import = "None"]
pub fn format(&mut self, val: GpuTextureFormat) -> &mut Self { self.set_format(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuStencilFaceState")]
    #[deprecated = "Use `set_stencil_back()` instead."]
#[js::import = "None"]
pub fn stencil_back(&mut self, val: &GpuStencilFaceState) -> &mut Self { self.set_stencil_back(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuStencilFaceState")]
    #[deprecated = "Use `set_stencil_front()` instead."]
#[js::import = "None"]
pub fn stencil_front(&mut self, val: &GpuStencilFaceState) -> &mut Self { self.set_stencil_front(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_stencil_read_mask()` instead."]
#[js::import = "None"]
pub fn stencil_read_mask(&mut self, val: u32) -> &mut Self { self.set_stencil_read_mask(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_stencil_write_mask()` instead."]
#[js::import = "None"]
pub fn stencil_write_mask(&mut self, val: u32) -> &mut Self { self.set_stencil_write_mask(val) { panic!("web-sys intrinsic") }
        self
    }
}
