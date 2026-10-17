#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `StorageEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StorageEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StorageEvent`*"]
#[js::opaque]
#[derive(Copy)]
pub struct StorageEvent(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `key` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StorageEvent/key)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StorageEvent`*"]
impl StorageEvent {
    #[js::import = "$0.key"]
pub fn key(&self) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `oldValue` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StorageEvent/oldValue)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StorageEvent`*"]
impl StorageEvent {
    #[js::import = "$0.oldValue"]
pub fn old_value(&self) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `newValue` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StorageEvent/newValue)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StorageEvent`*"]
impl StorageEvent {
    #[js::import = "$0.newValue"]
pub fn new_value(&self) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `url` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StorageEvent/url)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StorageEvent`*"]
impl StorageEvent {
    #[js::import = "$0.url"]
pub fn url(&self) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Storage")]
    #[doc = "Getter for the `storageArea` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StorageEvent/storageArea)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Storage`, `StorageEvent`*"]
impl StorageEvent {
    #[js::import = "$0.storageArea"]
pub fn storage_area(&self) -> Option<Storage> { panic!("web-sys intrinsic") }
}
    #[doc = "The `new StorageEvent(..)` constructor, creating a new instance of `StorageEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StorageEvent/StorageEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StorageEvent`*"]
impl StorageEvent {
    #[js::import = "new StorageEvent($0)"]
pub fn new(type_: &str) -> Result<StorageEvent, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "StorageEventInit")]
    #[doc = "The `new StorageEvent(..)` constructor, creating a new instance of `StorageEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StorageEvent/StorageEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StorageEvent`, `StorageEventInit`*"]
impl StorageEvent {
    #[js::import = "new StorageEvent($0, $1)"]
pub fn new_with_event_init_dict(type_: &str, event_init_dict: &StorageEventInit,) -> Result<StorageEvent, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `initStorageEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StorageEvent/initStorageEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StorageEvent`*"]
impl StorageEvent {
    #[js::import = "$0.initStorageEvent($1)"]
pub fn init_storage_event(&self, type_: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "The `initStorageEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StorageEvent/initStorageEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StorageEvent`*"]
impl StorageEvent {
    #[js::import = "$0.initStorageEvent($1, $2)"]
pub fn init_storage_event_with_can_bubble(&self, type_: &str, can_bubble: bool) { panic!("web-sys intrinsic") }
}
    #[doc = "The `initStorageEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StorageEvent/initStorageEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StorageEvent`*"]
impl StorageEvent {
    #[js::import = "$0.initStorageEvent($1, $2, $3)"]
pub fn init_storage_event_with_can_bubble_and_cancelable(&self, type_: &str, can_bubble: bool, cancelable: bool) { panic!("web-sys intrinsic") }
}
    #[doc = "The `initStorageEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StorageEvent/initStorageEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StorageEvent`*"]
impl StorageEvent {
    #[js::import = "$0.initStorageEvent($1, $2, $3, $4)"]
pub fn init_storage_event_with_can_bubble_and_cancelable_and_key(&self, type_: &str, can_bubble: bool, cancelable: bool, key: Option<&str>) { panic!("web-sys intrinsic") }
}
    #[doc = "The `initStorageEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StorageEvent/initStorageEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StorageEvent`*"]
impl StorageEvent {
    #[js::import = "$0.initStorageEvent($1, $2, $3, $4, $5)"]
pub fn init_storage_event_with_can_bubble_and_cancelable_and_key_and_old_value(&self, type_: &str, can_bubble: bool, cancelable: bool, key: Option<&str>, old_value: Option<&str>) { panic!("web-sys intrinsic") }
}
    #[doc = "The `initStorageEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StorageEvent/initStorageEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StorageEvent`*"]
impl StorageEvent {
    #[js::import = "$0.initStorageEvent($1, $2, $3, $4, $5, $6)"]
pub fn init_storage_event_with_can_bubble_and_cancelable_and_key_and_old_value_and_new_value(&self, type_: &str, can_bubble: bool, cancelable: bool, key: Option<&str>, old_value: Option<&str>, new_value: Option<&str>) { panic!("web-sys intrinsic") }
}
    #[doc = "The `initStorageEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StorageEvent/initStorageEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StorageEvent`*"]
impl StorageEvent {
    #[js::import = "$0.initStorageEvent($1, $2, $3, $4, $5, $6, $7)"]
pub fn init_storage_event_with_can_bubble_and_cancelable_and_key_and_old_value_and_new_value_and_url(&self, type_: &str, can_bubble: bool, cancelable: bool, key: Option<&str>, old_value: Option<&str>, new_value: Option<&str>, url: Option<&str>) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Storage")]
    #[doc = "The `initStorageEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StorageEvent/initStorageEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Storage`, `StorageEvent`*"]
impl StorageEvent {
    #[js::import = "$0.initStorageEvent($1, $2, $3, $4, $5, $6, $7, $8)"]
pub fn init_storage_event_with_can_bubble_and_cancelable_and_key_and_old_value_and_new_value_and_url_and_storage_area(&self, type_: &str, can_bubble: bool, cancelable: bool, key: Option<&str>, old_value: Option<&str>, new_value: Option<&str>, url: Option<&str>, storage_area: Option<&Storage>) { panic!("web-sys intrinsic") }
}
