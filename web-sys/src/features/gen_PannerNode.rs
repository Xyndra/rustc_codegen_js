#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PannerNode` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerNode`*"]
#[js::opaque]
#[derive(Copy)]
pub struct PannerNode(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "PanningModelType")]
    #[doc = "Getter for the `panningModel` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/panningModel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerNode`, `PanningModelType`*"]
impl PannerNode {
    #[js::import = "$0.panningModel"]
pub fn panning_model(&self) -> PanningModelType { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "PanningModelType")]
    #[doc = "Setter for the `panningModel` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/panningModel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerNode`, `PanningModelType`*"]
impl PannerNode {
    #[js::import = "$0.panningModel = $1"]
pub fn set_panning_model(&self, value: PanningModelType) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "AudioParam")]
    #[doc = "Getter for the `positionX` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/positionX)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioParam`, `PannerNode`*"]
impl PannerNode {
    #[js::import = "$0.positionX"]
pub fn position_x(&self) -> AudioParam { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "AudioParam")]
    #[doc = "Getter for the `positionY` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/positionY)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioParam`, `PannerNode`*"]
impl PannerNode {
    #[js::import = "$0.positionY"]
pub fn position_y(&self) -> AudioParam { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "AudioParam")]
    #[doc = "Getter for the `positionZ` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/positionZ)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioParam`, `PannerNode`*"]
impl PannerNode {
    #[js::import = "$0.positionZ"]
pub fn position_z(&self) -> AudioParam { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "AudioParam")]
    #[doc = "Getter for the `orientationX` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/orientationX)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioParam`, `PannerNode`*"]
impl PannerNode {
    #[js::import = "$0.orientationX"]
pub fn orientation_x(&self) -> AudioParam { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "AudioParam")]
    #[doc = "Getter for the `orientationY` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/orientationY)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioParam`, `PannerNode`*"]
impl PannerNode {
    #[js::import = "$0.orientationY"]
pub fn orientation_y(&self) -> AudioParam { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "AudioParam")]
    #[doc = "Getter for the `orientationZ` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/orientationZ)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioParam`, `PannerNode`*"]
impl PannerNode {
    #[js::import = "$0.orientationZ"]
pub fn orientation_z(&self) -> AudioParam { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "DistanceModelType")]
    #[doc = "Getter for the `distanceModel` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/distanceModel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DistanceModelType`, `PannerNode`*"]
impl PannerNode {
    #[js::import = "$0.distanceModel"]
pub fn distance_model(&self) -> DistanceModelType { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "DistanceModelType")]
    #[doc = "Setter for the `distanceModel` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/distanceModel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DistanceModelType`, `PannerNode`*"]
impl PannerNode {
    #[js::import = "$0.distanceModel = $1"]
pub fn set_distance_model(&self, value: DistanceModelType) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `refDistance` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/refDistance)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerNode`*"]
impl PannerNode {
    #[js::import = "$0.refDistance"]
pub fn ref_distance(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `refDistance` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/refDistance)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerNode`*"]
impl PannerNode {
    #[js::import = "$0.refDistance = $1"]
pub fn set_ref_distance(&self, value: f64) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `maxDistance` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/maxDistance)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerNode`*"]
impl PannerNode {
    #[js::import = "$0.maxDistance"]
pub fn max_distance(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `maxDistance` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/maxDistance)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerNode`*"]
impl PannerNode {
    #[js::import = "$0.maxDistance = $1"]
pub fn set_max_distance(&self, value: f64) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `rolloffFactor` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/rolloffFactor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerNode`*"]
impl PannerNode {
    #[js::import = "$0.rolloffFactor"]
pub fn rolloff_factor(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `rolloffFactor` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/rolloffFactor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerNode`*"]
impl PannerNode {
    #[js::import = "$0.rolloffFactor = $1"]
pub fn set_rolloff_factor(&self, value: f64) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `coneInnerAngle` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/coneInnerAngle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerNode`*"]
impl PannerNode {
    #[js::import = "$0.coneInnerAngle"]
pub fn cone_inner_angle(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `coneInnerAngle` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/coneInnerAngle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerNode`*"]
impl PannerNode {
    #[js::import = "$0.coneInnerAngle = $1"]
pub fn set_cone_inner_angle(&self, value: f64) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `coneOuterAngle` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/coneOuterAngle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerNode`*"]
impl PannerNode {
    #[js::import = "$0.coneOuterAngle"]
pub fn cone_outer_angle(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `coneOuterAngle` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/coneOuterAngle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerNode`*"]
impl PannerNode {
    #[js::import = "$0.coneOuterAngle = $1"]
pub fn set_cone_outer_angle(&self, value: f64) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `coneOuterGain` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/coneOuterGain)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerNode`*"]
impl PannerNode {
    #[js::import = "$0.coneOuterGain"]
pub fn cone_outer_gain(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `coneOuterGain` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/coneOuterGain)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerNode`*"]
impl PannerNode {
    #[js::import = "$0.coneOuterGain = $1"]
pub fn set_cone_outer_gain(&self, value: f64) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "BaseAudioContext")]
    #[doc = "The `new PannerNode(..)` constructor, creating a new instance of `PannerNode`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/PannerNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseAudioContext`, `PannerNode`*"]
impl PannerNode {
    #[js::import = "new PannerNode($0)"]
pub fn new(context: &BaseAudioContext) -> Result<PannerNode, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "BaseAudioContext", feature = "PannerOptions",))]
    #[doc = "The `new PannerNode(..)` constructor, creating a new instance of `PannerNode`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/PannerNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseAudioContext`, `PannerNode`, `PannerOptions`*"]
impl PannerNode {
    #[js::import = "new PannerNode($0, $1)"]
pub fn new_with_options(context: &BaseAudioContext, options: &PannerOptions,) -> Result<PannerNode, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `setOrientation()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/setOrientation)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerNode`*"]
impl PannerNode {
    #[js::import = "$0.setOrientation($1, $2, $3)"]
pub fn set_orientation(&self, x: f64, y: f64, z: f64) { panic!("web-sys intrinsic") }
}
    #[doc = "The `setPosition()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/setPosition)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerNode`*"]
impl PannerNode {
    #[js::import = "$0.setPosition($1, $2, $3)"]
pub fn set_position(&self, x: f64, y: f64, z: f64) { panic!("web-sys intrinsic") }
}
    #[doc = "The `setVelocity()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/setVelocity)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerNode`*"]
impl PannerNode {
    #[js::import = "$0.setVelocity($1, $2, $3)"]
pub fn set_velocity(&self, x: f64, y: f64, z: f64) { panic!("web-sys intrinsic") }
}
