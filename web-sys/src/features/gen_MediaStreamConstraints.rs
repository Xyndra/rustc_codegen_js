#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaStreamConstraints` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamConstraints`*"]
#[js::opaque]
#[derive(Copy)]
pub struct MediaStreamConstraints(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `audio` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamConstraints`*"]
#[js::import = "None"]
pub fn get_audio(this: &MediaStreamConstraints) -> ::wasm_bindgen::JsValue { panic!("web-sys intrinsic") }
    #[doc = "Change the `audio` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamConstraints`*"]
#[js::import = "None = $1"]
pub fn set_audio(this: &MediaStreamConstraints, val: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
    #[doc = "Get the `fake` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamConstraints`*"]
#[js::import = "None"]
pub fn get_fake(this: &MediaStreamConstraints) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `fake` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamConstraints`*"]
#[js::import = "None = $1"]
pub fn set_fake(this: &MediaStreamConstraints, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `peerIdentity` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamConstraints`*"]
#[js::import = "None"]
pub fn get_peer_identity(this: &MediaStreamConstraints) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `peerIdentity` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamConstraints`*"]
#[js::import = "None = $1"]
pub fn set_peer_identity(this: &MediaStreamConstraints, val: Option<&str>) { panic!("web-sys intrinsic") }
    #[doc = "Get the `picture` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamConstraints`*"]
#[js::import = "None"]
pub fn get_picture(this: &MediaStreamConstraints) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `picture` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamConstraints`*"]
#[js::import = "None = $1"]
pub fn set_picture(this: &MediaStreamConstraints, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `video` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamConstraints`*"]
#[js::import = "None"]
pub fn get_video(this: &MediaStreamConstraints) -> ::wasm_bindgen::JsValue { panic!("web-sys intrinsic") }
    #[doc = "Change the `video` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamConstraints`*"]
#[js::import = "None = $1"]
pub fn set_video(this: &MediaStreamConstraints, val: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
impl MediaStreamConstraints {
    #[doc = "Construct a new `MediaStreamConstraints`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamConstraints`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_audio()` instead."]
#[js::import = "None"]
pub fn audio(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self { self.set_audio(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_fake()` instead."]
#[js::import = "None"]
pub fn fake(&mut self, val: bool) -> &mut Self { self.set_fake(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_peer_identity()` instead."]
#[js::import = "None"]
pub fn peer_identity(&mut self, val: Option<&str>) -> &mut Self { self.set_peer_identity(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_picture()` instead."]
#[js::import = "None"]
pub fn picture(&mut self, val: bool) -> &mut Self { self.set_picture(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_video()` instead."]
#[js::import = "None"]
pub fn video(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self { self.set_video(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for MediaStreamConstraints {
    fn default() -> Self {
        Self::new()
    }
}
