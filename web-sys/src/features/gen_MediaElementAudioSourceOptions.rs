#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaElementAudioSourceOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaElementAudioSourceOptions`*"]
#[js::opaque]
#[derive(Copy)]
pub struct MediaElementAudioSourceOptions(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "HtmlMediaElement")]
    #[doc = "Get the `mediaElement` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`, `MediaElementAudioSourceOptions`*"]
#[js::import = "None"]
pub fn get_media_element(this: &MediaElementAudioSourceOptions) -> HtmlMediaElement { panic!("web-sys intrinsic") }
    #[cfg(feature = "HtmlMediaElement")]
    #[doc = "Change the `mediaElement` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`, `MediaElementAudioSourceOptions`*"]
#[js::import = "None = $1"]
pub fn set_media_element(this: &MediaElementAudioSourceOptions, val: &HtmlMediaElement) { panic!("web-sys intrinsic") }
impl MediaElementAudioSourceOptions {
    #[cfg(feature = "HtmlMediaElement")]
    #[doc = "Construct a new `MediaElementAudioSourceOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`, `MediaElementAudioSourceOptions`*"]
#[js::import = "None"]
pub fn new(media_element: &HtmlMediaElement) -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret.set_media_element(media_element);
        ret
    }
    #[cfg(feature = "HtmlMediaElement")]
    #[deprecated = "Use `set_media_element()` instead."]
#[js::import = "None"]
pub fn media_element(&mut self, val: &HtmlMediaElement) -> &mut Self { self.set_media_element(val) { panic!("web-sys intrinsic") }
        self
    }
}
