#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ResizeObserverEntry` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ResizeObserverEntry)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ResizeObserverEntry`*"]
#[js::opaque]
#[derive(Copy)]
pub struct ResizeObserverEntry(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "Element")]
    #[doc = "Getter for the `target` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ResizeObserverEntry/target)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`, `ResizeObserverEntry`*"]
impl ResizeObserverEntry {
    #[js::import = "$0.target"]
pub fn target(&self) -> Element { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "DomRectReadOnly")]
    #[doc = "Getter for the `contentRect` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ResizeObserverEntry/contentRect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRectReadOnly`, `ResizeObserverEntry`*"]
impl ResizeObserverEntry {
    #[js::import = "$0.contentRect"]
pub fn content_rect(&self) -> DomRectReadOnly { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `borderBoxSize` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ResizeObserverEntry/borderBoxSize)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ResizeObserverEntry`*"]
impl ResizeObserverEntry {
    #[js::import = "$0.borderBoxSize"]
pub fn border_box_size(&self) -> ::js_sys::Array { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `contentBoxSize` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ResizeObserverEntry/contentBoxSize)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ResizeObserverEntry`*"]
impl ResizeObserverEntry {
    #[js::import = "$0.contentBoxSize"]
pub fn content_box_size(&self) -> ::js_sys::Array { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `devicePixelContentBoxSize` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ResizeObserverEntry/devicePixelContentBoxSize)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ResizeObserverEntry`*"]
impl ResizeObserverEntry {
    #[js::import = "$0.devicePixelContentBoxSize"]
pub fn device_pixel_content_box_size(&self) -> ::js_sys::Array { panic!("web-sys intrinsic") }
}
