#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
#[cfg(web_sys_unstable_apis)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `XrRenderStateInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrRenderStateInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::opaque]
#[derive(Copy)]
pub struct XrRenderStateInit(pub ::wasm_bindgen::JsValue);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "XrWebGlLayer")]
    #[doc = "Get the `baseLayer` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrRenderStateInit`, `XrWebGlLayer`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None"]
pub fn get_base_layer(this: &XrRenderStateInit) -> Option<XrWebGlLayer> { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "XrWebGlLayer")]
    #[doc = "Change the `baseLayer` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrRenderStateInit`, `XrWebGlLayer`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None = $1"]
pub fn set_base_layer(this: &XrRenderStateInit, val: Option<&XrWebGlLayer>) { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `depthFar` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrRenderStateInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None"]
pub fn get_depth_far(this: &XrRenderStateInit) -> Option<f64> { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `depthFar` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrRenderStateInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None = $1"]
pub fn set_depth_far(this: &XrRenderStateInit, val: f64) { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `depthNear` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrRenderStateInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None"]
pub fn get_depth_near(this: &XrRenderStateInit) -> Option<f64> { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `depthNear` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrRenderStateInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None = $1"]
pub fn set_depth_near(this: &XrRenderStateInit, val: f64) { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `inlineVerticalFieldOfView` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrRenderStateInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None"]
pub fn get_inline_vertical_field_of_view(this: &XrRenderStateInit) -> Option<f64> { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `inlineVerticalFieldOfView` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrRenderStateInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None = $1"]
pub fn set_inline_vertical_field_of_view(this: &XrRenderStateInit, val: f64) { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `layers` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrRenderStateInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None"]
pub fn get_layers(this: &XrRenderStateInit) -> Option<::js_sys::Array> { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `layers` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrRenderStateInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None = $1"]
pub fn set_layers(this: &XrRenderStateInit, val: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
#[cfg(web_sys_unstable_apis)]
impl XrRenderStateInit {
    #[doc = "Construct a new `XrRenderStateInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrRenderStateInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "XrWebGlLayer")]
    #[deprecated = "Use `set_base_layer()` instead."]
#[js::import = "None"]
pub fn base_layer(&mut self, val: Option<&XrWebGlLayer>) -> &mut Self { self.set_base_layer(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_depth_far()` instead."]
#[js::import = "None"]
pub fn depth_far(&mut self, val: f64) -> &mut Self { self.set_depth_far(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_depth_near()` instead."]
#[js::import = "None"]
pub fn depth_near(&mut self, val: f64) -> &mut Self { self.set_depth_near(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_inline_vertical_field_of_view()` instead."]
#[js::import = "None"]
pub fn inline_vertical_field_of_view(&mut self, val: f64) -> &mut Self { self.set_inline_vertical_field_of_view(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_layers()` instead."]
#[js::import = "None"]
pub fn layers(&mut self, val: Option<&::wasm_bindgen::JsValue>) -> &mut Self { self.set_layers(val.unwrap_or(&::wasm_bindgen::JsValue::NULL)) { panic!("web-sys intrinsic") }
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for XrRenderStateInit {
    fn default() -> Self {
        Self::new()
    }
}
