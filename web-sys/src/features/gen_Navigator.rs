#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Navigator` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigator`*"]
#[js::opaque]
#[derive(Copy)]
pub struct Navigator(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "Clipboard")]
    #[doc = "Getter for the `clipboard` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/clipboard)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Clipboard`, `Navigator`*"]
impl Navigator {
    #[js::import = "$0.clipboard"]
pub fn clipboard(&self) -> Clipboard { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Permissions")]
    #[doc = "Getter for the `permissions` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/permissions)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigator`, `Permissions`*"]
impl Navigator {
    #[js::import = "$0.permissions"]
pub fn permissions(&self) -> Result<Permissions, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "MimeTypeArray")]
    #[doc = "Getter for the `mimeTypes` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/mimeTypes)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MimeTypeArray`, `Navigator`*"]
impl Navigator {
    #[js::import = "$0.mimeTypes"]
pub fn mime_types(&self) -> Result<MimeTypeArray, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "PluginArray")]
    #[doc = "Getter for the `plugins` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/plugins)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigator`, `PluginArray`*"]
impl Navigator {
    #[js::import = "$0.plugins"]
pub fn plugins(&self) -> Result<PluginArray, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `doNotTrack` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/doNotTrack)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigator`*"]
impl Navigator {
    #[js::import = "$0.doNotTrack"]
pub fn do_not_track(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `maxTouchPoints` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/maxTouchPoints)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigator`*"]
impl Navigator {
    #[js::import = "$0.maxTouchPoints"]
pub fn max_touch_points(&self) -> i32 { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "MediaCapabilities")]
    #[doc = "Getter for the `mediaCapabilities` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/mediaCapabilities)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaCapabilities`, `Navigator`*"]
impl Navigator {
    #[js::import = "$0.mediaCapabilities"]
pub fn media_capabilities(&self) -> MediaCapabilities { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "NetworkInformation")]
    #[doc = "Getter for the `connection` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/connection)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigator`, `NetworkInformation`*"]
impl Navigator {
    #[js::import = "$0.connection"]
pub fn connection(&self) -> Result<NetworkInformation, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `activeVRDisplays` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/activeVRDisplays)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigator`*"]
impl Navigator {
    #[js::import = "$0.activeVRDisplays"]
pub fn active_vr_displays(&self) -> ::js_sys::Array { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "MediaDevices")]
    #[doc = "Getter for the `mediaDevices` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/mediaDevices)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaDevices`, `Navigator`*"]
impl Navigator {
    #[js::import = "$0.mediaDevices"]
pub fn media_devices(&self) -> Result<MediaDevices, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "ServiceWorkerContainer")]
    #[doc = "Getter for the `serviceWorker` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/serviceWorker)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigator`, `ServiceWorkerContainer`*"]
impl Navigator {
    #[js::import = "$0.serviceWorker"]
pub fn service_worker(&self) -> ServiceWorkerContainer { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Presentation")]
    #[doc = "Getter for the `presentation` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/presentation)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigator`, `Presentation`*"]
impl Navigator {
    #[js::import = "$0.presentation"]
pub fn presentation(&self) -> Result<Option<Presentation>, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "CredentialsContainer")]
    #[doc = "Getter for the `credentials` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/credentials)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CredentialsContainer`, `Navigator`*"]
impl Navigator {
    #[js::import = "$0.credentials"]
pub fn credentials(&self) -> CredentialsContainer { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "UserActivation")]
    #[doc = "Getter for the `userActivation` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/userActivation)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigator`, `UserActivation`*"]
impl Navigator {
    #[js::import = "$0.userActivation"]
pub fn user_activation(&self) -> UserActivation { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "Bluetooth")]
    #[doc = "Getter for the `bluetooth` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/bluetooth)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Bluetooth`, `Navigator`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl Navigator {
    #[js::import = "$0.bluetooth"]
pub fn bluetooth(&self) -> Option<Bluetooth> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "MediaSession")]
    #[doc = "Getter for the `mediaSession` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/mediaSession)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaSession`, `Navigator`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl Navigator {
    #[js::import = "$0.mediaSession"]
pub fn media_session(&self) -> MediaSession { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "WakeLock")]
    #[doc = "Getter for the `wakeLock` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/wakeLock)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigator`, `WakeLock`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl Navigator {
    #[js::import = "$0.wakeLock"]
pub fn wake_lock(&self) -> WakeLock { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "Hid")]
    #[doc = "Getter for the `hid` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/hid)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Hid`, `Navigator`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl Navigator {
    #[js::import = "$0.hid"]
pub fn hid(&self) -> Hid { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "Serial")]
    #[doc = "Getter for the `serial` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/serial)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigator`, `Serial`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl Navigator {
    #[js::import = "$0.serial"]
pub fn serial(&self) -> Serial { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "Usb")]
    #[doc = "Getter for the `usb` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/usb)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigator`, `Usb`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl Navigator {
    #[js::import = "$0.usb"]
pub fn usb(&self) -> Usb { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "XrSystem")]
    #[doc = "Getter for the `xr` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/xr)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigator`, `XrSystem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl Navigator {
    #[js::import = "$0.xr"]
pub fn xr(&self) -> XrSystem { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "Scheduling")]
    #[doc = "Getter for the `scheduling` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/scheduling)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigator`, `Scheduling`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl Navigator {
    #[js::import = "$0.scheduling"]
pub fn scheduling(&self) -> Scheduling { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `hardwareConcurrency` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/hardwareConcurrency)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigator`*"]
impl Navigator {
    #[js::import = "$0.hardwareConcurrency"]
pub fn hardware_concurrency(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `deviceMemory` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/deviceMemory)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigator`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl Navigator {
    #[js::import = "$0.deviceMemory"]
pub fn device_memory(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "Gpu")]
    #[doc = "Getter for the `gpu` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/gpu)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Gpu`, `Navigator`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl Navigator {
    #[js::import = "$0.gpu"]
pub fn gpu(&self) -> Gpu { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Geolocation")]
    #[doc = "Getter for the `geolocation` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/geolocation)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Geolocation`, `Navigator`*"]
impl Navigator {
    #[js::import = "$0.geolocation"]
pub fn geolocation(&self) -> Result<Geolocation, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `appCodeName` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/appCodeName)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigator`*"]
impl Navigator {
    #[js::import = "$0.appCodeName"]
pub fn app_code_name(&self) -> Result<::alloc::string::String, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `appName` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/appName)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigator`*"]
impl Navigator {
    #[js::import = "$0.appName"]
pub fn app_name(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `appVersion` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/appVersion)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigator`*"]
impl Navigator {
    #[js::import = "$0.appVersion"]
pub fn app_version(&self) -> Result<::alloc::string::String, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `platform` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/platform)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigator`*"]
impl Navigator {
    #[js::import = "$0.platform"]
pub fn platform(&self) -> Result<::alloc::string::String, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `userAgent` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/userAgent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigator`*"]
impl Navigator {
    #[js::import = "$0.userAgent"]
pub fn user_agent(&self) -> Result<::alloc::string::String, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `product` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/product)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigator`*"]
impl Navigator {
    #[js::import = "$0.product"]
pub fn product(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `language` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/language)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigator`*"]
impl Navigator {
    #[js::import = "$0.language"]
pub fn language(&self) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `languages` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/languages)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigator`*"]
impl Navigator {
    #[js::import = "$0.languages"]
pub fn languages(&self) -> ::js_sys::Array { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "LockManager")]
    #[doc = "Getter for the `locks` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/locks)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `LockManager`, `Navigator`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl Navigator {
    #[js::import = "$0.locks"]
pub fn locks(&self) -> LockManager { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onLine` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/onLine)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigator`*"]
impl Navigator {
    #[js::import = "$0.onLine"]
pub fn on_line(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "StorageManager")]
    #[doc = "Getter for the `storage` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/storage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigator`, `StorageManager`*"]
impl Navigator {
    #[js::import = "$0.storage"]
pub fn storage(&self) -> StorageManager { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "NavigatorUaData")]
    #[doc = "Getter for the `userAgentData` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/userAgentData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigator`, `NavigatorUaData`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl Navigator {
    #[js::import = "$0.userAgentData"]
pub fn user_agent_data(&self) -> NavigatorUaData { panic!("web-sys intrinsic") }
}
    #[doc = "The `canShare()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/canShare)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigator`*"]
impl Navigator {
    #[js::import = "$0.canShare()"]
pub fn can_share(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "ShareData")]
    #[doc = "The `canShare()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/canShare)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigator`, `ShareData`*"]
impl Navigator {
    #[js::import = "$0.canShare($1)"]
pub fn can_share_with_data(&self, data: &ShareData) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "The `getGamepads()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/getGamepads)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigator`*"]
impl Navigator {
    #[js::import = "$0.getGamepads()"]
pub fn get_gamepads(&self) -> Result<::js_sys::Array, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `getVRDisplays()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/getVRDisplays)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigator`*"]
impl Navigator {
    #[js::import = "$0.getVRDisplays()"]
pub fn get_vr_displays(&self) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `requestMIDIAccess()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/requestMIDIAccess)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigator`*"]
impl Navigator {
    #[js::import = "$0.requestMIDIAccess()"]
pub fn request_midi_access(&self) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "MidiOptions")]
    #[doc = "The `requestMIDIAccess()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/requestMIDIAccess)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MidiOptions`, `Navigator`*"]
impl Navigator {
    #[js::import = "$0.requestMIDIAccess($1)"]
pub fn request_midi_access_with_options(&self, options: &MidiOptions) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `requestMediaKeySystemAccess()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/requestMediaKeySystemAccess)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigator`*"]
impl Navigator {
    #[js::import = "$0.requestMediaKeySystemAccess($1, $2)"]
pub fn request_media_key_system_access(&self, key_system: &str, supported_configurations: &::wasm_bindgen::JsValue) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "VrServiceTest")]
    #[doc = "The `requestVRServiceTest()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/requestVRServiceTest)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigator`, `VrServiceTest`*"]
impl Navigator {
    #[js::import = "$0.requestVRServiceTest()"]
pub fn request_vr_service_test(&self) -> VrServiceTest { panic!("web-sys intrinsic") }
}
    #[doc = "The `sendBeacon()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/sendBeacon)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigator`*"]
impl Navigator {
    #[js::import = "$0.sendBeacon($1)"]
pub fn send_beacon(&self, url: &str) -> Result<bool, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Blob")]
    #[doc = "The `sendBeacon()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/sendBeacon)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`, `Navigator`*"]
impl Navigator {
    #[js::import = "$0.sendBeacon($1, $2)"]
pub fn send_beacon_with_opt_blob(&self, url: &str, data: Option<&Blob>) -> Result<bool, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `sendBeacon()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/sendBeacon)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigator`*"]
impl Navigator {
    #[js::import = "$0.sendBeacon($1, $2)"]
pub fn send_beacon_with_opt_buffer_source(&self, url: &str, data: Option<&::js_sys::Object>) -> Result<bool, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `sendBeacon()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/sendBeacon)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigator`*"]
impl Navigator {
    #[js::import = "$0.sendBeacon($1, $2)"]
pub fn send_beacon_with_opt_u8_array(&self, url: &str, data: Option<&mut [u8]>) -> Result<bool, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `sendBeacon()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/sendBeacon)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigator`*"]
impl Navigator {
    #[js::import = "$0.sendBeacon($1, $2)"]
pub fn send_beacon_with_opt_js_u8_array(&self, url: &str, data: Option<&::js_sys::Uint8Array>) -> Result<bool, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "FormData")]
    #[doc = "The `sendBeacon()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/sendBeacon)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FormData`, `Navigator`*"]
impl Navigator {
    #[js::import = "$0.sendBeacon($1, $2)"]
pub fn send_beacon_with_opt_form_data(&self, url: &str, data: Option<&FormData>) -> Result<bool, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "UrlSearchParams")]
    #[doc = "The `sendBeacon()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/sendBeacon)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigator`, `UrlSearchParams`*"]
impl Navigator {
    #[js::import = "$0.sendBeacon($1, $2)"]
pub fn send_beacon_with_opt_url_search_params(&self, url: &str, data: Option<&UrlSearchParams>) -> Result<bool, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `sendBeacon()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/sendBeacon)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigator`*"]
impl Navigator {
    #[js::import = "$0.sendBeacon($1, $2)"]
pub fn send_beacon_with_opt_str(&self, url: &str, data: Option<&str>) -> Result<bool, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "ReadableStream")]
    #[doc = "The `sendBeacon()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/sendBeacon)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigator`, `ReadableStream`*"]
impl Navigator {
    #[js::import = "$0.sendBeacon($1, $2)"]
pub fn send_beacon_with_opt_readable_stream(&self, url: &str, data: Option<&ReadableStream>) -> Result<bool, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `share()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/share)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigator`*"]
impl Navigator {
    #[js::import = "$0.share()"]
pub fn share(&self) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "ShareData")]
    #[doc = "The `share()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/share)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigator`, `ShareData`*"]
impl Navigator {
    #[js::import = "$0.share($1)"]
pub fn share_with_data(&self, data: &ShareData) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[doc = "The `vibrate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/vibrate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigator`*"]
impl Navigator {
    #[js::import = "$0.vibrate($1)"]
pub fn vibrate_with_duration(&self, pattern: u32) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "The `vibrate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/vibrate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigator`*"]
impl Navigator {
    #[js::import = "$0.vibrate($1)"]
pub fn vibrate_with_pattern(&self, pattern: &::wasm_bindgen::JsValue) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "The `registerContentHandler()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/registerContentHandler)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigator`*"]
impl Navigator {
    #[js::import = "$0.registerContentHandler($1, $2, $3)"]
pub fn register_content_handler(&self, mime_type: &str, url: &str, title: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `registerProtocolHandler()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/registerProtocolHandler)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigator`*"]
impl Navigator {
    #[js::import = "$0.registerProtocolHandler($1, $2, $3)"]
pub fn register_protocol_handler(&self, scheme: &str, url: &str, title: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `taintEnabled()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Navigator/taintEnabled)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigator`*"]
impl Navigator {
    #[js::import = "$0.taintEnabled()"]
pub fn taint_enabled(&self) -> bool { panic!("web-sys intrinsic") }
}
