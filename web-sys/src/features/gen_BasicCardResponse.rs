#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `BasicCardResponse` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BasicCardResponse`*"]
#[js::opaque]
#[derive(Copy)]
pub struct BasicCardResponse(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "PaymentAddress")]
    #[doc = "Get the `billingAddress` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BasicCardResponse`, `PaymentAddress`*"]
#[js::import = "None"]
pub fn get_billing_address(this: &BasicCardResponse) -> Option<PaymentAddress> { panic!("web-sys intrinsic") }
    #[cfg(feature = "PaymentAddress")]
    #[doc = "Change the `billingAddress` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BasicCardResponse`, `PaymentAddress`*"]
#[js::import = "None = $1"]
pub fn set_billing_address(this: &BasicCardResponse, val: Option<&PaymentAddress>) { panic!("web-sys intrinsic") }
    #[doc = "Get the `cardNumber` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BasicCardResponse`*"]
#[js::import = "None"]
pub fn get_card_number(this: &BasicCardResponse) -> ::alloc::string::String { panic!("web-sys intrinsic") }
    #[doc = "Change the `cardNumber` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BasicCardResponse`*"]
#[js::import = "None = $1"]
pub fn set_card_number(this: &BasicCardResponse, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `cardSecurityCode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BasicCardResponse`*"]
#[js::import = "None"]
pub fn get_card_security_code(this: &BasicCardResponse) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `cardSecurityCode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BasicCardResponse`*"]
#[js::import = "None = $1"]
pub fn set_card_security_code(this: &BasicCardResponse, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `cardholderName` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BasicCardResponse`*"]
#[js::import = "None"]
pub fn get_cardholder_name(this: &BasicCardResponse) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `cardholderName` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BasicCardResponse`*"]
#[js::import = "None = $1"]
pub fn set_cardholder_name(this: &BasicCardResponse, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `expiryMonth` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BasicCardResponse`*"]
#[js::import = "None"]
pub fn get_expiry_month(this: &BasicCardResponse) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `expiryMonth` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BasicCardResponse`*"]
#[js::import = "None = $1"]
pub fn set_expiry_month(this: &BasicCardResponse, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `expiryYear` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BasicCardResponse`*"]
#[js::import = "None"]
pub fn get_expiry_year(this: &BasicCardResponse) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `expiryYear` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BasicCardResponse`*"]
#[js::import = "None = $1"]
pub fn set_expiry_year(this: &BasicCardResponse, val: &str) { panic!("web-sys intrinsic") }
impl BasicCardResponse {
    #[doc = "Construct a new `BasicCardResponse`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BasicCardResponse`*"]
#[js::import = "None"]
pub fn new(card_number: &str) -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret.set_card_number(card_number);
        ret
    }
    #[cfg(feature = "PaymentAddress")]
    #[deprecated = "Use `set_billing_address()` instead."]
#[js::import = "None"]
pub fn billing_address(&mut self, val: Option<&PaymentAddress>) -> &mut Self { self.set_billing_address(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_card_number()` instead."]
#[js::import = "None"]
pub fn card_number(&mut self, val: &str) -> &mut Self { self.set_card_number(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_card_security_code()` instead."]
#[js::import = "None"]
pub fn card_security_code(&mut self, val: &str) -> &mut Self { self.set_card_security_code(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_cardholder_name()` instead."]
#[js::import = "None"]
pub fn cardholder_name(&mut self, val: &str) -> &mut Self { self.set_cardholder_name(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_expiry_month()` instead."]
#[js::import = "None"]
pub fn expiry_month(&mut self, val: &str) -> &mut Self { self.set_expiry_month(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_expiry_year()` instead."]
#[js::import = "None"]
pub fn expiry_year(&mut self, val: &str) -> &mut Self { self.set_expiry_year(val) { panic!("web-sys intrinsic") }
        self
    }
}
