#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `BatteryManager` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BatteryManager)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BatteryManager`*"]
#[js::opaque]
#[derive(Copy)]
pub struct BatteryManager(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `charging` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BatteryManager/charging)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BatteryManager`*"]
impl BatteryManager {
    #[js::import = "$0.charging"]
pub fn charging(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `chargingTime` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BatteryManager/chargingTime)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BatteryManager`*"]
impl BatteryManager {
    #[js::import = "$0.chargingTime"]
pub fn charging_time(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `dischargingTime` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BatteryManager/dischargingTime)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BatteryManager`*"]
impl BatteryManager {
    #[js::import = "$0.dischargingTime"]
pub fn discharging_time(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `level` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BatteryManager/level)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BatteryManager`*"]
impl BatteryManager {
    #[js::import = "$0.level"]
pub fn level(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onchargingchange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BatteryManager/onchargingchange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BatteryManager`*"]
impl BatteryManager {
    #[js::import = "$0.onchargingchange"]
pub fn onchargingchange(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onchargingchange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BatteryManager/onchargingchange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BatteryManager`*"]
impl BatteryManager {
    #[js::import = "$0.onchargingchange = $1"]
pub fn set_onchargingchange(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onchargingtimechange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BatteryManager/onchargingtimechange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BatteryManager`*"]
impl BatteryManager {
    #[js::import = "$0.onchargingtimechange"]
pub fn onchargingtimechange(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onchargingtimechange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BatteryManager/onchargingtimechange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BatteryManager`*"]
impl BatteryManager {
    #[js::import = "$0.onchargingtimechange = $1"]
pub fn set_onchargingtimechange(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `ondischargingtimechange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BatteryManager/ondischargingtimechange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BatteryManager`*"]
impl BatteryManager {
    #[js::import = "$0.ondischargingtimechange"]
pub fn ondischargingtimechange(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `ondischargingtimechange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BatteryManager/ondischargingtimechange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BatteryManager`*"]
impl BatteryManager {
    #[js::import = "$0.ondischargingtimechange = $1"]
pub fn set_ondischargingtimechange(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onlevelchange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BatteryManager/onlevelchange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BatteryManager`*"]
impl BatteryManager {
    #[js::import = "$0.onlevelchange"]
pub fn onlevelchange(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onlevelchange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BatteryManager/onlevelchange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BatteryManager`*"]
impl BatteryManager {
    #[js::import = "$0.onlevelchange = $1"]
pub fn set_onlevelchange(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
