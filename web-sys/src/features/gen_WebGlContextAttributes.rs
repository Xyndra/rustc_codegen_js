#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WebGlContextAttributes` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlContextAttributes`*"]
#[js::opaque]
#[derive(Copy)]
pub struct WebGlContextAttributes(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `alpha` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlContextAttributes`*"]
#[js::import = "None"]
pub fn get_alpha(this: &WebGlContextAttributes) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `alpha` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlContextAttributes`*"]
#[js::import = "None = $1"]
pub fn set_alpha(this: &WebGlContextAttributes, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `antialias` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlContextAttributes`*"]
#[js::import = "None"]
pub fn get_antialias(this: &WebGlContextAttributes) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `antialias` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlContextAttributes`*"]
#[js::import = "None = $1"]
pub fn set_antialias(this: &WebGlContextAttributes, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `depth` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlContextAttributes`*"]
#[js::import = "None"]
pub fn get_depth(this: &WebGlContextAttributes) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `depth` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlContextAttributes`*"]
#[js::import = "None = $1"]
pub fn set_depth(this: &WebGlContextAttributes, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `failIfMajorPerformanceCaveat` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlContextAttributes`*"]
#[js::import = "None"]
pub fn get_fail_if_major_performance_caveat(this: &WebGlContextAttributes) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `failIfMajorPerformanceCaveat` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlContextAttributes`*"]
#[js::import = "None = $1"]
pub fn set_fail_if_major_performance_caveat(this: &WebGlContextAttributes, val: bool) { panic!("web-sys intrinsic") }
    #[cfg(feature = "WebGlPowerPreference")]
    #[doc = "Get the `powerPreference` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlContextAttributes`, `WebGlPowerPreference`*"]
#[js::import = "None"]
pub fn get_power_preference(this: &WebGlContextAttributes) -> Option<WebGlPowerPreference> { panic!("web-sys intrinsic") }
    #[cfg(feature = "WebGlPowerPreference")]
    #[doc = "Change the `powerPreference` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlContextAttributes`, `WebGlPowerPreference`*"]
#[js::import = "None = $1"]
pub fn set_power_preference(this: &WebGlContextAttributes, val: WebGlPowerPreference) { panic!("web-sys intrinsic") }
    #[doc = "Get the `premultipliedAlpha` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlContextAttributes`*"]
#[js::import = "None"]
pub fn get_premultiplied_alpha(this: &WebGlContextAttributes) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `premultipliedAlpha` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlContextAttributes`*"]
#[js::import = "None = $1"]
pub fn set_premultiplied_alpha(this: &WebGlContextAttributes, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `preserveDrawingBuffer` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlContextAttributes`*"]
#[js::import = "None"]
pub fn get_preserve_drawing_buffer(this: &WebGlContextAttributes) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `preserveDrawingBuffer` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlContextAttributes`*"]
#[js::import = "None = $1"]
pub fn set_preserve_drawing_buffer(this: &WebGlContextAttributes, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `stencil` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlContextAttributes`*"]
#[js::import = "None"]
pub fn get_stencil(this: &WebGlContextAttributes) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `stencil` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlContextAttributes`*"]
#[js::import = "None = $1"]
pub fn set_stencil(this: &WebGlContextAttributes, val: bool) { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `xrCompatible` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlContextAttributes`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None"]
pub fn get_xr_compatible(this: &WebGlContextAttributes) -> Option<bool> { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `xrCompatible` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlContextAttributes`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None = $1"]
pub fn set_xr_compatible(this: &WebGlContextAttributes, val: bool) { panic!("web-sys intrinsic") }
impl WebGlContextAttributes {
    #[doc = "Construct a new `WebGlContextAttributes`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlContextAttributes`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_alpha()` instead."]
#[js::import = "None"]
pub fn alpha(&mut self, val: bool) -> &mut Self { self.set_alpha(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_antialias()` instead."]
#[js::import = "None"]
pub fn antialias(&mut self, val: bool) -> &mut Self { self.set_antialias(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_depth()` instead."]
#[js::import = "None"]
pub fn depth(&mut self, val: bool) -> &mut Self { self.set_depth(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_fail_if_major_performance_caveat()` instead."]
#[js::import = "None"]
pub fn fail_if_major_performance_caveat(&mut self, val: bool) -> &mut Self { self.set_fail_if_major_performance_caveat(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(feature = "WebGlPowerPreference")]
    #[deprecated = "Use `set_power_preference()` instead."]
#[js::import = "None"]
pub fn power_preference(&mut self, val: WebGlPowerPreference) -> &mut Self { self.set_power_preference(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_premultiplied_alpha()` instead."]
#[js::import = "None"]
pub fn premultiplied_alpha(&mut self, val: bool) -> &mut Self { self.set_premultiplied_alpha(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_preserve_drawing_buffer()` instead."]
#[js::import = "None"]
pub fn preserve_drawing_buffer(&mut self, val: bool) -> &mut Self { self.set_preserve_drawing_buffer(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_stencil()` instead."]
#[js::import = "None"]
pub fn stencil(&mut self, val: bool) -> &mut Self { self.set_stencil(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_xr_compatible()` instead."]
#[js::import = "None"]
pub fn xr_compatible(&mut self, val: bool) -> &mut Self { self.set_xr_compatible(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for WebGlContextAttributes {
    fn default() -> Self {
        Self::new()
    }
}
