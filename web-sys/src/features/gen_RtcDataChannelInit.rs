#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcDataChannelInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannelInit`*"]
#[js::opaque]
#[derive(Copy)]
pub struct RtcDataChannelInit(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannelInit`*"]
#[js::import = "None"]
pub fn get_id(this: &RtcDataChannelInit) -> Option<u16> { panic!("web-sys intrinsic") }
    #[doc = "Change the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannelInit`*"]
#[js::import = "None = $1"]
pub fn set_id(this: &RtcDataChannelInit, val: u16) { panic!("web-sys intrinsic") }
    #[doc = "Get the `maxPacketLifeTime` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannelInit`*"]
#[js::import = "None"]
pub fn get_max_packet_life_time(this: &RtcDataChannelInit) -> Option<u16> { panic!("web-sys intrinsic") }
    #[doc = "Change the `maxPacketLifeTime` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannelInit`*"]
#[js::import = "None = $1"]
pub fn set_max_packet_life_time(this: &RtcDataChannelInit, val: u16) { panic!("web-sys intrinsic") }
    #[doc = "Get the `maxRetransmitTime` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannelInit`*"]
#[js::import = "None"]
pub fn get_max_retransmit_time(this: &RtcDataChannelInit) -> Option<u16> { panic!("web-sys intrinsic") }
    #[doc = "Change the `maxRetransmitTime` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannelInit`*"]
#[js::import = "None = $1"]
pub fn set_max_retransmit_time(this: &RtcDataChannelInit, val: u16) { panic!("web-sys intrinsic") }
    #[doc = "Get the `maxRetransmits` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannelInit`*"]
#[js::import = "None"]
pub fn get_max_retransmits(this: &RtcDataChannelInit) -> Option<u16> { panic!("web-sys intrinsic") }
    #[doc = "Change the `maxRetransmits` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannelInit`*"]
#[js::import = "None = $1"]
pub fn set_max_retransmits(this: &RtcDataChannelInit, val: u16) { panic!("web-sys intrinsic") }
    #[doc = "Get the `negotiated` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannelInit`*"]
#[js::import = "None"]
pub fn get_negotiated(this: &RtcDataChannelInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `negotiated` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannelInit`*"]
#[js::import = "None = $1"]
pub fn set_negotiated(this: &RtcDataChannelInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `ordered` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannelInit`*"]
#[js::import = "None"]
pub fn get_ordered(this: &RtcDataChannelInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `ordered` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannelInit`*"]
#[js::import = "None = $1"]
pub fn set_ordered(this: &RtcDataChannelInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `protocol` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannelInit`*"]
#[js::import = "None"]
pub fn get_protocol(this: &RtcDataChannelInit) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `protocol` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannelInit`*"]
#[js::import = "None = $1"]
pub fn set_protocol(this: &RtcDataChannelInit, val: &str) { panic!("web-sys intrinsic") }
impl RtcDataChannelInit {
    #[doc = "Construct a new `RtcDataChannelInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannelInit`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_id()` instead."]
#[js::import = "None"]
pub fn id(&mut self, val: u16) -> &mut Self { self.set_id(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_max_packet_life_time()` instead."]
#[js::import = "None"]
pub fn max_packet_life_time(&mut self, val: u16) -> &mut Self { self.set_max_packet_life_time(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_max_retransmit_time()` instead."]
#[js::import = "None"]
pub fn max_retransmit_time(&mut self, val: u16) -> &mut Self { self.set_max_retransmit_time(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_max_retransmits()` instead."]
#[js::import = "None"]
pub fn max_retransmits(&mut self, val: u16) -> &mut Self { self.set_max_retransmits(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_negotiated()` instead."]
#[js::import = "None"]
pub fn negotiated(&mut self, val: bool) -> &mut Self { self.set_negotiated(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_ordered()` instead."]
#[js::import = "None"]
pub fn ordered(&mut self, val: bool) -> &mut Self { self.set_ordered(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_protocol()` instead."]
#[js::import = "None"]
pub fn protocol(&mut self, val: &str) -> &mut Self { self.set_protocol(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for RtcDataChannelInit {
    fn default() -> Self {
        Self::new()
    }
}
