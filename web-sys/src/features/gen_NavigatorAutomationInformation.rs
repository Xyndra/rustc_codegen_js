#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `NavigatorAutomationInformation` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NavigatorAutomationInformation)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigatorAutomationInformation`*"]
#[js::opaque]
#[derive(Copy)]
pub struct NavigatorAutomationInformation(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `webdriver` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NavigatorAutomationInformation/webdriver)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigatorAutomationInformation`*"]
impl NavigatorAutomationInformation {
    #[js::import = "$0.webdriver"]
pub fn webdriver(&self) -> bool { panic!("web-sys intrinsic") }
}
