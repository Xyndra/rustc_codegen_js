#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
#[cfg(web_sys_unstable_apis)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuRenderPassTimestampWrites` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPassTimestampWrites`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::opaque]
#[derive(Copy)]
pub struct GpuRenderPassTimestampWrites(pub ::wasm_bindgen::JsValue);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `beginningOfPassWriteIndex` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPassTimestampWrites`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None"]
pub fn get_beginning_of_pass_write_index(this: &GpuRenderPassTimestampWrites) -> Option<u32> { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `beginningOfPassWriteIndex` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPassTimestampWrites`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None = $1"]
pub fn set_beginning_of_pass_write_index(this: &GpuRenderPassTimestampWrites, val: u32) { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `endOfPassWriteIndex` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPassTimestampWrites`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None"]
pub fn get_end_of_pass_write_index(this: &GpuRenderPassTimestampWrites) -> Option<u32> { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `endOfPassWriteIndex` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPassTimestampWrites`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None = $1"]
pub fn set_end_of_pass_write_index(this: &GpuRenderPassTimestampWrites, val: u32) { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuQuerySet")]
    #[doc = "Get the `querySet` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuQuerySet`, `GpuRenderPassTimestampWrites`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None"]
pub fn get_query_set(this: &GpuRenderPassTimestampWrites) -> GpuQuerySet { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuQuerySet")]
    #[doc = "Change the `querySet` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuQuerySet`, `GpuRenderPassTimestampWrites`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None = $1"]
pub fn set_query_set(this: &GpuRenderPassTimestampWrites, val: &GpuQuerySet) { panic!("web-sys intrinsic") }
#[cfg(web_sys_unstable_apis)]
impl GpuRenderPassTimestampWrites {
    #[cfg(feature = "GpuQuerySet")]
    #[doc = "Construct a new `GpuRenderPassTimestampWrites`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuQuerySet`, `GpuRenderPassTimestampWrites`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None"]
pub fn new(query_set: &GpuQuerySet) -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret.set_query_set(query_set);
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_beginning_of_pass_write_index()` instead."]
#[js::import = "None"]
pub fn beginning_of_pass_write_index(&mut self, val: u32) -> &mut Self { self.set_beginning_of_pass_write_index(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_end_of_pass_write_index()` instead."]
#[js::import = "None"]
pub fn end_of_pass_write_index(&mut self, val: u32) -> &mut Self { self.set_end_of_pass_write_index(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuQuerySet")]
    #[deprecated = "Use `set_query_set()` instead."]
#[js::import = "None"]
pub fn query_set(&mut self, val: &GpuQuerySet) -> &mut Self { self.set_query_set(val) { panic!("web-sys intrinsic") }
        self
    }
}
