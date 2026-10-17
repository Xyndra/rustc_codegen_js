#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ImageBitmapOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageBitmapOptions`*"]
#[js::opaque]
#[derive(Copy)]
pub struct ImageBitmapOptions(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "ColorSpaceConversion")]
    #[doc = "Get the `colorSpaceConversion` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ColorSpaceConversion`, `ImageBitmapOptions`*"]
#[js::import = "None"]
pub fn get_color_space_conversion(this: &ImageBitmapOptions) -> Option<ColorSpaceConversion> { panic!("web-sys intrinsic") }
    #[cfg(feature = "ColorSpaceConversion")]
    #[doc = "Change the `colorSpaceConversion` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ColorSpaceConversion`, `ImageBitmapOptions`*"]
#[js::import = "None = $1"]
pub fn set_color_space_conversion(this: &ImageBitmapOptions, val: ColorSpaceConversion) { panic!("web-sys intrinsic") }
    #[cfg(feature = "ImageOrientation")]
    #[doc = "Get the `imageOrientation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageBitmapOptions`, `ImageOrientation`*"]
#[js::import = "None"]
pub fn get_image_orientation(this: &ImageBitmapOptions) -> Option<ImageOrientation> { panic!("web-sys intrinsic") }
    #[cfg(feature = "ImageOrientation")]
    #[doc = "Change the `imageOrientation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageBitmapOptions`, `ImageOrientation`*"]
#[js::import = "None = $1"]
pub fn set_image_orientation(this: &ImageBitmapOptions, val: ImageOrientation) { panic!("web-sys intrinsic") }
    #[cfg(feature = "PremultiplyAlpha")]
    #[doc = "Get the `premultiplyAlpha` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageBitmapOptions`, `PremultiplyAlpha`*"]
#[js::import = "None"]
pub fn get_premultiply_alpha(this: &ImageBitmapOptions) -> Option<PremultiplyAlpha> { panic!("web-sys intrinsic") }
    #[cfg(feature = "PremultiplyAlpha")]
    #[doc = "Change the `premultiplyAlpha` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageBitmapOptions`, `PremultiplyAlpha`*"]
#[js::import = "None = $1"]
pub fn set_premultiply_alpha(this: &ImageBitmapOptions, val: PremultiplyAlpha) { panic!("web-sys intrinsic") }
    #[doc = "Get the `resizeHeight` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageBitmapOptions`*"]
#[js::import = "None"]
pub fn get_resize_height(this: &ImageBitmapOptions) -> Option<u32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `resizeHeight` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageBitmapOptions`*"]
#[js::import = "None = $1"]
pub fn set_resize_height(this: &ImageBitmapOptions, val: u32) { panic!("web-sys intrinsic") }
    #[cfg(feature = "ResizeQuality")]
    #[doc = "Get the `resizeQuality` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageBitmapOptions`, `ResizeQuality`*"]
#[js::import = "None"]
pub fn get_resize_quality(this: &ImageBitmapOptions) -> Option<ResizeQuality> { panic!("web-sys intrinsic") }
    #[cfg(feature = "ResizeQuality")]
    #[doc = "Change the `resizeQuality` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageBitmapOptions`, `ResizeQuality`*"]
#[js::import = "None = $1"]
pub fn set_resize_quality(this: &ImageBitmapOptions, val: ResizeQuality) { panic!("web-sys intrinsic") }
    #[doc = "Get the `resizeWidth` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageBitmapOptions`*"]
#[js::import = "None"]
pub fn get_resize_width(this: &ImageBitmapOptions) -> Option<u32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `resizeWidth` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageBitmapOptions`*"]
#[js::import = "None = $1"]
pub fn set_resize_width(this: &ImageBitmapOptions, val: u32) { panic!("web-sys intrinsic") }
impl ImageBitmapOptions {
    #[doc = "Construct a new `ImageBitmapOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageBitmapOptions`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[cfg(feature = "ColorSpaceConversion")]
    #[deprecated = "Use `set_color_space_conversion()` instead."]
#[js::import = "None"]
pub fn color_space_conversion(&mut self, val: ColorSpaceConversion) -> &mut Self { self.set_color_space_conversion(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(feature = "ImageOrientation")]
    #[deprecated = "Use `set_image_orientation()` instead."]
#[js::import = "None"]
pub fn image_orientation(&mut self, val: ImageOrientation) -> &mut Self { self.set_image_orientation(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(feature = "PremultiplyAlpha")]
    #[deprecated = "Use `set_premultiply_alpha()` instead."]
#[js::import = "None"]
pub fn premultiply_alpha(&mut self, val: PremultiplyAlpha) -> &mut Self { self.set_premultiply_alpha(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_resize_height()` instead."]
#[js::import = "None"]
pub fn resize_height(&mut self, val: u32) -> &mut Self { self.set_resize_height(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(feature = "ResizeQuality")]
    #[deprecated = "Use `set_resize_quality()` instead."]
#[js::import = "None"]
pub fn resize_quality(&mut self, val: ResizeQuality) -> &mut Self { self.set_resize_quality(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_resize_width()` instead."]
#[js::import = "None"]
pub fn resize_width(&mut self, val: u32) -> &mut Self { self.set_resize_width(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for ImageBitmapOptions {
    fn default() -> Self {
        Self::new()
    }
}
