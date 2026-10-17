#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
#[js::opaque]
#[derive(Copy)]
pub struct SvgElement(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `id` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/id)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.id"]
pub fn id(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `id` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/id)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.id = $1"]
pub fn set_id(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgAnimatedString")]
    #[doc = "Getter for the `className` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/className)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.className"]
pub fn class_name(&self) -> SvgAnimatedString { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgsvgElement")]
    #[doc = "Getter for the `ownerSVGElement` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ownerSVGElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`, `SvgsvgElement`*"]
impl SvgElement {
    #[js::import = "$0.ownerSVGElement"]
pub fn owner_svg_element(&self) -> Option<SvgsvgElement> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `viewportElement` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/viewportElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.viewportElement"]
pub fn viewport_element(&self) -> Option<SvgElement> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `oncopy` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/oncopy)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.oncopy"]
pub fn oncopy(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `oncopy` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/oncopy)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.oncopy = $1"]
pub fn set_oncopy(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `oncut` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/oncut)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.oncut"]
pub fn oncut(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `oncut` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/oncut)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.oncut = $1"]
pub fn set_oncut(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onpaste` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onpaste)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onpaste"]
pub fn onpaste(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onpaste` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onpaste)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onpaste = $1"]
pub fn set_onpaste(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "CssStyleDeclaration")]
    #[doc = "Getter for the `style` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/style)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssStyleDeclaration`, `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.style"]
pub fn style(&self) -> CssStyleDeclaration { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onabort` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onabort)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onabort"]
pub fn onabort(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onabort` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onabort)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onabort = $1"]
pub fn set_onabort(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onblur` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onblur)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onblur"]
pub fn onblur(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onblur` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onblur)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onblur = $1"]
pub fn set_onblur(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onfocus` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onfocus)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onfocus"]
pub fn onfocus(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onfocus` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onfocus)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onfocus = $1"]
pub fn set_onfocus(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onauxclick` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onauxclick)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onauxclick"]
pub fn onauxclick(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onauxclick` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onauxclick)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onauxclick = $1"]
pub fn set_onauxclick(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onbeforetoggle` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onbeforetoggle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onbeforetoggle"]
pub fn onbeforetoggle(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onbeforetoggle` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onbeforetoggle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onbeforetoggle = $1"]
pub fn set_onbeforetoggle(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `oncanplay` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/oncanplay)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.oncanplay"]
pub fn oncanplay(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `oncanplay` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/oncanplay)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.oncanplay = $1"]
pub fn set_oncanplay(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `oncanplaythrough` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/oncanplaythrough)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.oncanplaythrough"]
pub fn oncanplaythrough(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `oncanplaythrough` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/oncanplaythrough)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.oncanplaythrough = $1"]
pub fn set_oncanplaythrough(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onchange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onchange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onchange"]
pub fn onchange(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onchange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onchange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onchange = $1"]
pub fn set_onchange(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onclick` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onclick)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onclick"]
pub fn onclick(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onclick` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onclick)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onclick = $1"]
pub fn set_onclick(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onclose` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onclose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onclose"]
pub fn onclose(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onclose` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onclose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onclose = $1"]
pub fn set_onclose(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `oncontextmenu` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/oncontextmenu)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.oncontextmenu"]
pub fn oncontextmenu(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `oncontextmenu` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/oncontextmenu)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.oncontextmenu = $1"]
pub fn set_oncontextmenu(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `ondblclick` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ondblclick)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.ondblclick"]
pub fn ondblclick(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `ondblclick` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ondblclick)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.ondblclick = $1"]
pub fn set_ondblclick(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `ondrag` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ondrag)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.ondrag"]
pub fn ondrag(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `ondrag` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ondrag)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.ondrag = $1"]
pub fn set_ondrag(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `ondragend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ondragend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.ondragend"]
pub fn ondragend(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `ondragend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ondragend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.ondragend = $1"]
pub fn set_ondragend(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `ondragenter` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ondragenter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.ondragenter"]
pub fn ondragenter(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `ondragenter` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ondragenter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.ondragenter = $1"]
pub fn set_ondragenter(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `ondragexit` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ondragexit)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.ondragexit"]
pub fn ondragexit(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `ondragexit` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ondragexit)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.ondragexit = $1"]
pub fn set_ondragexit(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `ondragleave` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ondragleave)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.ondragleave"]
pub fn ondragleave(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `ondragleave` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ondragleave)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.ondragleave = $1"]
pub fn set_ondragleave(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `ondragover` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ondragover)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.ondragover"]
pub fn ondragover(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `ondragover` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ondragover)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.ondragover = $1"]
pub fn set_ondragover(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `ondragstart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ondragstart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.ondragstart"]
pub fn ondragstart(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `ondragstart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ondragstart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.ondragstart = $1"]
pub fn set_ondragstart(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `ondrop` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ondrop)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.ondrop"]
pub fn ondrop(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `ondrop` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ondrop)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.ondrop = $1"]
pub fn set_ondrop(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `ondurationchange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ondurationchange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.ondurationchange"]
pub fn ondurationchange(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `ondurationchange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ondurationchange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.ondurationchange = $1"]
pub fn set_ondurationchange(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onemptied` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onemptied)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onemptied"]
pub fn onemptied(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onemptied` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onemptied)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onemptied = $1"]
pub fn set_onemptied(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onended` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onended)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onended"]
pub fn onended(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onended` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onended)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onended = $1"]
pub fn set_onended(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `oninput` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/oninput)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.oninput"]
pub fn oninput(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `oninput` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/oninput)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.oninput = $1"]
pub fn set_oninput(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onbeforeinput` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onbeforeinput)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onbeforeinput"]
pub fn onbeforeinput(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onbeforeinput` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onbeforeinput)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onbeforeinput = $1"]
pub fn set_onbeforeinput(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `oninvalid` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/oninvalid)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.oninvalid"]
pub fn oninvalid(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `oninvalid` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/oninvalid)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.oninvalid = $1"]
pub fn set_oninvalid(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onkeydown` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onkeydown)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onkeydown"]
pub fn onkeydown(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onkeydown` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onkeydown)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onkeydown = $1"]
pub fn set_onkeydown(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onkeypress` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onkeypress)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onkeypress"]
pub fn onkeypress(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onkeypress` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onkeypress)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onkeypress = $1"]
pub fn set_onkeypress(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onkeyup` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onkeyup)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onkeyup"]
pub fn onkeyup(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onkeyup` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onkeyup)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onkeyup = $1"]
pub fn set_onkeyup(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onload` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onload)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onload"]
pub fn onload(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onload` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onload)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onload = $1"]
pub fn set_onload(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onloadeddata` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onloadeddata)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onloadeddata"]
pub fn onloadeddata(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onloadeddata` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onloadeddata)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onloadeddata = $1"]
pub fn set_onloadeddata(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onloadedmetadata` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onloadedmetadata)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onloadedmetadata"]
pub fn onloadedmetadata(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onloadedmetadata` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onloadedmetadata)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onloadedmetadata = $1"]
pub fn set_onloadedmetadata(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onloadend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onloadend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onloadend"]
pub fn onloadend(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onloadend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onloadend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onloadend = $1"]
pub fn set_onloadend(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onloadstart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onloadstart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onloadstart"]
pub fn onloadstart(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onloadstart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onloadstart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onloadstart = $1"]
pub fn set_onloadstart(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onmousedown` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onmousedown)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onmousedown"]
pub fn onmousedown(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onmousedown` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onmousedown)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onmousedown = $1"]
pub fn set_onmousedown(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onmouseenter` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onmouseenter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onmouseenter"]
pub fn onmouseenter(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onmouseenter` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onmouseenter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onmouseenter = $1"]
pub fn set_onmouseenter(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onmouseleave` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onmouseleave)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onmouseleave"]
pub fn onmouseleave(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onmouseleave` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onmouseleave)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onmouseleave = $1"]
pub fn set_onmouseleave(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onmousemove` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onmousemove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onmousemove"]
pub fn onmousemove(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onmousemove` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onmousemove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onmousemove = $1"]
pub fn set_onmousemove(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onmouseout` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onmouseout)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onmouseout"]
pub fn onmouseout(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onmouseout` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onmouseout)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onmouseout = $1"]
pub fn set_onmouseout(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onmouseover` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onmouseover)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onmouseover"]
pub fn onmouseover(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onmouseover` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onmouseover)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onmouseover = $1"]
pub fn set_onmouseover(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onmouseup` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onmouseup)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onmouseup"]
pub fn onmouseup(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onmouseup` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onmouseup)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onmouseup = $1"]
pub fn set_onmouseup(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onwheel` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onwheel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onwheel"]
pub fn onwheel(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onwheel` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onwheel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onwheel = $1"]
pub fn set_onwheel(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onpause` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onpause)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onpause"]
pub fn onpause(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onpause` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onpause)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onpause = $1"]
pub fn set_onpause(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onplay` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onplay)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onplay"]
pub fn onplay(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onplay` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onplay)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onplay = $1"]
pub fn set_onplay(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onplaying` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onplaying)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onplaying"]
pub fn onplaying(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onplaying` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onplaying)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onplaying = $1"]
pub fn set_onplaying(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onprogress` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onprogress)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onprogress"]
pub fn onprogress(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onprogress` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onprogress)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onprogress = $1"]
pub fn set_onprogress(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onratechange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onratechange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onratechange"]
pub fn onratechange(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onratechange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onratechange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onratechange = $1"]
pub fn set_onratechange(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onreset` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onreset)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onreset"]
pub fn onreset(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onreset` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onreset)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onreset = $1"]
pub fn set_onreset(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onresize` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onresize)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onresize"]
pub fn onresize(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onresize` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onresize)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onresize = $1"]
pub fn set_onresize(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onscroll` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onscroll)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onscroll"]
pub fn onscroll(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onscroll` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onscroll)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onscroll = $1"]
pub fn set_onscroll(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onseeked` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onseeked)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onseeked"]
pub fn onseeked(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onseeked` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onseeked)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onseeked = $1"]
pub fn set_onseeked(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onseeking` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onseeking)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onseeking"]
pub fn onseeking(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onseeking` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onseeking)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onseeking = $1"]
pub fn set_onseeking(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onselect` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onselect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onselect"]
pub fn onselect(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onselect` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onselect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onselect = $1"]
pub fn set_onselect(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onshow` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onshow)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onshow"]
pub fn onshow(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onshow` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onshow)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onshow = $1"]
pub fn set_onshow(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onstalled` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onstalled)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onstalled"]
pub fn onstalled(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onstalled` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onstalled)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onstalled = $1"]
pub fn set_onstalled(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onsubmit` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onsubmit)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onsubmit"]
pub fn onsubmit(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onsubmit` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onsubmit)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onsubmit = $1"]
pub fn set_onsubmit(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onsuspend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onsuspend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onsuspend"]
pub fn onsuspend(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onsuspend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onsuspend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onsuspend = $1"]
pub fn set_onsuspend(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `ontimeupdate` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ontimeupdate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.ontimeupdate"]
pub fn ontimeupdate(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `ontimeupdate` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ontimeupdate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.ontimeupdate = $1"]
pub fn set_ontimeupdate(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onvolumechange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onvolumechange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onvolumechange"]
pub fn onvolumechange(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onvolumechange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onvolumechange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onvolumechange = $1"]
pub fn set_onvolumechange(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onwaiting` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onwaiting)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onwaiting"]
pub fn onwaiting(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onwaiting` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onwaiting)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onwaiting = $1"]
pub fn set_onwaiting(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onselectstart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onselectstart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onselectstart"]
pub fn onselectstart(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onselectstart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onselectstart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onselectstart = $1"]
pub fn set_onselectstart(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `ontoggle` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ontoggle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.ontoggle"]
pub fn ontoggle(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `ontoggle` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ontoggle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.ontoggle = $1"]
pub fn set_ontoggle(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onpointercancel` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onpointercancel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onpointercancel"]
pub fn onpointercancel(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onpointercancel` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onpointercancel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onpointercancel = $1"]
pub fn set_onpointercancel(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onpointerdown` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onpointerdown)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onpointerdown"]
pub fn onpointerdown(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onpointerdown` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onpointerdown)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onpointerdown = $1"]
pub fn set_onpointerdown(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onpointerup` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onpointerup)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onpointerup"]
pub fn onpointerup(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onpointerup` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onpointerup)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onpointerup = $1"]
pub fn set_onpointerup(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onpointermove` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onpointermove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onpointermove"]
pub fn onpointermove(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onpointermove` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onpointermove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onpointermove = $1"]
pub fn set_onpointermove(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onpointerout` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onpointerout)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onpointerout"]
pub fn onpointerout(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onpointerout` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onpointerout)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onpointerout = $1"]
pub fn set_onpointerout(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onpointerover` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onpointerover)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onpointerover"]
pub fn onpointerover(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onpointerover` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onpointerover)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onpointerover = $1"]
pub fn set_onpointerover(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onpointerenter` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onpointerenter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onpointerenter"]
pub fn onpointerenter(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onpointerenter` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onpointerenter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onpointerenter = $1"]
pub fn set_onpointerenter(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onpointerleave` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onpointerleave)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onpointerleave"]
pub fn onpointerleave(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onpointerleave` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onpointerleave)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onpointerleave = $1"]
pub fn set_onpointerleave(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `ongotpointercapture` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ongotpointercapture)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.ongotpointercapture"]
pub fn ongotpointercapture(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `ongotpointercapture` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ongotpointercapture)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.ongotpointercapture = $1"]
pub fn set_ongotpointercapture(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onlostpointercapture` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onlostpointercapture)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onlostpointercapture"]
pub fn onlostpointercapture(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onlostpointercapture` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onlostpointercapture)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onlostpointercapture = $1"]
pub fn set_onlostpointercapture(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onanimationcancel` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onanimationcancel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onanimationcancel"]
pub fn onanimationcancel(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onanimationcancel` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onanimationcancel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onanimationcancel = $1"]
pub fn set_onanimationcancel(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onanimationend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onanimationend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onanimationend"]
pub fn onanimationend(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onanimationend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onanimationend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onanimationend = $1"]
pub fn set_onanimationend(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onanimationiteration` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onanimationiteration)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onanimationiteration"]
pub fn onanimationiteration(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onanimationiteration` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onanimationiteration)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onanimationiteration = $1"]
pub fn set_onanimationiteration(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onanimationstart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onanimationstart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onanimationstart"]
pub fn onanimationstart(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onanimationstart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onanimationstart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onanimationstart = $1"]
pub fn set_onanimationstart(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `ontransitioncancel` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ontransitioncancel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.ontransitioncancel"]
pub fn ontransitioncancel(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `ontransitioncancel` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ontransitioncancel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.ontransitioncancel = $1"]
pub fn set_ontransitioncancel(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `ontransitionend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ontransitionend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.ontransitionend"]
pub fn ontransitionend(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `ontransitionend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ontransitionend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.ontransitionend = $1"]
pub fn set_ontransitionend(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `ontransitionrun` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ontransitionrun)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.ontransitionrun"]
pub fn ontransitionrun(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `ontransitionrun` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ontransitionrun)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.ontransitionrun = $1"]
pub fn set_ontransitionrun(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `ontransitionstart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ontransitionstart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.ontransitionstart"]
pub fn ontransitionstart(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `ontransitionstart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ontransitionstart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.ontransitionstart = $1"]
pub fn set_ontransitionstart(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onwebkitanimationend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onwebkitanimationend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onwebkitanimationend"]
pub fn onwebkitanimationend(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onwebkitanimationend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onwebkitanimationend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onwebkitanimationend = $1"]
pub fn set_onwebkitanimationend(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onwebkitanimationiteration` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onwebkitanimationiteration)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onwebkitanimationiteration"]
pub fn onwebkitanimationiteration(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onwebkitanimationiteration` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onwebkitanimationiteration)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onwebkitanimationiteration = $1"]
pub fn set_onwebkitanimationiteration(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onwebkitanimationstart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onwebkitanimationstart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onwebkitanimationstart"]
pub fn onwebkitanimationstart(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onwebkitanimationstart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onwebkitanimationstart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onwebkitanimationstart = $1"]
pub fn set_onwebkitanimationstart(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onwebkittransitionend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onwebkittransitionend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onwebkittransitionend"]
pub fn onwebkittransitionend(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onwebkittransitionend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onwebkittransitionend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onwebkittransitionend = $1"]
pub fn set_onwebkittransitionend(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "DomStringMap")]
    #[doc = "Getter for the `dataset` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/dataset)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomStringMap`, `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.dataset"]
pub fn dataset(&self) -> DomStringMap { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `nonce` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/nonce)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.nonce"]
pub fn nonce(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `nonce` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/nonce)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.nonce = $1"]
pub fn set_nonce(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `autofocus` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/autofocus)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.autofocus"]
pub fn autofocus(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `autofocus` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/autofocus)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.autofocus = $1"]
pub fn set_autofocus(&self, value: bool) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `tabIndex` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/tabIndex)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.tabIndex"]
pub fn tab_index(&self) -> i32 { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `tabIndex` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/tabIndex)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.tabIndex = $1"]
pub fn set_tab_index(&self, value: i32) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onerror"]
pub fn onerror(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/onerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.onerror = $1"]
pub fn set_onerror(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `ontouchstart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ontouchstart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.ontouchstart"]
pub fn ontouchstart(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `ontouchstart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ontouchstart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.ontouchstart = $1"]
pub fn set_ontouchstart(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `ontouchend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ontouchend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.ontouchend"]
pub fn ontouchend(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `ontouchend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ontouchend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.ontouchend = $1"]
pub fn set_ontouchend(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `ontouchmove` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ontouchmove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.ontouchmove"]
pub fn ontouchmove(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `ontouchmove` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ontouchmove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.ontouchmove = $1"]
pub fn set_ontouchmove(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `ontouchcancel` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ontouchcancel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.ontouchcancel"]
pub fn ontouchcancel(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `ontouchcancel` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/ontouchcancel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.ontouchcancel = $1"]
pub fn set_ontouchcancel(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "The `blur()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/blur)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.blur()"]
pub fn blur(&self) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `focus()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/focus)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.focus()"]
pub fn focus(&self) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "FocusOptions")]
    #[doc = "The `focus()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGElement/focus)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FocusOptions`, `SvgElement`*"]
impl SvgElement {
    #[js::import = "$0.focus($1)"]
pub fn focus_with_options(&self, options: &FocusOptions) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
