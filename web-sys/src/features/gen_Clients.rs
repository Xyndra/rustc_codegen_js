#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Clients` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Clients)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Clients`*"]
#[js::opaque]
#[derive(Copy)]
pub struct Clients(pub ::wasm_bindgen::JsValue);
    #[doc = "The `claim()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Clients/claim)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Clients`*"]
impl Clients {
    #[js::import = "$0.claim()"]
pub fn claim(&self) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[doc = "The `get()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Clients/get)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Clients`*"]
impl Clients {
    #[js::import = "$0.get($1)"]
pub fn get(&self, id: &str) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[doc = "The `matchAll()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Clients/matchAll)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Clients`*"]
impl Clients {
    #[js::import = "$0.matchAll()"]
pub fn match_all(&self) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "ClientQueryOptions")]
    #[doc = "The `matchAll()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Clients/matchAll)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ClientQueryOptions`, `Clients`*"]
impl Clients {
    #[js::import = "$0.matchAll($1)"]
pub fn match_all_with_options(&self, options: &ClientQueryOptions) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[doc = "The `openWindow()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Clients/openWindow)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Clients`*"]
impl Clients {
    #[js::import = "$0.openWindow($1)"]
pub fn open_window(&self, url: &str) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
