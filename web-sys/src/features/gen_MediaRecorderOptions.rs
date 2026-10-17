#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaRecorderOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaRecorderOptions`*"]
#[js::opaque]
#[derive(Copy)]
pub struct MediaRecorderOptions(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `audioBitsPerSecond` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaRecorderOptions`*"]
#[js::import = "None"]
pub fn get_audio_bits_per_second(this: &MediaRecorderOptions) -> Option<u32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `audioBitsPerSecond` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaRecorderOptions`*"]
#[js::import = "None = $1"]
pub fn set_audio_bits_per_second(this: &MediaRecorderOptions, val: u32) { panic!("web-sys intrinsic") }
    #[doc = "Get the `bitsPerSecond` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaRecorderOptions`*"]
#[js::import = "None"]
pub fn get_bits_per_second(this: &MediaRecorderOptions) -> Option<u32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `bitsPerSecond` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaRecorderOptions`*"]
#[js::import = "None = $1"]
pub fn set_bits_per_second(this: &MediaRecorderOptions, val: u32) { panic!("web-sys intrinsic") }
    #[doc = "Get the `mimeType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaRecorderOptions`*"]
#[js::import = "None"]
pub fn get_mime_type(this: &MediaRecorderOptions) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `mimeType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaRecorderOptions`*"]
#[js::import = "None = $1"]
pub fn set_mime_type(this: &MediaRecorderOptions, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `videoBitsPerSecond` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaRecorderOptions`*"]
#[js::import = "None"]
pub fn get_video_bits_per_second(this: &MediaRecorderOptions) -> Option<u32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `videoBitsPerSecond` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaRecorderOptions`*"]
#[js::import = "None = $1"]
pub fn set_video_bits_per_second(this: &MediaRecorderOptions, val: u32) { panic!("web-sys intrinsic") }
impl MediaRecorderOptions {
    #[doc = "Construct a new `MediaRecorderOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaRecorderOptions`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_audio_bits_per_second()` instead."]
#[js::import = "None"]
pub fn audio_bits_per_second(&mut self, val: u32) -> &mut Self { self.set_audio_bits_per_second(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_bits_per_second()` instead."]
#[js::import = "None"]
pub fn bits_per_second(&mut self, val: u32) -> &mut Self { self.set_bits_per_second(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_mime_type()` instead."]
#[js::import = "None"]
pub fn mime_type(&mut self, val: &str) -> &mut Self { self.set_mime_type(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_video_bits_per_second()` instead."]
#[js::import = "None"]
pub fn video_bits_per_second(&mut self, val: u32) -> &mut Self { self.set_video_bits_per_second(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for MediaRecorderOptions {
    fn default() -> Self {
        Self::new()
    }
}
