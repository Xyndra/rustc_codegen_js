#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WorkerNavigator` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerNavigator)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerNavigator`*"]
#[js::opaque]
#[derive(Copy)]
pub struct WorkerNavigator(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "NetworkInformation")]
    #[doc = "Getter for the `connection` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerNavigator/connection)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkInformation`, `WorkerNavigator`*"]
impl WorkerNavigator {
    #[js::import = "$0.connection"]
pub fn connection(&self) -> Result<NetworkInformation, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "MediaCapabilities")]
    #[doc = "Getter for the `mediaCapabilities` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerNavigator/mediaCapabilities)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaCapabilities`, `WorkerNavigator`*"]
impl WorkerNavigator {
    #[js::import = "$0.mediaCapabilities"]
pub fn media_capabilities(&self) -> MediaCapabilities { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "Serial")]
    #[doc = "Getter for the `serial` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerNavigator/serial)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Serial`, `WorkerNavigator`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl WorkerNavigator {
    #[js::import = "$0.serial"]
pub fn serial(&self) -> Serial { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "Usb")]
    #[doc = "Getter for the `usb` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerNavigator/usb)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Usb`, `WorkerNavigator`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl WorkerNavigator {
    #[js::import = "$0.usb"]
pub fn usb(&self) -> Usb { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `hardwareConcurrency` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerNavigator/hardwareConcurrency)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerNavigator`*"]
impl WorkerNavigator {
    #[js::import = "$0.hardwareConcurrency"]
pub fn hardware_concurrency(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `deviceMemory` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerNavigator/deviceMemory)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerNavigator`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl WorkerNavigator {
    #[js::import = "$0.deviceMemory"]
pub fn device_memory(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "Gpu")]
    #[doc = "Getter for the `gpu` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerNavigator/gpu)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Gpu`, `WorkerNavigator`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl WorkerNavigator {
    #[js::import = "$0.gpu"]
pub fn gpu(&self) -> Gpu { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `appCodeName` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerNavigator/appCodeName)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerNavigator`*"]
impl WorkerNavigator {
    #[js::import = "$0.appCodeName"]
pub fn app_code_name(&self) -> Result<::alloc::string::String, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `appName` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerNavigator/appName)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerNavigator`*"]
impl WorkerNavigator {
    #[js::import = "$0.appName"]
pub fn app_name(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `appVersion` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerNavigator/appVersion)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerNavigator`*"]
impl WorkerNavigator {
    #[js::import = "$0.appVersion"]
pub fn app_version(&self) -> Result<::alloc::string::String, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `platform` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerNavigator/platform)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerNavigator`*"]
impl WorkerNavigator {
    #[js::import = "$0.platform"]
pub fn platform(&self) -> Result<::alloc::string::String, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `userAgent` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerNavigator/userAgent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerNavigator`*"]
impl WorkerNavigator {
    #[js::import = "$0.userAgent"]
pub fn user_agent(&self) -> Result<::alloc::string::String, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `product` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerNavigator/product)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerNavigator`*"]
impl WorkerNavigator {
    #[js::import = "$0.product"]
pub fn product(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `language` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerNavigator/language)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerNavigator`*"]
impl WorkerNavigator {
    #[js::import = "$0.language"]
pub fn language(&self) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `languages` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerNavigator/languages)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerNavigator`*"]
impl WorkerNavigator {
    #[js::import = "$0.languages"]
pub fn languages(&self) -> ::js_sys::Array { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "LockManager")]
    #[doc = "Getter for the `locks` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerNavigator/locks)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `LockManager`, `WorkerNavigator`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl WorkerNavigator {
    #[js::import = "$0.locks"]
pub fn locks(&self) -> LockManager { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onLine` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerNavigator/onLine)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerNavigator`*"]
impl WorkerNavigator {
    #[js::import = "$0.onLine"]
pub fn on_line(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "StorageManager")]
    #[doc = "Getter for the `storage` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerNavigator/storage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StorageManager`, `WorkerNavigator`*"]
impl WorkerNavigator {
    #[js::import = "$0.storage"]
pub fn storage(&self) -> StorageManager { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "NavigatorUaData")]
    #[doc = "Getter for the `userAgentData` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerNavigator/userAgentData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigatorUaData`, `WorkerNavigator`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl WorkerNavigator {
    #[js::import = "$0.userAgentData"]
pub fn user_agent_data(&self) -> NavigatorUaData { panic!("web-sys intrinsic") }
}
    #[doc = "The `taintEnabled()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerNavigator/taintEnabled)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerNavigator`*"]
impl WorkerNavigator {
    #[js::import = "$0.taintEnabled()"]
pub fn taint_enabled(&self) -> bool { panic!("web-sys intrinsic") }
}
