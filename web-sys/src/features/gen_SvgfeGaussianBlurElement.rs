#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgfeGaussianBlurElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEGaussianBlurElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgfeGaussianBlurElement`*"]
#[js::opaque]
#[derive(Copy)]
pub struct SvgfeGaussianBlurElement(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "SvgAnimatedString")]
    #[doc = "Getter for the `in1` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEGaussianBlurElement/in1)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgfeGaussianBlurElement`*"]
impl SvgfeGaussianBlurElement {
    #[js::import = "$0.in1"]
pub fn in1(&self) -> SvgAnimatedString { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgAnimatedNumber")]
    #[doc = "Getter for the `stdDeviationX` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEGaussianBlurElement/stdDeviationX)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeGaussianBlurElement`*"]
impl SvgfeGaussianBlurElement {
    #[js::import = "$0.stdDeviationX"]
pub fn std_deviation_x(&self) -> SvgAnimatedNumber { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgAnimatedNumber")]
    #[doc = "Getter for the `stdDeviationY` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEGaussianBlurElement/stdDeviationY)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeGaussianBlurElement`*"]
impl SvgfeGaussianBlurElement {
    #[js::import = "$0.stdDeviationY"]
pub fn std_deviation_y(&self) -> SvgAnimatedNumber { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `x` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEGaussianBlurElement/x)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeGaussianBlurElement`*"]
impl SvgfeGaussianBlurElement {
    #[js::import = "$0.x"]
pub fn x(&self) -> SvgAnimatedLength { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `y` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEGaussianBlurElement/y)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeGaussianBlurElement`*"]
impl SvgfeGaussianBlurElement {
    #[js::import = "$0.y"]
pub fn y(&self) -> SvgAnimatedLength { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `width` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEGaussianBlurElement/width)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeGaussianBlurElement`*"]
impl SvgfeGaussianBlurElement {
    #[js::import = "$0.width"]
pub fn width(&self) -> SvgAnimatedLength { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `height` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEGaussianBlurElement/height)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeGaussianBlurElement`*"]
impl SvgfeGaussianBlurElement {
    #[js::import = "$0.height"]
pub fn height(&self) -> SvgAnimatedLength { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgAnimatedString")]
    #[doc = "Getter for the `result` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEGaussianBlurElement/result)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgfeGaussianBlurElement`*"]
impl SvgfeGaussianBlurElement {
    #[js::import = "$0.result"]
pub fn result(&self) -> SvgAnimatedString { panic!("web-sys intrinsic") }
}
    #[doc = "The `setStdDeviation()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEGaussianBlurElement/setStdDeviation)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgfeGaussianBlurElement`*"]
impl SvgfeGaussianBlurElement {
    #[js::import = "$0.setStdDeviation($1, $2)"]
pub fn set_std_deviation(&self, std_deviation_x: f32, std_deviation_y: f32) { panic!("web-sys intrinsic") }
}
