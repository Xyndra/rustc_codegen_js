#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `NotificationEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NotificationEventInit`*"]
#[js::opaque]
#[derive(Copy)]
pub struct NotificationEventInit(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NotificationEventInit`*"]
#[js::import = "None"]
pub fn get_bubbles(this: &NotificationEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NotificationEventInit`*"]
#[js::import = "None = $1"]
pub fn set_bubbles(this: &NotificationEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NotificationEventInit`*"]
#[js::import = "None"]
pub fn get_cancelable(this: &NotificationEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NotificationEventInit`*"]
#[js::import = "None = $1"]
pub fn set_cancelable(this: &NotificationEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NotificationEventInit`*"]
#[js::import = "None"]
pub fn get_composed(this: &NotificationEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NotificationEventInit`*"]
#[js::import = "None = $1"]
pub fn set_composed(this: &NotificationEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[cfg(feature = "Notification")]
    #[doc = "Get the `notification` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Notification`, `NotificationEventInit`*"]
#[js::import = "None"]
pub fn get_notification(this: &NotificationEventInit) -> Notification { panic!("web-sys intrinsic") }
    #[cfg(feature = "Notification")]
    #[doc = "Change the `notification` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Notification`, `NotificationEventInit`*"]
#[js::import = "None = $1"]
pub fn set_notification(this: &NotificationEventInit, val: &Notification) { panic!("web-sys intrinsic") }
impl NotificationEventInit {
    #[cfg(feature = "Notification")]
    #[doc = "Construct a new `NotificationEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Notification`, `NotificationEventInit`*"]
#[js::import = "None"]
pub fn new(notification: &Notification) -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret.set_notification(notification);
        ret
    }
    #[deprecated = "Use `set_bubbles()` instead."]
#[js::import = "None"]
pub fn bubbles(&mut self, val: bool) -> &mut Self { self.set_bubbles(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_cancelable()` instead."]
#[js::import = "None"]
pub fn cancelable(&mut self, val: bool) -> &mut Self { self.set_cancelable(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_composed()` instead."]
#[js::import = "None"]
pub fn composed(&mut self, val: bool) -> &mut Self { self.set_composed(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(feature = "Notification")]
    #[deprecated = "Use `set_notification()` instead."]
#[js::import = "None"]
pub fn notification(&mut self, val: &Notification) -> &mut Self { self.set_notification(val) { panic!("web-sys intrinsic") }
        self
    }
}
