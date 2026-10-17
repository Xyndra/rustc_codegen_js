#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaConfiguration` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaConfiguration`*"]
#[js::opaque]
#[derive(Copy)]
pub struct MediaConfiguration(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "AudioConfiguration")]
    #[doc = "Get the `audio` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioConfiguration`, `MediaConfiguration`*"]
#[js::import = "None"]
pub fn get_audio(this: &MediaConfiguration) -> Option<AudioConfiguration> { panic!("web-sys intrinsic") }
    #[cfg(feature = "AudioConfiguration")]
    #[doc = "Change the `audio` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioConfiguration`, `MediaConfiguration`*"]
#[js::import = "None = $1"]
pub fn set_audio(this: &MediaConfiguration, val: &AudioConfiguration) { panic!("web-sys intrinsic") }
    #[cfg(feature = "VideoConfiguration")]
    #[doc = "Get the `video` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaConfiguration`, `VideoConfiguration`*"]
#[js::import = "None"]
pub fn get_video(this: &MediaConfiguration) -> Option<VideoConfiguration> { panic!("web-sys intrinsic") }
    #[cfg(feature = "VideoConfiguration")]
    #[doc = "Change the `video` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaConfiguration`, `VideoConfiguration`*"]
#[js::import = "None = $1"]
pub fn set_video(this: &MediaConfiguration, val: &VideoConfiguration) { panic!("web-sys intrinsic") }
impl MediaConfiguration {
    #[doc = "Construct a new `MediaConfiguration`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaConfiguration`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[cfg(feature = "AudioConfiguration")]
    #[deprecated = "Use `set_audio()` instead."]
#[js::import = "None"]
pub fn audio(&mut self, val: &AudioConfiguration) -> &mut Self { self.set_audio(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(feature = "VideoConfiguration")]
    #[deprecated = "Use `set_video()` instead."]
#[js::import = "None"]
pub fn video(&mut self, val: &VideoConfiguration) -> &mut Self { self.set_video(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for MediaConfiguration {
    fn default() -> Self {
        Self::new()
    }
}
