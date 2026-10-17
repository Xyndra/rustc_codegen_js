#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DecoderDoctorNotification` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DecoderDoctorNotification`*"]
#[js::opaque]
#[derive(Copy)]
pub struct DecoderDoctorNotification(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `decodeIssue` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DecoderDoctorNotification`*"]
#[js::import = "None"]
pub fn get_decode_issue(this: &DecoderDoctorNotification) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `decodeIssue` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DecoderDoctorNotification`*"]
#[js::import = "None = $1"]
pub fn set_decode_issue(this: &DecoderDoctorNotification, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `decoderDoctorReportId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DecoderDoctorNotification`*"]
#[js::import = "None"]
pub fn get_decoder_doctor_report_id(this: &DecoderDoctorNotification,) -> ::alloc::string::String { panic!("web-sys intrinsic") }
    #[doc = "Change the `decoderDoctorReportId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DecoderDoctorNotification`*"]
#[js::import = "None = $1"]
pub fn set_decoder_doctor_report_id(this: &DecoderDoctorNotification, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `docURL` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DecoderDoctorNotification`*"]
#[js::import = "None"]
pub fn get_doc_url(this: &DecoderDoctorNotification) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `docURL` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DecoderDoctorNotification`*"]
#[js::import = "None = $1"]
pub fn set_doc_url(this: &DecoderDoctorNotification, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `formats` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DecoderDoctorNotification`*"]
#[js::import = "None"]
pub fn get_formats(this: &DecoderDoctorNotification) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `formats` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DecoderDoctorNotification`*"]
#[js::import = "None = $1"]
pub fn set_formats(this: &DecoderDoctorNotification, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `isSolved` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DecoderDoctorNotification`*"]
#[js::import = "None"]
pub fn get_is_solved(this: &DecoderDoctorNotification) -> bool { panic!("web-sys intrinsic") }
    #[doc = "Change the `isSolved` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DecoderDoctorNotification`*"]
#[js::import = "None = $1"]
pub fn set_is_solved(this: &DecoderDoctorNotification, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `resourceURL` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DecoderDoctorNotification`*"]
#[js::import = "None"]
pub fn get_resource_url(this: &DecoderDoctorNotification) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `resourceURL` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DecoderDoctorNotification`*"]
#[js::import = "None = $1"]
pub fn set_resource_url(this: &DecoderDoctorNotification, val: &str) { panic!("web-sys intrinsic") }
    #[cfg(feature = "DecoderDoctorNotificationType")]
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DecoderDoctorNotification`, `DecoderDoctorNotificationType`*"]
#[js::import = "None"]
pub fn get_type(this: &DecoderDoctorNotification) -> DecoderDoctorNotificationType { panic!("web-sys intrinsic") }
    #[cfg(feature = "DecoderDoctorNotificationType")]
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DecoderDoctorNotification`, `DecoderDoctorNotificationType`*"]
#[js::import = "None = $1"]
pub fn set_type(this: &DecoderDoctorNotification, val: DecoderDoctorNotificationType) { panic!("web-sys intrinsic") }
impl DecoderDoctorNotification {
    #[cfg(feature = "DecoderDoctorNotificationType")]
    #[doc = "Construct a new `DecoderDoctorNotification`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DecoderDoctorNotification`, `DecoderDoctorNotificationType`*"]
#[js::import = "None"]
pub fn new(decoder_doctor_report_id: &str, is_solved: bool, type_: DecoderDoctorNotificationType,) -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret.set_decoder_doctor_report_id(decoder_doctor_report_id);
        ret.set_is_solved(is_solved);
        ret.set_type(type_);
        ret
    }
    #[deprecated = "Use `set_decode_issue()` instead."]
#[js::import = "None"]
pub fn decode_issue(&mut self, val: &str) -> &mut Self { self.set_decode_issue(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_decoder_doctor_report_id()` instead."]
#[js::import = "None"]
pub fn decoder_doctor_report_id(&mut self, val: &str) -> &mut Self { self.set_decoder_doctor_report_id(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_doc_url()` instead."]
#[js::import = "None"]
pub fn doc_url(&mut self, val: &str) -> &mut Self { self.set_doc_url(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_formats()` instead."]
#[js::import = "None"]
pub fn formats(&mut self, val: &str) -> &mut Self { self.set_formats(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_is_solved()` instead."]
#[js::import = "None"]
pub fn is_solved(&mut self, val: bool) -> &mut Self { self.set_is_solved(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_resource_url()` instead."]
#[js::import = "None"]
pub fn resource_url(&mut self, val: &str) -> &mut Self { self.set_resource_url(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(feature = "DecoderDoctorNotificationType")]
    #[deprecated = "Use `set_type()` instead."]
#[js::import = "None"]
pub fn type_(&mut self, val: DecoderDoctorNotificationType) -> &mut Self { self.set_type(val) { panic!("web-sys intrinsic") }
        self
    }
}
