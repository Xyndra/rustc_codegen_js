#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaStreamAudioSourceOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamAudioSourceOptions`*"]
#[js::opaque]
#[derive(Copy)]
pub struct MediaStreamAudioSourceOptions(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "MediaStream")]
    #[doc = "Get the `mediaStream` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStream`, `MediaStreamAudioSourceOptions`*"]
#[js::import = "None"]
pub fn get_media_stream(this: &MediaStreamAudioSourceOptions) -> MediaStream { panic!("web-sys intrinsic") }
    #[cfg(feature = "MediaStream")]
    #[doc = "Change the `mediaStream` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStream`, `MediaStreamAudioSourceOptions`*"]
#[js::import = "None = $1"]
pub fn set_media_stream(this: &MediaStreamAudioSourceOptions, val: &MediaStream) { panic!("web-sys intrinsic") }
impl MediaStreamAudioSourceOptions {
    #[cfg(feature = "MediaStream")]
    #[doc = "Construct a new `MediaStreamAudioSourceOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStream`, `MediaStreamAudioSourceOptions`*"]
#[js::import = "None"]
pub fn new(media_stream: &MediaStream) -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret.set_media_stream(media_stream);
        ret
    }
    #[cfg(feature = "MediaStream")]
    #[deprecated = "Use `set_media_stream()` instead."]
#[js::import = "None"]
pub fn media_stream(&mut self, val: &MediaStream) -> &mut Self { self.set_media_stream(val) { panic!("web-sys intrinsic") }
        self
    }
}
