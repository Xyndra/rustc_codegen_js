#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlIFrameElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlIFrameElement`*"]
#[js::opaque]
#[derive(Copy)]
pub struct HtmlIFrameElement(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `src` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/src)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlIFrameElement`*"]
impl HtmlIFrameElement {
    #[js::import = "$0.src"]
pub fn src(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `src` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/src)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlIFrameElement`*"]
impl HtmlIFrameElement {
    #[js::import = "$0.src = $1"]
pub fn set_src(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `srcdoc` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/srcdoc)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlIFrameElement`*"]
impl HtmlIFrameElement {
    #[js::import = "$0.srcdoc"]
pub fn srcdoc(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `srcdoc` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/srcdoc)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlIFrameElement`*"]
impl HtmlIFrameElement {
    #[js::import = "$0.srcdoc = $1"]
pub fn set_srcdoc(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `name` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/name)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlIFrameElement`*"]
impl HtmlIFrameElement {
    #[js::import = "$0.name"]
pub fn name(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `name` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/name)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlIFrameElement`*"]
impl HtmlIFrameElement {
    #[js::import = "$0.name = $1"]
pub fn set_name(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "DomTokenList")]
    #[doc = "Getter for the `sandbox` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/sandbox)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomTokenList`, `HtmlIFrameElement`*"]
impl HtmlIFrameElement {
    #[js::import = "$0.sandbox"]
pub fn sandbox(&self) -> DomTokenList { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `allowFullscreen` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/allowFullscreen)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlIFrameElement`*"]
impl HtmlIFrameElement {
    #[js::import = "$0.allowFullscreen"]
pub fn allow_fullscreen(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `allowFullscreen` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/allowFullscreen)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlIFrameElement`*"]
impl HtmlIFrameElement {
    #[js::import = "$0.allowFullscreen = $1"]
pub fn set_allow_fullscreen(&self, value: bool) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `allowPaymentRequest` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/allowPaymentRequest)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlIFrameElement`*"]
impl HtmlIFrameElement {
    #[js::import = "$0.allowPaymentRequest"]
pub fn allow_payment_request(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `allowPaymentRequest` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/allowPaymentRequest)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlIFrameElement`*"]
impl HtmlIFrameElement {
    #[js::import = "$0.allowPaymentRequest = $1"]
pub fn set_allow_payment_request(&self, value: bool) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `width` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/width)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlIFrameElement`*"]
impl HtmlIFrameElement {
    #[js::import = "$0.width"]
pub fn width(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `width` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/width)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlIFrameElement`*"]
impl HtmlIFrameElement {
    #[js::import = "$0.width = $1"]
pub fn set_width(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `height` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/height)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlIFrameElement`*"]
impl HtmlIFrameElement {
    #[js::import = "$0.height"]
pub fn height(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `height` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/height)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlIFrameElement`*"]
impl HtmlIFrameElement {
    #[js::import = "$0.height = $1"]
pub fn set_height(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `referrerPolicy` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/referrerPolicy)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlIFrameElement`*"]
impl HtmlIFrameElement {
    #[js::import = "$0.referrerPolicy"]
pub fn referrer_policy(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `referrerPolicy` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/referrerPolicy)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlIFrameElement`*"]
impl HtmlIFrameElement {
    #[js::import = "$0.referrerPolicy = $1"]
pub fn set_referrer_policy(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Document")]
    #[doc = "Getter for the `contentDocument` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/contentDocument)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `HtmlIFrameElement`*"]
impl HtmlIFrameElement {
    #[js::import = "$0.contentDocument"]
pub fn content_document(&self) -> Option<Document> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Window")]
    #[doc = "Getter for the `contentWindow` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/contentWindow)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlIFrameElement`, `Window`*"]
impl HtmlIFrameElement {
    #[js::import = "$0.contentWindow"]
pub fn content_window(&self) -> Option<Window> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `align` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/align)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlIFrameElement`*"]
impl HtmlIFrameElement {
    #[js::import = "$0.align"]
pub fn align(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `align` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/align)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlIFrameElement`*"]
impl HtmlIFrameElement {
    #[js::import = "$0.align = $1"]
pub fn set_align(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `scrolling` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/scrolling)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlIFrameElement`*"]
impl HtmlIFrameElement {
    #[js::import = "$0.scrolling"]
pub fn scrolling(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `scrolling` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/scrolling)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlIFrameElement`*"]
impl HtmlIFrameElement {
    #[js::import = "$0.scrolling = $1"]
pub fn set_scrolling(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `frameBorder` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/frameBorder)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlIFrameElement`*"]
impl HtmlIFrameElement {
    #[js::import = "$0.frameBorder"]
pub fn frame_border(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `frameBorder` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/frameBorder)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlIFrameElement`*"]
impl HtmlIFrameElement {
    #[js::import = "$0.frameBorder = $1"]
pub fn set_frame_border(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `longDesc` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/longDesc)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlIFrameElement`*"]
impl HtmlIFrameElement {
    #[js::import = "$0.longDesc"]
pub fn long_desc(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `longDesc` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/longDesc)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlIFrameElement`*"]
impl HtmlIFrameElement {
    #[js::import = "$0.longDesc = $1"]
pub fn set_long_desc(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `marginHeight` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/marginHeight)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlIFrameElement`*"]
impl HtmlIFrameElement {
    #[js::import = "$0.marginHeight"]
pub fn margin_height(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `marginHeight` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/marginHeight)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlIFrameElement`*"]
impl HtmlIFrameElement {
    #[js::import = "$0.marginHeight = $1"]
pub fn set_margin_height(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `marginWidth` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/marginWidth)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlIFrameElement`*"]
impl HtmlIFrameElement {
    #[js::import = "$0.marginWidth"]
pub fn margin_width(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `marginWidth` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/marginWidth)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlIFrameElement`*"]
impl HtmlIFrameElement {
    #[js::import = "$0.marginWidth = $1"]
pub fn set_margin_width(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Document")]
    #[doc = "The `getSVGDocument()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/getSVGDocument)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `HtmlIFrameElement`*"]
impl HtmlIFrameElement {
    #[js::import = "$0.getSVGDocument()"]
pub fn get_svg_document(&self) -> Option<Document> { panic!("web-sys intrinsic") }
}
