#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaEncodingConfiguration` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaEncodingConfiguration`*"]
#[js::opaque]
#[derive(Copy)]
pub struct MediaEncodingConfiguration(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "AudioConfiguration")]
    #[doc = "Get the `audio` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioConfiguration`, `MediaEncodingConfiguration`*"]
#[js::import = "None"]
pub fn get_audio(this: &MediaEncodingConfiguration) -> Option<AudioConfiguration> { panic!("web-sys intrinsic") }
    #[cfg(feature = "AudioConfiguration")]
    #[doc = "Change the `audio` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioConfiguration`, `MediaEncodingConfiguration`*"]
#[js::import = "None = $1"]
pub fn set_audio(this: &MediaEncodingConfiguration, val: &AudioConfiguration) { panic!("web-sys intrinsic") }
    #[cfg(feature = "VideoConfiguration")]
    #[doc = "Get the `video` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaEncodingConfiguration`, `VideoConfiguration`*"]
#[js::import = "None"]
pub fn get_video(this: &MediaEncodingConfiguration) -> Option<VideoConfiguration> { panic!("web-sys intrinsic") }
    #[cfg(feature = "VideoConfiguration")]
    #[doc = "Change the `video` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaEncodingConfiguration`, `VideoConfiguration`*"]
#[js::import = "None = $1"]
pub fn set_video(this: &MediaEncodingConfiguration, val: &VideoConfiguration) { panic!("web-sys intrinsic") }
    #[cfg(feature = "MediaEncodingType")]
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaEncodingConfiguration`, `MediaEncodingType`*"]
#[js::import = "None"]
pub fn get_type(this: &MediaEncodingConfiguration) -> MediaEncodingType { panic!("web-sys intrinsic") }
    #[cfg(feature = "MediaEncodingType")]
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaEncodingConfiguration`, `MediaEncodingType`*"]
#[js::import = "None = $1"]
pub fn set_type(this: &MediaEncodingConfiguration, val: MediaEncodingType) { panic!("web-sys intrinsic") }
impl MediaEncodingConfiguration {
    #[cfg(feature = "MediaEncodingType")]
    #[doc = "Construct a new `MediaEncodingConfiguration`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaEncodingConfiguration`, `MediaEncodingType`*"]
#[js::import = "None"]
pub fn new(type_: MediaEncodingType) -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
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
    #[cfg(feature = "MediaEncodingType")]
    #[deprecated = "Use `set_type()` instead."]
#[js::import = "None"]
pub fn type_(&mut self, val: MediaEncodingType) -> &mut Self { self.set_type(val) { panic!("web-sys intrinsic") }
        self
    }
}
