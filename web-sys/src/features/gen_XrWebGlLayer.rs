#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
#[cfg(web_sys_unstable_apis)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `XrWebGlLayer` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRWebGLLayer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrWebGlLayer`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::opaque]
#[derive(Copy)]
pub struct XrWebGlLayer(pub ::wasm_bindgen::JsValue);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `antialias` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRWebGLLayer/antialias)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrWebGlLayer`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl XrWebGlLayer {
    #[js::import = "$0.antialias"]
pub fn antialias(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `ignoreDepthValues` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRWebGLLayer/ignoreDepthValues)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrWebGlLayer`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl XrWebGlLayer {
    #[js::import = "$0.ignoreDepthValues"]
pub fn ignore_depth_values(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `fixedFoveation` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRWebGLLayer/fixedFoveation)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrWebGlLayer`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl XrWebGlLayer {
    #[js::import = "$0.fixedFoveation"]
pub fn fixed_foveation(&self) -> Option<f32> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Setter for the `fixedFoveation` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRWebGLLayer/fixedFoveation)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrWebGlLayer`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl XrWebGlLayer {
    #[js::import = "$0.fixedFoveation = $1"]
pub fn set_fixed_foveation(&self, value: Option<f32>) { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "WebGlFramebuffer")]
    #[doc = "Getter for the `framebuffer` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRWebGLLayer/framebuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlFramebuffer`, `XrWebGlLayer`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl XrWebGlLayer {
    #[js::import = "$0.framebuffer"]
pub fn framebuffer(&self) -> Option<WebGlFramebuffer> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `framebufferWidth` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRWebGLLayer/framebufferWidth)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrWebGlLayer`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl XrWebGlLayer {
    #[js::import = "$0.framebufferWidth"]
pub fn framebuffer_width(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `framebufferHeight` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRWebGLLayer/framebufferHeight)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrWebGlLayer`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl XrWebGlLayer {
    #[js::import = "$0.framebufferHeight"]
pub fn framebuffer_height(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(feature = "WebGlRenderingContext", feature = "XrSession",))]
    #[doc = "The `new XrWebGlLayer(..)` constructor, creating a new instance of `XrWebGlLayer`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRWebGLLayer/XRWebGLLayer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`, `XrSession`, `XrWebGlLayer`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl XrWebGlLayer {
    #[js::import = "new XRWebGLLayer($0, $1)"]
pub fn new_with_web_gl_rendering_context(session: &XrSession, context: &WebGlRenderingContext,) -> Result<XrWebGlLayer, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(feature = "WebGl2RenderingContext", feature = "XrSession",))]
    #[doc = "The `new XrWebGlLayer(..)` constructor, creating a new instance of `XrWebGlLayer`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRWebGLLayer/XRWebGLLayer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `XrSession`, `XrWebGlLayer`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl XrWebGlLayer {
    #[js::import = "new XRWebGLLayer($0, $1)"]
pub fn new_with_web_gl2_rendering_context(session: &XrSession, context: &WebGl2RenderingContext,) -> Result<XrWebGlLayer, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(
        feature = "WebGlRenderingContext",
        feature = "XrSession",
        feature = "XrWebGlLayerInit",
    ))]
    #[doc = "The `new XrWebGlLayer(..)` constructor, creating a new instance of `XrWebGlLayer`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRWebGLLayer/XRWebGLLayer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`, `XrSession`, `XrWebGlLayer`, `XrWebGlLayerInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl XrWebGlLayer {
    #[js::import = "new XRWebGLLayer($0, $1, $2)"]
pub fn new_with_web_gl_rendering_context_and_layer_init(session: &XrSession, context: &WebGlRenderingContext, layer_init: &XrWebGlLayerInit,) -> Result<XrWebGlLayer, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(
        feature = "WebGl2RenderingContext",
        feature = "XrSession",
        feature = "XrWebGlLayerInit",
    ))]
    #[doc = "The `new XrWebGlLayer(..)` constructor, creating a new instance of `XrWebGlLayer`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRWebGLLayer/XRWebGLLayer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGl2RenderingContext`, `XrSession`, `XrWebGlLayer`, `XrWebGlLayerInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl XrWebGlLayer {
    #[js::import = "new XRWebGLLayer($0, $1, $2)"]
pub fn new_with_web_gl2_rendering_context_and_layer_init(session: &XrSession, context: &WebGl2RenderingContext, layer_init: &XrWebGlLayerInit,) -> Result<XrWebGlLayer, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "XrSession")]
    #[doc = "The `getNativeFramebufferScaleFactor()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRWebGLLayer/getNativeFramebufferScaleFactor_static)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrSession`, `XrWebGlLayer`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl XrWebGlLayer {
    #[js::import = "XRWebGLLayer.getNativeFramebufferScaleFactor"]
pub fn get_native_framebuffer_scale_factor(session: &XrSession) -> f64 { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(feature = "XrView", feature = "XrViewport",))]
    #[doc = "The `getViewport()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRWebGLLayer/getViewport)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrView`, `XrViewport`, `XrWebGlLayer`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl XrWebGlLayer {
    #[js::import = "$0.getViewport($1)"]
pub fn get_viewport(&self, view: &XrView) -> Option<XrViewport> { panic!("web-sys intrinsic") }
}
