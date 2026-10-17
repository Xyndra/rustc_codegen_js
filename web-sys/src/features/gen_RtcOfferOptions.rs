#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcOfferOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOfferOptions`*"]
#[js::opaque]
#[derive(Copy)]
pub struct RtcOfferOptions(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `iceRestart` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOfferOptions`*"]
#[js::import = "None"]
pub fn get_ice_restart(this: &RtcOfferOptions) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `iceRestart` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOfferOptions`*"]
#[js::import = "None = $1"]
pub fn set_ice_restart(this: &RtcOfferOptions, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `offerToReceiveAudio` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOfferOptions`*"]
#[js::import = "None"]
pub fn get_offer_to_receive_audio(this: &RtcOfferOptions) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `offerToReceiveAudio` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOfferOptions`*"]
#[js::import = "None = $1"]
pub fn set_offer_to_receive_audio(this: &RtcOfferOptions, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `offerToReceiveVideo` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOfferOptions`*"]
#[js::import = "None"]
pub fn get_offer_to_receive_video(this: &RtcOfferOptions) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `offerToReceiveVideo` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOfferOptions`*"]
#[js::import = "None = $1"]
pub fn set_offer_to_receive_video(this: &RtcOfferOptions, val: bool) { panic!("web-sys intrinsic") }
impl RtcOfferOptions {
    #[doc = "Construct a new `RtcOfferOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOfferOptions`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_ice_restart()` instead."]
#[js::import = "None"]
pub fn ice_restart(&mut self, val: bool) -> &mut Self { self.set_ice_restart(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_offer_to_receive_audio()` instead."]
#[js::import = "None"]
pub fn offer_to_receive_audio(&mut self, val: bool) -> &mut Self { self.set_offer_to_receive_audio(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_offer_to_receive_video()` instead."]
#[js::import = "None"]
pub fn offer_to_receive_video(&mut self, val: bool) -> &mut Self { self.set_offer_to_receive_video(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for RtcOfferOptions {
    fn default() -> Self {
        Self::new()
    }
}
