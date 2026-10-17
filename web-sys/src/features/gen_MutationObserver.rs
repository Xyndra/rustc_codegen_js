#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MutationObserver` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MutationObserver)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationObserver`*"]
#[js::opaque]
#[derive(Copy)]
pub struct MutationObserver(pub ::wasm_bindgen::JsValue);
    #[doc = "The `new MutationObserver(..)` constructor, creating a new instance of `MutationObserver`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MutationObserver/MutationObserver)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationObserver`*"]
impl MutationObserver {
    #[js::import = "new MutationObserver($0)"]
pub fn new(mutation_callback: &::js_sys::Function) -> Result<MutationObserver, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `disconnect()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MutationObserver/disconnect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationObserver`*"]
impl MutationObserver {
    #[js::import = "$0.disconnect()"]
pub fn disconnect(&self) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Node")]
    #[doc = "The `observe()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MutationObserver/observe)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationObserver`, `Node`*"]
impl MutationObserver {
    #[js::import = "$0.observe($1)"]
pub fn observe(&self, target: &Node) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "MutationObserverInit", feature = "Node",))]
    #[doc = "The `observe()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MutationObserver/observe)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationObserver`, `MutationObserverInit`, `Node`*"]
impl MutationObserver {
    #[js::import = "$0.observe($1, $2)"]
pub fn observe_with_options(&self, target: &Node, options: &MutationObserverInit) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `takeRecords()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MutationObserver/takeRecords)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationObserver`*"]
impl MutationObserver {
    #[js::import = "$0.takeRecords()"]
pub fn take_records(&self) -> ::js_sys::Array { panic!("web-sys intrinsic") }
}
