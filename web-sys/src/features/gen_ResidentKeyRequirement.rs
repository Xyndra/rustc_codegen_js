#![allow(unused_imports)]
#![allow(clippy::all)]
#[doc = "The `ResidentKeyRequirement` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `ResidentKeyRequirement`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResidentKeyRequirement {
    Discouraged = "discouraged",
    Preferred = "preferred",
    Required = "required",
}
