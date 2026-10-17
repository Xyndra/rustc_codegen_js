#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `IntlUtils` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IntlUtils)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IntlUtils`*"]
#[js::opaque]
#[derive(Copy)]
pub struct IntlUtils(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "DisplayNameResult")]
    #[doc = "The `getDisplayNames()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IntlUtils/getDisplayNames)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DisplayNameResult`, `IntlUtils`*"]
impl IntlUtils {
    #[js::import = "$0.getDisplayNames($1)"]
pub fn get_display_names(&self, locales: &::wasm_bindgen::JsValue) -> Result<DisplayNameResult, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "DisplayNameOptions", feature = "DisplayNameResult",))]
    #[doc = "The `getDisplayNames()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IntlUtils/getDisplayNames)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DisplayNameOptions`, `DisplayNameResult`, `IntlUtils`*"]
impl IntlUtils {
    #[js::import = "$0.getDisplayNames($1, $2)"]
pub fn get_display_names_with_options(&self, locales: &::wasm_bindgen::JsValue, options: &DisplayNameOptions) -> Result<DisplayNameResult, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "LocaleInfo")]
    #[doc = "The `getLocaleInfo()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IntlUtils/getLocaleInfo)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IntlUtils`, `LocaleInfo`*"]
impl IntlUtils {
    #[js::import = "$0.getLocaleInfo($1)"]
pub fn get_locale_info(&self, locales: &::wasm_bindgen::JsValue) -> Result<LocaleInfo, JsValue> { panic!("web-sys intrinsic") }
}
