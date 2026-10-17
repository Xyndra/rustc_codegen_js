#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `XPathExpression` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XPathExpression)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XPathExpression`*"]
#[js::opaque]
#[derive(Copy)]
pub struct XPathExpression(pub ::wasm_bindgen::JsValue);
    #[cfg(all(feature = "Node", feature = "XPathResult",))]
    #[doc = "The `evaluate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XPathExpression/evaluate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`, `XPathExpression`, `XPathResult`*"]
impl XPathExpression {
    #[js::import = "$0.evaluate($1)"]
pub fn evaluate(&self, context_node: &Node) -> Result<XPathResult, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "Node", feature = "XPathResult",))]
    #[doc = "The `evaluate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XPathExpression/evaluate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`, `XPathExpression`, `XPathResult`*"]
impl XPathExpression {
    #[js::import = "$0.evaluate($1, $2)"]
pub fn evaluate_with_type(&self, context_node: &Node, type_: u16) -> Result<XPathResult, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "Node", feature = "XPathResult",))]
    #[doc = "The `evaluate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XPathExpression/evaluate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`, `XPathExpression`, `XPathResult`*"]
impl XPathExpression {
    #[js::import = "$0.evaluate($1, $2, $3)"]
pub fn evaluate_with_type_and_result(&self, context_node: &Node, type_: u16, result: Option<&::js_sys::Object>) -> Result<XPathResult, JsValue> { panic!("web-sys intrinsic") }
}
