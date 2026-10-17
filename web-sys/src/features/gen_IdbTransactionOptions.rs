#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
#[cfg(web_sys_unstable_apis)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `IdbTransactionOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbTransactionOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::opaque]
#[derive(Copy)]
pub struct IdbTransactionOptions(pub ::wasm_bindgen::JsValue);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "IdbTransactionDurability")]
    #[doc = "Get the `durability` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbTransactionDurability`, `IdbTransactionOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None"]
pub fn get_durability(this: &IdbTransactionOptions) -> Option<IdbTransactionDurability> { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "IdbTransactionDurability")]
    #[doc = "Change the `durability` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbTransactionDurability`, `IdbTransactionOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None = $1"]
pub fn set_durability(this: &IdbTransactionOptions, val: IdbTransactionDurability) { panic!("web-sys intrinsic") }
#[cfg(web_sys_unstable_apis)]
impl IdbTransactionOptions {
    #[doc = "Construct a new `IdbTransactionOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbTransactionOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "IdbTransactionDurability")]
    #[deprecated = "Use `set_durability()` instead."]
#[js::import = "None"]
pub fn durability(&mut self, val: IdbTransactionDurability) -> &mut Self { self.set_durability(val) { panic!("web-sys intrinsic") }
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for IdbTransactionOptions {
    fn default() -> Self {
        Self::new()
    }
}
