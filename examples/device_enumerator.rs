#![no_std]
#![feature(
    //trait_alias,
    //lazy_type_alias,
    //associated_type_defaults,
    //min_specialization,
    //generic_const_exprs,
    //type_alias_impl_trait,
    negative_impls
    //impl_trait_in_assoc_type,
)]
extern crate alloc;

use alloc::string::String;
use wdk_alloc::WdkAllocator;
use wdk_sys::HID_COLLECTION_INFORMATION;
use swdk::rt::wdk_sys::{
    NTSTATUS, PCUNICODE_STRING,
    PDRIVER_OBJECT, PWDFDEVICE_INIT, STATUS_SUCCESS,
    STATUS_UNSUCCESSFUL, WDFDEVICE, WDFDRIVER, WDFIOTARGET,
};
use swdk::val::WdfIoTargetError::IoCtlTargetSendError;
use swdk::{debug, declare_ctx_descriptor, error, if_nterror_return_ntstatus, info, ioctl, Handle};
use swdk::bd::{WdfDriverConf, WdfDriverSetup, WdfObjAttrs};
use swdk::ctx::WdfCtxNoneDesc;
use swdk::ioctl::{IoCtlRequest, IoCtlResponse};
use swdk::op::{AsWdfOwned, AsWdfOwner};

#[cfg(not(test))]
#[global_allocator]
static GLOBAL_ALLOCATOR: WdkAllocator = WdkAllocator;

#[derive(Default)]
struct DeviceData {
    pub model: String
}
declare_ctx_descriptor!(DeviceData);

#[unsafe(export_name = "DriverEntry")]
pub unsafe extern "system" fn driver_entry(
    driver_obj: PDRIVER_OBJECT,
    registry_path: PCUNICODE_STRING,
) -> NTSTATUS {
    debug!("DriverEntry launched from WDF");
    if_nterror_return_ntstatus!(
        Handle::<WDFDRIVER>::from_owned_with_attrs(
            driver_obj,
            WdfDriverConf {
                setup: WdfDriverSetup {
                    on_driver_unload: Some(
                        on_driver_unload
                    ),
                    on_device_add: Some(
                        on_driver_device_add
                    ),
                    ..WdfDriverSetup::default()
                },
                registry_path,
            },
            Some(WdfObjAttrs::<WdfCtxNoneDesc>::default())
        )
    );
    STATUS_SUCCESS
}

unsafe extern "C" fn on_driver_unload(_driver: WDFDRIVER) {
    info!("Driver unload event triggered.");
}

#[unsafe(link_section = "PAGE")]
unsafe extern "C" fn on_driver_device_add(
    _driver: WDFDRIVER,
    device_init: PWDFDEVICE_INIT,
) -> NTSTATUS {
    debug!("Entering in function on_driver_device_add");
    let device_handle = if_nterror_return_ntstatus!(
        Handle::<WDFDEVICE>::from_owned(
            device_init,
            Some(WdfObjAttrs::<DeviceData>::default())
        )
    );

    debug!("Getting device capabilities");
    let iot_handler = if_nterror_return_ntstatus!(
        Handle::<WDFIOTARGET>::from_owner(&device_handle)
    );

    // get device capabilities
    let device_info: IoCtlResponse<HID_COLLECTION_INFORMATION> = if_nterror_return_ntstatus!(
        iot_handler.send_ioctl(IoCtlRequest::with_command(
        ioctl::commands::IOCTL_HID_GET_COLLECTION_INFORMATION,
    )).map_err(|err| {
            match err {
                IoCtlTargetSendError(status) => {
                    error!(
                        "'WdfIoTargetSendIoctlSynchronously' \
                        failed for command \
                        '0x{:08X} with status 0x{:08X}",
                        status.command,
                        status.ntstatus,
                    );
                    debug!("IOCTL sent request: {:?}", status.request);
                    status.ntstatus
                },
                err => {
                    error!("Failed to get device capabilities from IOCTL: {:?}", err);
                    STATUS_UNSUCCESSFUL
                }
            }
        }
    ));

    // add name to ctx
    info!("Device capabilities: {}", gamepad_model);

    STATUS_SUCCESS
}

fn main() {
    // placeholder for test
}