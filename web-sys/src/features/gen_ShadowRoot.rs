#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ShadowRoot` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ShadowRoot)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ShadowRoot`*"]
#[js::opaque]
#[derive(Copy)]
pub struct ShadowRoot(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "ShadowRootMode")]
    #[doc = "Getter for the `mode` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ShadowRoot/mode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ShadowRoot`, `ShadowRootMode`*"]
impl ShadowRoot {
    #[js::import = "$0.mode"]
pub fn mode(&self) -> ShadowRootMode { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Element")]
    #[doc = "Getter for the `host` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ShadowRoot/host)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`, `ShadowRoot`*"]
impl ShadowRoot {
    #[js::import = "$0.host"]
pub fn host(&self) -> Element { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `innerHTML` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ShadowRoot/innerHTML)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ShadowRoot`*"]
impl ShadowRoot {
    #[js::import = "$0.innerHTML"]
pub fn inner_html(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `innerHTML` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ShadowRoot/innerHTML)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ShadowRoot`*"]
impl ShadowRoot {
    #[js::import = "$0.innerHTML = $1"]
pub fn set_inner_html(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Element")]
    #[doc = "Getter for the `activeElement` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ShadowRoot/activeElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`, `ShadowRoot`*"]
impl ShadowRoot {
    #[js::import = "$0.activeElement"]
pub fn active_element(&self) -> Option<Element> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Element")]
    #[doc = "Getter for the `pointerLockElement` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ShadowRoot/pointerLockElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`, `ShadowRoot`*"]
impl ShadowRoot {
    #[js::import = "$0.pointerLockElement"]
pub fn pointer_lock_element(&self) -> Option<Element> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Element")]
    #[doc = "Getter for the `fullscreenElement` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ShadowRoot/fullscreenElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`, `ShadowRoot`*"]
impl ShadowRoot {
    #[js::import = "$0.fullscreenElement"]
pub fn fullscreen_element(&self) -> Option<Element> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "StyleSheetList")]
    #[doc = "Getter for the `styleSheets` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ShadowRoot/styleSheets)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ShadowRoot`, `StyleSheetList`*"]
impl ShadowRoot {
    #[js::import = "$0.styleSheets"]
pub fn style_sheets(&self) -> StyleSheetList { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `adoptedStyleSheets` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ShadowRoot/adoptedStyleSheets)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ShadowRoot`*"]
impl ShadowRoot {
    #[js::import = "$0.adoptedStyleSheets"]
pub fn adopted_style_sheets(&self) -> ::js_sys::Array { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `adoptedStyleSheets` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ShadowRoot/adoptedStyleSheets)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ShadowRoot`*"]
impl ShadowRoot {
    #[js::import = "$0.adoptedStyleSheets = $1"]
pub fn set_adopted_style_sheets(&self, value: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "Element")]
    #[doc = "Getter for the `pictureInPictureElement` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ShadowRoot/pictureInPictureElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`, `ShadowRoot`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl ShadowRoot {
    #[js::import = "$0.pictureInPictureElement"]
pub fn picture_in_picture_element(&self) -> Option<Element> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Element")]
    #[doc = "The `getElementById()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ShadowRoot/getElementById)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`, `ShadowRoot`*"]
impl ShadowRoot {
    #[js::import = "$0.getElementById($1)"]
pub fn get_element_by_id(&self, element_id: &str) -> Option<Element> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "HtmlCollection")]
    #[doc = "The `getElementsByClassName()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ShadowRoot/getElementsByClassName)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlCollection`, `ShadowRoot`*"]
impl ShadowRoot {
    #[js::import = "$0.getElementsByClassName($1)"]
pub fn get_elements_by_class_name(&self, class_names: &str) -> HtmlCollection { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "HtmlCollection")]
    #[doc = "The `getElementsByTagName()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ShadowRoot/getElementsByTagName)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlCollection`, `ShadowRoot`*"]
impl ShadowRoot {
    #[js::import = "$0.getElementsByTagName($1)"]
pub fn get_elements_by_tag_name(&self, local_name: &str) -> HtmlCollection { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "HtmlCollection")]
    #[doc = "The `getElementsByTagNameNS()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ShadowRoot/getElementsByTagNameNS)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlCollection`, `ShadowRoot`*"]
impl ShadowRoot {
    #[js::import = "$0.getElementsByTagNameNS($1, $2)"]
pub fn get_elements_by_tag_name_ns(&self, namespace: Option<&str>, local_name: &str) -> HtmlCollection { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Element")]
    #[doc = "The `elementFromPoint()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ShadowRoot/elementFromPoint)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`, `ShadowRoot`*"]
impl ShadowRoot {
    #[js::import = "$0.elementFromPoint($1, $2)"]
pub fn element_from_point(&self, x: f32, y: f32) -> Option<Element> { panic!("web-sys intrinsic") }
}
    #[doc = "The `elementsFromPoint()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ShadowRoot/elementsFromPoint)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ShadowRoot`*"]
impl ShadowRoot {
    #[js::import = "$0.elementsFromPoint($1, $2)"]
pub fn elements_from_point(&self, x: f32, y: f32) -> ::js_sys::Array { panic!("web-sys intrinsic") }
}
