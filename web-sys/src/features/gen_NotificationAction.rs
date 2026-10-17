#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `NotificationAction` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NotificationAction`*"]
#[js::opaque]
#[derive(Copy)]
pub struct NotificationAction(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `action` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NotificationAction`*"]
#[js::import = "None"]
pub fn get_action(this: &NotificationAction) -> ::alloc::string::String { panic!("web-sys intrinsic") }
    #[doc = "Change the `action` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NotificationAction`*"]
#[js::import = "None = $1"]
pub fn set_action(this: &NotificationAction, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `icon` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NotificationAction`*"]
#[js::import = "None"]
pub fn get_icon(this: &NotificationAction) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `icon` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NotificationAction`*"]
#[js::import = "None = $1"]
pub fn set_icon(this: &NotificationAction, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `title` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NotificationAction`*"]
#[js::import = "None"]
pub fn get_title(this: &NotificationAction) -> ::alloc::string::String { panic!("web-sys intrinsic") }
    #[doc = "Change the `title` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NotificationAction`*"]
#[js::import = "None = $1"]
pub fn set_title(this: &NotificationAction, val: &str) { panic!("web-sys intrinsic") }
impl NotificationAction {
    #[doc = "Construct a new `NotificationAction`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NotificationAction`*"]
#[js::import = "None"]
pub fn new(action: &str, title: &str) -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret.set_action(action);
        ret.set_title(title);
        ret
    }
    #[deprecated = "Use `set_action()` instead."]
#[js::import = "None"]
pub fn action(&mut self, val: &str) -> &mut Self { self.set_action(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_icon()` instead."]
#[js::import = "None"]
pub fn icon(&mut self, val: &str) -> &mut Self { self.set_icon(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_title()` instead."]
#[js::import = "None"]
pub fn title(&mut self, val: &str) -> &mut Self { self.set_title(val) { panic!("web-sys intrinsic") }
        self
    }
}
