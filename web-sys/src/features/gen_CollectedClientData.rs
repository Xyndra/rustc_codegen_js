#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CollectedClientData` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CollectedClientData`*"]
#[js::opaque]
#[derive(Copy)]
pub struct CollectedClientData(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `challenge` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CollectedClientData`*"]
#[js::import = "None"]
pub fn get_challenge(this: &CollectedClientData) -> ::alloc::string::String { panic!("web-sys intrinsic") }
    #[doc = "Change the `challenge` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CollectedClientData`*"]
#[js::import = "None = $1"]
pub fn set_challenge(this: &CollectedClientData, val: &str) { panic!("web-sys intrinsic") }
    #[cfg(feature = "AuthenticationExtensionsClientInputs")]
    #[doc = "Get the `clientExtensions` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientInputs`, `CollectedClientData`*"]
    #[deprecated]
#[js::import = "None"]
pub fn get_client_extensions(this: &CollectedClientData,) -> Option<AuthenticationExtensionsClientInputs> { panic!("web-sys intrinsic") }
    #[cfg(feature = "AuthenticationExtensionsClientInputs")]
    #[doc = "Change the `clientExtensions` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientInputs`, `CollectedClientData`*"]
    #[deprecated]
#[js::import = "None = $1"]
pub fn set_client_extensions(this: &CollectedClientData, val: &AuthenticationExtensionsClientInputs,) { panic!("web-sys intrinsic") }
    #[doc = "Get the `crossOrigin` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CollectedClientData`*"]
#[js::import = "None"]
pub fn get_cross_origin(this: &CollectedClientData) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `crossOrigin` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CollectedClientData`*"]
#[js::import = "None = $1"]
pub fn set_cross_origin(this: &CollectedClientData, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `hashAlgorithm` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CollectedClientData`*"]
    #[deprecated]
#[js::import = "None"]
pub fn get_hash_algorithm(this: &CollectedClientData) -> ::alloc::string::String { panic!("web-sys intrinsic") }
    #[doc = "Change the `hashAlgorithm` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CollectedClientData`*"]
    #[deprecated]
#[js::import = "None = $1"]
pub fn set_hash_algorithm(this: &CollectedClientData, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `origin` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CollectedClientData`*"]
#[js::import = "None"]
pub fn get_origin(this: &CollectedClientData) -> ::alloc::string::String { panic!("web-sys intrinsic") }
    #[doc = "Change the `origin` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CollectedClientData`*"]
#[js::import = "None = $1"]
pub fn set_origin(this: &CollectedClientData, val: &str) { panic!("web-sys intrinsic") }
    #[cfg(feature = "TokenBinding")]
    #[doc = "Get the `tokenBinding` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CollectedClientData`, `TokenBinding`*"]
#[js::import = "None"]
pub fn get_token_binding(this: &CollectedClientData) -> Option<TokenBinding> { panic!("web-sys intrinsic") }
    #[cfg(feature = "TokenBinding")]
    #[doc = "Change the `tokenBinding` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CollectedClientData`, `TokenBinding`*"]
#[js::import = "None = $1"]
pub fn set_token_binding(this: &CollectedClientData, val: &TokenBinding) { panic!("web-sys intrinsic") }
    #[doc = "Get the `tokenBindingId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CollectedClientData`*"]
    #[deprecated]
#[js::import = "None"]
pub fn get_token_binding_id(this: &CollectedClientData) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `tokenBindingId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CollectedClientData`*"]
    #[deprecated]
#[js::import = "None = $1"]
pub fn set_token_binding_id(this: &CollectedClientData, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CollectedClientData`*"]
#[js::import = "None"]
pub fn get_type(this: &CollectedClientData) -> ::alloc::string::String { panic!("web-sys intrinsic") }
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CollectedClientData`*"]
#[js::import = "None = $1"]
pub fn set_type(this: &CollectedClientData, val: &str) { panic!("web-sys intrinsic") }
impl CollectedClientData {
    #[doc = "Construct a new `CollectedClientData`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CollectedClientData`*"]
#[js::import = "None"]
pub fn new(challenge: &str, hash_algorithm: &str, origin: &str, type_: &str) -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret.set_challenge(challenge);
        ret.set_hash_algorithm(hash_algorithm);
        ret.set_origin(origin);
        ret.set_type(type_);
        ret
    }
    #[deprecated = "Use `set_challenge()` instead."]
#[js::import = "None"]
pub fn challenge(&mut self, val: &str) -> &mut Self { self.set_challenge(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(feature = "AuthenticationExtensionsClientInputs")]
    #[deprecated = "Use `set_client_extensions()` instead."]
#[js::import = "None"]
pub fn client_extensions(&mut self, val: &AuthenticationExtensionsClientInputs) -> &mut Self { self.set_client_extensions(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_cross_origin()` instead."]
#[js::import = "None"]
pub fn cross_origin(&mut self, val: bool) -> &mut Self { self.set_cross_origin(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_hash_algorithm()` instead."]
#[js::import = "None"]
pub fn hash_algorithm(&mut self, val: &str) -> &mut Self { self.set_hash_algorithm(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_origin()` instead."]
#[js::import = "None"]
pub fn origin(&mut self, val: &str) -> &mut Self { self.set_origin(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(feature = "TokenBinding")]
    #[deprecated = "Use `set_token_binding()` instead."]
#[js::import = "None"]
pub fn token_binding(&mut self, val: &TokenBinding) -> &mut Self { self.set_token_binding(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_token_binding_id()` instead."]
#[js::import = "None"]
pub fn token_binding_id(&mut self, val: &str) -> &mut Self { self.set_token_binding_id(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_type()` instead."]
#[js::import = "None"]
pub fn type_(&mut self, val: &str) -> &mut Self { self.set_type(val) { panic!("web-sys intrinsic") }
        self
    }
}
