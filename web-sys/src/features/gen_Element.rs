#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Element` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
#[js::opaque]
#[derive(Copy)]
pub struct Element(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `namespaceURI` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/namespaceURI)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.namespaceURI"]
pub fn namespace_uri(&self) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `prefix` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/prefix)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.prefix"]
pub fn prefix(&self) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `localName` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/localName)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.localName"]
pub fn local_name(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `tagName` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/tagName)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.tagName"]
pub fn tag_name(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `id` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/id)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.id"]
pub fn id(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `id` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/id)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.id = $1"]
pub fn set_id(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `className` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/className)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.className"]
pub fn class_name(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `className` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/className)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.className = $1"]
pub fn set_class_name(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "DomTokenList")]
    #[doc = "Getter for the `classList` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/classList)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomTokenList`, `Element`*"]
impl Element {
    #[js::import = "$0.classList"]
pub fn class_list(&self) -> DomTokenList { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "NamedNodeMap")]
    #[doc = "Getter for the `attributes` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/attributes)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`, `NamedNodeMap`*"]
impl Element {
    #[js::import = "$0.attributes"]
pub fn attributes(&self) -> NamedNodeMap { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `scrollTop` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/scrollTop)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.scrollTop"]
pub fn scroll_top(&self) -> i32 { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `scrollTop` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/scrollTop)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.scrollTop = $1"]
pub fn set_scroll_top(&self, value: i32) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `scrollLeft` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/scrollLeft)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.scrollLeft"]
pub fn scroll_left(&self) -> i32 { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `scrollLeft` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/scrollLeft)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.scrollLeft = $1"]
pub fn set_scroll_left(&self, value: i32) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `scrollWidth` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/scrollWidth)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.scrollWidth"]
pub fn scroll_width(&self) -> i32 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `scrollHeight` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/scrollHeight)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.scrollHeight"]
pub fn scroll_height(&self) -> i32 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `clientTop` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/clientTop)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.clientTop"]
pub fn client_top(&self) -> i32 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `clientLeft` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/clientLeft)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.clientLeft"]
pub fn client_left(&self) -> i32 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `clientWidth` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/clientWidth)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.clientWidth"]
pub fn client_width(&self) -> i32 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `clientHeight` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/clientHeight)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.clientHeight"]
pub fn client_height(&self) -> i32 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `innerHTML` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/innerHTML)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.innerHTML"]
pub fn inner_html(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `innerHTML` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/innerHTML)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.innerHTML = $1"]
pub fn set_inner_html(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `outerHTML` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/outerHTML)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.outerHTML"]
pub fn outer_html(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `outerHTML` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/outerHTML)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.outerHTML = $1"]
pub fn set_outer_html(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "ShadowRoot")]
    #[doc = "Getter for the `shadowRoot` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/shadowRoot)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`, `ShadowRoot`*"]
impl Element {
    #[js::import = "$0.shadowRoot"]
pub fn shadow_root(&self) -> Option<ShadowRoot> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "HtmlSlotElement")]
    #[doc = "Getter for the `assignedSlot` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/assignedSlot)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`, `HtmlSlotElement`*"]
impl Element {
    #[js::import = "$0.assignedSlot"]
pub fn assigned_slot(&self) -> Option<HtmlSlotElement> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `slot` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/slot)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.slot"]
pub fn slot(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `slot` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/slot)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.slot = $1"]
pub fn set_slot(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `previousElementSibling` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/previousElementSibling)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.previousElementSibling"]
pub fn previous_element_sibling(&self) -> Option<Element> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `nextElementSibling` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/nextElementSibling)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.nextElementSibling"]
pub fn next_element_sibling(&self) -> Option<Element> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "HtmlCollection")]
    #[doc = "Getter for the `children` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/children)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`, `HtmlCollection`*"]
impl Element {
    #[js::import = "$0.children"]
pub fn children(&self) -> HtmlCollection { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `firstElementChild` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/firstElementChild)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.firstElementChild"]
pub fn first_element_child(&self) -> Option<Element> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `lastElementChild` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/lastElementChild)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.lastElementChild"]
pub fn last_element_child(&self) -> Option<Element> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `childElementCount` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/childElementCount)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.childElementCount"]
pub fn child_element_count(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "ShadowRoot", feature = "ShadowRootInit",))]
    #[doc = "The `attachShadow()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/attachShadow)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`, `ShadowRoot`, `ShadowRootInit`*"]
impl Element {
    #[js::import = "$0.attachShadow($1)"]
pub fn attach_shadow(&self, shadow_root_init_dict: &ShadowRootInit) -> Result<ShadowRoot, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `closest()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/closest)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.closest($1)"]
pub fn closest(&self, selector: &str) -> Result<Option<Element>, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `getAttribute()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/getAttribute)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.getAttribute($1)"]
pub fn get_attribute(&self, name: &str) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
}
    #[doc = "The `getAttributeNS()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/getAttributeNS)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.getAttributeNS($1, $2)"]
pub fn get_attribute_ns(&self, namespace: Option<&str>, local_name: &str) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
}
    #[doc = "The `getAttributeNames()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/getAttributeNames)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.getAttributeNames()"]
pub fn get_attribute_names(&self) -> ::js_sys::Array { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Attr")]
    #[doc = "The `getAttributeNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/getAttributeNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Attr`, `Element`*"]
impl Element {
    #[js::import = "$0.getAttributeNode($1)"]
pub fn get_attribute_node(&self, name: &str) -> Option<Attr> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Attr")]
    #[doc = "The `getAttributeNodeNS()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/getAttributeNodeNS)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Attr`, `Element`*"]
impl Element {
    #[js::import = "$0.getAttributeNodeNS($1, $2)"]
pub fn get_attribute_node_ns(&self, namespace_uri: Option<&str>, local_name: &str) -> Option<Attr> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "DomRect")]
    #[doc = "The `getBoundingClientRect()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/getBoundingClientRect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRect`, `Element`*"]
impl Element {
    #[js::import = "$0.getBoundingClientRect()"]
pub fn get_bounding_client_rect(&self) -> DomRect { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "DomRectList")]
    #[doc = "The `getClientRects()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/getClientRects)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRectList`, `Element`*"]
impl Element {
    #[js::import = "$0.getClientRects()"]
pub fn get_client_rects(&self) -> DomRectList { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "HtmlCollection")]
    #[doc = "The `getElementsByClassName()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/getElementsByClassName)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`, `HtmlCollection`*"]
impl Element {
    #[js::import = "$0.getElementsByClassName($1)"]
pub fn get_elements_by_class_name(&self, class_names: &str) -> HtmlCollection { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "HtmlCollection")]
    #[doc = "The `getElementsByTagName()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/getElementsByTagName)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`, `HtmlCollection`*"]
impl Element {
    #[js::import = "$0.getElementsByTagName($1)"]
pub fn get_elements_by_tag_name(&self, local_name: &str) -> HtmlCollection { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "HtmlCollection")]
    #[doc = "The `getElementsByTagNameNS()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/getElementsByTagNameNS)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`, `HtmlCollection`*"]
impl Element {
    #[js::import = "$0.getElementsByTagNameNS($1, $2)"]
pub fn get_elements_by_tag_name_ns(&self, namespace: Option<&str>, local_name: &str) -> Result<HtmlCollection, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `hasAttribute()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/hasAttribute)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.hasAttribute($1)"]
pub fn has_attribute(&self, name: &str) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "The `hasAttributeNS()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/hasAttributeNS)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.hasAttributeNS($1, $2)"]
pub fn has_attribute_ns(&self, namespace: Option<&str>, local_name: &str) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "The `hasAttributes()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/hasAttributes)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.hasAttributes()"]
pub fn has_attributes(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "The `hasPointerCapture()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/hasPointerCapture)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.hasPointerCapture($1)"]
pub fn has_pointer_capture(&self, pointer_id: i32) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "The `insertAdjacentElement()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/insertAdjacentElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.insertAdjacentElement($1, $2)"]
pub fn insert_adjacent_element(&self, where_: &str, element: &Element) -> Result<Option<Element>, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `insertAdjacentHTML()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/insertAdjacentHTML)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.insertAdjacentHTML($1, $2)"]
pub fn insert_adjacent_html(&self, position: &str, text: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `insertAdjacentText()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/insertAdjacentText)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.insertAdjacentText($1, $2)"]
pub fn insert_adjacent_text(&self, where_: &str, data: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `matches()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/matches)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.matches($1)"]
pub fn matches(&self, selector: &str) -> Result<bool, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `querySelector()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/querySelector)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.querySelector($1)"]
pub fn query_selector(&self, selectors: &str) -> Result<Option<Element>, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "NodeList")]
    #[doc = "The `querySelectorAll()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/querySelectorAll)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`, `NodeList`*"]
impl Element {
    #[js::import = "$0.querySelectorAll($1)"]
pub fn query_selector_all(&self, selectors: &str) -> Result<NodeList, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `releaseCapture()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/releaseCapture)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.releaseCapture()"]
pub fn release_capture(&self) { panic!("web-sys intrinsic") }
}
    #[doc = "The `releasePointerCapture()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/releasePointerCapture)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.releasePointerCapture($1)"]
pub fn release_pointer_capture(&self, pointer_id: i32) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `removeAttribute()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/removeAttribute)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.removeAttribute($1)"]
pub fn remove_attribute(&self, name: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `removeAttributeNS()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/removeAttributeNS)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.removeAttributeNS($1, $2)"]
pub fn remove_attribute_ns(&self, namespace: Option<&str>, local_name: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Attr")]
    #[doc = "The `removeAttributeNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/removeAttributeNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Attr`, `Element`*"]
impl Element {
    #[js::import = "$0.removeAttributeNode($1)"]
pub fn remove_attribute_node(&self, old_attr: &Attr) -> Result<Option<Attr>, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `requestFullscreen()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/requestFullscreen)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.requestFullscreen()"]
pub fn request_fullscreen(&self) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `requestPointerLock()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/requestPointerLock)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.requestPointerLock()"]
pub fn request_pointer_lock(&self) { panic!("web-sys intrinsic") }
}
    #[doc = "The `scroll()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/scroll)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.scroll($1, $2)"]
pub fn scroll_with_x_and_y(&self, x: f64, y: f64) { panic!("web-sys intrinsic") }
}
    #[doc = "The `scroll()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/scroll)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.scroll()"]
pub fn scroll(&self) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "ScrollToOptions")]
    #[doc = "The `scroll()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/scroll)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`, `ScrollToOptions`*"]
impl Element {
    #[js::import = "$0.scroll($1)"]
pub fn scroll_with_scroll_to_options(&self, options: &ScrollToOptions) { panic!("web-sys intrinsic") }
}
    #[doc = "The `scrollBy()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/scrollBy)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.scrollBy($1, $2)"]
pub fn scroll_by_with_x_and_y(&self, x: f64, y: f64) { panic!("web-sys intrinsic") }
}
    #[doc = "The `scrollBy()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/scrollBy)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.scrollBy()"]
pub fn scroll_by(&self) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "ScrollToOptions")]
    #[doc = "The `scrollBy()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/scrollBy)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`, `ScrollToOptions`*"]
impl Element {
    #[js::import = "$0.scrollBy($1)"]
pub fn scroll_by_with_scroll_to_options(&self, options: &ScrollToOptions) { panic!("web-sys intrinsic") }
}
    #[doc = "The `scrollIntoView()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/scrollIntoView)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.scrollIntoView()"]
pub fn scroll_into_view(&self) { panic!("web-sys intrinsic") }
}
    #[doc = "The `scrollIntoView()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/scrollIntoView)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.scrollIntoView($1)"]
pub fn scroll_into_view_with_bool(&self, arg: bool) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "ScrollIntoViewOptions")]
    #[doc = "The `scrollIntoView()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/scrollIntoView)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`, `ScrollIntoViewOptions`*"]
impl Element {
    #[js::import = "$0.scrollIntoView($1)"]
pub fn scroll_into_view_with_scroll_into_view_options(&self, arg: &ScrollIntoViewOptions) { panic!("web-sys intrinsic") }
}
    #[doc = "The `scrollTo()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/scrollTo)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.scrollTo($1, $2)"]
pub fn scroll_to_with_x_and_y(&self, x: f64, y: f64) { panic!("web-sys intrinsic") }
}
    #[doc = "The `scrollTo()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/scrollTo)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.scrollTo()"]
pub fn scroll_to(&self) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "ScrollToOptions")]
    #[doc = "The `scrollTo()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/scrollTo)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`, `ScrollToOptions`*"]
impl Element {
    #[js::import = "$0.scrollTo($1)"]
pub fn scroll_to_with_scroll_to_options(&self, options: &ScrollToOptions) { panic!("web-sys intrinsic") }
}
    #[doc = "The `setAttribute()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/setAttribute)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.setAttribute($1, $2)"]
pub fn set_attribute(&self, name: &str, value: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `setAttributeNS()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/setAttributeNS)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.setAttributeNS($1, $2, $3)"]
pub fn set_attribute_ns(&self, namespace: Option<&str>, name: &str, value: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Attr")]
    #[doc = "The `setAttributeNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/setAttributeNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Attr`, `Element`*"]
impl Element {
    #[js::import = "$0.setAttributeNode($1)"]
pub fn set_attribute_node(&self, new_attr: &Attr) -> Result<Option<Attr>, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Attr")]
    #[doc = "The `setAttributeNodeNS()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/setAttributeNodeNS)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Attr`, `Element`*"]
impl Element {
    #[js::import = "$0.setAttributeNodeNS($1)"]
pub fn set_attribute_node_ns(&self, new_attr: &Attr) -> Result<Option<Attr>, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `setCapture()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/setCapture)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.setCapture()"]
pub fn set_capture(&self) { panic!("web-sys intrinsic") }
}
    #[doc = "The `setCapture()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/setCapture)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.setCapture($1)"]
pub fn set_capture_with_retarget_to_element(&self, retarget_to_element: bool) { panic!("web-sys intrinsic") }
}
    #[doc = "The `setPointerCapture()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/setPointerCapture)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.setPointerCapture($1)"]
pub fn set_pointer_capture(&self, pointer_id: i32) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `toggleAttribute()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/toggleAttribute)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.toggleAttribute($1)"]
pub fn toggle_attribute(&self, name: &str) -> Result<bool, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `toggleAttribute()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/toggleAttribute)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.toggleAttribute($1, $2)"]
pub fn toggle_attribute_with_force(&self, name: &str, force: bool) -> Result<bool, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `webkitMatchesSelector()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/webkitMatchesSelector)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.webkitMatchesSelector($1)"]
pub fn webkit_matches_selector(&self, selector: &str) -> Result<bool, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "Animation")]
    #[doc = "The `animate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/animate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Animation`, `Element`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl Element {
    #[js::import = "$0.animate($1)"]
pub fn animate(&self, keyframes: Option<&::js_sys::Object>) -> Animation { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "Animation")]
    #[doc = "The `animate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/animate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Animation`, `Element`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl Element {
    #[js::import = "$0.animate($1, $2)"]
pub fn animate_with_f64(&self, keyframes: Option<&::js_sys::Object>, options: f64) -> Animation { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(feature = "Animation", feature = "KeyframeAnimationOptions",))]
    #[doc = "The `animate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/animate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Animation`, `Element`, `KeyframeAnimationOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl Element {
    #[js::import = "$0.animate($1, $2)"]
pub fn animate_with_keyframe_animation_options(&self, keyframes: Option<&::js_sys::Object>, options: &KeyframeAnimationOptions) -> Animation { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `getAnimations()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/getAnimations)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl Element {
    #[js::import = "$0.getAnimations()"]
pub fn get_animations(&self) -> ::js_sys::Array { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GetAnimationsOptions")]
    #[doc = "The `getAnimations()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/getAnimations)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`, `GetAnimationsOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl Element {
    #[js::import = "$0.getAnimations($1)"]
pub fn get_animations_with_options(&self, options: &GetAnimationsOptions) -> ::js_sys::Array { panic!("web-sys intrinsic") }
}
    #[doc = "The `after()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/after)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.after($1)"]
pub fn after_with_node(&self, nodes: &::js_sys::Array) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `after()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/after)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.after()"]
pub fn after_with_node_0(&self) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `after()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/after)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.after($1)"]
pub fn after_with_node_1(&self, nodes_1: &Node) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `after()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/after)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.after($1, $2)"]
pub fn after_with_node_2(&self, nodes_1: &Node, nodes_2: &Node) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `after()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/after)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.after($1, $2, $3)"]
pub fn after_with_node_3(&self, nodes_1: &Node, nodes_2: &Node, nodes_3: &Node) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `after()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/after)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.after($1, $2, $3, $4)"]
pub fn after_with_node_4(&self, nodes_1: &Node, nodes_2: &Node, nodes_3: &Node, nodes_4: &Node) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `after()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/after)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.after($1, $2, $3, $4, $5)"]
pub fn after_with_node_5(&self, nodes_1: &Node, nodes_2: &Node, nodes_3: &Node, nodes_4: &Node, nodes_5: &Node) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `after()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/after)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.after($1, $2, $3, $4, $5, $6)"]
pub fn after_with_node_6(&self, nodes_1: &Node, nodes_2: &Node, nodes_3: &Node, nodes_4: &Node, nodes_5: &Node, nodes_6: &Node) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `after()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/after)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.after($1, $2, $3, $4, $5, $6, $7)"]
pub fn after_with_node_7(&self, nodes_1: &Node, nodes_2: &Node, nodes_3: &Node, nodes_4: &Node, nodes_5: &Node, nodes_6: &Node, nodes_7: &Node) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `after()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/after)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.after($1)"]
pub fn after_with_str(&self, nodes: &::js_sys::Array) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `after()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/after)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.after()"]
pub fn after_with_str_0(&self) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `after()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/after)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.after($1)"]
pub fn after_with_str_1(&self, nodes_1: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `after()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/after)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.after($1, $2)"]
pub fn after_with_str_2(&self, nodes_1: &str, nodes_2: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `after()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/after)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.after($1, $2, $3)"]
pub fn after_with_str_3(&self, nodes_1: &str, nodes_2: &str, nodes_3: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `after()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/after)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.after($1, $2, $3, $4)"]
pub fn after_with_str_4(&self, nodes_1: &str, nodes_2: &str, nodes_3: &str, nodes_4: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `after()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/after)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.after($1, $2, $3, $4, $5)"]
pub fn after_with_str_5(&self, nodes_1: &str, nodes_2: &str, nodes_3: &str, nodes_4: &str, nodes_5: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `after()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/after)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.after($1, $2, $3, $4, $5, $6)"]
pub fn after_with_str_6(&self, nodes_1: &str, nodes_2: &str, nodes_3: &str, nodes_4: &str, nodes_5: &str, nodes_6: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `after()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/after)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.after($1, $2, $3, $4, $5, $6, $7)"]
pub fn after_with_str_7(&self, nodes_1: &str, nodes_2: &str, nodes_3: &str, nodes_4: &str, nodes_5: &str, nodes_6: &str, nodes_7: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `before()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/before)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.before($1)"]
pub fn before_with_node(&self, nodes: &::js_sys::Array) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `before()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/before)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.before()"]
pub fn before_with_node_0(&self) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `before()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/before)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.before($1)"]
pub fn before_with_node_1(&self, nodes_1: &Node) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `before()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/before)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.before($1, $2)"]
pub fn before_with_node_2(&self, nodes_1: &Node, nodes_2: &Node) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `before()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/before)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.before($1, $2, $3)"]
pub fn before_with_node_3(&self, nodes_1: &Node, nodes_2: &Node, nodes_3: &Node) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `before()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/before)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.before($1, $2, $3, $4)"]
pub fn before_with_node_4(&self, nodes_1: &Node, nodes_2: &Node, nodes_3: &Node, nodes_4: &Node) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `before()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/before)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.before($1, $2, $3, $4, $5)"]
pub fn before_with_node_5(&self, nodes_1: &Node, nodes_2: &Node, nodes_3: &Node, nodes_4: &Node, nodes_5: &Node) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `before()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/before)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.before($1, $2, $3, $4, $5, $6)"]
pub fn before_with_node_6(&self, nodes_1: &Node, nodes_2: &Node, nodes_3: &Node, nodes_4: &Node, nodes_5: &Node, nodes_6: &Node) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `before()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/before)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.before($1, $2, $3, $4, $5, $6, $7)"]
pub fn before_with_node_7(&self, nodes_1: &Node, nodes_2: &Node, nodes_3: &Node, nodes_4: &Node, nodes_5: &Node, nodes_6: &Node, nodes_7: &Node) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `before()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/before)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.before($1)"]
pub fn before_with_str(&self, nodes: &::js_sys::Array) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `before()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/before)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.before()"]
pub fn before_with_str_0(&self) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `before()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/before)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.before($1)"]
pub fn before_with_str_1(&self, nodes_1: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `before()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/before)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.before($1, $2)"]
pub fn before_with_str_2(&self, nodes_1: &str, nodes_2: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `before()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/before)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.before($1, $2, $3)"]
pub fn before_with_str_3(&self, nodes_1: &str, nodes_2: &str, nodes_3: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `before()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/before)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.before($1, $2, $3, $4)"]
pub fn before_with_str_4(&self, nodes_1: &str, nodes_2: &str, nodes_3: &str, nodes_4: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `before()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/before)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.before($1, $2, $3, $4, $5)"]
pub fn before_with_str_5(&self, nodes_1: &str, nodes_2: &str, nodes_3: &str, nodes_4: &str, nodes_5: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `before()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/before)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.before($1, $2, $3, $4, $5, $6)"]
pub fn before_with_str_6(&self, nodes_1: &str, nodes_2: &str, nodes_3: &str, nodes_4: &str, nodes_5: &str, nodes_6: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `before()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/before)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.before($1, $2, $3, $4, $5, $6, $7)"]
pub fn before_with_str_7(&self, nodes_1: &str, nodes_2: &str, nodes_3: &str, nodes_4: &str, nodes_5: &str, nodes_6: &str, nodes_7: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `remove()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/remove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.remove()"]
pub fn remove(&self) { panic!("web-sys intrinsic") }
}
    #[doc = "The `replaceWith()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceWith)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.replaceWith($1)"]
pub fn replace_with_with_node(&self, nodes: &::js_sys::Array) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `replaceWith()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceWith)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.replaceWith()"]
pub fn replace_with_with_node_0(&self) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `replaceWith()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceWith)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.replaceWith($1)"]
pub fn replace_with_with_node_1(&self, nodes_1: &Node) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `replaceWith()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceWith)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.replaceWith($1, $2)"]
pub fn replace_with_with_node_2(&self, nodes_1: &Node, nodes_2: &Node) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `replaceWith()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceWith)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.replaceWith($1, $2, $3)"]
pub fn replace_with_with_node_3(&self, nodes_1: &Node, nodes_2: &Node, nodes_3: &Node) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `replaceWith()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceWith)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.replaceWith($1, $2, $3, $4)"]
pub fn replace_with_with_node_4(&self, nodes_1: &Node, nodes_2: &Node, nodes_3: &Node, nodes_4: &Node) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `replaceWith()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceWith)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.replaceWith($1, $2, $3, $4, $5)"]
pub fn replace_with_with_node_5(&self, nodes_1: &Node, nodes_2: &Node, nodes_3: &Node, nodes_4: &Node, nodes_5: &Node) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `replaceWith()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceWith)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.replaceWith($1, $2, $3, $4, $5, $6)"]
pub fn replace_with_with_node_6(&self, nodes_1: &Node, nodes_2: &Node, nodes_3: &Node, nodes_4: &Node, nodes_5: &Node, nodes_6: &Node) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `replaceWith()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceWith)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.replaceWith($1, $2, $3, $4, $5, $6, $7)"]
pub fn replace_with_with_node_7(&self, nodes_1: &Node, nodes_2: &Node, nodes_3: &Node, nodes_4: &Node, nodes_5: &Node, nodes_6: &Node, nodes_7: &Node) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `replaceWith()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceWith)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.replaceWith($1)"]
pub fn replace_with_with_str(&self, nodes: &::js_sys::Array) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `replaceWith()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceWith)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.replaceWith()"]
pub fn replace_with_with_str_0(&self) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `replaceWith()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceWith)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.replaceWith($1)"]
pub fn replace_with_with_str_1(&self, nodes_1: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `replaceWith()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceWith)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.replaceWith($1, $2)"]
pub fn replace_with_with_str_2(&self, nodes_1: &str, nodes_2: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `replaceWith()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceWith)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.replaceWith($1, $2, $3)"]
pub fn replace_with_with_str_3(&self, nodes_1: &str, nodes_2: &str, nodes_3: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `replaceWith()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceWith)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.replaceWith($1, $2, $3, $4)"]
pub fn replace_with_with_str_4(&self, nodes_1: &str, nodes_2: &str, nodes_3: &str, nodes_4: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `replaceWith()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceWith)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.replaceWith($1, $2, $3, $4, $5)"]
pub fn replace_with_with_str_5(&self, nodes_1: &str, nodes_2: &str, nodes_3: &str, nodes_4: &str, nodes_5: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `replaceWith()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceWith)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.replaceWith($1, $2, $3, $4, $5, $6)"]
pub fn replace_with_with_str_6(&self, nodes_1: &str, nodes_2: &str, nodes_3: &str, nodes_4: &str, nodes_5: &str, nodes_6: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `replaceWith()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceWith)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.replaceWith($1, $2, $3, $4, $5, $6, $7)"]
pub fn replace_with_with_str_7(&self, nodes_1: &str, nodes_2: &str, nodes_3: &str, nodes_4: &str, nodes_5: &str, nodes_6: &str, nodes_7: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "DomPoint", feature = "DomPointInit", feature = "Text",))]
    #[doc = "The `convertPointFromNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/convertPointFromNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPoint`, `DomPointInit`, `Element`, `Text`*"]
impl Element {
    #[js::import = "$0.convertPointFromNode($1, $2)"]
pub fn convert_point_from_node_with_text(&self, point: &DomPointInit, from: &Text) -> Result<DomPoint, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "DomPoint", feature = "DomPointInit",))]
    #[doc = "The `convertPointFromNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/convertPointFromNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPoint`, `DomPointInit`, `Element`*"]
impl Element {
    #[js::import = "$0.convertPointFromNode($1, $2)"]
pub fn convert_point_from_node_with_element(&self, point: &DomPointInit, from: &Element) -> Result<DomPoint, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "Document", feature = "DomPoint", feature = "DomPointInit",))]
    #[doc = "The `convertPointFromNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/convertPointFromNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `DomPoint`, `DomPointInit`, `Element`*"]
impl Element {
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
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/convertPointFromNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `DomPoint`, `DomPointInit`, `Element`, `Text`*"]
impl Element {
    #[js::import = "$0.convertPointFromNode($1, $2, $3)"]
pub fn convert_point_from_node_with_text_and_options(&self, point: &DomPointInit, from: &Text, options: &ConvertCoordinateOptions) -> Result<DomPoint, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(
        feature = "ConvertCoordinateOptions",
        feature = "DomPoint",
        feature = "DomPointInit",
    ))]
    #[doc = "The `convertPointFromNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/convertPointFromNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `DomPoint`, `DomPointInit`, `Element`*"]
impl Element {
    #[js::import = "$0.convertPointFromNode($1, $2, $3)"]
pub fn convert_point_from_node_with_element_and_options(&self, point: &DomPointInit, from: &Element, options: &ConvertCoordinateOptions) -> Result<DomPoint, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(
        feature = "ConvertCoordinateOptions",
        feature = "Document",
        feature = "DomPoint",
        feature = "DomPointInit",
    ))]
    #[doc = "The `convertPointFromNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/convertPointFromNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `Document`, `DomPoint`, `DomPointInit`, `Element`*"]
impl Element {
    #[js::import = "$0.convertPointFromNode($1, $2, $3)"]
pub fn convert_point_from_node_with_document_and_options(&self, point: &DomPointInit, from: &Document, options: &ConvertCoordinateOptions) -> Result<DomPoint, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "DomQuad", feature = "Text",))]
    #[doc = "The `convertQuadFromNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/convertQuadFromNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomQuad`, `Element`, `Text`*"]
impl Element {
    #[js::import = "$0.convertQuadFromNode($1, $2)"]
pub fn convert_quad_from_node_with_text(&self, quad: &DomQuad, from: &Text) -> Result<DomQuad, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "DomQuad")]
    #[doc = "The `convertQuadFromNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/convertQuadFromNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomQuad`, `Element`*"]
impl Element {
    #[js::import = "$0.convertQuadFromNode($1, $2)"]
pub fn convert_quad_from_node_with_element(&self, quad: &DomQuad, from: &Element) -> Result<DomQuad, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "Document", feature = "DomQuad",))]
    #[doc = "The `convertQuadFromNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/convertQuadFromNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `DomQuad`, `Element`*"]
impl Element {
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
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/convertQuadFromNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `DomQuad`, `Element`, `Text`*"]
impl Element {
    #[js::import = "$0.convertQuadFromNode($1, $2, $3)"]
pub fn convert_quad_from_node_with_text_and_options(&self, quad: &DomQuad, from: &Text, options: &ConvertCoordinateOptions) -> Result<DomQuad, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "ConvertCoordinateOptions", feature = "DomQuad",))]
    #[doc = "The `convertQuadFromNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/convertQuadFromNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `DomQuad`, `Element`*"]
impl Element {
    #[js::import = "$0.convertQuadFromNode($1, $2, $3)"]
pub fn convert_quad_from_node_with_element_and_options(&self, quad: &DomQuad, from: &Element, options: &ConvertCoordinateOptions) -> Result<DomQuad, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(
        feature = "ConvertCoordinateOptions",
        feature = "Document",
        feature = "DomQuad",
    ))]
    #[doc = "The `convertQuadFromNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/convertQuadFromNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `Document`, `DomQuad`, `Element`*"]
impl Element {
    #[js::import = "$0.convertQuadFromNode($1, $2, $3)"]
pub fn convert_quad_from_node_with_document_and_options(&self, quad: &DomQuad, from: &Document, options: &ConvertCoordinateOptions) -> Result<DomQuad, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "DomQuad", feature = "DomRectReadOnly", feature = "Text",))]
    #[doc = "The `convertRectFromNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/convertRectFromNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomQuad`, `DomRectReadOnly`, `Element`, `Text`*"]
impl Element {
    #[js::import = "$0.convertRectFromNode($1, $2)"]
pub fn convert_rect_from_node_with_text(&self, rect: &DomRectReadOnly, from: &Text) -> Result<DomQuad, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "DomQuad", feature = "DomRectReadOnly",))]
    #[doc = "The `convertRectFromNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/convertRectFromNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomQuad`, `DomRectReadOnly`, `Element`*"]
impl Element {
    #[js::import = "$0.convertRectFromNode($1, $2)"]
pub fn convert_rect_from_node_with_element(&self, rect: &DomRectReadOnly, from: &Element) -> Result<DomQuad, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "Document", feature = "DomQuad", feature = "DomRectReadOnly",))]
    #[doc = "The `convertRectFromNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/convertRectFromNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `DomQuad`, `DomRectReadOnly`, `Element`*"]
impl Element {
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
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/convertRectFromNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `DomQuad`, `DomRectReadOnly`, `Element`, `Text`*"]
impl Element {
    #[js::import = "$0.convertRectFromNode($1, $2, $3)"]
pub fn convert_rect_from_node_with_text_and_options(&self, rect: &DomRectReadOnly, from: &Text, options: &ConvertCoordinateOptions) -> Result<DomQuad, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(
        feature = "ConvertCoordinateOptions",
        feature = "DomQuad",
        feature = "DomRectReadOnly",
    ))]
    #[doc = "The `convertRectFromNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/convertRectFromNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `DomQuad`, `DomRectReadOnly`, `Element`*"]
impl Element {
    #[js::import = "$0.convertRectFromNode($1, $2, $3)"]
pub fn convert_rect_from_node_with_element_and_options(&self, rect: &DomRectReadOnly, from: &Element, options: &ConvertCoordinateOptions) -> Result<DomQuad, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(
        feature = "ConvertCoordinateOptions",
        feature = "Document",
        feature = "DomQuad",
        feature = "DomRectReadOnly",
    ))]
    #[doc = "The `convertRectFromNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/convertRectFromNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `Document`, `DomQuad`, `DomRectReadOnly`, `Element`*"]
impl Element {
    #[js::import = "$0.convertRectFromNode($1, $2, $3)"]
pub fn convert_rect_from_node_with_document_and_options(&self, rect: &DomRectReadOnly, from: &Document, options: &ConvertCoordinateOptions) -> Result<DomQuad, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `getBoxQuads()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/getBoxQuads)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.getBoxQuads()"]
pub fn get_box_quads(&self) -> Result<::js_sys::Array, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "BoxQuadOptions")]
    #[doc = "The `getBoxQuads()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/getBoxQuads)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BoxQuadOptions`, `Element`*"]
impl Element {
    #[js::import = "$0.getBoxQuads($1)"]
pub fn get_box_quads_with_options(&self, options: &BoxQuadOptions) -> Result<::js_sys::Array, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.append($1)"]
pub fn append_with_node(&self, nodes: &::js_sys::Array) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.append()"]
pub fn append_with_node_0(&self) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.append($1)"]
pub fn append_with_node_1(&self, nodes_1: &Node) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.append($1, $2)"]
pub fn append_with_node_2(&self, nodes_1: &Node, nodes_2: &Node) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.append($1, $2, $3)"]
pub fn append_with_node_3(&self, nodes_1: &Node, nodes_2: &Node, nodes_3: &Node) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.append($1, $2, $3, $4)"]
pub fn append_with_node_4(&self, nodes_1: &Node, nodes_2: &Node, nodes_3: &Node, nodes_4: &Node) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.append($1, $2, $3, $4, $5)"]
pub fn append_with_node_5(&self, nodes_1: &Node, nodes_2: &Node, nodes_3: &Node, nodes_4: &Node, nodes_5: &Node) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.append($1, $2, $3, $4, $5, $6)"]
pub fn append_with_node_6(&self, nodes_1: &Node, nodes_2: &Node, nodes_3: &Node, nodes_4: &Node, nodes_5: &Node, nodes_6: &Node) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.append($1, $2, $3, $4, $5, $6, $7)"]
pub fn append_with_node_7(&self, nodes_1: &Node, nodes_2: &Node, nodes_3: &Node, nodes_4: &Node, nodes_5: &Node, nodes_6: &Node, nodes_7: &Node) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.append($1)"]
pub fn append_with_str(&self, nodes: &::js_sys::Array) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.append()"]
pub fn append_with_str_0(&self) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.append($1)"]
pub fn append_with_str_1(&self, nodes_1: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.append($1, $2)"]
pub fn append_with_str_2(&self, nodes_1: &str, nodes_2: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.append($1, $2, $3)"]
pub fn append_with_str_3(&self, nodes_1: &str, nodes_2: &str, nodes_3: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.append($1, $2, $3, $4)"]
pub fn append_with_str_4(&self, nodes_1: &str, nodes_2: &str, nodes_3: &str, nodes_4: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.append($1, $2, $3, $4, $5)"]
pub fn append_with_str_5(&self, nodes_1: &str, nodes_2: &str, nodes_3: &str, nodes_4: &str, nodes_5: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.append($1, $2, $3, $4, $5, $6)"]
pub fn append_with_str_6(&self, nodes_1: &str, nodes_2: &str, nodes_3: &str, nodes_4: &str, nodes_5: &str, nodes_6: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.append($1, $2, $3, $4, $5, $6, $7)"]
pub fn append_with_str_7(&self, nodes_1: &str, nodes_2: &str, nodes_3: &str, nodes_4: &str, nodes_5: &str, nodes_6: &str, nodes_7: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `prepend()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/prepend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.prepend($1)"]
pub fn prepend_with_node(&self, nodes: &::js_sys::Array) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `prepend()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/prepend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.prepend()"]
pub fn prepend_with_node_0(&self) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `prepend()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/prepend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.prepend($1)"]
pub fn prepend_with_node_1(&self, nodes_1: &Node) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `prepend()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/prepend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.prepend($1, $2)"]
pub fn prepend_with_node_2(&self, nodes_1: &Node, nodes_2: &Node) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `prepend()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/prepend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.prepend($1, $2, $3)"]
pub fn prepend_with_node_3(&self, nodes_1: &Node, nodes_2: &Node, nodes_3: &Node) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `prepend()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/prepend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.prepend($1, $2, $3, $4)"]
pub fn prepend_with_node_4(&self, nodes_1: &Node, nodes_2: &Node, nodes_3: &Node, nodes_4: &Node) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `prepend()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/prepend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.prepend($1, $2, $3, $4, $5)"]
pub fn prepend_with_node_5(&self, nodes_1: &Node, nodes_2: &Node, nodes_3: &Node, nodes_4: &Node, nodes_5: &Node) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `prepend()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/prepend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.prepend($1, $2, $3, $4, $5, $6)"]
pub fn prepend_with_node_6(&self, nodes_1: &Node, nodes_2: &Node, nodes_3: &Node, nodes_4: &Node, nodes_5: &Node, nodes_6: &Node) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `prepend()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/prepend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.prepend($1, $2, $3, $4, $5, $6, $7)"]
pub fn prepend_with_node_7(&self, nodes_1: &Node, nodes_2: &Node, nodes_3: &Node, nodes_4: &Node, nodes_5: &Node, nodes_6: &Node, nodes_7: &Node) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `prepend()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/prepend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.prepend($1)"]
pub fn prepend_with_str(&self, nodes: &::js_sys::Array) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `prepend()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/prepend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.prepend()"]
pub fn prepend_with_str_0(&self) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `prepend()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/prepend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.prepend($1)"]
pub fn prepend_with_str_1(&self, nodes_1: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `prepend()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/prepend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.prepend($1, $2)"]
pub fn prepend_with_str_2(&self, nodes_1: &str, nodes_2: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `prepend()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/prepend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.prepend($1, $2, $3)"]
pub fn prepend_with_str_3(&self, nodes_1: &str, nodes_2: &str, nodes_3: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `prepend()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/prepend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.prepend($1, $2, $3, $4)"]
pub fn prepend_with_str_4(&self, nodes_1: &str, nodes_2: &str, nodes_3: &str, nodes_4: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `prepend()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/prepend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.prepend($1, $2, $3, $4, $5)"]
pub fn prepend_with_str_5(&self, nodes_1: &str, nodes_2: &str, nodes_3: &str, nodes_4: &str, nodes_5: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `prepend()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/prepend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.prepend($1, $2, $3, $4, $5, $6)"]
pub fn prepend_with_str_6(&self, nodes_1: &str, nodes_2: &str, nodes_3: &str, nodes_4: &str, nodes_5: &str, nodes_6: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `prepend()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/prepend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.prepend($1, $2, $3, $4, $5, $6, $7)"]
pub fn prepend_with_str_7(&self, nodes_1: &str, nodes_2: &str, nodes_3: &str, nodes_4: &str, nodes_5: &str, nodes_6: &str, nodes_7: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `replaceChildren()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceChildren)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.replaceChildren($1)"]
pub fn replace_children_with_node(&self, nodes: &::js_sys::Array) { panic!("web-sys intrinsic") }
}
    #[doc = "The `replaceChildren()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceChildren)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.replaceChildren()"]
pub fn replace_children_with_node_0(&self) { panic!("web-sys intrinsic") }
}
    #[doc = "The `replaceChildren()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceChildren)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.replaceChildren($1)"]
pub fn replace_children_with_node_1(&self, nodes_1: &Node) { panic!("web-sys intrinsic") }
}
    #[doc = "The `replaceChildren()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceChildren)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.replaceChildren($1, $2)"]
pub fn replace_children_with_node_2(&self, nodes_1: &Node, nodes_2: &Node) { panic!("web-sys intrinsic") }
}
    #[doc = "The `replaceChildren()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceChildren)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.replaceChildren($1, $2, $3)"]
pub fn replace_children_with_node_3(&self, nodes_1: &Node, nodes_2: &Node, nodes_3: &Node) { panic!("web-sys intrinsic") }
}
    #[doc = "The `replaceChildren()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceChildren)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.replaceChildren($1, $2, $3, $4)"]
pub fn replace_children_with_node_4(&self, nodes_1: &Node, nodes_2: &Node, nodes_3: &Node, nodes_4: &Node) { panic!("web-sys intrinsic") }
}
    #[doc = "The `replaceChildren()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceChildren)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.replaceChildren($1, $2, $3, $4, $5)"]
pub fn replace_children_with_node_5(&self, nodes_1: &Node, nodes_2: &Node, nodes_3: &Node, nodes_4: &Node, nodes_5: &Node) { panic!("web-sys intrinsic") }
}
    #[doc = "The `replaceChildren()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceChildren)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.replaceChildren($1, $2, $3, $4, $5, $6)"]
pub fn replace_children_with_node_6(&self, nodes_1: &Node, nodes_2: &Node, nodes_3: &Node, nodes_4: &Node, nodes_5: &Node, nodes_6: &Node) { panic!("web-sys intrinsic") }
}
    #[doc = "The `replaceChildren()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceChildren)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.replaceChildren($1, $2, $3, $4, $5, $6, $7)"]
pub fn replace_children_with_node_7(&self, nodes_1: &Node, nodes_2: &Node, nodes_3: &Node, nodes_4: &Node, nodes_5: &Node, nodes_6: &Node, nodes_7: &Node) { panic!("web-sys intrinsic") }
}
    #[doc = "The `replaceChildren()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceChildren)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.replaceChildren($1)"]
pub fn replace_children_with_str(&self, nodes: &::js_sys::Array) { panic!("web-sys intrinsic") }
}
    #[doc = "The `replaceChildren()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceChildren)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.replaceChildren()"]
pub fn replace_children_with_str_0(&self) { panic!("web-sys intrinsic") }
}
    #[doc = "The `replaceChildren()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceChildren)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.replaceChildren($1)"]
pub fn replace_children_with_str_1(&self, nodes_1: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "The `replaceChildren()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceChildren)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.replaceChildren($1, $2)"]
pub fn replace_children_with_str_2(&self, nodes_1: &str, nodes_2: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "The `replaceChildren()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceChildren)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.replaceChildren($1, $2, $3)"]
pub fn replace_children_with_str_3(&self, nodes_1: &str, nodes_2: &str, nodes_3: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "The `replaceChildren()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceChildren)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.replaceChildren($1, $2, $3, $4)"]
pub fn replace_children_with_str_4(&self, nodes_1: &str, nodes_2: &str, nodes_3: &str, nodes_4: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "The `replaceChildren()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceChildren)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.replaceChildren($1, $2, $3, $4, $5)"]
pub fn replace_children_with_str_5(&self, nodes_1: &str, nodes_2: &str, nodes_3: &str, nodes_4: &str, nodes_5: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "The `replaceChildren()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceChildren)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.replaceChildren($1, $2, $3, $4, $5, $6)"]
pub fn replace_children_with_str_6(&self, nodes_1: &str, nodes_2: &str, nodes_3: &str, nodes_4: &str, nodes_5: &str, nodes_6: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "The `replaceChildren()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Element/replaceChildren)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`*"]
impl Element {
    #[js::import = "$0.replaceChildren($1, $2, $3, $4, $5, $6, $7)"]
pub fn replace_children_with_str_7(&self, nodes_1: &str, nodes_2: &str, nodes_3: &str, nodes_4: &str, nodes_5: &str, nodes_6: &str, nodes_7: &str) { panic!("web-sys intrinsic") }
}
