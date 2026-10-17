#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `VideoConfiguration` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoConfiguration`*"]
#[js::opaque]
#[derive(Copy)]
pub struct VideoConfiguration(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `bitrate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoConfiguration`*"]
#[js::import = "None"]
pub fn get_bitrate(this: &VideoConfiguration) -> Option<f64> { panic!("web-sys intrinsic") }
    #[doc = "Change the `bitrate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoConfiguration`*"]
#[js::import = "None = $1"]
pub fn set_bitrate(this: &VideoConfiguration, val: f64) { panic!("web-sys intrinsic") }
    #[doc = "Get the `contentType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoConfiguration`*"]
#[js::import = "None"]
pub fn get_content_type(this: &VideoConfiguration) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `contentType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoConfiguration`*"]
#[js::import = "None = $1"]
pub fn set_content_type(this: &VideoConfiguration, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `framerate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoConfiguration`*"]
#[js::import = "None"]
pub fn get_framerate(this: &VideoConfiguration) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `framerate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoConfiguration`*"]
#[js::import = "None = $1"]
pub fn set_framerate(this: &VideoConfiguration, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `height` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoConfiguration`*"]
#[js::import = "None"]
pub fn get_height(this: &VideoConfiguration) -> Option<u32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `height` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoConfiguration`*"]
#[js::import = "None = $1"]
pub fn set_height(this: &VideoConfiguration, val: u32) { panic!("web-sys intrinsic") }
    #[doc = "Get the `width` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoConfiguration`*"]
#[js::import = "None"]
pub fn get_width(this: &VideoConfiguration) -> Option<u32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `width` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoConfiguration`*"]
#[js::import = "None = $1"]
pub fn set_width(this: &VideoConfiguration, val: u32) { panic!("web-sys intrinsic") }
impl VideoConfiguration {
    #[doc = "Construct a new `VideoConfiguration`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoConfiguration`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_bitrate()` instead."]
#[js::import = "None"]
pub fn bitrate(&mut self, val: f64) -> &mut Self { self.set_bitrate(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_content_type()` instead."]
#[js::import = "None"]
pub fn content_type(&mut self, val: &str) -> &mut Self { self.set_content_type(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_framerate()` instead."]
#[js::import = "None"]
pub fn framerate(&mut self, val: &str) -> &mut Self { self.set_framerate(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_height()` instead."]
#[js::import = "None"]
pub fn height(&mut self, val: u32) -> &mut Self { self.set_height(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_width()` instead."]
#[js::import = "None"]
pub fn width(&mut self, val: u32) -> &mut Self { self.set_width(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for VideoConfiguration {
    fn default() -> Self {
        Self::new()
    }
}
