#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgFilterElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFilterElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgFilterElement`*"]
#[js::opaque]
#[derive(Copy)]
pub struct SvgFilterElement(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "SvgAnimatedEnumeration")]
    #[doc = "Getter for the `filterUnits` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFilterElement/filterUnits)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedEnumeration`, `SvgFilterElement`*"]
impl SvgFilterElement {
    #[js::import = "$0.filterUnits"]
pub fn filter_units(&self) -> SvgAnimatedEnumeration { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgAnimatedEnumeration")]
    #[doc = "Getter for the `primitiveUnits` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFilterElement/primitiveUnits)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedEnumeration`, `SvgFilterElement`*"]
impl SvgFilterElement {
    #[js::import = "$0.primitiveUnits"]
pub fn primitive_units(&self) -> SvgAnimatedEnumeration { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `x` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFilterElement/x)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgFilterElement`*"]
impl SvgFilterElement {
    #[js::import = "$0.x"]
pub fn x(&self) -> SvgAnimatedLength { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `y` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFilterElement/y)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgFilterElement`*"]
impl SvgFilterElement {
    #[js::import = "$0.y"]
pub fn y(&self) -> SvgAnimatedLength { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `width` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFilterElement/width)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgFilterElement`*"]
impl SvgFilterElement {
    #[js::import = "$0.width"]
pub fn width(&self) -> SvgAnimatedLength { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `height` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFilterElement/height)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgFilterElement`*"]
impl SvgFilterElement {
    #[js::import = "$0.height"]
pub fn height(&self) -> SvgAnimatedLength { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgAnimatedString")]
    #[doc = "Getter for the `href` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFilterElement/href)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgFilterElement`*"]
impl SvgFilterElement {
    #[js::import = "$0.href"]
pub fn href(&self) -> SvgAnimatedString { panic!("web-sys intrinsic") }
}
