pub mod css {
    #![allow(unused_imports)]
    #![allow(clippy::all)]
    use super::super::*;
        #[doc = "The `CSS.escape()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSS/escape)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `css`*"]
#[js::import = "CSS.escape"]
pub fn escape(ident: &str) -> ::alloc::string::String { panic!("web-sys intrinsic") }
        #[doc = "The `CSS.supports()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSS/supports)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `css`*"]
#[js::import = "CSS.supports"]
pub fn supports_with_value(property: &str, value: &str) -> Result<bool, JsValue> { panic!("web-sys intrinsic") }
        #[doc = "The `CSS.supports()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSS/supports)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `css`*"]
#[js::import = "CSS.supports"]
pub fn supports(condition_text: &str) -> Result<bool, JsValue> { panic!("web-sys intrinsic") }
}
