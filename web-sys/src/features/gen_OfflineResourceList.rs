#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `OfflineResourceList` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineResourceList)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineResourceList`*"]
#[js::opaque]
#[derive(Copy)]
pub struct OfflineResourceList(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `status` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineResourceList/status)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineResourceList`*"]
impl OfflineResourceList {
    #[js::import = "$0.status"]
pub fn status(&self) -> Result<u16, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onchecking` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineResourceList/onchecking)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineResourceList`*"]
impl OfflineResourceList {
    #[js::import = "$0.onchecking"]
pub fn onchecking(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onchecking` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineResourceList/onchecking)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineResourceList`*"]
impl OfflineResourceList {
    #[js::import = "$0.onchecking = $1"]
pub fn set_onchecking(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineResourceList/onerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineResourceList`*"]
impl OfflineResourceList {
    #[js::import = "$0.onerror"]
pub fn onerror(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineResourceList/onerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineResourceList`*"]
impl OfflineResourceList {
    #[js::import = "$0.onerror = $1"]
pub fn set_onerror(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onnoupdate` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineResourceList/onnoupdate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineResourceList`*"]
impl OfflineResourceList {
    #[js::import = "$0.onnoupdate"]
pub fn onnoupdate(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onnoupdate` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineResourceList/onnoupdate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineResourceList`*"]
impl OfflineResourceList {
    #[js::import = "$0.onnoupdate = $1"]
pub fn set_onnoupdate(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `ondownloading` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineResourceList/ondownloading)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineResourceList`*"]
impl OfflineResourceList {
    #[js::import = "$0.ondownloading"]
pub fn ondownloading(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `ondownloading` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineResourceList/ondownloading)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineResourceList`*"]
impl OfflineResourceList {
    #[js::import = "$0.ondownloading = $1"]
pub fn set_ondownloading(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onprogress` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineResourceList/onprogress)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineResourceList`*"]
impl OfflineResourceList {
    #[js::import = "$0.onprogress"]
pub fn onprogress(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onprogress` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineResourceList/onprogress)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineResourceList`*"]
impl OfflineResourceList {
    #[js::import = "$0.onprogress = $1"]
pub fn set_onprogress(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onupdateready` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineResourceList/onupdateready)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineResourceList`*"]
impl OfflineResourceList {
    #[js::import = "$0.onupdateready"]
pub fn onupdateready(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onupdateready` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineResourceList/onupdateready)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineResourceList`*"]
impl OfflineResourceList {
    #[js::import = "$0.onupdateready = $1"]
pub fn set_onupdateready(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `oncached` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineResourceList/oncached)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineResourceList`*"]
impl OfflineResourceList {
    #[js::import = "$0.oncached"]
pub fn oncached(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `oncached` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineResourceList/oncached)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineResourceList`*"]
impl OfflineResourceList {
    #[js::import = "$0.oncached = $1"]
pub fn set_oncached(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onobsolete` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineResourceList/onobsolete)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineResourceList`*"]
impl OfflineResourceList {
    #[js::import = "$0.onobsolete"]
pub fn onobsolete(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onobsolete` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineResourceList/onobsolete)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineResourceList`*"]
impl OfflineResourceList {
    #[js::import = "$0.onobsolete = $1"]
pub fn set_onobsolete(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "The `swapCache()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineResourceList/swapCache)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineResourceList`*"]
impl OfflineResourceList {
    #[js::import = "$0.swapCache()"]
pub fn swap_cache(&self) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `update()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineResourceList/update)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineResourceList`*"]
impl OfflineResourceList {
    #[js::import = "$0.update()"]
pub fn update(&self) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
impl OfflineResourceList {
    #[doc = "The `OfflineResourceList.UNCACHED` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineResourceList`*"]
    pub const UNCACHED: u16 = 0i64 as u16;
    #[doc = "The `OfflineResourceList.IDLE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineResourceList`*"]
    pub const IDLE: u16 = 1u64 as u16;
    #[doc = "The `OfflineResourceList.CHECKING` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineResourceList`*"]
    pub const CHECKING: u16 = 2u64 as u16;
    #[doc = "The `OfflineResourceList.DOWNLOADING` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineResourceList`*"]
    pub const DOWNLOADING: u16 = 3u64 as u16;
    #[doc = "The `OfflineResourceList.UPDATEREADY` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineResourceList`*"]
    pub const UPDATEREADY: u16 = 4u64 as u16;
    #[doc = "The `OfflineResourceList.OBSOLETE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineResourceList`*"]
    pub const OBSOLETE: u16 = 5u64 as u16;
}
