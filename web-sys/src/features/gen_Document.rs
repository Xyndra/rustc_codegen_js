#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Document` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
#[js::opaque]
#[derive(Copy)]
pub struct Document(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "DomImplementation")]
    #[doc = "Getter for the `implementation` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/implementation)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `DomImplementation`*"]
impl Document {
    #[js::import = "$0.implementation"]
pub fn implementation(&self) -> Result<DomImplementation, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `URL` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/URL)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.URL"]
pub fn url(&self) -> Result<::alloc::string::String, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `documentURI` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/documentURI)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.documentURI"]
pub fn document_uri(&self) -> Result<::alloc::string::String, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `compatMode` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/compatMode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.compatMode"]
pub fn compat_mode(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `characterSet` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/characterSet)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.characterSet"]
pub fn character_set(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `charset` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/charset)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.charset"]
pub fn charset(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `inputEncoding` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/inputEncoding)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.inputEncoding"]
pub fn input_encoding(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `contentType` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/contentType)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.contentType"]
pub fn content_type(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "DocumentType")]
    #[doc = "Getter for the `doctype` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/doctype)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `DocumentType`*"]
impl Document {
    #[js::import = "$0.doctype"]
pub fn doctype(&self) -> Option<DocumentType> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Element")]
    #[doc = "Getter for the `documentElement` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/documentElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `Element`*"]
impl Document {
    #[js::import = "$0.documentElement"]
pub fn document_element(&self) -> Option<Element> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Location")]
    #[doc = "Getter for the `location` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/location)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `Location`*"]
impl Document {
    #[js::import = "$0.location"]
pub fn location(&self) -> Option<Location> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `referrer` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/referrer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.referrer"]
pub fn referrer(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `lastModified` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/lastModified)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.lastModified"]
pub fn last_modified(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `readyState` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/readyState)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.readyState"]
pub fn ready_state(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `title` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/title)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.title"]
pub fn title(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `title` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/title)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.title = $1"]
pub fn set_title(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `dir` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/dir)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.dir"]
pub fn dir(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `dir` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/dir)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.dir = $1"]
pub fn set_dir(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "HtmlElement")]
    #[doc = "Getter for the `body` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/body)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `HtmlElement`*"]
impl Document {
    #[js::import = "$0.body"]
pub fn body(&self) -> Option<HtmlElement> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "HtmlElement")]
    #[doc = "Setter for the `body` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/body)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `HtmlElement`*"]
impl Document {
    #[js::import = "$0.body = $1"]
pub fn set_body(&self, value: Option<&HtmlElement>) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "HtmlHeadElement")]
    #[doc = "Getter for the `head` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/head)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `HtmlHeadElement`*"]
impl Document {
    #[js::import = "$0.head"]
pub fn head(&self) -> Option<HtmlHeadElement> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "HtmlCollection")]
    #[doc = "Getter for the `images` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/images)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `HtmlCollection`*"]
impl Document {
    #[js::import = "$0.images"]
pub fn images(&self) -> HtmlCollection { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "HtmlCollection")]
    #[doc = "Getter for the `embeds` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/embeds)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `HtmlCollection`*"]
impl Document {
    #[js::import = "$0.embeds"]
pub fn embeds(&self) -> HtmlCollection { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "HtmlCollection")]
    #[doc = "Getter for the `plugins` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/plugins)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `HtmlCollection`*"]
impl Document {
    #[js::import = "$0.plugins"]
pub fn plugins(&self) -> HtmlCollection { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "HtmlCollection")]
    #[doc = "Getter for the `links` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/links)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `HtmlCollection`*"]
impl Document {
    #[js::import = "$0.links"]
pub fn links(&self) -> HtmlCollection { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "HtmlCollection")]
    #[doc = "Getter for the `forms` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/forms)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `HtmlCollection`*"]
impl Document {
    #[js::import = "$0.forms"]
pub fn forms(&self) -> HtmlCollection { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "HtmlCollection")]
    #[doc = "Getter for the `scripts` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/scripts)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `HtmlCollection`*"]
impl Document {
    #[js::import = "$0.scripts"]
pub fn scripts(&self) -> HtmlCollection { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Window")]
    #[doc = "Getter for the `defaultView` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/defaultView)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `Window`*"]
impl Document {
    #[js::import = "$0.defaultView"]
pub fn default_view(&self) -> Option<Window> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onreadystatechange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onreadystatechange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onreadystatechange"]
pub fn onreadystatechange(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onreadystatechange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onreadystatechange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onreadystatechange = $1"]
pub fn set_onreadystatechange(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onbeforescriptexecute` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onbeforescriptexecute)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onbeforescriptexecute"]
pub fn onbeforescriptexecute(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onbeforescriptexecute` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onbeforescriptexecute)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onbeforescriptexecute = $1"]
pub fn set_onbeforescriptexecute(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onafterscriptexecute` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onafterscriptexecute)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onafterscriptexecute"]
pub fn onafterscriptexecute(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onafterscriptexecute` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onafterscriptexecute)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onafterscriptexecute = $1"]
pub fn set_onafterscriptexecute(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onselectionchange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onselectionchange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onselectionchange"]
pub fn onselectionchange(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onselectionchange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onselectionchange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onselectionchange = $1"]
pub fn set_onselectionchange(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Element")]
    #[doc = "Getter for the `currentScript` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/currentScript)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `Element`*"]
impl Document {
    #[js::import = "$0.currentScript"]
pub fn current_script(&self) -> Option<Element> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "HtmlCollection")]
    #[doc = "Getter for the `anchors` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/anchors)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `HtmlCollection`*"]
impl Document {
    #[js::import = "$0.anchors"]
pub fn anchors(&self) -> HtmlCollection { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "HtmlCollection")]
    #[doc = "Getter for the `applets` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/applets)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `HtmlCollection`*"]
impl Document {
    #[js::import = "$0.applets"]
pub fn applets(&self) -> HtmlCollection { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `fullscreen` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/fullscreen)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.fullscreen"]
pub fn fullscreen(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `fullscreenEnabled` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/fullscreenEnabled)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.fullscreenEnabled"]
pub fn fullscreen_enabled(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onfullscreenchange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onfullscreenchange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onfullscreenchange"]
pub fn onfullscreenchange(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onfullscreenchange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onfullscreenchange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onfullscreenchange = $1"]
pub fn set_onfullscreenchange(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onfullscreenerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onfullscreenerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onfullscreenerror"]
pub fn onfullscreenerror(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onfullscreenerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onfullscreenerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onfullscreenerror = $1"]
pub fn set_onfullscreenerror(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onpointerlockchange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onpointerlockchange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onpointerlockchange"]
pub fn onpointerlockchange(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onpointerlockchange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onpointerlockchange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onpointerlockchange = $1"]
pub fn set_onpointerlockchange(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onpointerlockerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onpointerlockerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onpointerlockerror"]
pub fn onpointerlockerror(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onpointerlockerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onpointerlockerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onpointerlockerror = $1"]
pub fn set_onpointerlockerror(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `hidden` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/hidden)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.hidden"]
pub fn hidden(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "VisibilityState")]
    #[doc = "Getter for the `visibilityState` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/visibilityState)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `VisibilityState`*"]
impl Document {
    #[js::import = "$0.visibilityState"]
pub fn visibility_state(&self) -> VisibilityState { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onvisibilitychange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onvisibilitychange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onvisibilitychange"]
pub fn onvisibilitychange(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onvisibilitychange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onvisibilitychange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onvisibilitychange = $1"]
pub fn set_onvisibilitychange(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `selectedStyleSheetSet` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/selectedStyleSheetSet)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.selectedStyleSheetSet"]
pub fn selected_style_sheet_set(&self) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `selectedStyleSheetSet` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/selectedStyleSheetSet)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.selectedStyleSheetSet = $1"]
pub fn set_selected_style_sheet_set(&self, value: Option<&str>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `lastStyleSheetSet` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/lastStyleSheetSet)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.lastStyleSheetSet"]
pub fn last_style_sheet_set(&self) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `preferredStyleSheetSet` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/preferredStyleSheetSet)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.preferredStyleSheetSet"]
pub fn preferred_style_sheet_set(&self) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "DomStringList")]
    #[doc = "Getter for the `styleSheetSets` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/styleSheetSets)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `DomStringList`*"]
impl Document {
    #[js::import = "$0.styleSheetSets"]
pub fn style_sheet_sets(&self) -> DomStringList { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Element")]
    #[doc = "Getter for the `scrollingElement` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/scrollingElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `Element`*"]
impl Document {
    #[js::import = "$0.scrollingElement"]
pub fn scrolling_element(&self) -> Option<Element> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "DocumentTimeline")]
    #[doc = "Getter for the `timeline` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/timeline)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `DocumentTimeline`*"]
impl Document {
    #[js::import = "$0.timeline"]
pub fn timeline(&self) -> DocumentTimeline { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgsvgElement")]
    #[doc = "Getter for the `rootElement` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/rootElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `SvgsvgElement`*"]
impl Document {
    #[js::import = "$0.rootElement"]
pub fn root_element(&self) -> Option<SvgsvgElement> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `pictureInPictureEnabled` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/pictureInPictureEnabled)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl Document {
    #[js::import = "$0.pictureInPictureEnabled"]
pub fn picture_in_picture_enabled(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `oncopy` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/oncopy)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.oncopy"]
pub fn oncopy(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `oncopy` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/oncopy)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.oncopy = $1"]
pub fn set_oncopy(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `oncut` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/oncut)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.oncut"]
pub fn oncut(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `oncut` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/oncut)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.oncut = $1"]
pub fn set_oncut(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onpaste` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onpaste)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onpaste"]
pub fn onpaste(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onpaste` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onpaste)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onpaste = $1"]
pub fn set_onpaste(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Element")]
    #[doc = "Getter for the `activeElement` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/activeElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `Element`*"]
impl Document {
    #[js::import = "$0.activeElement"]
pub fn active_element(&self) -> Option<Element> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Element")]
    #[doc = "Getter for the `pointerLockElement` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/pointerLockElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `Element`*"]
impl Document {
    #[js::import = "$0.pointerLockElement"]
pub fn pointer_lock_element(&self) -> Option<Element> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Element")]
    #[doc = "Getter for the `fullscreenElement` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/fullscreenElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `Element`*"]
impl Document {
    #[js::import = "$0.fullscreenElement"]
pub fn fullscreen_element(&self) -> Option<Element> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "StyleSheetList")]
    #[doc = "Getter for the `styleSheets` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/styleSheets)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `StyleSheetList`*"]
impl Document {
    #[js::import = "$0.styleSheets"]
pub fn style_sheets(&self) -> StyleSheetList { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `adoptedStyleSheets` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/adoptedStyleSheets)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.adoptedStyleSheets"]
pub fn adopted_style_sheets(&self) -> ::js_sys::Array { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `adoptedStyleSheets` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/adoptedStyleSheets)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.adoptedStyleSheets = $1"]
pub fn set_adopted_style_sheets(&self, value: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "Element")]
    #[doc = "Getter for the `pictureInPictureElement` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/pictureInPictureElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `Element`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl Document {
    #[js::import = "$0.pictureInPictureElement"]
pub fn picture_in_picture_element(&self) -> Option<Element> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "FontFaceSet")]
    #[doc = "Getter for the `fonts` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/fonts)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `FontFaceSet`*"]
impl Document {
    #[js::import = "$0.fonts"]
pub fn fonts(&self) -> FontFaceSet { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onabort` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onabort)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onabort"]
pub fn onabort(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onabort` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onabort)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onabort = $1"]
pub fn set_onabort(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onblur` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onblur)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onblur"]
pub fn onblur(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onblur` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onblur)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onblur = $1"]
pub fn set_onblur(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onfocus` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onfocus)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onfocus"]
pub fn onfocus(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onfocus` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onfocus)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onfocus = $1"]
pub fn set_onfocus(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onauxclick` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onauxclick)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onauxclick"]
pub fn onauxclick(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onauxclick` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onauxclick)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onauxclick = $1"]
pub fn set_onauxclick(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onbeforetoggle` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onbeforetoggle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onbeforetoggle"]
pub fn onbeforetoggle(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onbeforetoggle` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onbeforetoggle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onbeforetoggle = $1"]
pub fn set_onbeforetoggle(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `oncanplay` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/oncanplay)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.oncanplay"]
pub fn oncanplay(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `oncanplay` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/oncanplay)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.oncanplay = $1"]
pub fn set_oncanplay(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `oncanplaythrough` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/oncanplaythrough)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.oncanplaythrough"]
pub fn oncanplaythrough(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `oncanplaythrough` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/oncanplaythrough)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.oncanplaythrough = $1"]
pub fn set_oncanplaythrough(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onchange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onchange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onchange"]
pub fn onchange(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onchange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onchange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onchange = $1"]
pub fn set_onchange(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onclick` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onclick)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onclick"]
pub fn onclick(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onclick` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onclick)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onclick = $1"]
pub fn set_onclick(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onclose` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onclose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onclose"]
pub fn onclose(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onclose` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onclose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onclose = $1"]
pub fn set_onclose(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `oncontextmenu` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/oncontextmenu)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.oncontextmenu"]
pub fn oncontextmenu(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `oncontextmenu` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/oncontextmenu)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.oncontextmenu = $1"]
pub fn set_oncontextmenu(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `ondblclick` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ondblclick)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.ondblclick"]
pub fn ondblclick(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `ondblclick` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ondblclick)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.ondblclick = $1"]
pub fn set_ondblclick(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `ondrag` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ondrag)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.ondrag"]
pub fn ondrag(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `ondrag` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ondrag)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.ondrag = $1"]
pub fn set_ondrag(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `ondragend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ondragend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.ondragend"]
pub fn ondragend(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `ondragend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ondragend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.ondragend = $1"]
pub fn set_ondragend(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `ondragenter` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ondragenter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.ondragenter"]
pub fn ondragenter(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `ondragenter` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ondragenter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.ondragenter = $1"]
pub fn set_ondragenter(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `ondragexit` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ondragexit)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.ondragexit"]
pub fn ondragexit(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `ondragexit` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ondragexit)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.ondragexit = $1"]
pub fn set_ondragexit(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `ondragleave` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ondragleave)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.ondragleave"]
pub fn ondragleave(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `ondragleave` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ondragleave)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.ondragleave = $1"]
pub fn set_ondragleave(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `ondragover` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ondragover)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.ondragover"]
pub fn ondragover(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `ondragover` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ondragover)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.ondragover = $1"]
pub fn set_ondragover(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `ondragstart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ondragstart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.ondragstart"]
pub fn ondragstart(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `ondragstart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ondragstart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.ondragstart = $1"]
pub fn set_ondragstart(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `ondrop` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ondrop)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.ondrop"]
pub fn ondrop(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `ondrop` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ondrop)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.ondrop = $1"]
pub fn set_ondrop(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `ondurationchange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ondurationchange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.ondurationchange"]
pub fn ondurationchange(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `ondurationchange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ondurationchange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.ondurationchange = $1"]
pub fn set_ondurationchange(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onemptied` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onemptied)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onemptied"]
pub fn onemptied(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onemptied` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onemptied)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onemptied = $1"]
pub fn set_onemptied(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onended` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onended)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onended"]
pub fn onended(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onended` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onended)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onended = $1"]
pub fn set_onended(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `oninput` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/oninput)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.oninput"]
pub fn oninput(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `oninput` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/oninput)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.oninput = $1"]
pub fn set_oninput(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onbeforeinput` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onbeforeinput)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onbeforeinput"]
pub fn onbeforeinput(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onbeforeinput` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onbeforeinput)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onbeforeinput = $1"]
pub fn set_onbeforeinput(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `oninvalid` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/oninvalid)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.oninvalid"]
pub fn oninvalid(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `oninvalid` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/oninvalid)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.oninvalid = $1"]
pub fn set_oninvalid(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onkeydown` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onkeydown)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onkeydown"]
pub fn onkeydown(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onkeydown` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onkeydown)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onkeydown = $1"]
pub fn set_onkeydown(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onkeypress` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onkeypress)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onkeypress"]
pub fn onkeypress(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onkeypress` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onkeypress)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onkeypress = $1"]
pub fn set_onkeypress(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onkeyup` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onkeyup)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onkeyup"]
pub fn onkeyup(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onkeyup` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onkeyup)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onkeyup = $1"]
pub fn set_onkeyup(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onload` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onload)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onload"]
pub fn onload(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onload` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onload)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onload = $1"]
pub fn set_onload(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onloadeddata` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onloadeddata)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onloadeddata"]
pub fn onloadeddata(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onloadeddata` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onloadeddata)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onloadeddata = $1"]
pub fn set_onloadeddata(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onloadedmetadata` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onloadedmetadata)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onloadedmetadata"]
pub fn onloadedmetadata(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onloadedmetadata` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onloadedmetadata)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onloadedmetadata = $1"]
pub fn set_onloadedmetadata(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onloadend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onloadend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onloadend"]
pub fn onloadend(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onloadend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onloadend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onloadend = $1"]
pub fn set_onloadend(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onloadstart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onloadstart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onloadstart"]
pub fn onloadstart(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onloadstart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onloadstart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onloadstart = $1"]
pub fn set_onloadstart(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onmousedown` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onmousedown)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onmousedown"]
pub fn onmousedown(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onmousedown` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onmousedown)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onmousedown = $1"]
pub fn set_onmousedown(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onmouseenter` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onmouseenter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onmouseenter"]
pub fn onmouseenter(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onmouseenter` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onmouseenter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onmouseenter = $1"]
pub fn set_onmouseenter(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onmouseleave` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onmouseleave)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onmouseleave"]
pub fn onmouseleave(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onmouseleave` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onmouseleave)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onmouseleave = $1"]
pub fn set_onmouseleave(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onmousemove` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onmousemove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onmousemove"]
pub fn onmousemove(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onmousemove` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onmousemove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onmousemove = $1"]
pub fn set_onmousemove(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onmouseout` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onmouseout)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onmouseout"]
pub fn onmouseout(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onmouseout` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onmouseout)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onmouseout = $1"]
pub fn set_onmouseout(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onmouseover` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onmouseover)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onmouseover"]
pub fn onmouseover(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onmouseover` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onmouseover)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onmouseover = $1"]
pub fn set_onmouseover(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onmouseup` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onmouseup)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onmouseup"]
pub fn onmouseup(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onmouseup` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onmouseup)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onmouseup = $1"]
pub fn set_onmouseup(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onwheel` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onwheel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onwheel"]
pub fn onwheel(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onwheel` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onwheel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onwheel = $1"]
pub fn set_onwheel(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onpause` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onpause)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onpause"]
pub fn onpause(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onpause` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onpause)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onpause = $1"]
pub fn set_onpause(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onplay` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onplay)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onplay"]
pub fn onplay(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onplay` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onplay)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onplay = $1"]
pub fn set_onplay(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onplaying` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onplaying)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onplaying"]
pub fn onplaying(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onplaying` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onplaying)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onplaying = $1"]
pub fn set_onplaying(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onprogress` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onprogress)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onprogress"]
pub fn onprogress(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onprogress` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onprogress)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onprogress = $1"]
pub fn set_onprogress(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onratechange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onratechange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onratechange"]
pub fn onratechange(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onratechange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onratechange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onratechange = $1"]
pub fn set_onratechange(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onreset` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onreset)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onreset"]
pub fn onreset(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onreset` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onreset)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onreset = $1"]
pub fn set_onreset(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onresize` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onresize)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onresize"]
pub fn onresize(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onresize` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onresize)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onresize = $1"]
pub fn set_onresize(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onscroll` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onscroll)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onscroll"]
pub fn onscroll(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onscroll` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onscroll)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onscroll = $1"]
pub fn set_onscroll(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onseeked` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onseeked)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onseeked"]
pub fn onseeked(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onseeked` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onseeked)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onseeked = $1"]
pub fn set_onseeked(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onseeking` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onseeking)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onseeking"]
pub fn onseeking(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onseeking` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onseeking)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onseeking = $1"]
pub fn set_onseeking(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onselect` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onselect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onselect"]
pub fn onselect(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onselect` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onselect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onselect = $1"]
pub fn set_onselect(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onshow` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onshow)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onshow"]
pub fn onshow(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onshow` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onshow)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onshow = $1"]
pub fn set_onshow(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onstalled` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onstalled)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onstalled"]
pub fn onstalled(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onstalled` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onstalled)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onstalled = $1"]
pub fn set_onstalled(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onsubmit` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onsubmit)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onsubmit"]
pub fn onsubmit(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onsubmit` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onsubmit)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onsubmit = $1"]
pub fn set_onsubmit(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onsuspend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onsuspend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onsuspend"]
pub fn onsuspend(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onsuspend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onsuspend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onsuspend = $1"]
pub fn set_onsuspend(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `ontimeupdate` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ontimeupdate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.ontimeupdate"]
pub fn ontimeupdate(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `ontimeupdate` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ontimeupdate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.ontimeupdate = $1"]
pub fn set_ontimeupdate(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onvolumechange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onvolumechange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onvolumechange"]
pub fn onvolumechange(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onvolumechange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onvolumechange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onvolumechange = $1"]
pub fn set_onvolumechange(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onwaiting` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onwaiting)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onwaiting"]
pub fn onwaiting(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onwaiting` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onwaiting)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onwaiting = $1"]
pub fn set_onwaiting(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onselectstart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onselectstart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onselectstart"]
pub fn onselectstart(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onselectstart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onselectstart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onselectstart = $1"]
pub fn set_onselectstart(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `ontoggle` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ontoggle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.ontoggle"]
pub fn ontoggle(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `ontoggle` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ontoggle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.ontoggle = $1"]
pub fn set_ontoggle(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onpointercancel` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onpointercancel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onpointercancel"]
pub fn onpointercancel(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onpointercancel` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onpointercancel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onpointercancel = $1"]
pub fn set_onpointercancel(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onpointerdown` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onpointerdown)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onpointerdown"]
pub fn onpointerdown(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onpointerdown` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onpointerdown)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onpointerdown = $1"]
pub fn set_onpointerdown(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onpointerup` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onpointerup)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onpointerup"]
pub fn onpointerup(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onpointerup` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onpointerup)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onpointerup = $1"]
pub fn set_onpointerup(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onpointermove` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onpointermove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onpointermove"]
pub fn onpointermove(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onpointermove` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onpointermove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onpointermove = $1"]
pub fn set_onpointermove(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onpointerout` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onpointerout)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onpointerout"]
pub fn onpointerout(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onpointerout` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onpointerout)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onpointerout = $1"]
pub fn set_onpointerout(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onpointerover` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onpointerover)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onpointerover"]
pub fn onpointerover(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onpointerover` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onpointerover)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onpointerover = $1"]
pub fn set_onpointerover(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onpointerenter` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onpointerenter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onpointerenter"]
pub fn onpointerenter(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onpointerenter` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onpointerenter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onpointerenter = $1"]
pub fn set_onpointerenter(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onpointerleave` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onpointerleave)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onpointerleave"]
pub fn onpointerleave(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onpointerleave` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onpointerleave)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onpointerleave = $1"]
pub fn set_onpointerleave(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `ongotpointercapture` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ongotpointercapture)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.ongotpointercapture"]
pub fn ongotpointercapture(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `ongotpointercapture` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ongotpointercapture)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.ongotpointercapture = $1"]
pub fn set_ongotpointercapture(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onlostpointercapture` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onlostpointercapture)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onlostpointercapture"]
pub fn onlostpointercapture(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onlostpointercapture` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onlostpointercapture)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onlostpointercapture = $1"]
pub fn set_onlostpointercapture(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onanimationcancel` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onanimationcancel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onanimationcancel"]
pub fn onanimationcancel(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onanimationcancel` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onanimationcancel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onanimationcancel = $1"]
pub fn set_onanimationcancel(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onanimationend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onanimationend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onanimationend"]
pub fn onanimationend(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onanimationend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onanimationend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onanimationend = $1"]
pub fn set_onanimationend(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onanimationiteration` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onanimationiteration)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onanimationiteration"]
pub fn onanimationiteration(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onanimationiteration` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onanimationiteration)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onanimationiteration = $1"]
pub fn set_onanimationiteration(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onanimationstart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onanimationstart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onanimationstart"]
pub fn onanimationstart(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onanimationstart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onanimationstart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onanimationstart = $1"]
pub fn set_onanimationstart(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `ontransitioncancel` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ontransitioncancel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.ontransitioncancel"]
pub fn ontransitioncancel(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `ontransitioncancel` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ontransitioncancel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.ontransitioncancel = $1"]
pub fn set_ontransitioncancel(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `ontransitionend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ontransitionend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.ontransitionend"]
pub fn ontransitionend(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `ontransitionend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ontransitionend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.ontransitionend = $1"]
pub fn set_ontransitionend(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `ontransitionrun` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ontransitionrun)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.ontransitionrun"]
pub fn ontransitionrun(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `ontransitionrun` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ontransitionrun)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.ontransitionrun = $1"]
pub fn set_ontransitionrun(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `ontransitionstart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ontransitionstart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.ontransitionstart"]
pub fn ontransitionstart(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `ontransitionstart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ontransitionstart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.ontransitionstart = $1"]
pub fn set_ontransitionstart(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onwebkitanimationend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onwebkitanimationend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onwebkitanimationend"]
pub fn onwebkitanimationend(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onwebkitanimationend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onwebkitanimationend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onwebkitanimationend = $1"]
pub fn set_onwebkitanimationend(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onwebkitanimationiteration` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onwebkitanimationiteration)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onwebkitanimationiteration"]
pub fn onwebkitanimationiteration(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onwebkitanimationiteration` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onwebkitanimationiteration)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onwebkitanimationiteration = $1"]
pub fn set_onwebkitanimationiteration(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onwebkitanimationstart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onwebkitanimationstart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onwebkitanimationstart"]
pub fn onwebkitanimationstart(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onwebkitanimationstart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onwebkitanimationstart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onwebkitanimationstart = $1"]
pub fn set_onwebkitanimationstart(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onwebkittransitionend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onwebkittransitionend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onwebkittransitionend"]
pub fn onwebkittransitionend(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onwebkittransitionend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onwebkittransitionend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onwebkittransitionend = $1"]
pub fn set_onwebkittransitionend(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onerror"]
pub fn onerror(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/onerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.onerror = $1"]
pub fn set_onerror(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "HtmlCollection")]
    #[doc = "Getter for the `children` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/children)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `HtmlCollection`*"]
impl Document {
    #[js::import = "$0.children"]
pub fn children(&self) -> HtmlCollection { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Element")]
    #[doc = "Getter for the `firstElementChild` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/firstElementChild)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `Element`*"]
impl Document {
    #[js::import = "$0.firstElementChild"]
pub fn first_element_child(&self) -> Option<Element> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Element")]
    #[doc = "Getter for the `lastElementChild` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/lastElementChild)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `Element`*"]
impl Document {
    #[js::import = "$0.lastElementChild"]
pub fn last_element_child(&self) -> Option<Element> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `childElementCount` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/childElementCount)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.childElementCount"]
pub fn child_element_count(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `ontouchstart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ontouchstart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.ontouchstart"]
pub fn ontouchstart(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `ontouchstart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ontouchstart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.ontouchstart = $1"]
pub fn set_ontouchstart(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `ontouchend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ontouchend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.ontouchend"]
pub fn ontouchend(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `ontouchend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ontouchend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.ontouchend = $1"]
pub fn set_ontouchend(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `ontouchmove` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ontouchmove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.ontouchmove"]
pub fn ontouchmove(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `ontouchmove` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ontouchmove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.ontouchmove = $1"]
pub fn set_ontouchmove(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `ontouchcancel` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ontouchcancel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.ontouchcancel"]
pub fn ontouchcancel(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `ontouchcancel` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/ontouchcancel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.ontouchcancel = $1"]
pub fn set_ontouchcancel(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "The `new Document(..)` constructor, creating a new instance of `Document`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/Document)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "new Document()"]
pub fn new() -> Result<Document, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `adoptNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/adoptNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.adoptNode($1)"]
pub fn adopt_node(&self, node: &Node) -> Result<Node, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "CaretPosition")]
    #[doc = "The `caretPositionFromPoint()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/caretPositionFromPoint)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CaretPosition`, `Document`*"]
impl Document {
    #[js::import = "$0.caretPositionFromPoint($1, $2)"]
pub fn caret_position_from_point(&self, x: f32, y: f32) -> Option<CaretPosition> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Attr")]
    #[doc = "The `createAttribute()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/createAttribute)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Attr`, `Document`*"]
impl Document {
    #[js::import = "$0.createAttribute($1)"]
pub fn create_attribute(&self, name: &str) -> Result<Attr, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Attr")]
    #[doc = "The `createAttributeNS()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/createAttributeNS)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Attr`, `Document`*"]
impl Document {
    #[js::import = "$0.createAttributeNS($1, $2)"]
pub fn create_attribute_ns(&self, namespace: Option<&str>, name: &str) -> Result<Attr, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "CdataSection")]
    #[doc = "The `createCDATASection()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/createCDATASection)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CdataSection`, `Document`*"]
impl Document {
    #[js::import = "$0.createCDATASection($1)"]
pub fn create_cdata_section(&self, data: &str) -> Result<CdataSection, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Comment")]
    #[doc = "The `createComment()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/createComment)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Comment`, `Document`*"]
impl Document {
    #[js::import = "$0.createComment($1)"]
pub fn create_comment(&self, data: &str) -> Comment { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "DocumentFragment")]
    #[doc = "The `createDocumentFragment()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/createDocumentFragment)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `DocumentFragment`*"]
impl Document {
    #[js::import = "$0.createDocumentFragment()"]
pub fn create_document_fragment(&self) -> DocumentFragment { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Element")]
    #[doc = "The `createElement()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/createElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `Element`*"]
impl Document {
    #[js::import = "$0.createElement($1)"]
pub fn create_element(&self, local_name: &str) -> Result<Element, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "Element", feature = "ElementCreationOptions",))]
    #[doc = "The `createElement()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/createElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `Element`, `ElementCreationOptions`*"]
impl Document {
    #[js::import = "$0.createElement($1, $2)"]
pub fn create_element_with_element_creation_options(&self, local_name: &str, options: &ElementCreationOptions) -> Result<Element, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Element")]
    #[doc = "The `createElement()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/createElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `Element`*"]
impl Document {
    #[js::import = "$0.createElement($1, $2)"]
pub fn create_element_with_str(&self, local_name: &str, options: &str) -> Result<Element, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Element")]
    #[doc = "The `createElementNS()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/createElementNS)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `Element`*"]
impl Document {
    #[js::import = "$0.createElementNS($1, $2)"]
pub fn create_element_ns(&self, namespace: Option<&str>, qualified_name: &str) -> Result<Element, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "Element", feature = "ElementCreationOptions",))]
    #[doc = "The `createElementNS()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/createElementNS)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `Element`, `ElementCreationOptions`*"]
impl Document {
    #[js::import = "$0.createElementNS($1, $2, $3)"]
pub fn create_element_ns_with_element_creation_options(&self, namespace: Option<&str>, qualified_name: &str, options: &ElementCreationOptions) -> Result<Element, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Element")]
    #[doc = "The `createElementNS()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/createElementNS)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `Element`*"]
impl Document {
    #[js::import = "$0.createElementNS($1, $2, $3)"]
pub fn create_element_ns_with_str(&self, namespace: Option<&str>, qualified_name: &str, options: &str) -> Result<Element, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Event")]
    #[doc = "The `createEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/createEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `Event`*"]
impl Document {
    #[js::import = "$0.createEvent($1)"]
pub fn create_event(&self, interface: &str) -> Result<Event, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "NodeIterator")]
    #[doc = "The `createNodeIterator()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/createNodeIterator)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `NodeIterator`*"]
impl Document {
    #[js::import = "$0.createNodeIterator($1)"]
pub fn create_node_iterator(&self, root: &Node) -> Result<NodeIterator, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "NodeIterator")]
    #[doc = "The `createNodeIterator()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/createNodeIterator)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `NodeIterator`*"]
impl Document {
    #[js::import = "$0.createNodeIterator($1, $2)"]
pub fn create_node_iterator_with_what_to_show(&self, root: &Node, what_to_show: u32) -> Result<NodeIterator, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "NodeFilter", feature = "NodeIterator",))]
    #[doc = "The `createNodeIterator()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/createNodeIterator)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `NodeFilter`, `NodeIterator`*"]
impl Document {
    #[js::import = "$0.createNodeIterator($1, $2, $3)"]
pub fn create_node_iterator_with_what_to_show_and_filter(&self, root: &Node, what_to_show: u32, filter: Option<&NodeFilter>) -> Result<NodeIterator, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "ProcessingInstruction")]
    #[doc = "The `createProcessingInstruction()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/createProcessingInstruction)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `ProcessingInstruction`*"]
impl Document {
    #[js::import = "$0.createProcessingInstruction($1, $2)"]
pub fn create_processing_instruction(&self, target: &str, data: &str) -> Result<ProcessingInstruction, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Range")]
    #[doc = "The `createRange()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/createRange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `Range`*"]
impl Document {
    #[js::import = "$0.createRange()"]
pub fn create_range(&self) -> Result<Range, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Text")]
    #[doc = "The `createTextNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/createTextNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `Text`*"]
impl Document {
    #[js::import = "$0.createTextNode($1)"]
pub fn create_text_node(&self, data: &str) -> Text { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "TreeWalker")]
    #[doc = "The `createTreeWalker()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/createTreeWalker)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `TreeWalker`*"]
impl Document {
    #[js::import = "$0.createTreeWalker($1)"]
pub fn create_tree_walker(&self, root: &Node) -> Result<TreeWalker, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "TreeWalker")]
    #[doc = "The `createTreeWalker()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/createTreeWalker)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `TreeWalker`*"]
impl Document {
    #[js::import = "$0.createTreeWalker($1, $2)"]
pub fn create_tree_walker_with_what_to_show(&self, root: &Node, what_to_show: u32) -> Result<TreeWalker, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "NodeFilter", feature = "TreeWalker",))]
    #[doc = "The `createTreeWalker()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/createTreeWalker)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `NodeFilter`, `TreeWalker`*"]
impl Document {
    #[js::import = "$0.createTreeWalker($1, $2, $3)"]
pub fn create_tree_walker_with_what_to_show_and_filter(&self, root: &Node, what_to_show: u32, filter: Option<&NodeFilter>) -> Result<TreeWalker, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `enableStyleSheetsForSet()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/enableStyleSheetsForSet)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.enableStyleSheetsForSet($1)"]
pub fn enable_style_sheets_for_set(&self, name: Option<&str>) { panic!("web-sys intrinsic") }
}
    #[doc = "The `exitFullscreen()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/exitFullscreen)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.exitFullscreen()"]
pub fn exit_fullscreen(&self) { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `exitPictureInPicture()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/exitPictureInPicture)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl Document {
    #[js::import = "$0.exitPictureInPicture()"]
pub fn exit_picture_in_picture(&self) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[doc = "The `exitPointerLock()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/exitPointerLock)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.exitPointerLock()"]
pub fn exit_pointer_lock(&self) { panic!("web-sys intrinsic") }
}
    #[doc = "The `getAnimations()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/getAnimations)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.getAnimations()"]
pub fn get_animations(&self) -> ::js_sys::Array { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Element")]
    #[doc = "The `getElementById()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/getElementById)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `Element`*"]
impl Document {
    #[js::import = "$0.getElementById($1)"]
pub fn get_element_by_id(&self, element_id: &str) -> Option<Element> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "HtmlCollection")]
    #[doc = "The `getElementsByClassName()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/getElementsByClassName)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `HtmlCollection`*"]
impl Document {
    #[js::import = "$0.getElementsByClassName($1)"]
pub fn get_elements_by_class_name(&self, class_names: &str) -> HtmlCollection { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "NodeList")]
    #[doc = "The `getElementsByName()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/getElementsByName)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `NodeList`*"]
impl Document {
    #[js::import = "$0.getElementsByName($1)"]
pub fn get_elements_by_name(&self, element_name: &str) -> NodeList { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "HtmlCollection")]
    #[doc = "The `getElementsByTagName()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/getElementsByTagName)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `HtmlCollection`*"]
impl Document {
    #[js::import = "$0.getElementsByTagName($1)"]
pub fn get_elements_by_tag_name(&self, local_name: &str) -> HtmlCollection { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "HtmlCollection")]
    #[doc = "The `getElementsByTagNameNS()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/getElementsByTagNameNS)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `HtmlCollection`*"]
impl Document {
    #[js::import = "$0.getElementsByTagNameNS($1, $2)"]
pub fn get_elements_by_tag_name_ns(&self, namespace: Option<&str>, local_name: &str) -> Result<HtmlCollection, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Selection")]
    #[doc = "The `getSelection()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/getSelection)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `Selection`*"]
impl Document {
    #[js::import = "$0.getSelection()"]
pub fn get_selection(&self) -> Result<Option<Selection>, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `hasFocus()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/hasFocus)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.hasFocus()"]
pub fn has_focus(&self) -> Result<bool, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `importNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/importNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.importNode($1)"]
pub fn import_node(&self, node: &Node) -> Result<Node, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `importNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/importNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.importNode($1, $2)"]
pub fn import_node_with_deep(&self, node: &Node, deep: bool) -> Result<Node, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Element")]
    #[doc = "The `querySelector()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/querySelector)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `Element`*"]
impl Document {
    #[js::import = "$0.querySelector($1)"]
pub fn query_selector(&self, selectors: &str) -> Result<Option<Element>, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "NodeList")]
    #[doc = "The `querySelectorAll()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/querySelectorAll)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `NodeList`*"]
impl Document {
    #[js::import = "$0.querySelectorAll($1)"]
pub fn query_selector_all(&self, selectors: &str) -> Result<NodeList, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `releaseCapture()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/releaseCapture)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.releaseCapture()"]
pub fn release_capture(&self) { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "ViewTransition")]
    #[doc = "The `startViewTransition()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/startViewTransition)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `ViewTransition`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl Document {
    #[js::import = "$0.startViewTransition()"]
pub fn start_view_transition(&self) -> Result<ViewTransition, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "ViewTransition")]
    #[doc = "The `startViewTransition()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/startViewTransition)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `ViewTransition`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl Document {
    #[js::import = "$0.startViewTransition($1)"]
pub fn start_view_transition_with_update_callback(&self, update_callback: Option<&::js_sys::Function>) -> Result<ViewTransition, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Element")]
    #[doc = "The `elementFromPoint()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/elementFromPoint)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `Element`*"]
impl Document {
    #[js::import = "$0.elementFromPoint($1, $2)"]
pub fn element_from_point(&self, x: f32, y: f32) -> Option<Element> { panic!("web-sys intrinsic") }
}
    #[doc = "The `elementsFromPoint()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/elementsFromPoint)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.elementsFromPoint($1, $2)"]
pub fn elements_from_point(&self, x: f32, y: f32) -> ::js_sys::Array { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "DomPoint", feature = "DomPointInit", feature = "Text",))]
    #[doc = "The `convertPointFromNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/convertPointFromNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `DomPoint`, `DomPointInit`, `Text`*"]
impl Document {
    #[js::import = "$0.convertPointFromNode($1, $2)"]
pub fn convert_point_from_node_with_text(&self, point: &DomPointInit, from: &Text) -> Result<DomPoint, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "DomPoint", feature = "DomPointInit", feature = "Element",))]
    #[doc = "The `convertPointFromNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/convertPointFromNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `DomPoint`, `DomPointInit`, `Element`*"]
impl Document {
    #[js::import = "$0.convertPointFromNode($1, $2)"]
pub fn convert_point_from_node_with_element(&self, point: &DomPointInit, from: &Element) -> Result<DomPoint, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "DomPoint", feature = "DomPointInit",))]
    #[doc = "The `convertPointFromNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/convertPointFromNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `DomPoint`, `DomPointInit`*"]
impl Document {
    #[js::import = "$0.convertPointFromNode($1, $2)"]
pub fn convert_point_from_node_with_document(&self, point: &DomPointInit, from: &Document) -> Result<DomPoint, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(
        feature = "ConvertCoordinateOptions",
        feature = "DomPoint",
        feature = "DomPointInit",
        feature = "Text",
    ))]
    #[doc = "The `convertPointFromNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/convertPointFromNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `Document`, `DomPoint`, `DomPointInit`, `Text`*"]
impl Document {
    #[js::import = "$0.convertPointFromNode($1, $2, $3)"]
pub fn convert_point_from_node_with_text_and_options(&self, point: &DomPointInit, from: &Text, options: &ConvertCoordinateOptions) -> Result<DomPoint, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(
        feature = "ConvertCoordinateOptions",
        feature = "DomPoint",
        feature = "DomPointInit",
        feature = "Element",
    ))]
    #[doc = "The `convertPointFromNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/convertPointFromNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `Document`, `DomPoint`, `DomPointInit`, `Element`*"]
impl Document {
    #[js::import = "$0.convertPointFromNode($1, $2, $3)"]
pub fn convert_point_from_node_with_element_and_options(&self, point: &DomPointInit, from: &Element, options: &ConvertCoordinateOptions) -> Result<DomPoint, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(
        feature = "ConvertCoordinateOptions",
        feature = "DomPoint",
        feature = "DomPointInit",
    ))]
    #[doc = "The `convertPointFromNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/convertPointFromNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `Document`, `DomPoint`, `DomPointInit`*"]
impl Document {
    #[js::import = "$0.convertPointFromNode($1, $2, $3)"]
pub fn convert_point_from_node_with_document_and_options(&self, point: &DomPointInit, from: &Document, options: &ConvertCoordinateOptions) -> Result<DomPoint, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "DomQuad", feature = "Text",))]
    #[doc = "The `convertQuadFromNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/convertQuadFromNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `DomQuad`, `Text`*"]
impl Document {
    #[js::import = "$0.convertQuadFromNode($1, $2)"]
pub fn convert_quad_from_node_with_text(&self, quad: &DomQuad, from: &Text) -> Result<DomQuad, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "DomQuad", feature = "Element",))]
    #[doc = "The `convertQuadFromNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/convertQuadFromNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `DomQuad`, `Element`*"]
impl Document {
    #[js::import = "$0.convertQuadFromNode($1, $2)"]
pub fn convert_quad_from_node_with_element(&self, quad: &DomQuad, from: &Element) -> Result<DomQuad, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "DomQuad")]
    #[doc = "The `convertQuadFromNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/convertQuadFromNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `DomQuad`*"]
impl Document {
    #[js::import = "$0.convertQuadFromNode($1, $2)"]
pub fn convert_quad_from_node_with_document(&self, quad: &DomQuad, from: &Document) -> Result<DomQuad, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(
        feature = "ConvertCoordinateOptions",
        feature = "DomQuad",
        feature = "Text",
    ))]
    #[doc = "The `convertQuadFromNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/convertQuadFromNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `Document`, `DomQuad`, `Text`*"]
impl Document {
    #[js::import = "$0.convertQuadFromNode($1, $2, $3)"]
pub fn convert_quad_from_node_with_text_and_options(&self, quad: &DomQuad, from: &Text, options: &ConvertCoordinateOptions) -> Result<DomQuad, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(
        feature = "ConvertCoordinateOptions",
        feature = "DomQuad",
        feature = "Element",
    ))]
    #[doc = "The `convertQuadFromNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/convertQuadFromNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `Document`, `DomQuad`, `Element`*"]
impl Document {
    #[js::import = "$0.convertQuadFromNode($1, $2, $3)"]
pub fn convert_quad_from_node_with_element_and_options(&self, quad: &DomQuad, from: &Element, options: &ConvertCoordinateOptions) -> Result<DomQuad, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "ConvertCoordinateOptions", feature = "DomQuad",))]
    #[doc = "The `convertQuadFromNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/convertQuadFromNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `Document`, `DomQuad`*"]
impl Document {
    #[js::import = "$0.convertQuadFromNode($1, $2, $3)"]
pub fn convert_quad_from_node_with_document_and_options(&self, quad: &DomQuad, from: &Document, options: &ConvertCoordinateOptions) -> Result<DomQuad, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "DomQuad", feature = "DomRectReadOnly", feature = "Text",))]
    #[doc = "The `convertRectFromNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/convertRectFromNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `DomQuad`, `DomRectReadOnly`, `Text`*"]
impl Document {
    #[js::import = "$0.convertRectFromNode($1, $2)"]
pub fn convert_rect_from_node_with_text(&self, rect: &DomRectReadOnly, from: &Text) -> Result<DomQuad, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "DomQuad", feature = "DomRectReadOnly", feature = "Element",))]
    #[doc = "The `convertRectFromNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/convertRectFromNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `DomQuad`, `DomRectReadOnly`, `Element`*"]
impl Document {
    #[js::import = "$0.convertRectFromNode($1, $2)"]
pub fn convert_rect_from_node_with_element(&self, rect: &DomRectReadOnly, from: &Element) -> Result<DomQuad, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "DomQuad", feature = "DomRectReadOnly",))]
    #[doc = "The `convertRectFromNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/convertRectFromNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `DomQuad`, `DomRectReadOnly`*"]
impl Document {
    #[js::import = "$0.convertRectFromNode($1, $2)"]
pub fn convert_rect_from_node_with_document(&self, rect: &DomRectReadOnly, from: &Document) -> Result<DomQuad, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(
        feature = "ConvertCoordinateOptions",
        feature = "DomQuad",
        feature = "DomRectReadOnly",
        feature = "Text",
    ))]
    #[doc = "The `convertRectFromNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/convertRectFromNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `Document`, `DomQuad`, `DomRectReadOnly`, `Text`*"]
impl Document {
    #[js::import = "$0.convertRectFromNode($1, $2, $3)"]
pub fn convert_rect_from_node_with_text_and_options(&self, rect: &DomRectReadOnly, from: &Text, options: &ConvertCoordinateOptions) -> Result<DomQuad, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(
        feature = "ConvertCoordinateOptions",
        feature = "DomQuad",
        feature = "DomRectReadOnly",
        feature = "Element",
    ))]
    #[doc = "The `convertRectFromNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/convertRectFromNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `Document`, `DomQuad`, `DomRectReadOnly`, `Element`*"]
impl Document {
    #[js::import = "$0.convertRectFromNode($1, $2, $3)"]
pub fn convert_rect_from_node_with_element_and_options(&self, rect: &DomRectReadOnly, from: &Element, options: &ConvertCoordinateOptions) -> Result<DomQuad, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(
        feature = "ConvertCoordinateOptions",
        feature = "DomQuad",
        feature = "DomRectReadOnly",
    ))]
    #[doc = "The `convertRectFromNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/convertRectFromNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `Document`, `DomQuad`, `DomRectReadOnly`*"]
impl Document {
    #[js::import = "$0.convertRectFromNode($1, $2, $3)"]
pub fn convert_rect_from_node_with_document_and_options(&self, rect: &DomRectReadOnly, from: &Document, options: &ConvertCoordinateOptions) -> Result<DomQuad, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `getBoxQuads()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/getBoxQuads)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.getBoxQuads()"]
pub fn get_box_quads(&self) -> Result<::js_sys::Array, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "BoxQuadOptions")]
    #[doc = "The `getBoxQuads()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/getBoxQuads)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BoxQuadOptions`, `Document`*"]
impl Document {
    #[js::import = "$0.getBoxQuads($1)"]
pub fn get_box_quads_with_options(&self, options: &BoxQuadOptions) -> Result<::js_sys::Array, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.append($1)"]
pub fn append_with_node(&self, nodes: &::js_sys::Array) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.append()"]
pub fn append_with_node_0(&self) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.append($1)"]
pub fn append_with_node_1(&self, nodes_1: &Node) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.append($1, $2)"]
pub fn append_with_node_2(&self, nodes_1: &Node, nodes_2: &Node) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.append($1, $2, $3)"]
pub fn append_with_node_3(&self, nodes_1: &Node, nodes_2: &Node, nodes_3: &Node) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.append($1, $2, $3, $4)"]
pub fn append_with_node_4(&self, nodes_1: &Node, nodes_2: &Node, nodes_3: &Node, nodes_4: &Node) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.append($1, $2, $3, $4, $5)"]
pub fn append_with_node_5(&self, nodes_1: &Node, nodes_2: &Node, nodes_3: &Node, nodes_4: &Node, nodes_5: &Node) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.append($1, $2, $3, $4, $5, $6)"]
pub fn append_with_node_6(&self, nodes_1: &Node, nodes_2: &Node, nodes_3: &Node, nodes_4: &Node, nodes_5: &Node, nodes_6: &Node) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.append($1, $2, $3, $4, $5, $6, $7)"]
pub fn append_with_node_7(&self, nodes_1: &Node, nodes_2: &Node, nodes_3: &Node, nodes_4: &Node, nodes_5: &Node, nodes_6: &Node, nodes_7: &Node) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.append($1)"]
pub fn append_with_str(&self, nodes: &::js_sys::Array) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.append()"]
pub fn append_with_str_0(&self) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.append($1)"]
pub fn append_with_str_1(&self, nodes_1: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.append($1, $2)"]
pub fn append_with_str_2(&self, nodes_1: &str, nodes_2: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.append($1, $2, $3)"]
pub fn append_with_str_3(&self, nodes_1: &str, nodes_2: &str, nodes_3: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.append($1, $2, $3, $4)"]
pub fn append_with_str_4(&self, nodes_1: &str, nodes_2: &str, nodes_3: &str, nodes_4: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.append($1, $2, $3, $4, $5)"]
pub fn append_with_str_5(&self, nodes_1: &str, nodes_2: &str, nodes_3: &str, nodes_4: &str, nodes_5: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.append($1, $2, $3, $4, $5, $6)"]
pub fn append_with_str_6(&self, nodes_1: &str, nodes_2: &str, nodes_3: &str, nodes_4: &str, nodes_5: &str, nodes_6: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.append($1, $2, $3, $4, $5, $6, $7)"]
pub fn append_with_str_7(&self, nodes_1: &str, nodes_2: &str, nodes_3: &str, nodes_4: &str, nodes_5: &str, nodes_6: &str, nodes_7: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `prepend()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/prepend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.prepend($1)"]
pub fn prepend_with_node(&self, nodes: &::js_sys::Array) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `prepend()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/prepend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.prepend()"]
pub fn prepend_with_node_0(&self) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `prepend()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/prepend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.prepend($1)"]
pub fn prepend_with_node_1(&self, nodes_1: &Node) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `prepend()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/prepend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.prepend($1, $2)"]
pub fn prepend_with_node_2(&self, nodes_1: &Node, nodes_2: &Node) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `prepend()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/prepend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.prepend($1, $2, $3)"]
pub fn prepend_with_node_3(&self, nodes_1: &Node, nodes_2: &Node, nodes_3: &Node) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `prepend()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/prepend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.prepend($1, $2, $3, $4)"]
pub fn prepend_with_node_4(&self, nodes_1: &Node, nodes_2: &Node, nodes_3: &Node, nodes_4: &Node) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `prepend()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/prepend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.prepend($1, $2, $3, $4, $5)"]
pub fn prepend_with_node_5(&self, nodes_1: &Node, nodes_2: &Node, nodes_3: &Node, nodes_4: &Node, nodes_5: &Node) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `prepend()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/prepend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.prepend($1, $2, $3, $4, $5, $6)"]
pub fn prepend_with_node_6(&self, nodes_1: &Node, nodes_2: &Node, nodes_3: &Node, nodes_4: &Node, nodes_5: &Node, nodes_6: &Node) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `prepend()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/prepend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.prepend($1, $2, $3, $4, $5, $6, $7)"]
pub fn prepend_with_node_7(&self, nodes_1: &Node, nodes_2: &Node, nodes_3: &Node, nodes_4: &Node, nodes_5: &Node, nodes_6: &Node, nodes_7: &Node) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `prepend()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/prepend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.prepend($1)"]
pub fn prepend_with_str(&self, nodes: &::js_sys::Array) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `prepend()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/prepend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.prepend()"]
pub fn prepend_with_str_0(&self) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `prepend()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/prepend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.prepend($1)"]
pub fn prepend_with_str_1(&self, nodes_1: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `prepend()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/prepend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.prepend($1, $2)"]
pub fn prepend_with_str_2(&self, nodes_1: &str, nodes_2: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `prepend()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/prepend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.prepend($1, $2, $3)"]
pub fn prepend_with_str_3(&self, nodes_1: &str, nodes_2: &str, nodes_3: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `prepend()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/prepend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.prepend($1, $2, $3, $4)"]
pub fn prepend_with_str_4(&self, nodes_1: &str, nodes_2: &str, nodes_3: &str, nodes_4: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `prepend()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/prepend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.prepend($1, $2, $3, $4, $5)"]
pub fn prepend_with_str_5(&self, nodes_1: &str, nodes_2: &str, nodes_3: &str, nodes_4: &str, nodes_5: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `prepend()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/prepend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.prepend($1, $2, $3, $4, $5, $6)"]
pub fn prepend_with_str_6(&self, nodes_1: &str, nodes_2: &str, nodes_3: &str, nodes_4: &str, nodes_5: &str, nodes_6: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `prepend()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/prepend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.prepend($1, $2, $3, $4, $5, $6, $7)"]
pub fn prepend_with_str_7(&self, nodes_1: &str, nodes_2: &str, nodes_3: &str, nodes_4: &str, nodes_5: &str, nodes_6: &str, nodes_7: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `replaceChildren()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/replaceChildren)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.replaceChildren($1)"]
pub fn replace_children_with_node(&self, nodes: &::js_sys::Array) { panic!("web-sys intrinsic") }
}
    #[doc = "The `replaceChildren()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/replaceChildren)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.replaceChildren()"]
pub fn replace_children_with_node_0(&self) { panic!("web-sys intrinsic") }
}
    #[doc = "The `replaceChildren()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/replaceChildren)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.replaceChildren($1)"]
pub fn replace_children_with_node_1(&self, nodes_1: &Node) { panic!("web-sys intrinsic") }
}
    #[doc = "The `replaceChildren()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/replaceChildren)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.replaceChildren($1, $2)"]
pub fn replace_children_with_node_2(&self, nodes_1: &Node, nodes_2: &Node) { panic!("web-sys intrinsic") }
}
    #[doc = "The `replaceChildren()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/replaceChildren)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.replaceChildren($1, $2, $3)"]
pub fn replace_children_with_node_3(&self, nodes_1: &Node, nodes_2: &Node, nodes_3: &Node) { panic!("web-sys intrinsic") }
}
    #[doc = "The `replaceChildren()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/replaceChildren)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.replaceChildren($1, $2, $3, $4)"]
pub fn replace_children_with_node_4(&self, nodes_1: &Node, nodes_2: &Node, nodes_3: &Node, nodes_4: &Node) { panic!("web-sys intrinsic") }
}
    #[doc = "The `replaceChildren()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/replaceChildren)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.replaceChildren($1, $2, $3, $4, $5)"]
pub fn replace_children_with_node_5(&self, nodes_1: &Node, nodes_2: &Node, nodes_3: &Node, nodes_4: &Node, nodes_5: &Node) { panic!("web-sys intrinsic") }
}
    #[doc = "The `replaceChildren()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/replaceChildren)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.replaceChildren($1, $2, $3, $4, $5, $6)"]
pub fn replace_children_with_node_6(&self, nodes_1: &Node, nodes_2: &Node, nodes_3: &Node, nodes_4: &Node, nodes_5: &Node, nodes_6: &Node) { panic!("web-sys intrinsic") }
}
    #[doc = "The `replaceChildren()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/replaceChildren)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.replaceChildren($1, $2, $3, $4, $5, $6, $7)"]
pub fn replace_children_with_node_7(&self, nodes_1: &Node, nodes_2: &Node, nodes_3: &Node, nodes_4: &Node, nodes_5: &Node, nodes_6: &Node, nodes_7: &Node) { panic!("web-sys intrinsic") }
}
    #[doc = "The `replaceChildren()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/replaceChildren)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.replaceChildren($1)"]
pub fn replace_children_with_str(&self, nodes: &::js_sys::Array) { panic!("web-sys intrinsic") }
}
    #[doc = "The `replaceChildren()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/replaceChildren)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.replaceChildren()"]
pub fn replace_children_with_str_0(&self) { panic!("web-sys intrinsic") }
}
    #[doc = "The `replaceChildren()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/replaceChildren)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.replaceChildren($1)"]
pub fn replace_children_with_str_1(&self, nodes_1: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "The `replaceChildren()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/replaceChildren)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.replaceChildren($1, $2)"]
pub fn replace_children_with_str_2(&self, nodes_1: &str, nodes_2: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "The `replaceChildren()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/replaceChildren)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.replaceChildren($1, $2, $3)"]
pub fn replace_children_with_str_3(&self, nodes_1: &str, nodes_2: &str, nodes_3: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "The `replaceChildren()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/replaceChildren)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.replaceChildren($1, $2, $3, $4)"]
pub fn replace_children_with_str_4(&self, nodes_1: &str, nodes_2: &str, nodes_3: &str, nodes_4: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "The `replaceChildren()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/replaceChildren)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.replaceChildren($1, $2, $3, $4, $5)"]
pub fn replace_children_with_str_5(&self, nodes_1: &str, nodes_2: &str, nodes_3: &str, nodes_4: &str, nodes_5: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "The `replaceChildren()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/replaceChildren)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.replaceChildren($1, $2, $3, $4, $5, $6)"]
pub fn replace_children_with_str_6(&self, nodes_1: &str, nodes_2: &str, nodes_3: &str, nodes_4: &str, nodes_5: &str, nodes_6: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "The `replaceChildren()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/replaceChildren)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.replaceChildren($1, $2, $3, $4, $5, $6, $7)"]
pub fn replace_children_with_str_7(&self, nodes_1: &str, nodes_2: &str, nodes_3: &str, nodes_4: &str, nodes_5: &str, nodes_6: &str, nodes_7: &str) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "XPathExpression")]
    #[doc = "The `createExpression()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/createExpression)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `XPathExpression`*"]
impl Document {
    #[js::import = "$0.createExpression($1)"]
pub fn create_expression(&self, expression: &str) -> Result<XPathExpression, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "XPathExpression")]
    #[doc = "The `createExpression()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/createExpression)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `XPathExpression`*"]
impl Document {
    #[js::import = "$0.createExpression($1, $2)"]
pub fn create_expression_with_opt_callback(&self, expression: &str, resolver: Option<&::js_sys::Function>) -> Result<XPathExpression, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "XPathExpression", feature = "XPathNsResolver",))]
    #[doc = "The `createExpression()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/createExpression)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `XPathExpression`, `XPathNsResolver`*"]
impl Document {
    #[js::import = "$0.createExpression($1, $2)"]
pub fn create_expression_with_opt_x_path_ns_resolver(&self, expression: &str, resolver: Option<&XPathNsResolver>) -> Result<XPathExpression, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `createNSResolver()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/createNSResolver)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`*"]
impl Document {
    #[js::import = "$0.createNSResolver($1)"]
pub fn create_ns_resolver(&self, node_resolver: &Node) -> Node { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "XPathResult")]
    #[doc = "The `evaluate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/evaluate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `XPathResult`*"]
impl Document {
    #[js::import = "$0.evaluate($1, $2)"]
pub fn evaluate(&self, expression: &str, context_node: &Node) -> Result<XPathResult, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "XPathResult")]
    #[doc = "The `evaluate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/evaluate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `XPathResult`*"]
impl Document {
    #[js::import = "$0.evaluate($1, $2, $3)"]
pub fn evaluate_with_opt_callback(&self, expression: &str, context_node: &Node, resolver: Option<&::js_sys::Function>) -> Result<XPathResult, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "XPathNsResolver", feature = "XPathResult",))]
    #[doc = "The `evaluate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/evaluate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `XPathNsResolver`, `XPathResult`*"]
impl Document {
    #[js::import = "$0.evaluate($1, $2, $3)"]
pub fn evaluate_with_opt_x_path_ns_resolver(&self, expression: &str, context_node: &Node, resolver: Option<&XPathNsResolver>) -> Result<XPathResult, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "XPathResult")]
    #[doc = "The `evaluate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/evaluate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `XPathResult`*"]
impl Document {
    #[js::import = "$0.evaluate($1, $2, $3, $4)"]
pub fn evaluate_with_opt_callback_and_type(&self, expression: &str, context_node: &Node, resolver: Option<&::js_sys::Function>, type_: u16) -> Result<XPathResult, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "XPathNsResolver", feature = "XPathResult",))]
    #[doc = "The `evaluate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/evaluate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `XPathNsResolver`, `XPathResult`*"]
impl Document {
    #[js::import = "$0.evaluate($1, $2, $3, $4)"]
pub fn evaluate_with_opt_x_path_ns_resolver_and_type(&self, expression: &str, context_node: &Node, resolver: Option<&XPathNsResolver>, type_: u16) -> Result<XPathResult, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "XPathResult")]
    #[doc = "The `evaluate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/evaluate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `XPathResult`*"]
impl Document {
    #[js::import = "$0.evaluate($1, $2, $3, $4, $5)"]
pub fn evaluate_with_opt_callback_and_type_and_result(&self, expression: &str, context_node: &Node, resolver: Option<&::js_sys::Function>, type_: u16, result: Option<&::js_sys::Object>) -> Result<XPathResult, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "XPathNsResolver", feature = "XPathResult",))]
    #[doc = "The `evaluate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Document/evaluate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `XPathNsResolver`, `XPathResult`*"]
impl Document {
    #[js::import = "$0.evaluate($1, $2, $3, $4, $5)"]
pub fn evaluate_with_opt_x_path_ns_resolver_and_type_and_result(&self, expression: &str, context_node: &Node, resolver: Option<&XPathNsResolver>, type_: u16, result: Option<&::js_sys::Object>) -> Result<XPathResult, JsValue> { panic!("web-sys intrinsic") }
}
