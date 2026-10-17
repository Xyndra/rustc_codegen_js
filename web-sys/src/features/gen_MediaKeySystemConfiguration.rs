#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaKeySystemConfiguration` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySystemConfiguration`*"]
#[js::opaque]
#[derive(Copy)]
pub struct MediaKeySystemConfiguration(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `audioCapabilities` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySystemConfiguration`*"]
#[js::import = "None"]
pub fn get_audio_capabilities(this: &MediaKeySystemConfiguration) -> Option<::js_sys::Array> { panic!("web-sys intrinsic") }
    #[doc = "Change the `audioCapabilities` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySystemConfiguration`*"]
#[js::import = "None = $1"]
pub fn set_audio_capabilities(this: &MediaKeySystemConfiguration, val: &::wasm_bindgen::JsValue,) { panic!("web-sys intrinsic") }
    #[cfg(feature = "MediaKeysRequirement")]
    #[doc = "Get the `distinctiveIdentifier` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySystemConfiguration`, `MediaKeysRequirement`*"]
#[js::import = "None"]
pub fn get_distinctive_identifier(this: &MediaKeySystemConfiguration,) -> Option<MediaKeysRequirement> { panic!("web-sys intrinsic") }
    #[cfg(feature = "MediaKeysRequirement")]
    #[doc = "Change the `distinctiveIdentifier` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySystemConfiguration`, `MediaKeysRequirement`*"]
#[js::import = "None = $1"]
pub fn set_distinctive_identifier(this: &MediaKeySystemConfiguration, val: MediaKeysRequirement,) { panic!("web-sys intrinsic") }
    #[doc = "Get the `initDataTypes` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySystemConfiguration`*"]
#[js::import = "None"]
pub fn get_init_data_types(this: &MediaKeySystemConfiguration) -> Option<::js_sys::Array> { panic!("web-sys intrinsic") }
    #[doc = "Change the `initDataTypes` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySystemConfiguration`*"]
#[js::import = "None = $1"]
pub fn set_init_data_types(this: &MediaKeySystemConfiguration, val: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
    #[doc = "Get the `label` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySystemConfiguration`*"]
#[js::import = "None"]
pub fn get_label(this: &MediaKeySystemConfiguration) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `label` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySystemConfiguration`*"]
#[js::import = "None = $1"]
pub fn set_label(this: &MediaKeySystemConfiguration, val: &str) { panic!("web-sys intrinsic") }
    #[cfg(feature = "MediaKeysRequirement")]
    #[doc = "Get the `persistentState` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySystemConfiguration`, `MediaKeysRequirement`*"]
#[js::import = "None"]
pub fn get_persistent_state(this: &MediaKeySystemConfiguration) -> Option<MediaKeysRequirement> { panic!("web-sys intrinsic") }
    #[cfg(feature = "MediaKeysRequirement")]
    #[doc = "Change the `persistentState` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySystemConfiguration`, `MediaKeysRequirement`*"]
#[js::import = "None = $1"]
pub fn set_persistent_state(this: &MediaKeySystemConfiguration, val: MediaKeysRequirement) { panic!("web-sys intrinsic") }
    #[doc = "Get the `sessionTypes` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySystemConfiguration`*"]
#[js::import = "None"]
pub fn get_session_types(this: &MediaKeySystemConfiguration) -> Option<::js_sys::Array> { panic!("web-sys intrinsic") }
    #[doc = "Change the `sessionTypes` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySystemConfiguration`*"]
#[js::import = "None = $1"]
pub fn set_session_types(this: &MediaKeySystemConfiguration, val: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
    #[doc = "Get the `videoCapabilities` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySystemConfiguration`*"]
#[js::import = "None"]
pub fn get_video_capabilities(this: &MediaKeySystemConfiguration) -> Option<::js_sys::Array> { panic!("web-sys intrinsic") }
    #[doc = "Change the `videoCapabilities` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySystemConfiguration`*"]
#[js::import = "None = $1"]
pub fn set_video_capabilities(this: &MediaKeySystemConfiguration, val: &::wasm_bindgen::JsValue,) { panic!("web-sys intrinsic") }
impl MediaKeySystemConfiguration {
    #[doc = "Construct a new `MediaKeySystemConfiguration`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySystemConfiguration`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_audio_capabilities()` instead."]
#[js::import = "None"]
pub fn audio_capabilities(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self { self.set_audio_capabilities(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(feature = "MediaKeysRequirement")]
    #[deprecated = "Use `set_distinctive_identifier()` instead."]
#[js::import = "None"]
pub fn distinctive_identifier(&mut self, val: MediaKeysRequirement) -> &mut Self { self.set_distinctive_identifier(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_init_data_types()` instead."]
#[js::import = "None"]
pub fn init_data_types(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self { self.set_init_data_types(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_label()` instead."]
#[js::import = "None"]
pub fn label(&mut self, val: &str) -> &mut Self { self.set_label(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(feature = "MediaKeysRequirement")]
    #[deprecated = "Use `set_persistent_state()` instead."]
#[js::import = "None"]
pub fn persistent_state(&mut self, val: MediaKeysRequirement) -> &mut Self { self.set_persistent_state(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_session_types()` instead."]
#[js::import = "None"]
pub fn session_types(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self { self.set_session_types(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_video_capabilities()` instead."]
#[js::import = "None"]
pub fn video_capabilities(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self { self.set_video_capabilities(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for MediaKeySystemConfiguration {
    fn default() -> Self {
        Self::new()
    }
}
