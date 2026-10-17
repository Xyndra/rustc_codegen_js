#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AuthenticationExtensionsClientOutputs` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientOutputs`*"]
#[js::opaque]
#[derive(Copy)]
pub struct AuthenticationExtensionsClientOutputs(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `appid` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientOutputs`*"]
#[js::import = "None"]
pub fn get_appid(this: &AuthenticationExtensionsClientOutputs) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `appid` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientOutputs`*"]
#[js::import = "None = $1"]
pub fn set_appid(this: &AuthenticationExtensionsClientOutputs, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `appidExclude` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientOutputs`*"]
#[js::import = "None"]
pub fn get_appid_exclude(this: &AuthenticationExtensionsClientOutputs) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `appidExclude` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientOutputs`*"]
#[js::import = "None = $1"]
pub fn set_appid_exclude(this: &AuthenticationExtensionsClientOutputs, val: bool) { panic!("web-sys intrinsic") }
    #[cfg(feature = "CredentialPropertiesOutput")]
    #[doc = "Get the `credProps` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientOutputs`, `CredentialPropertiesOutput`*"]
#[js::import = "None"]
pub fn get_cred_props(this: &AuthenticationExtensionsClientOutputs,) -> Option<CredentialPropertiesOutput> { panic!("web-sys intrinsic") }
    #[cfg(feature = "CredentialPropertiesOutput")]
    #[doc = "Change the `credProps` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientOutputs`, `CredentialPropertiesOutput`*"]
#[js::import = "None = $1"]
pub fn set_cred_props(this: &AuthenticationExtensionsClientOutputs, val: &CredentialPropertiesOutput,) { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "AuthenticationExtensionsDevicePublicKeyOutputs")]
    #[doc = "Get the `devicePubKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientOutputs`, `AuthenticationExtensionsDevicePublicKeyOutputs`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None"]
pub fn get_device_pub_key(this: &AuthenticationExtensionsClientOutputs,) -> Option<AuthenticationExtensionsDevicePublicKeyOutputs> { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "AuthenticationExtensionsDevicePublicKeyOutputs")]
    #[doc = "Change the `devicePubKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientOutputs`, `AuthenticationExtensionsDevicePublicKeyOutputs`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None = $1"]
pub fn set_device_pub_key(this: &AuthenticationExtensionsClientOutputs, val: &AuthenticationExtensionsDevicePublicKeyOutputs,) { panic!("web-sys intrinsic") }
    #[cfg(feature = "AuthenticationExtensionsLargeBlobOutputs")]
    #[doc = "Get the `largeBlob` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientOutputs`, `AuthenticationExtensionsLargeBlobOutputs`*"]
#[js::import = "None"]
pub fn get_large_blob(this: &AuthenticationExtensionsClientOutputs,) -> Option<AuthenticationExtensionsLargeBlobOutputs> { panic!("web-sys intrinsic") }
    #[cfg(feature = "AuthenticationExtensionsLargeBlobOutputs")]
    #[doc = "Change the `largeBlob` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientOutputs`, `AuthenticationExtensionsLargeBlobOutputs`*"]
#[js::import = "None = $1"]
pub fn set_large_blob(this: &AuthenticationExtensionsClientOutputs, val: &AuthenticationExtensionsLargeBlobOutputs,) { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "AuthenticationExtensionsPrfOutputs")]
    #[doc = "Get the `prf` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientOutputs`, `AuthenticationExtensionsPrfOutputs`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None"]
pub fn get_prf(this: &AuthenticationExtensionsClientOutputs,) -> Option<AuthenticationExtensionsPrfOutputs> { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "AuthenticationExtensionsPrfOutputs")]
    #[doc = "Change the `prf` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientOutputs`, `AuthenticationExtensionsPrfOutputs`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None = $1"]
pub fn set_prf(this: &AuthenticationExtensionsClientOutputs, val: &AuthenticationExtensionsPrfOutputs,) { panic!("web-sys intrinsic") }
    #[doc = "Get the `uvm` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientOutputs`*"]
#[js::import = "None"]
pub fn get_uvm(this: &AuthenticationExtensionsClientOutputs) -> Option<::js_sys::Array> { panic!("web-sys intrinsic") }
    #[doc = "Change the `uvm` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientOutputs`*"]
#[js::import = "None = $1"]
pub fn set_uvm(this: &AuthenticationExtensionsClientOutputs, val: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
impl AuthenticationExtensionsClientOutputs {
    #[doc = "Construct a new `AuthenticationExtensionsClientOutputs`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientOutputs`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_appid()` instead."]
#[js::import = "None"]
pub fn appid(&mut self, val: bool) -> &mut Self { self.set_appid(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_appid_exclude()` instead."]
#[js::import = "None"]
pub fn appid_exclude(&mut self, val: bool) -> &mut Self { self.set_appid_exclude(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(feature = "CredentialPropertiesOutput")]
    #[deprecated = "Use `set_cred_props()` instead."]
#[js::import = "None"]
pub fn cred_props(&mut self, val: &CredentialPropertiesOutput) -> &mut Self { self.set_cred_props(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "AuthenticationExtensionsDevicePublicKeyOutputs")]
    #[deprecated = "Use `set_device_pub_key()` instead."]
#[js::import = "None"]
pub fn device_pub_key(&mut self, val: &AuthenticationExtensionsDevicePublicKeyOutputs,) -> &mut Self { self.set_device_pub_key(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(feature = "AuthenticationExtensionsLargeBlobOutputs")]
    #[deprecated = "Use `set_large_blob()` instead."]
#[js::import = "None"]
pub fn large_blob(&mut self, val: &AuthenticationExtensionsLargeBlobOutputs) -> &mut Self { self.set_large_blob(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "AuthenticationExtensionsPrfOutputs")]
    #[deprecated = "Use `set_prf()` instead."]
#[js::import = "None"]
pub fn prf(&mut self, val: &AuthenticationExtensionsPrfOutputs) -> &mut Self { self.set_prf(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_uvm()` instead."]
#[js::import = "None"]
pub fn uvm(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self { self.set_uvm(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for AuthenticationExtensionsClientOutputs {
    fn default() -> Self {
        Self::new()
    }
}
