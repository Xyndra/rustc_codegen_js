#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Presentation` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Presentation)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Presentation`*"]
#[js::opaque]
#[derive(Copy)]
pub struct Presentation(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "PresentationRequest")]
    #[doc = "Getter for the `defaultRequest` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Presentation/defaultRequest)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Presentation`, `PresentationRequest`*"]
impl Presentation {
    #[js::import = "$0.defaultRequest"]
pub fn default_request(&self) -> Option<PresentationRequest> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "PresentationRequest")]
    #[doc = "Setter for the `defaultRequest` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Presentation/defaultRequest)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Presentation`, `PresentationRequest`*"]
impl Presentation {
    #[js::import = "$0.defaultRequest = $1"]
pub fn set_default_request(&self, value: Option<&PresentationRequest>) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "PresentationReceiver")]
    #[doc = "Getter for the `receiver` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Presentation/receiver)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Presentation`, `PresentationReceiver`*"]
impl Presentation {
    #[js::import = "$0.receiver"]
pub fn receiver(&self) -> Option<PresentationReceiver> { panic!("web-sys intrinsic") }
}
