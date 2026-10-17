#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ProfileTimelineMarker` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineMarker`*"]
#[js::opaque]
#[derive(Copy)]
pub struct ProfileTimelineMarker(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `causeName` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineMarker`*"]
#[js::import = "None"]
pub fn get_cause_name(this: &ProfileTimelineMarker) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `causeName` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineMarker`*"]
#[js::import = "None = $1"]
pub fn set_cause_name(this: &ProfileTimelineMarker, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `end` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineMarker`*"]
#[js::import = "None"]
pub fn get_end(this: &ProfileTimelineMarker) -> Option<f64> { panic!("web-sys intrinsic") }
    #[doc = "Change the `end` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineMarker`*"]
#[js::import = "None = $1"]
pub fn set_end(this: &ProfileTimelineMarker, val: f64) { panic!("web-sys intrinsic") }
    #[doc = "Get the `endStack` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineMarker`*"]
#[js::import = "None"]
pub fn get_end_stack(this: &ProfileTimelineMarker) -> Option<::js_sys::Object> { panic!("web-sys intrinsic") }
    #[doc = "Change the `endStack` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineMarker`*"]
#[js::import = "None = $1"]
pub fn set_end_stack(this: &ProfileTimelineMarker, val: Option<&::js_sys::Object>) { panic!("web-sys intrinsic") }
    #[doc = "Get the `eventPhase` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineMarker`*"]
#[js::import = "None"]
pub fn get_event_phase(this: &ProfileTimelineMarker) -> Option<u16> { panic!("web-sys intrinsic") }
    #[doc = "Change the `eventPhase` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineMarker`*"]
#[js::import = "None = $1"]
pub fn set_event_phase(this: &ProfileTimelineMarker, val: u16) { panic!("web-sys intrinsic") }
    #[doc = "Get the `isAnimationOnly` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineMarker`*"]
#[js::import = "None"]
pub fn get_is_animation_only(this: &ProfileTimelineMarker) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `isAnimationOnly` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineMarker`*"]
#[js::import = "None = $1"]
pub fn set_is_animation_only(this: &ProfileTimelineMarker, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `isOffMainThread` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineMarker`*"]
#[js::import = "None"]
pub fn get_is_off_main_thread(this: &ProfileTimelineMarker) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `isOffMainThread` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineMarker`*"]
#[js::import = "None = $1"]
pub fn set_is_off_main_thread(this: &ProfileTimelineMarker, val: bool) { panic!("web-sys intrinsic") }
    #[cfg(feature = "ProfileTimelineMessagePortOperationType")]
    #[doc = "Get the `messagePortOperation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineMarker`, `ProfileTimelineMessagePortOperationType`*"]
#[js::import = "None"]
pub fn get_message_port_operation(this: &ProfileTimelineMarker,) -> Option<ProfileTimelineMessagePortOperationType> { panic!("web-sys intrinsic") }
    #[cfg(feature = "ProfileTimelineMessagePortOperationType")]
    #[doc = "Change the `messagePortOperation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineMarker`, `ProfileTimelineMessagePortOperationType`*"]
#[js::import = "None = $1"]
pub fn set_message_port_operation(this: &ProfileTimelineMarker, val: ProfileTimelineMessagePortOperationType,) { panic!("web-sys intrinsic") }
    #[doc = "Get the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineMarker`*"]
#[js::import = "None"]
pub fn get_name(this: &ProfileTimelineMarker) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineMarker`*"]
#[js::import = "None = $1"]
pub fn set_name(this: &ProfileTimelineMarker, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `processType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineMarker`*"]
#[js::import = "None"]
pub fn get_process_type(this: &ProfileTimelineMarker) -> Option<u16> { panic!("web-sys intrinsic") }
    #[doc = "Change the `processType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineMarker`*"]
#[js::import = "None = $1"]
pub fn set_process_type(this: &ProfileTimelineMarker, val: u16) { panic!("web-sys intrinsic") }
    #[doc = "Get the `rectangles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineMarker`*"]
#[js::import = "None"]
pub fn get_rectangles(this: &ProfileTimelineMarker) -> Option<::js_sys::Array> { panic!("web-sys intrinsic") }
    #[doc = "Change the `rectangles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineMarker`*"]
#[js::import = "None = $1"]
pub fn set_rectangles(this: &ProfileTimelineMarker, val: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
    #[doc = "Get the `stack` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineMarker`*"]
#[js::import = "None"]
pub fn get_stack(this: &ProfileTimelineMarker) -> Option<::js_sys::Object> { panic!("web-sys intrinsic") }
    #[doc = "Change the `stack` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineMarker`*"]
#[js::import = "None = $1"]
pub fn set_stack(this: &ProfileTimelineMarker, val: Option<&::js_sys::Object>) { panic!("web-sys intrinsic") }
    #[doc = "Get the `start` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineMarker`*"]
#[js::import = "None"]
pub fn get_start(this: &ProfileTimelineMarker) -> Option<f64> { panic!("web-sys intrinsic") }
    #[doc = "Change the `start` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineMarker`*"]
#[js::import = "None = $1"]
pub fn set_start(this: &ProfileTimelineMarker, val: f64) { panic!("web-sys intrinsic") }
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineMarker`*"]
#[js::import = "None"]
pub fn get_type(this: &ProfileTimelineMarker) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineMarker`*"]
#[js::import = "None = $1"]
pub fn set_type(this: &ProfileTimelineMarker, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `unixTime` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineMarker`*"]
#[js::import = "None"]
pub fn get_unix_time(this: &ProfileTimelineMarker) -> Option<f64> { panic!("web-sys intrinsic") }
    #[doc = "Change the `unixTime` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineMarker`*"]
#[js::import = "None = $1"]
pub fn set_unix_time(this: &ProfileTimelineMarker, val: f64) { panic!("web-sys intrinsic") }
    #[cfg(feature = "ProfileTimelineWorkerOperationType")]
    #[doc = "Get the `workerOperation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineMarker`, `ProfileTimelineWorkerOperationType`*"]
#[js::import = "None"]
pub fn get_worker_operation(this: &ProfileTimelineMarker,) -> Option<ProfileTimelineWorkerOperationType> { panic!("web-sys intrinsic") }
    #[cfg(feature = "ProfileTimelineWorkerOperationType")]
    #[doc = "Change the `workerOperation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineMarker`, `ProfileTimelineWorkerOperationType`*"]
#[js::import = "None = $1"]
pub fn set_worker_operation(this: &ProfileTimelineMarker, val: ProfileTimelineWorkerOperationType,) { panic!("web-sys intrinsic") }
impl ProfileTimelineMarker {
    #[doc = "Construct a new `ProfileTimelineMarker`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineMarker`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_cause_name()` instead."]
#[js::import = "None"]
pub fn cause_name(&mut self, val: &str) -> &mut Self { self.set_cause_name(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_end()` instead."]
#[js::import = "None"]
pub fn end(&mut self, val: f64) -> &mut Self { self.set_end(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_end_stack()` instead."]
#[js::import = "None"]
pub fn end_stack(&mut self, val: Option<&::js_sys::Object>) -> &mut Self { self.set_end_stack(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_event_phase()` instead."]
#[js::import = "None"]
pub fn event_phase(&mut self, val: u16) -> &mut Self { self.set_event_phase(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_is_animation_only()` instead."]
#[js::import = "None"]
pub fn is_animation_only(&mut self, val: bool) -> &mut Self { self.set_is_animation_only(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_is_off_main_thread()` instead."]
#[js::import = "None"]
pub fn is_off_main_thread(&mut self, val: bool) -> &mut Self { self.set_is_off_main_thread(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(feature = "ProfileTimelineMessagePortOperationType")]
    #[deprecated = "Use `set_message_port_operation()` instead."]
#[js::import = "None"]
pub fn message_port_operation(&mut self, val: ProfileTimelineMessagePortOperationType,) -> &mut Self { self.set_message_port_operation(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_name()` instead."]
#[js::import = "None"]
pub fn name(&mut self, val: &str) -> &mut Self { self.set_name(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_process_type()` instead."]
#[js::import = "None"]
pub fn process_type(&mut self, val: u16) -> &mut Self { self.set_process_type(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_rectangles()` instead."]
#[js::import = "None"]
pub fn rectangles(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self { self.set_rectangles(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_stack()` instead."]
#[js::import = "None"]
pub fn stack(&mut self, val: Option<&::js_sys::Object>) -> &mut Self { self.set_stack(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_start()` instead."]
#[js::import = "None"]
pub fn start(&mut self, val: f64) -> &mut Self { self.set_start(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_type()` instead."]
#[js::import = "None"]
pub fn type_(&mut self, val: &str) -> &mut Self { self.set_type(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_unix_time()` instead."]
#[js::import = "None"]
pub fn unix_time(&mut self, val: f64) -> &mut Self { self.set_unix_time(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(feature = "ProfileTimelineWorkerOperationType")]
    #[deprecated = "Use `set_worker_operation()` instead."]
#[js::import = "None"]
pub fn worker_operation(&mut self, val: ProfileTimelineWorkerOperationType) -> &mut Self { self.set_worker_operation(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for ProfileTimelineMarker {
    fn default() -> Self {
        Self::new()
    }
}
