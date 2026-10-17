#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PaymentAddress` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentAddress)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PaymentAddress`*"]
#[js::opaque]
#[derive(Copy)]
pub struct PaymentAddress(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `country` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentAddress/country)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PaymentAddress`*"]
impl PaymentAddress {
    #[js::import = "$0.country"]
pub fn country(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `addressLine` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentAddress/addressLine)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PaymentAddress`*"]
impl PaymentAddress {
    #[js::import = "$0.addressLine"]
pub fn address_line(&self) -> ::js_sys::Array { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `region` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentAddress/region)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PaymentAddress`*"]
impl PaymentAddress {
    #[js::import = "$0.region"]
pub fn region(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `city` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentAddress/city)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PaymentAddress`*"]
impl PaymentAddress {
    #[js::import = "$0.city"]
pub fn city(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `dependentLocality` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentAddress/dependentLocality)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PaymentAddress`*"]
impl PaymentAddress {
    #[js::import = "$0.dependentLocality"]
pub fn dependent_locality(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `postalCode` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentAddress/postalCode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PaymentAddress`*"]
impl PaymentAddress {
    #[js::import = "$0.postalCode"]
pub fn postal_code(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `sortingCode` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentAddress/sortingCode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PaymentAddress`*"]
impl PaymentAddress {
    #[js::import = "$0.sortingCode"]
pub fn sorting_code(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `languageCode` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentAddress/languageCode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PaymentAddress`*"]
impl PaymentAddress {
    #[js::import = "$0.languageCode"]
pub fn language_code(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `organization` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentAddress/organization)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PaymentAddress`*"]
impl PaymentAddress {
    #[js::import = "$0.organization"]
pub fn organization(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `recipient` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentAddress/recipient)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PaymentAddress`*"]
impl PaymentAddress {
    #[js::import = "$0.recipient"]
pub fn recipient(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `phone` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentAddress/phone)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PaymentAddress`*"]
impl PaymentAddress {
    #[js::import = "$0.phone"]
pub fn phone(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "The `toJSON()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentAddress/toJSON)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PaymentAddress`*"]
impl PaymentAddress {
    #[js::import = "$0.toJSON()"]
pub fn to_json(&self) -> ::js_sys::Object { panic!("web-sys intrinsic") }
}
