#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
#[cfg(web_sys_unstable_apis)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PictureInPictureEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PictureInPictureEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PictureInPictureEvent`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::opaque]
#[derive(Copy)]
pub struct PictureInPictureEvent(pub ::wasm_bindgen::JsValue);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "PictureInPictureWindow")]
    #[doc = "Getter for the `pictureInPictureWindow` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PictureInPictureEvent/pictureInPictureWindow)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PictureInPictureEvent`, `PictureInPictureWindow`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl PictureInPictureEvent {
    #[js::import = "$0.pictureInPictureWindow"]
pub fn picture_in_picture_window(&self) -> PictureInPictureWindow { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "PictureInPictureEventInit")]
    #[doc = "The `new PictureInPictureEvent(..)` constructor, creating a new instance of `PictureInPictureEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PictureInPictureEvent/PictureInPictureEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PictureInPictureEvent`, `PictureInPictureEventInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl PictureInPictureEvent {
    #[js::import = "new PictureInPictureEvent($0, $1)"]
pub fn new(type_: &str, event_init_dict: &PictureInPictureEventInit,) -> Result<PictureInPictureEvent, JsValue> { panic!("web-sys intrinsic") }
}
