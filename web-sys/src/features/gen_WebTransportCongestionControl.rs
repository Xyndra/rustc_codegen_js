#![allow(unused_imports)]
#![allow(clippy::all)]
#[cfg(web_sys_unstable_apis)]
#[doc = "The `WebTransportCongestionControl` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `WebTransportCongestionControl`*"]
#[doc = ""]
#[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
#[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WebTransportCongestionControl {
    Default = "default",
    Throughput = "throughput",
    LowLatency = "low-latency",
}
