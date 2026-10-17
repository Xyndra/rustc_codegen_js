#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `IntersectionObserver` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IntersectionObserver)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IntersectionObserver`*"]
#[js::opaque]
#[derive(Copy)]
pub struct IntersectionObserver(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "Element")]
    #[doc = "Getter for the `root` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IntersectionObserver/root)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`, `IntersectionObserver`*"]
impl IntersectionObserver {
    #[js::import = "$0.root"]
pub fn root(&self) -> Option<Element> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `rootMargin` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IntersectionObserver/rootMargin)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IntersectionObserver`*"]
impl IntersectionObserver {
    #[js::import = "$0.rootMargin"]
pub fn root_margin(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `thresholds` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IntersectionObserver/thresholds)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IntersectionObserver`*"]
impl IntersectionObserver {
    #[js::import = "$0.thresholds"]
pub fn thresholds(&self) -> ::js_sys::Array { panic!("web-sys intrinsic") }
}
    #[doc = "The `new IntersectionObserver(..)` constructor, creating a new instance of `IntersectionObserver`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IntersectionObserver/IntersectionObserver)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IntersectionObserver`*"]
impl IntersectionObserver {
    #[js::import = "new IntersectionObserver($0)"]
pub fn new(intersection_callback: &::js_sys::Function) -> Result<IntersectionObserver, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "IntersectionObserverInit")]
    #[doc = "The `new IntersectionObserver(..)` constructor, creating a new instance of `IntersectionObserver`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IntersectionObserver/IntersectionObserver)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IntersectionObserver`, `IntersectionObserverInit`*"]
impl IntersectionObserver {
    #[js::import = "new IntersectionObserver($0, $1)"]
pub fn new_with_options(intersection_callback: &::js_sys::Function, options: &IntersectionObserverInit,) -> Result<IntersectionObserver, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `disconnect()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IntersectionObserver/disconnect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IntersectionObserver`*"]
impl IntersectionObserver {
    #[js::import = "$0.disconnect()"]
pub fn disconnect(&self) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Element")]
    #[doc = "The `observe()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IntersectionObserver/observe)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`, `IntersectionObserver`*"]
impl IntersectionObserver {
    #[js::import = "$0.observe($1)"]
pub fn observe(&self, target: &Element) { panic!("web-sys intrinsic") }
}
    #[doc = "The `takeRecords()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IntersectionObserver/takeRecords)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IntersectionObserver`*"]
impl IntersectionObserver {
    #[js::import = "$0.takeRecords()"]
pub fn take_records(&self) -> ::js_sys::Array { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Element")]
    #[doc = "The `unobserve()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IntersectionObserver/unobserve)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`, `IntersectionObserver`*"]
impl IntersectionObserver {
    #[js::import = "$0.unobserve($1)"]
pub fn unobserve(&self, target: &Element) { panic!("web-sys intrinsic") }
}
