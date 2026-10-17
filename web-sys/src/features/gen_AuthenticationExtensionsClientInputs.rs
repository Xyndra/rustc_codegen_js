#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AuthenticationExtensionsClientInputs` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientInputs`*"]
#[js::opaque]
#[derive(Copy)]
pub struct AuthenticationExtensionsClientInputs(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `appid` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientInputs`*"]
#[js::import = "None"]
pub fn get_appid(this: &AuthenticationExtensionsClientInputs,) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `appid` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientInputs`*"]
#[js::import = "None = $1"]
pub fn set_appid(this: &AuthenticationExtensionsClientInputs, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `appidExclude` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientInputs`*"]
#[js::import = "None"]
pub fn get_appid_exclude(this: &AuthenticationExtensionsClientInputs,) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `appidExclude` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientInputs`*"]
#[js::import = "None = $1"]
pub fn set_appid_exclude(this: &AuthenticationExtensionsClientInputs, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `credProps` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientInputs`*"]
#[js::import = "None"]
pub fn get_cred_props(this: &AuthenticationExtensionsClientInputs) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `credProps` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientInputs`*"]
#[js::import = "None = $1"]
pub fn set_cred_props(this: &AuthenticationExtensionsClientInputs, val: bool) { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "AuthenticationExtensionsDevicePublicKeyInputs")]
    #[doc = "Get the `devicePubKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientInputs`, `AuthenticationExtensionsDevicePublicKeyInputs`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None"]
pub fn get_device_pub_key(this: &AuthenticationExtensionsClientInputs,) -> Option<AuthenticationExtensionsDevicePublicKeyInputs> { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "AuthenticationExtensionsDevicePublicKeyInputs")]
    #[doc = "Change the `devicePubKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientInputs`, `AuthenticationExtensionsDevicePublicKeyInputs`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None = $1"]
pub fn set_device_pub_key(this: &AuthenticationExtensionsClientInputs, val: &AuthenticationExtensionsDevicePublicKeyInputs,) { panic!("web-sys intrinsic") }
    #[cfg(feature = "AuthenticationExtensionsLargeBlobInputs")]
    #[doc = "Get the `largeBlob` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientInputs`, `AuthenticationExtensionsLargeBlobInputs`*"]
#[js::import = "None"]
pub fn get_large_blob(this: &AuthenticationExtensionsClientInputs,) -> Option<AuthenticationExtensionsLargeBlobInputs> { panic!("web-sys intrinsic") }
    #[cfg(feature = "AuthenticationExtensionsLargeBlobInputs")]
    #[doc = "Change the `largeBlob` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientInputs`, `AuthenticationExtensionsLargeBlobInputs`*"]
#[js::import = "None = $1"]
pub fn set_large_blob(this: &AuthenticationExtensionsClientInputs, val: &AuthenticationExtensionsLargeBlobInputs,) { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "AuthenticationExtensionsPrfInputs")]
    #[doc = "Get the `prf` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientInputs`, `AuthenticationExtensionsPrfInputs`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None"]
pub fn get_prf(this: &AuthenticationExtensionsClientInputs,) -> Option<AuthenticationExtensionsPrfInputs> { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "AuthenticationExtensionsPrfInputs")]
    #[doc = "Change the `prf` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientInputs`, `AuthenticationExtensionsPrfInputs`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None = $1"]
pub fn set_prf(this: &AuthenticationExtensionsClientInputs, val: &AuthenticationExtensionsPrfInputs,) { panic!("web-sys intrinsic") }
    #[doc = "Get the `uvm` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientInputs`*"]
#[js::import = "None"]
pub fn get_uvm(this: &AuthenticationExtensionsClientInputs) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `uvm` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientInputs`*"]
#[js::import = "None = $1"]
pub fn set_uvm(this: &AuthenticationExtensionsClientInputs, val: bool) { panic!("web-sys intrinsic") }
impl AuthenticationExtensionsClientInputs {
    #[doc = "Construct a new `AuthenticationExtensionsClientInputs`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientInputs`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_appid()` instead."]
#[js::import = "None"]
pub fn appid(&mut self, val: &str) -> &mut Self { self.set_appid(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_appid_exclude()` instead."]
#[js::import = "None"]
pub fn appid_exclude(&mut self, val: &str) -> &mut Self { self.set_appid_exclude(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_cred_props()` instead."]
#[js::import = "None"]
pub fn cred_props(&mut self, val: bool) -> &mut Self { self.set_cred_props(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "AuthenticationExtensionsDevicePublicKeyInputs")]
    #[deprecated = "Use `set_device_pub_key()` instead."]
#[js::import = "None"]
pub fn device_pub_key(&mut self, val: &AuthenticationExtensionsDevicePublicKeyInputs,) -> &mut Self { self.set_device_pub_key(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(feature = "AuthenticationExtensionsLargeBlobInputs")]
    #[deprecated = "Use `set_large_blob()` instead."]
#[js::import = "None"]
pub fn large_blob(&mut self, val: &AuthenticationExtensionsLargeBlobInputs) -> &mut Self { self.set_large_blob(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "AuthenticationExtensionsPrfInputs")]
    #[deprecated = "Use `set_prf()` instead."]
#[js::import = "None"]
pub fn prf(&mut self, val: &AuthenticationExtensionsPrfInputs) -> &mut Self { self.set_prf(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_uvm()` instead."]
#[js::import = "None"]
pub fn uvm(&mut self, val: bool) -> &mut Self { self.set_uvm(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for AuthenticationExtensionsClientInputs {
    fn default() -> Self {
        Self::new()
    }
}
