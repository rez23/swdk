use alloc::vec::Vec;
use swdk::declare_ctx_descriptor;
use crate::device::models::GamepadModels;

pub mod models;
pub mod policy;

#[derive(Default)]
pub struct DeviceData {
    pub release: bool,
    pub model: GamepadModels,
    pub allowed_pid: Vec<u16>,
}
declare_ctx_descriptor!(DeviceData);