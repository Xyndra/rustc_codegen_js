#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlProgressElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLProgressElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlProgressElement`*"]
#[js::opaque]
#[derive(Copy)]
pub struct HtmlProgressElement(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `value` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLProgressElement/value)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlProgressElement`*"]
impl HtmlProgressElement {
    #[js::import = "$0.value"]
pub fn value(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `value` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLProgressElement/value)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlProgressElement`*"]
impl HtmlProgressElement {
    #[js::import = "$0.value = $1"]
pub fn set_value(&self, value: f64) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `max` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLProgressElement/max)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlProgressElement`*"]
impl HtmlProgressElement {
    #[js::import = "$0.max"]
pub fn max(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `max` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLProgressElement/max)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlProgressElement`*"]
impl HtmlProgressElement {
    #[js::import = "$0.max = $1"]
pub fn set_max(&self, value: f64) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `position` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLProgressElement/position)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlProgressElement`*"]
impl HtmlProgressElement {
    #[js::import = "$0.position"]
pub fn position(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "NodeList")]
    #[doc = "Getter for the `labels` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLProgressElement/labels)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlProgressElement`, `NodeList`*"]
impl HtmlProgressElement {
    #[js::import = "$0.labels"]
pub fn labels(&self) -> NodeList { panic!("web-sys intrinsic") }
}
