#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `FontFaceDescriptors` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceDescriptors`*"]
#[js::opaque]
#[derive(Copy)]
pub struct FontFaceDescriptors(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `display` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceDescriptors`*"]
#[js::import = "None"]
pub fn get_display(this: &FontFaceDescriptors) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `display` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceDescriptors`*"]
#[js::import = "None = $1"]
pub fn set_display(this: &FontFaceDescriptors, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `featureSettings` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceDescriptors`*"]
#[js::import = "None"]
pub fn get_feature_settings(this: &FontFaceDescriptors) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `featureSettings` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceDescriptors`*"]
#[js::import = "None = $1"]
pub fn set_feature_settings(this: &FontFaceDescriptors, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `stretch` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceDescriptors`*"]
#[js::import = "None"]
pub fn get_stretch(this: &FontFaceDescriptors) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `stretch` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceDescriptors`*"]
#[js::import = "None = $1"]
pub fn set_stretch(this: &FontFaceDescriptors, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `style` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceDescriptors`*"]
#[js::import = "None"]
pub fn get_style(this: &FontFaceDescriptors) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `style` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceDescriptors`*"]
#[js::import = "None = $1"]
pub fn set_style(this: &FontFaceDescriptors, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `unicodeRange` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceDescriptors`*"]
#[js::import = "None"]
pub fn get_unicode_range(this: &FontFaceDescriptors) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `unicodeRange` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceDescriptors`*"]
#[js::import = "None = $1"]
pub fn set_unicode_range(this: &FontFaceDescriptors, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `variant` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceDescriptors`*"]
#[js::import = "None"]
pub fn get_variant(this: &FontFaceDescriptors) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `variant` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceDescriptors`*"]
#[js::import = "None = $1"]
pub fn set_variant(this: &FontFaceDescriptors, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `variationSettings` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceDescriptors`*"]
#[js::import = "None"]
pub fn get_variation_settings(this: &FontFaceDescriptors) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `variationSettings` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceDescriptors`*"]
#[js::import = "None = $1"]
pub fn set_variation_settings(this: &FontFaceDescriptors, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `weight` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceDescriptors`*"]
#[js::import = "None"]
pub fn get_weight(this: &FontFaceDescriptors) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `weight` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceDescriptors`*"]
#[js::import = "None = $1"]
pub fn set_weight(this: &FontFaceDescriptors, val: &str) { panic!("web-sys intrinsic") }
impl FontFaceDescriptors {
    #[doc = "Construct a new `FontFaceDescriptors`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceDescriptors`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_display()` instead."]
#[js::import = "None"]
pub fn display(&mut self, val: &str) -> &mut Self { self.set_display(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_feature_settings()` instead."]
#[js::import = "None"]
pub fn feature_settings(&mut self, val: &str) -> &mut Self { self.set_feature_settings(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_stretch()` instead."]
#[js::import = "None"]
pub fn stretch(&mut self, val: &str) -> &mut Self { self.set_stretch(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_style()` instead."]
#[js::import = "None"]
pub fn style(&mut self, val: &str) -> &mut Self { self.set_style(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_unicode_range()` instead."]
#[js::import = "None"]
pub fn unicode_range(&mut self, val: &str) -> &mut Self { self.set_unicode_range(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_variant()` instead."]
#[js::import = "None"]
pub fn variant(&mut self, val: &str) -> &mut Self { self.set_variant(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_variation_settings()` instead."]
#[js::import = "None"]
pub fn variation_settings(&mut self, val: &str) -> &mut Self { self.set_variation_settings(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_weight()` instead."]
#[js::import = "None"]
pub fn weight(&mut self, val: &str) -> &mut Self { self.set_weight(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for FontFaceDescriptors {
    fn default() -> Self {
        Self::new()
    }
}
