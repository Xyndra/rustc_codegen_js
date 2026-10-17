#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `External` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/External)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `External`*"]
#[js::opaque]
#[derive(Copy)]
pub struct External(pub ::wasm_bindgen::JsValue);
    #[doc = "The `AddSearchProvider()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/External/AddSearchProvider)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `External`*"]
impl External {
    #[js::import = "$0.AddSearchProvider($1)"]
pub fn add_search_provider(&self, a_description_url: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "The `IsSearchProviderInstalled()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/External/IsSearchProviderInstalled)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `External`*"]
impl External {
    #[js::import = "$0.IsSearchProviderInstalled($1)"]
pub fn is_search_provider_installed(&self, a_search_url: &str) -> u32 { panic!("web-sys intrinsic") }
}
