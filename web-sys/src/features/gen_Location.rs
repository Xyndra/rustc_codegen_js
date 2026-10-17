#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Location` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Location)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Location`*"]
#[js::opaque]
#[derive(Copy)]
pub struct Location(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `href` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Location/href)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Location`*"]
impl Location {
    #[js::import = "$0.href"]
pub fn href(&self) -> Result<::alloc::string::String, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `href` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Location/href)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Location`*"]
impl Location {
    #[js::import = "$0.href = $1"]
pub fn set_href(&self, value: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `origin` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Location/origin)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Location`*"]
impl Location {
    #[js::import = "$0.origin"]
pub fn origin(&self) -> Result<::alloc::string::String, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `protocol` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Location/protocol)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Location`*"]
impl Location {
    #[js::import = "$0.protocol"]
pub fn protocol(&self) -> Result<::alloc::string::String, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `protocol` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Location/protocol)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Location`*"]
impl Location {
    #[js::import = "$0.protocol = $1"]
pub fn set_protocol(&self, value: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `host` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Location/host)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Location`*"]
impl Location {
    #[js::import = "$0.host"]
pub fn host(&self) -> Result<::alloc::string::String, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `host` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Location/host)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Location`*"]
impl Location {
    #[js::import = "$0.host = $1"]
pub fn set_host(&self, value: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `hostname` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Location/hostname)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Location`*"]
impl Location {
    #[js::import = "$0.hostname"]
pub fn hostname(&self) -> Result<::alloc::string::String, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `hostname` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Location/hostname)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Location`*"]
impl Location {
    #[js::import = "$0.hostname = $1"]
pub fn set_hostname(&self, value: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `port` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Location/port)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Location`*"]
impl Location {
    #[js::import = "$0.port"]
pub fn port(&self) -> Result<::alloc::string::String, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `port` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Location/port)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Location`*"]
impl Location {
    #[js::import = "$0.port = $1"]
pub fn set_port(&self, value: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `pathname` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Location/pathname)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Location`*"]
impl Location {
    #[js::import = "$0.pathname"]
pub fn pathname(&self) -> Result<::alloc::string::String, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `pathname` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Location/pathname)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Location`*"]
impl Location {
    #[js::import = "$0.pathname = $1"]
pub fn set_pathname(&self, value: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `search` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Location/search)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Location`*"]
impl Location {
    #[js::import = "$0.search"]
pub fn search(&self) -> Result<::alloc::string::String, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `search` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Location/search)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Location`*"]
impl Location {
    #[js::import = "$0.search = $1"]
pub fn set_search(&self, value: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `hash` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Location/hash)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Location`*"]
impl Location {
    #[js::import = "$0.hash"]
pub fn hash(&self) -> Result<::alloc::string::String, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `hash` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Location/hash)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Location`*"]
impl Location {
    #[js::import = "$0.hash = $1"]
pub fn set_hash(&self, value: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `assign()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Location/assign)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Location`*"]
impl Location {
    #[js::import = "$0.assign($1)"]
pub fn assign(&self, url: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `reload()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Location/reload)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Location`*"]
impl Location {
    #[js::import = "$0.reload()"]
pub fn reload(&self) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `reload()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Location/reload)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Location`*"]
impl Location {
    #[js::import = "$0.reload($1)"]
pub fn reload_with_forceget(&self, forceget: bool) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `replace()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Location/replace)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Location`*"]
impl Location {
    #[js::import = "$0.replace($1)"]
pub fn replace(&self, url: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
