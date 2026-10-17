#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcDataChannelEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannelEventInit`*"]
#[js::opaque]
#[derive(Copy)]
pub struct RtcDataChannelEventInit(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannelEventInit`*"]
#[js::import = "None"]
pub fn get_bubbles(this: &RtcDataChannelEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannelEventInit`*"]
#[js::import = "None = $1"]
pub fn set_bubbles(this: &RtcDataChannelEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannelEventInit`*"]
#[js::import = "None"]
pub fn get_cancelable(this: &RtcDataChannelEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannelEventInit`*"]
#[js::import = "None = $1"]
pub fn set_cancelable(this: &RtcDataChannelEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannelEventInit`*"]
#[js::import = "None"]
pub fn get_composed(this: &RtcDataChannelEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannelEventInit`*"]
#[js::import = "None = $1"]
pub fn set_composed(this: &RtcDataChannelEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[cfg(feature = "RtcDataChannel")]
    #[doc = "Get the `channel` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannel`, `RtcDataChannelEventInit`*"]
#[js::import = "None"]
pub fn get_channel(this: &RtcDataChannelEventInit) -> RtcDataChannel { panic!("web-sys intrinsic") }
    #[cfg(feature = "RtcDataChannel")]
    #[doc = "Change the `channel` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannel`, `RtcDataChannelEventInit`*"]
#[js::import = "None = $1"]
pub fn set_channel(this: &RtcDataChannelEventInit, val: &RtcDataChannel) { panic!("web-sys intrinsic") }
impl RtcDataChannelEventInit {
    #[cfg(feature = "RtcDataChannel")]
    #[doc = "Construct a new `RtcDataChannelEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannel`, `RtcDataChannelEventInit`*"]
#[js::import = "None"]
pub fn new(channel: &RtcDataChannel) -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret.set_channel(channel);
        ret
    }
    #[deprecated = "Use `set_bubbles()` instead."]
#[js::import = "None"]
pub fn bubbles(&mut self, val: bool) -> &mut Self { self.set_bubbles(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_cancelable()` instead."]
#[js::import = "None"]
pub fn cancelable(&mut self, val: bool) -> &mut Self { self.set_cancelable(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_composed()` instead."]
#[js::import = "None"]
pub fn composed(&mut self, val: bool) -> &mut Self { self.set_composed(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(feature = "RtcDataChannel")]
    #[deprecated = "Use `set_channel()` instead."]
#[js::import = "None"]
pub fn channel(&mut self, val: &RtcDataChannel) -> &mut Self { self.set_channel(val) { panic!("web-sys intrinsic") }
        self
    }
}
