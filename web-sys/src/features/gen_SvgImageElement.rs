#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgImageElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGImageElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgImageElement`*"]
#[js::opaque]
#[derive(Copy)]
pub struct SvgImageElement(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `x` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGImageElement/x)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgImageElement`*"]
impl SvgImageElement {
    #[js::import = "$0.x"]
pub fn x(&self) -> SvgAnimatedLength { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `y` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGImageElement/y)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgImageElement`*"]
impl SvgImageElement {
    #[js::import = "$0.y"]
pub fn y(&self) -> SvgAnimatedLength { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `width` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGImageElement/width)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgImageElement`*"]
impl SvgImageElement {
    #[js::import = "$0.width"]
pub fn width(&self) -> SvgAnimatedLength { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `height` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGImageElement/height)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgImageElement`*"]
impl SvgImageElement {
    #[js::import = "$0.height"]
pub fn height(&self) -> SvgAnimatedLength { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgAnimatedPreserveAspectRatio")]
    #[doc = "Getter for the `preserveAspectRatio` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGImageElement/preserveAspectRatio)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedPreserveAspectRatio`, `SvgImageElement`*"]
impl SvgImageElement {
    #[js::import = "$0.preserveAspectRatio"]
pub fn preserve_aspect_ratio(&self) -> SvgAnimatedPreserveAspectRatio { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgAnimatedString")]
    #[doc = "Getter for the `href` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGImageElement/href)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgImageElement`*"]
impl SvgImageElement {
    #[js::import = "$0.href"]
pub fn href(&self) -> SvgAnimatedString { panic!("web-sys intrinsic") }
}
