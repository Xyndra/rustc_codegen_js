#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `IntersectionObserverEntry` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IntersectionObserverEntry)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IntersectionObserverEntry`*"]
#[js::opaque]
#[derive(Copy)]
pub struct IntersectionObserverEntry(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `time` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IntersectionObserverEntry/time)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IntersectionObserverEntry`*"]
impl IntersectionObserverEntry {
    #[js::import = "$0.time"]
pub fn time(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "DomRectReadOnly")]
    #[doc = "Getter for the `rootBounds` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IntersectionObserverEntry/rootBounds)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRectReadOnly`, `IntersectionObserverEntry`*"]
impl IntersectionObserverEntry {
    #[js::import = "$0.rootBounds"]
pub fn root_bounds(&self) -> Option<DomRectReadOnly> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "DomRectReadOnly")]
    #[doc = "Getter for the `boundingClientRect` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IntersectionObserverEntry/boundingClientRect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRectReadOnly`, `IntersectionObserverEntry`*"]
impl IntersectionObserverEntry {
    #[js::import = "$0.boundingClientRect"]
pub fn bounding_client_rect(&self) -> DomRectReadOnly { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "DomRectReadOnly")]
    #[doc = "Getter for the `intersectionRect` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IntersectionObserverEntry/intersectionRect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRectReadOnly`, `IntersectionObserverEntry`*"]
impl IntersectionObserverEntry {
    #[js::import = "$0.intersectionRect"]
pub fn intersection_rect(&self) -> DomRectReadOnly { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `isIntersecting` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IntersectionObserverEntry/isIntersecting)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IntersectionObserverEntry`*"]
impl IntersectionObserverEntry {
    #[js::import = "$0.isIntersecting"]
pub fn is_intersecting(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `intersectionRatio` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IntersectionObserverEntry/intersectionRatio)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IntersectionObserverEntry`*"]
impl IntersectionObserverEntry {
    #[js::import = "$0.intersectionRatio"]
pub fn intersection_ratio(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Element")]
    #[doc = "Getter for the `target` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IntersectionObserverEntry/target)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`, `IntersectionObserverEntry`*"]
impl IntersectionObserverEntry {
    #[js::import = "$0.target"]
pub fn target(&self) -> Element { panic!("web-sys intrinsic") }
}
