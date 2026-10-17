#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PannerOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
#[js::opaque]
#[derive(Copy)]
pub struct PannerOptions(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `channelCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
#[js::import = "None"]
pub fn get_channel_count(this: &PannerOptions) -> Option<u32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `channelCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
#[js::import = "None = $1"]
pub fn set_channel_count(this: &PannerOptions, val: u32) { panic!("web-sys intrinsic") }
    #[cfg(feature = "ChannelCountMode")]
    #[doc = "Get the `channelCountMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelCountMode`, `PannerOptions`*"]
#[js::import = "None"]
pub fn get_channel_count_mode(this: &PannerOptions) -> Option<ChannelCountMode> { panic!("web-sys intrinsic") }
    #[cfg(feature = "ChannelCountMode")]
    #[doc = "Change the `channelCountMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelCountMode`, `PannerOptions`*"]
#[js::import = "None = $1"]
pub fn set_channel_count_mode(this: &PannerOptions, val: ChannelCountMode) { panic!("web-sys intrinsic") }
    #[cfg(feature = "ChannelInterpretation")]
    #[doc = "Get the `channelInterpretation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelInterpretation`, `PannerOptions`*"]
#[js::import = "None"]
pub fn get_channel_interpretation(this: &PannerOptions) -> Option<ChannelInterpretation> { panic!("web-sys intrinsic") }
    #[cfg(feature = "ChannelInterpretation")]
    #[doc = "Change the `channelInterpretation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelInterpretation`, `PannerOptions`*"]
#[js::import = "None = $1"]
pub fn set_channel_interpretation(this: &PannerOptions, val: ChannelInterpretation) { panic!("web-sys intrinsic") }
    #[doc = "Get the `coneInnerAngle` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
#[js::import = "None"]
pub fn get_cone_inner_angle(this: &PannerOptions) -> Option<f64> { panic!("web-sys intrinsic") }
    #[doc = "Change the `coneInnerAngle` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
#[js::import = "None = $1"]
pub fn set_cone_inner_angle(this: &PannerOptions, val: f64) { panic!("web-sys intrinsic") }
    #[doc = "Get the `coneOuterAngle` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
#[js::import = "None"]
pub fn get_cone_outer_angle(this: &PannerOptions) -> Option<f64> { panic!("web-sys intrinsic") }
    #[doc = "Change the `coneOuterAngle` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
#[js::import = "None = $1"]
pub fn set_cone_outer_angle(this: &PannerOptions, val: f64) { panic!("web-sys intrinsic") }
    #[doc = "Get the `coneOuterGain` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
#[js::import = "None"]
pub fn get_cone_outer_gain(this: &PannerOptions) -> Option<f64> { panic!("web-sys intrinsic") }
    #[doc = "Change the `coneOuterGain` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
#[js::import = "None = $1"]
pub fn set_cone_outer_gain(this: &PannerOptions, val: f64) { panic!("web-sys intrinsic") }
    #[cfg(feature = "DistanceModelType")]
    #[doc = "Get the `distanceModel` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DistanceModelType`, `PannerOptions`*"]
#[js::import = "None"]
pub fn get_distance_model(this: &PannerOptions) -> Option<DistanceModelType> { panic!("web-sys intrinsic") }
    #[cfg(feature = "DistanceModelType")]
    #[doc = "Change the `distanceModel` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DistanceModelType`, `PannerOptions`*"]
#[js::import = "None = $1"]
pub fn set_distance_model(this: &PannerOptions, val: DistanceModelType) { panic!("web-sys intrinsic") }
    #[doc = "Get the `maxDistance` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
#[js::import = "None"]
pub fn get_max_distance(this: &PannerOptions) -> Option<f64> { panic!("web-sys intrinsic") }
    #[doc = "Change the `maxDistance` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
#[js::import = "None = $1"]
pub fn set_max_distance(this: &PannerOptions, val: f64) { panic!("web-sys intrinsic") }
    #[doc = "Get the `orientationX` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
#[js::import = "None"]
pub fn get_orientation_x(this: &PannerOptions) -> Option<f32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `orientationX` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
#[js::import = "None = $1"]
pub fn set_orientation_x(this: &PannerOptions, val: f32) { panic!("web-sys intrinsic") }
    #[doc = "Get the `orientationY` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
#[js::import = "None"]
pub fn get_orientation_y(this: &PannerOptions) -> Option<f32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `orientationY` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
#[js::import = "None = $1"]
pub fn set_orientation_y(this: &PannerOptions, val: f32) { panic!("web-sys intrinsic") }
    #[doc = "Get the `orientationZ` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
#[js::import = "None"]
pub fn get_orientation_z(this: &PannerOptions) -> Option<f32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `orientationZ` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
#[js::import = "None = $1"]
pub fn set_orientation_z(this: &PannerOptions, val: f32) { panic!("web-sys intrinsic") }
    #[cfg(feature = "PanningModelType")]
    #[doc = "Get the `panningModel` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`, `PanningModelType`*"]
#[js::import = "None"]
pub fn get_panning_model(this: &PannerOptions) -> Option<PanningModelType> { panic!("web-sys intrinsic") }
    #[cfg(feature = "PanningModelType")]
    #[doc = "Change the `panningModel` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`, `PanningModelType`*"]
#[js::import = "None = $1"]
pub fn set_panning_model(this: &PannerOptions, val: PanningModelType) { panic!("web-sys intrinsic") }
    #[doc = "Get the `positionX` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
#[js::import = "None"]
pub fn get_position_x(this: &PannerOptions) -> Option<f32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `positionX` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
#[js::import = "None = $1"]
pub fn set_position_x(this: &PannerOptions, val: f32) { panic!("web-sys intrinsic") }
    #[doc = "Get the `positionY` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
#[js::import = "None"]
pub fn get_position_y(this: &PannerOptions) -> Option<f32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `positionY` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
#[js::import = "None = $1"]
pub fn set_position_y(this: &PannerOptions, val: f32) { panic!("web-sys intrinsic") }
    #[doc = "Get the `positionZ` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
#[js::import = "None"]
pub fn get_position_z(this: &PannerOptions) -> Option<f32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `positionZ` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
#[js::import = "None = $1"]
pub fn set_position_z(this: &PannerOptions, val: f32) { panic!("web-sys intrinsic") }
    #[doc = "Get the `refDistance` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
#[js::import = "None"]
pub fn get_ref_distance(this: &PannerOptions) -> Option<f64> { panic!("web-sys intrinsic") }
    #[doc = "Change the `refDistance` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
#[js::import = "None = $1"]
pub fn set_ref_distance(this: &PannerOptions, val: f64) { panic!("web-sys intrinsic") }
    #[doc = "Get the `rolloffFactor` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
#[js::import = "None"]
pub fn get_rolloff_factor(this: &PannerOptions) -> Option<f64> { panic!("web-sys intrinsic") }
    #[doc = "Change the `rolloffFactor` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
#[js::import = "None = $1"]
pub fn set_rolloff_factor(this: &PannerOptions, val: f64) { panic!("web-sys intrinsic") }
impl PannerOptions {
    #[doc = "Construct a new `PannerOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_channel_count()` instead."]
#[js::import = "None"]
pub fn channel_count(&mut self, val: u32) -> &mut Self { self.set_channel_count(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(feature = "ChannelCountMode")]
    #[deprecated = "Use `set_channel_count_mode()` instead."]
#[js::import = "None"]
pub fn channel_count_mode(&mut self, val: ChannelCountMode) -> &mut Self { self.set_channel_count_mode(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(feature = "ChannelInterpretation")]
    #[deprecated = "Use `set_channel_interpretation()` instead."]
#[js::import = "None"]
pub fn channel_interpretation(&mut self, val: ChannelInterpretation) -> &mut Self { self.set_channel_interpretation(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_cone_inner_angle()` instead."]
#[js::import = "None"]
pub fn cone_inner_angle(&mut self, val: f64) -> &mut Self { self.set_cone_inner_angle(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_cone_outer_angle()` instead."]
#[js::import = "None"]
pub fn cone_outer_angle(&mut self, val: f64) -> &mut Self { self.set_cone_outer_angle(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_cone_outer_gain()` instead."]
#[js::import = "None"]
pub fn cone_outer_gain(&mut self, val: f64) -> &mut Self { self.set_cone_outer_gain(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(feature = "DistanceModelType")]
    #[deprecated = "Use `set_distance_model()` instead."]
#[js::import = "None"]
pub fn distance_model(&mut self, val: DistanceModelType) -> &mut Self { self.set_distance_model(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_max_distance()` instead."]
#[js::import = "None"]
pub fn max_distance(&mut self, val: f64) -> &mut Self { self.set_max_distance(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_orientation_x()` instead."]
#[js::import = "None"]
pub fn orientation_x(&mut self, val: f32) -> &mut Self { self.set_orientation_x(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_orientation_y()` instead."]
#[js::import = "None"]
pub fn orientation_y(&mut self, val: f32) -> &mut Self { self.set_orientation_y(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_orientation_z()` instead."]
#[js::import = "None"]
pub fn orientation_z(&mut self, val: f32) -> &mut Self { self.set_orientation_z(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(feature = "PanningModelType")]
    #[deprecated = "Use `set_panning_model()` instead."]
#[js::import = "None"]
pub fn panning_model(&mut self, val: PanningModelType) -> &mut Self { self.set_panning_model(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_position_x()` instead."]
#[js::import = "None"]
pub fn position_x(&mut self, val: f32) -> &mut Self { self.set_position_x(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_position_y()` instead."]
#[js::import = "None"]
pub fn position_y(&mut self, val: f32) -> &mut Self { self.set_position_y(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_position_z()` instead."]
#[js::import = "None"]
pub fn position_z(&mut self, val: f32) -> &mut Self { self.set_position_z(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_ref_distance()` instead."]
#[js::import = "None"]
pub fn ref_distance(&mut self, val: f64) -> &mut Self { self.set_ref_distance(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_rolloff_factor()` instead."]
#[js::import = "None"]
pub fn rolloff_factor(&mut self, val: f64) -> &mut Self { self.set_rolloff_factor(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for PannerOptions {
    fn default() -> Self {
        Self::new()
    }
}
