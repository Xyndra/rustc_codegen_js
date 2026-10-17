#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WebglCompressedTextureS3tcSrgb` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_compressed_texture_s3tc_srgb)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebglCompressedTextureS3tcSrgb`*"]
#[js::opaque]
#[derive(Copy)]
pub struct WebglCompressedTextureS3tcSrgb(pub ::wasm_bindgen::JsValue);
impl WebglCompressedTextureS3tcSrgb {
    #[doc = "The `WEBGL_compressed_texture_s3tc_srgb.COMPRESSED_SRGB_S3TC_DXT1_EXT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebglCompressedTextureS3tcSrgb`*"]
    pub const COMPRESSED_SRGB_S3TC_DXT1_EXT: u32 = 35916u64 as u32;
    #[doc = "The `WEBGL_compressed_texture_s3tc_srgb.COMPRESSED_SRGB_ALPHA_S3TC_DXT1_EXT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebglCompressedTextureS3tcSrgb`*"]
    pub const COMPRESSED_SRGB_ALPHA_S3TC_DXT1_EXT: u32 = 35917u64 as u32;
    #[doc = "The `WEBGL_compressed_texture_s3tc_srgb.COMPRESSED_SRGB_ALPHA_S3TC_DXT3_EXT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebglCompressedTextureS3tcSrgb`*"]
    pub const COMPRESSED_SRGB_ALPHA_S3TC_DXT3_EXT: u32 = 35918u64 as u32;
    #[doc = "The `WEBGL_compressed_texture_s3tc_srgb.COMPRESSED_SRGB_ALPHA_S3TC_DXT5_EXT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebglCompressedTextureS3tcSrgb`*"]
    pub const COMPRESSED_SRGB_ALPHA_S3TC_DXT5_EXT: u32 = 35919u64 as u32;
}
