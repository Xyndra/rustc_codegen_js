#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ResizeObserver` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ResizeObserver)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ResizeObserver`*"]
#[js::opaque]
#[derive(Copy)]
pub struct ResizeObserver(pub ::wasm_bindgen::JsValue);
    #[doc = "The `new ResizeObserver(..)` constructor, creating a new instance of `ResizeObserver`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ResizeObserver/ResizeObserver)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ResizeObserver`*"]
impl ResizeObserver {
    #[js::import = "new ResizeObserver($0)"]
pub fn new(callback: &::js_sys::Function) -> Result<ResizeObserver, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `disconnect()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ResizeObserver/disconnect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ResizeObserver`*"]
impl ResizeObserver {
    #[js::import = "$0.disconnect()"]
pub fn disconnect(&self) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Element")]
    #[doc = "The `observe()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ResizeObserver/observe)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`, `ResizeObserver`*"]
impl ResizeObserver {
    #[js::import = "$0.observe($1)"]
pub fn observe(&self, target: &Element) { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "Element", feature = "ResizeObserverOptions",))]
    #[doc = "The `observe()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ResizeObserver/observe)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`, `ResizeObserver`, `ResizeObserverOptions`*"]
impl ResizeObserver {
    #[js::import = "$0.observe($1, $2)"]
pub fn observe_with_options(&self, target: &Element, options: &ResizeObserverOptions) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Element")]
    #[doc = "The `unobserve()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ResizeObserver/unobserve)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`, `ResizeObserver`*"]
impl ResizeObserver {
    #[js::import = "$0.unobserve($1)"]
pub fn unobserve(&self, target: &Element) { panic!("web-sys intrinsic") }
}
