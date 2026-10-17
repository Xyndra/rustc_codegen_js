#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WebglCompressedTextureS3tc` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_compressed_texture_s3tc)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebglCompressedTextureS3tc`*"]
#[js::opaque]
#[derive(Copy)]
pub struct WebglCompressedTextureS3tc(pub ::wasm_bindgen::JsValue);
impl WebglCompressedTextureS3tc {
    #[doc = "The `WEBGL_compressed_texture_s3tc.COMPRESSED_RGB_S3TC_DXT1_EXT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebglCompressedTextureS3tc`*"]
    pub const COMPRESSED_RGB_S3TC_DXT1_EXT: u32 = 33776u64 as u32;
    #[doc = "The `WEBGL_compressed_texture_s3tc.COMPRESSED_RGBA_S3TC_DXT1_EXT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebglCompressedTextureS3tc`*"]
    pub const COMPRESSED_RGBA_S3TC_DXT1_EXT: u32 = 33777u64 as u32;
    #[doc = "The `WEBGL_compressed_texture_s3tc.COMPRESSED_RGBA_S3TC_DXT3_EXT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebglCompressedTextureS3tc`*"]
    pub const COMPRESSED_RGBA_S3TC_DXT3_EXT: u32 = 33778u64 as u32;
    #[doc = "The `WEBGL_compressed_texture_s3tc.COMPRESSED_RGBA_S3TC_DXT5_EXT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebglCompressedTextureS3tc`*"]
    pub const COMPRESSED_RGBA_S3TC_DXT5_EXT: u32 = 33779u64 as u32;
}
