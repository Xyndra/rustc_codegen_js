#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgComponentTransferFunctionElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGComponentTransferFunctionElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgComponentTransferFunctionElement`*"]
#[js::opaque]
#[derive(Copy)]
pub struct SvgComponentTransferFunctionElement(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "SvgAnimatedEnumeration")]
    #[doc = "Getter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGComponentTransferFunctionElement/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedEnumeration`, `SvgComponentTransferFunctionElement`*"]
impl SvgComponentTransferFunctionElement {
    #[js::import = "$0.type"]
pub fn type_(&self) -> SvgAnimatedEnumeration { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgAnimatedNumberList")]
    #[doc = "Getter for the `tableValues` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGComponentTransferFunctionElement/tableValues)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedNumberList`, `SvgComponentTransferFunctionElement`*"]
impl SvgComponentTransferFunctionElement {
    #[js::import = "$0.tableValues"]
pub fn table_values(&self) -> SvgAnimatedNumberList { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgAnimatedNumber")]
    #[doc = "Getter for the `slope` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGComponentTransferFunctionElement/slope)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgComponentTransferFunctionElement`*"]
impl SvgComponentTransferFunctionElement {
    #[js::import = "$0.slope"]
pub fn slope(&self) -> SvgAnimatedNumber { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgAnimatedNumber")]
    #[doc = "Getter for the `intercept` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGComponentTransferFunctionElement/intercept)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgComponentTransferFunctionElement`*"]
impl SvgComponentTransferFunctionElement {
    #[js::import = "$0.intercept"]
pub fn intercept(&self) -> SvgAnimatedNumber { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgAnimatedNumber")]
    #[doc = "Getter for the `amplitude` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGComponentTransferFunctionElement/amplitude)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgComponentTransferFunctionElement`*"]
impl SvgComponentTransferFunctionElement {
    #[js::import = "$0.amplitude"]
pub fn amplitude(&self) -> SvgAnimatedNumber { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgAnimatedNumber")]
    #[doc = "Getter for the `exponent` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGComponentTransferFunctionElement/exponent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgComponentTransferFunctionElement`*"]
impl SvgComponentTransferFunctionElement {
    #[js::import = "$0.exponent"]
pub fn exponent(&self) -> SvgAnimatedNumber { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgAnimatedNumber")]
    #[doc = "Getter for the `offset` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGComponentTransferFunctionElement/offset)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgComponentTransferFunctionElement`*"]
impl SvgComponentTransferFunctionElement {
    #[js::import = "$0.offset"]
pub fn offset(&self) -> SvgAnimatedNumber { panic!("web-sys intrinsic") }
}
impl SvgComponentTransferFunctionElement {
    #[doc = "The `SVGComponentTransferFunctionElement.SVG_FECOMPONENTTRANSFER_TYPE_UNKNOWN` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgComponentTransferFunctionElement`*"]
    pub const SVG_FECOMPONENTTRANSFER_TYPE_UNKNOWN: u16 = 0i64 as u16;
    #[doc = "The `SVGComponentTransferFunctionElement.SVG_FECOMPONENTTRANSFER_TYPE_IDENTITY` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgComponentTransferFunctionElement`*"]
    pub const SVG_FECOMPONENTTRANSFER_TYPE_IDENTITY: u16 = 1u64 as u16;
    #[doc = "The `SVGComponentTransferFunctionElement.SVG_FECOMPONENTTRANSFER_TYPE_TABLE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgComponentTransferFunctionElement`*"]
    pub const SVG_FECOMPONENTTRANSFER_TYPE_TABLE: u16 = 2u64 as u16;
    #[doc = "The `SVGComponentTransferFunctionElement.SVG_FECOMPONENTTRANSFER_TYPE_DISCRETE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgComponentTransferFunctionElement`*"]
    pub const SVG_FECOMPONENTTRANSFER_TYPE_DISCRETE: u16 = 3u64 as u16;
    #[doc = "The `SVGComponentTransferFunctionElement.SVG_FECOMPONENTTRANSFER_TYPE_LINEAR` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgComponentTransferFunctionElement`*"]
    pub const SVG_FECOMPONENTTRANSFER_TYPE_LINEAR: u16 = 4u64 as u16;
    #[doc = "The `SVGComponentTransferFunctionElement.SVG_FECOMPONENTTRANSFER_TYPE_GAMMA` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgComponentTransferFunctionElement`*"]
    pub const SVG_FECOMPONENTTRANSFER_TYPE_GAMMA: u16 = 5u64 as u16;
}
