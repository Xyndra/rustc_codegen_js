#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlAnchorElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
#[js::opaque]
#[derive(Copy)]
pub struct HtmlAnchorElement(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `target` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/target)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
impl HtmlAnchorElement {
    #[js::import = "$0.target"]
pub fn target(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `target` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/target)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
impl HtmlAnchorElement {
    #[js::import = "$0.target = $1"]
pub fn set_target(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `download` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/download)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
impl HtmlAnchorElement {
    #[js::import = "$0.download"]
pub fn download(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `download` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/download)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
impl HtmlAnchorElement {
    #[js::import = "$0.download = $1"]
pub fn set_download(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `ping` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/ping)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
impl HtmlAnchorElement {
    #[js::import = "$0.ping"]
pub fn ping(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `ping` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/ping)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
impl HtmlAnchorElement {
    #[js::import = "$0.ping = $1"]
pub fn set_ping(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `rel` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/rel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
impl HtmlAnchorElement {
    #[js::import = "$0.rel"]
pub fn rel(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `rel` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/rel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
impl HtmlAnchorElement {
    #[js::import = "$0.rel = $1"]
pub fn set_rel(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `referrerPolicy` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/referrerPolicy)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
impl HtmlAnchorElement {
    #[js::import = "$0.referrerPolicy"]
pub fn referrer_policy(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `referrerPolicy` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/referrerPolicy)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
impl HtmlAnchorElement {
    #[js::import = "$0.referrerPolicy = $1"]
pub fn set_referrer_policy(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "DomTokenList")]
    #[doc = "Getter for the `relList` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/relList)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomTokenList`, `HtmlAnchorElement`*"]
impl HtmlAnchorElement {
    #[js::import = "$0.relList"]
pub fn rel_list(&self) -> DomTokenList { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `hreflang` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/hreflang)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
impl HtmlAnchorElement {
    #[js::import = "$0.hreflang"]
pub fn hreflang(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `hreflang` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/hreflang)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
impl HtmlAnchorElement {
    #[js::import = "$0.hreflang = $1"]
pub fn set_hreflang(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
impl HtmlAnchorElement {
    #[js::import = "$0.type"]
pub fn type_(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
impl HtmlAnchorElement {
    #[js::import = "$0.type = $1"]
pub fn set_type(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `text` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/text)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
impl HtmlAnchorElement {
    #[js::import = "$0.text"]
pub fn text(&self) -> Result<::alloc::string::String, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `text` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/text)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
impl HtmlAnchorElement {
    #[js::import = "$0.text = $1"]
pub fn set_text(&self, value: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `coords` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/coords)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
impl HtmlAnchorElement {
    #[js::import = "$0.coords"]
pub fn coords(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `coords` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/coords)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
impl HtmlAnchorElement {
    #[js::import = "$0.coords = $1"]
pub fn set_coords(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `charset` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/charset)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
impl HtmlAnchorElement {
    #[js::import = "$0.charset"]
pub fn charset(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `charset` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/charset)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
impl HtmlAnchorElement {
    #[js::import = "$0.charset = $1"]
pub fn set_charset(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `name` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/name)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
impl HtmlAnchorElement {
    #[js::import = "$0.name"]
pub fn name(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `name` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/name)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
impl HtmlAnchorElement {
    #[js::import = "$0.name = $1"]
pub fn set_name(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `rev` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/rev)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
impl HtmlAnchorElement {
    #[js::import = "$0.rev"]
pub fn rev(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `rev` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/rev)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
impl HtmlAnchorElement {
    #[js::import = "$0.rev = $1"]
pub fn set_rev(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `shape` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/shape)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
impl HtmlAnchorElement {
    #[js::import = "$0.shape"]
pub fn shape(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `shape` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/shape)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
impl HtmlAnchorElement {
    #[js::import = "$0.shape = $1"]
pub fn set_shape(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `href` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/href)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
impl HtmlAnchorElement {
    #[js::import = "$0.href"]
pub fn href(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `href` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/href)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
impl HtmlAnchorElement {
    #[js::import = "$0.href = $1"]
pub fn set_href(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `origin` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/origin)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
impl HtmlAnchorElement {
    #[js::import = "$0.origin"]
pub fn origin(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `protocol` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/protocol)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
impl HtmlAnchorElement {
    #[js::import = "$0.protocol"]
pub fn protocol(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `protocol` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/protocol)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
impl HtmlAnchorElement {
    #[js::import = "$0.protocol = $1"]
pub fn set_protocol(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `username` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/username)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
impl HtmlAnchorElement {
    #[js::import = "$0.username"]
pub fn username(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `username` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/username)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
impl HtmlAnchorElement {
    #[js::import = "$0.username = $1"]
pub fn set_username(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `password` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/password)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
impl HtmlAnchorElement {
    #[js::import = "$0.password"]
pub fn password(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `password` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/password)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
impl HtmlAnchorElement {
    #[js::import = "$0.password = $1"]
pub fn set_password(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `host` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/host)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
impl HtmlAnchorElement {
    #[js::import = "$0.host"]
pub fn host(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `host` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/host)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
impl HtmlAnchorElement {
    #[js::import = "$0.host = $1"]
pub fn set_host(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `hostname` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/hostname)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
impl HtmlAnchorElement {
    #[js::import = "$0.hostname"]
pub fn hostname(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `hostname` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/hostname)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
impl HtmlAnchorElement {
    #[js::import = "$0.hostname = $1"]
pub fn set_hostname(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `port` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/port)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
impl HtmlAnchorElement {
    #[js::import = "$0.port"]
pub fn port(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `port` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/port)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
impl HtmlAnchorElement {
    #[js::import = "$0.port = $1"]
pub fn set_port(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `pathname` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/pathname)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
impl HtmlAnchorElement {
    #[js::import = "$0.pathname"]
pub fn pathname(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `pathname` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/pathname)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
impl HtmlAnchorElement {
    #[js::import = "$0.pathname = $1"]
pub fn set_pathname(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `search` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/search)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
impl HtmlAnchorElement {
    #[js::import = "$0.search"]
pub fn search(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `search` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/search)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
impl HtmlAnchorElement {
    #[js::import = "$0.search = $1"]
pub fn set_search(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `hash` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/hash)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
impl HtmlAnchorElement {
    #[js::import = "$0.hash"]
pub fn hash(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `hash` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/hash)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
impl HtmlAnchorElement {
    #[js::import = "$0.hash = $1"]
pub fn set_hash(&self, value: &str) { panic!("web-sys intrinsic") }
}
