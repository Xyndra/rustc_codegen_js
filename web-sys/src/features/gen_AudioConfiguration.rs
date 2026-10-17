#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AudioConfiguration` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioConfiguration`*"]
#[js::opaque]
#[derive(Copy)]
pub struct AudioConfiguration(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `bitrate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioConfiguration`*"]
#[js::import = "None"]
pub fn get_bitrate(this: &AudioConfiguration) -> Option<f64> { panic!("web-sys intrinsic") }
    #[doc = "Change the `bitrate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioConfiguration`*"]
#[js::import = "None = $1"]
pub fn set_bitrate(this: &AudioConfiguration, val: f64) { panic!("web-sys intrinsic") }
    #[doc = "Get the `channels` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioConfiguration`*"]
#[js::import = "None"]
pub fn get_channels(this: &AudioConfiguration) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `channels` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioConfiguration`*"]
#[js::import = "None = $1"]
pub fn set_channels(this: &AudioConfiguration, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `contentType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioConfiguration`*"]
#[js::import = "None"]
pub fn get_content_type(this: &AudioConfiguration) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `contentType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioConfiguration`*"]
#[js::import = "None = $1"]
pub fn set_content_type(this: &AudioConfiguration, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `samplerate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioConfiguration`*"]
#[js::import = "None"]
pub fn get_samplerate(this: &AudioConfiguration) -> Option<u32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `samplerate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioConfiguration`*"]
#[js::import = "None = $1"]
pub fn set_samplerate(this: &AudioConfiguration, val: u32) { panic!("web-sys intrinsic") }
impl AudioConfiguration {
    #[doc = "Construct a new `AudioConfiguration`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioConfiguration`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_bitrate()` instead."]
#[js::import = "None"]
pub fn bitrate(&mut self, val: f64) -> &mut Self { self.set_bitrate(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_channels()` instead."]
#[js::import = "None"]
pub fn channels(&mut self, val: &str) -> &mut Self { self.set_channels(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_content_type()` instead."]
#[js::import = "None"]
pub fn content_type(&mut self, val: &str) -> &mut Self { self.set_content_type(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_samplerate()` instead."]
#[js::import = "None"]
pub fn samplerate(&mut self, val: u32) -> &mut Self { self.set_samplerate(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for AudioConfiguration {
    fn default() -> Self {
        Self::new()
    }
}
