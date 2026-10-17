#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcConfiguration` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcConfiguration`*"]
#[js::opaque]
#[derive(Copy)]
pub struct RtcConfiguration(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "RtcBundlePolicy")]
    #[doc = "Get the `bundlePolicy` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcBundlePolicy`, `RtcConfiguration`*"]
#[js::import = "None"]
pub fn get_bundle_policy(this: &RtcConfiguration) -> Option<RtcBundlePolicy> { panic!("web-sys intrinsic") }
    #[cfg(feature = "RtcBundlePolicy")]
    #[doc = "Change the `bundlePolicy` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcBundlePolicy`, `RtcConfiguration`*"]
#[js::import = "None = $1"]
pub fn set_bundle_policy(this: &RtcConfiguration, val: RtcBundlePolicy) { panic!("web-sys intrinsic") }
    #[doc = "Get the `certificates` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcConfiguration`*"]
#[js::import = "None"]
pub fn get_certificates(this: &RtcConfiguration) -> Option<::js_sys::Array> { panic!("web-sys intrinsic") }
    #[doc = "Change the `certificates` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcConfiguration`*"]
#[js::import = "None = $1"]
pub fn set_certificates(this: &RtcConfiguration, val: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
    #[doc = "Get the `iceServers` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcConfiguration`*"]
#[js::import = "None"]
pub fn get_ice_servers(this: &RtcConfiguration) -> Option<::js_sys::Array> { panic!("web-sys intrinsic") }
    #[doc = "Change the `iceServers` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcConfiguration`*"]
#[js::import = "None = $1"]
pub fn set_ice_servers(this: &RtcConfiguration, val: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
    #[cfg(feature = "RtcIceTransportPolicy")]
    #[doc = "Get the `iceTransportPolicy` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcConfiguration`, `RtcIceTransportPolicy`*"]
#[js::import = "None"]
pub fn get_ice_transport_policy(this: &RtcConfiguration) -> Option<RtcIceTransportPolicy> { panic!("web-sys intrinsic") }
    #[cfg(feature = "RtcIceTransportPolicy")]
    #[doc = "Change the `iceTransportPolicy` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcConfiguration`, `RtcIceTransportPolicy`*"]
#[js::import = "None = $1"]
pub fn set_ice_transport_policy(this: &RtcConfiguration, val: RtcIceTransportPolicy) { panic!("web-sys intrinsic") }
    #[doc = "Get the `peerIdentity` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcConfiguration`*"]
#[js::import = "None"]
pub fn get_peer_identity(this: &RtcConfiguration) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `peerIdentity` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcConfiguration`*"]
#[js::import = "None = $1"]
pub fn set_peer_identity(this: &RtcConfiguration, val: Option<&str>) { panic!("web-sys intrinsic") }
impl RtcConfiguration {
    #[doc = "Construct a new `RtcConfiguration`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcConfiguration`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[cfg(feature = "RtcBundlePolicy")]
    #[deprecated = "Use `set_bundle_policy()` instead."]
#[js::import = "None"]
pub fn bundle_policy(&mut self, val: RtcBundlePolicy) -> &mut Self { self.set_bundle_policy(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_certificates()` instead."]
#[js::import = "None"]
pub fn certificates(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self { self.set_certificates(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_ice_servers()` instead."]
#[js::import = "None"]
pub fn ice_servers(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self { self.set_ice_servers(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(feature = "RtcIceTransportPolicy")]
    #[deprecated = "Use `set_ice_transport_policy()` instead."]
#[js::import = "None"]
pub fn ice_transport_policy(&mut self, val: RtcIceTransportPolicy) -> &mut Self { self.set_ice_transport_policy(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_peer_identity()` instead."]
#[js::import = "None"]
pub fn peer_identity(&mut self, val: Option<&str>) -> &mut Self { self.set_peer_identity(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for RtcConfiguration {
    fn default() -> Self {
        Self::new()
    }
}
