#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `VttCue` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VttCue`*"]
#[js::opaque]
#[derive(Copy)]
pub struct VttCue(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "VttRegion")]
    #[doc = "Getter for the `region` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/region)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VttCue`, `VttRegion`*"]
impl VttCue {
    #[js::import = "$0.region"]
pub fn region(&self) -> Option<VttRegion> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "VttRegion")]
    #[doc = "Setter for the `region` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/region)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VttCue`, `VttRegion`*"]
impl VttCue {
    #[js::import = "$0.region = $1"]
pub fn set_region(&self, value: Option<&VttRegion>) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "DirectionSetting")]
    #[doc = "Getter for the `vertical` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/vertical)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DirectionSetting`, `VttCue`*"]
impl VttCue {
    #[js::import = "$0.vertical"]
pub fn vertical(&self) -> DirectionSetting { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "DirectionSetting")]
    #[doc = "Setter for the `vertical` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/vertical)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DirectionSetting`, `VttCue`*"]
impl VttCue {
    #[js::import = "$0.vertical = $1"]
pub fn set_vertical(&self, value: DirectionSetting) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `snapToLines` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/snapToLines)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VttCue`*"]
impl VttCue {
    #[js::import = "$0.snapToLines"]
pub fn snap_to_lines(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `snapToLines` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/snapToLines)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VttCue`*"]
impl VttCue {
    #[js::import = "$0.snapToLines = $1"]
pub fn set_snap_to_lines(&self, value: bool) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `line` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/line)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VttCue`*"]
impl VttCue {
    #[js::import = "$0.line"]
pub fn line(&self) -> ::wasm_bindgen::JsValue { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `line` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/line)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VttCue`*"]
    #[deprecated]
impl VttCue {
    #[js::import = "$0.line = $1"]
pub fn set_line(&self, value: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `line` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/line)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VttCue`*"]
impl VttCue {
    #[js::import = "$0.line = $1"]
pub fn set_line_f64(&self, value: f64) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "AutoKeyword")]
    #[doc = "Setter for the `line` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/line)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AutoKeyword`, `VttCue`*"]
impl VttCue {
    #[js::import = "$0.line = $1"]
pub fn set_line_auto_keyword(&self, value: AutoKeyword) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "LineAlignSetting")]
    #[doc = "Getter for the `lineAlign` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/lineAlign)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `LineAlignSetting`, `VttCue`*"]
impl VttCue {
    #[js::import = "$0.lineAlign"]
pub fn line_align(&self) -> LineAlignSetting { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "LineAlignSetting")]
    #[doc = "Setter for the `lineAlign` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/lineAlign)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `LineAlignSetting`, `VttCue`*"]
impl VttCue {
    #[js::import = "$0.lineAlign = $1"]
pub fn set_line_align(&self, value: LineAlignSetting) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `position` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/position)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VttCue`*"]
impl VttCue {
    #[js::import = "$0.position"]
pub fn position(&self) -> ::wasm_bindgen::JsValue { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `position` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/position)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VttCue`*"]
    #[deprecated]
impl VttCue {
    #[js::import = "$0.position = $1"]
pub fn set_position(&self, value: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `position` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/position)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VttCue`*"]
impl VttCue {
    #[js::import = "$0.position = $1"]
pub fn set_position_f64(&self, value: f64) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "AutoKeyword")]
    #[doc = "Setter for the `position` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/position)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AutoKeyword`, `VttCue`*"]
impl VttCue {
    #[js::import = "$0.position = $1"]
pub fn set_position_auto_keyword(&self, value: AutoKeyword) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "PositionAlignSetting")]
    #[doc = "Getter for the `positionAlign` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/positionAlign)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PositionAlignSetting`, `VttCue`*"]
impl VttCue {
    #[js::import = "$0.positionAlign"]
pub fn position_align(&self) -> PositionAlignSetting { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "PositionAlignSetting")]
    #[doc = "Setter for the `positionAlign` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/positionAlign)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PositionAlignSetting`, `VttCue`*"]
impl VttCue {
    #[js::import = "$0.positionAlign = $1"]
pub fn set_position_align(&self, value: PositionAlignSetting) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `size` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/size)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VttCue`*"]
impl VttCue {
    #[js::import = "$0.size"]
pub fn size(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `size` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/size)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VttCue`*"]
impl VttCue {
    #[js::import = "$0.size = $1"]
pub fn set_size(&self, value: f64) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "AlignSetting")]
    #[doc = "Getter for the `align` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/align)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AlignSetting`, `VttCue`*"]
impl VttCue {
    #[js::import = "$0.align"]
pub fn align(&self) -> AlignSetting { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "AlignSetting")]
    #[doc = "Setter for the `align` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/align)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AlignSetting`, `VttCue`*"]
impl VttCue {
    #[js::import = "$0.align = $1"]
pub fn set_align(&self, value: AlignSetting) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `text` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/text)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VttCue`*"]
impl VttCue {
    #[js::import = "$0.text"]
pub fn text(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `text` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/text)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VttCue`*"]
impl VttCue {
    #[js::import = "$0.text = $1"]
pub fn set_text(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "The `new VttCue(..)` constructor, creating a new instance of `VttCue`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/VTTCue)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VttCue`*"]
impl VttCue {
    #[js::import = "new VTTCue($0, $1, $2)"]
pub fn new(start_time: f64, end_time: f64, text: &str) -> Result<VttCue, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "DocumentFragment")]
    #[doc = "The `getCueAsHTML()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VTTCue/getCueAsHTML)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`, `VttCue`*"]
impl VttCue {
    #[js::import = "$0.getCueAsHTML()"]
pub fn get_cue_as_html(&self) -> DocumentFragment { panic!("web-sys intrinsic") }
}
