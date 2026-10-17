#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaDecodingConfiguration` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaDecodingConfiguration`*"]
#[js::opaque]
#[derive(Copy)]
pub struct MediaDecodingConfiguration(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "AudioConfiguration")]
    #[doc = "Get the `audio` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioConfiguration`, `MediaDecodingConfiguration`*"]
#[js::import = "None"]
pub fn get_audio(this: &MediaDecodingConfiguration) -> Option<AudioConfiguration> { panic!("web-sys intrinsic") }
    #[cfg(feature = "AudioConfiguration")]
    #[doc = "Change the `audio` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioConfiguration`, `MediaDecodingConfiguration`*"]
#[js::import = "None = $1"]
pub fn set_audio(this: &MediaDecodingConfiguration, val: &AudioConfiguration) { panic!("web-sys intrinsic") }
    #[cfg(feature = "VideoConfiguration")]
    #[doc = "Get the `video` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaDecodingConfiguration`, `VideoConfiguration`*"]
#[js::import = "None"]
pub fn get_video(this: &MediaDecodingConfiguration) -> Option<VideoConfiguration> { panic!("web-sys intrinsic") }
    #[cfg(feature = "VideoConfiguration")]
    #[doc = "Change the `video` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaDecodingConfiguration`, `VideoConfiguration`*"]
#[js::import = "None = $1"]
pub fn set_video(this: &MediaDecodingConfiguration, val: &VideoConfiguration) { panic!("web-sys intrinsic") }
    #[cfg(feature = "MediaDecodingType")]
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaDecodingConfiguration`, `MediaDecodingType`*"]
#[js::import = "None"]
pub fn get_type(this: &MediaDecodingConfiguration) -> MediaDecodingType { panic!("web-sys intrinsic") }
    #[cfg(feature = "MediaDecodingType")]
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaDecodingConfiguration`, `MediaDecodingType`*"]
#[js::import = "None = $1"]
pub fn set_type(this: &MediaDecodingConfiguration, val: MediaDecodingType) { panic!("web-sys intrinsic") }
impl MediaDecodingConfiguration {
    #[cfg(feature = "MediaDecodingType")]
    #[doc = "Construct a new `MediaDecodingConfiguration`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaDecodingConfiguration`, `MediaDecodingType`*"]
#[js::import = "None"]
pub fn new(type_: MediaDecodingType) -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret.set_type(type_);
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
    #[cfg(feature = "MediaDecodingType")]
    #[deprecated = "Use `set_type()` instead."]
#[js::import = "None"]
pub fn type_(&mut self, val: MediaDecodingType) -> &mut Self { self.set_type(val) { panic!("web-sys intrinsic") }
        self
    }
}
