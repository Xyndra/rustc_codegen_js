#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Notification` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Notification)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Notification`*"]
#[js::opaque]
#[derive(Copy)]
pub struct Notification(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "NotificationPermission")]
    #[doc = "Getter for the `permission` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Notification/permission)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Notification`, `NotificationPermission`*"]
impl Notification {
    #[js::import = "$0.permission"]
pub fn permission() -> NotificationPermission { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `maxActions` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Notification/maxActions)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Notification`*"]
impl Notification {
    #[js::import = "$0.maxActions"]
pub fn max_actions() -> u32 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onclick` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Notification/onclick)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Notification`*"]
impl Notification {
    #[js::import = "$0.onclick"]
pub fn onclick(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onclick` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Notification/onclick)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Notification`*"]
impl Notification {
    #[js::import = "$0.onclick = $1"]
pub fn set_onclick(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onshow` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Notification/onshow)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Notification`*"]
impl Notification {
    #[js::import = "$0.onshow"]
pub fn onshow(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onshow` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Notification/onshow)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Notification`*"]
impl Notification {
    #[js::import = "$0.onshow = $1"]
pub fn set_onshow(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Notification/onerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Notification`*"]
impl Notification {
    #[js::import = "$0.onerror"]
pub fn onerror(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Notification/onerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Notification`*"]
impl Notification {
    #[js::import = "$0.onerror = $1"]
pub fn set_onerror(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onclose` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Notification/onclose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Notification`*"]
impl Notification {
    #[js::import = "$0.onclose"]
pub fn onclose(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onclose` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Notification/onclose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Notification`*"]
impl Notification {
    #[js::import = "$0.onclose = $1"]
pub fn set_onclose(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `title` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Notification/title)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Notification`*"]
impl Notification {
    #[js::import = "$0.title"]
pub fn title(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "NotificationDirection")]
    #[doc = "Getter for the `dir` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Notification/dir)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Notification`, `NotificationDirection`*"]
impl Notification {
    #[js::import = "$0.dir"]
pub fn dir(&self) -> NotificationDirection { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `lang` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Notification/lang)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Notification`*"]
impl Notification {
    #[js::import = "$0.lang"]
pub fn lang(&self) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `body` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Notification/body)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Notification`*"]
impl Notification {
    #[js::import = "$0.body"]
pub fn body(&self) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `tag` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Notification/tag)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Notification`*"]
impl Notification {
    #[js::import = "$0.tag"]
pub fn tag(&self) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `image` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Notification/image)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Notification`*"]
impl Notification {
    #[js::import = "$0.image"]
pub fn image(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `icon` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Notification/icon)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Notification`*"]
impl Notification {
    #[js::import = "$0.icon"]
pub fn icon(&self) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `badge` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Notification/badge)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Notification`*"]
impl Notification {
    #[js::import = "$0.badge"]
pub fn badge(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `vibrate` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Notification/vibrate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Notification`*"]
impl Notification {
    #[js::import = "$0.vibrate"]
pub fn vibrate(&self) -> ::js_sys::Array { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Notification/timestamp)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Notification`*"]
impl Notification {
    #[js::import = "$0.timestamp"]
pub fn timestamp(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `renotify` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Notification/renotify)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Notification`*"]
impl Notification {
    #[js::import = "$0.renotify"]
pub fn renotify(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `silent` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Notification/silent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Notification`*"]
impl Notification {
    #[js::import = "$0.silent"]
pub fn silent(&self) -> Option<bool> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `requireInteraction` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Notification/requireInteraction)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Notification`*"]
impl Notification {
    #[js::import = "$0.requireInteraction"]
pub fn require_interaction(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `data` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Notification/data)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Notification`*"]
impl Notification {
    #[js::import = "$0.data"]
pub fn data(&self) -> ::wasm_bindgen::JsValue { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `actions` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Notification/actions)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Notification`*"]
impl Notification {
    #[js::import = "$0.actions"]
pub fn actions(&self) -> ::js_sys::Array { panic!("web-sys intrinsic") }
}
    #[doc = "The `new Notification(..)` constructor, creating a new instance of `Notification`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Notification/Notification)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Notification`*"]
impl Notification {
    #[js::import = "new Notification($0)"]
pub fn new(title: &str) -> Result<Notification, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "NotificationOptions")]
    #[doc = "The `new Notification(..)` constructor, creating a new instance of `Notification`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Notification/Notification)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Notification`, `NotificationOptions`*"]
impl Notification {
    #[js::import = "new Notification($0, $1)"]
pub fn new_with_options(title: &str, options: &NotificationOptions,) -> Result<Notification, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `close()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Notification/close)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Notification`*"]
impl Notification {
    #[js::import = "$0.close()"]
pub fn close(&self) { panic!("web-sys intrinsic") }
}
    #[doc = "The `requestPermission()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Notification/requestPermission_static)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Notification`*"]
impl Notification {
    #[js::import = "Notification.requestPermission"]
pub fn request_permission() -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `requestPermission()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Notification/requestPermission_static)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Notification`*"]
impl Notification {
    #[js::import = "Notification.requestPermission"]
pub fn request_permission_with_permission_callback(permission_callback: &::js_sys::Function,) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
