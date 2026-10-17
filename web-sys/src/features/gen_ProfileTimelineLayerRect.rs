#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ProfileTimelineLayerRect` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineLayerRect`*"]
#[js::opaque]
#[derive(Copy)]
pub struct ProfileTimelineLayerRect(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `height` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineLayerRect`*"]
#[js::import = "None"]
pub fn get_height(this: &ProfileTimelineLayerRect) -> Option<i32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `height` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineLayerRect`*"]
#[js::import = "None = $1"]
pub fn set_height(this: &ProfileTimelineLayerRect, val: i32) { panic!("web-sys intrinsic") }
    #[doc = "Get the `width` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineLayerRect`*"]
#[js::import = "None"]
pub fn get_width(this: &ProfileTimelineLayerRect) -> Option<i32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `width` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineLayerRect`*"]
#[js::import = "None = $1"]
pub fn set_width(this: &ProfileTimelineLayerRect, val: i32) { panic!("web-sys intrinsic") }
    #[doc = "Get the `x` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineLayerRect`*"]
#[js::import = "None"]
pub fn get_x(this: &ProfileTimelineLayerRect) -> Option<i32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `x` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineLayerRect`*"]
#[js::import = "None = $1"]
pub fn set_x(this: &ProfileTimelineLayerRect, val: i32) { panic!("web-sys intrinsic") }
    #[doc = "Get the `y` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineLayerRect`*"]
#[js::import = "None"]
pub fn get_y(this: &ProfileTimelineLayerRect) -> Option<i32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `y` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineLayerRect`*"]
#[js::import = "None = $1"]
pub fn set_y(this: &ProfileTimelineLayerRect, val: i32) { panic!("web-sys intrinsic") }
impl ProfileTimelineLayerRect {
    #[doc = "Construct a new `ProfileTimelineLayerRect`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineLayerRect`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_height()` instead."]
#[js::import = "None"]
pub fn height(&mut self, val: i32) -> &mut Self { self.set_height(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_width()` instead."]
#[js::import = "None"]
pub fn width(&mut self, val: i32) -> &mut Self { self.set_width(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_x()` instead."]
#[js::import = "None"]
pub fn x(&mut self, val: i32) -> &mut Self { self.set_x(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_y()` instead."]
#[js::import = "None"]
pub fn y(&mut self, val: i32) -> &mut Self { self.set_y(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for ProfileTimelineLayerRect {
    fn default() -> Self {
        Self::new()
    }
}
