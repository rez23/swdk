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

mod device;

use swdk::builders::{
    WdfDriverConf, WdfDriverSetup, WdfObjAttrs,
};
use swdk::context::WdfCtxNoneDesc;
use swdk::handle::Handle;
use swdk::ioctl::{IoCtlRequest, IoCtlResponse};
use swdk::operators::{AsWdfOwned, AsWdfOwner};
use swdk::rt::wdk_sys::{
    HID_COLLECTION_INFORMATION, NTSTATUS, PCUNICODE_STRING,
    PDRIVER_OBJECT, PWDFDEVICE_INIT, STATUS_SUCCESS,
    STATUS_UNSUCCESSFUL, WDFDEVICE, WDFDRIVER, WDFIOTARGET,
};
use swdk::values::WdfIoTargetError::IoCtlTargetSendError;
use swdk::{
    debug, error, if_nterror_return_ntstatus, info, ioctl,
};

#[cfg(not(test))]
use swdk::rt::WdkAllocator;

use crate::device::DeviceData;
use crate::device::models::GamepadModels;

#[cfg(not(test))]
#[global_allocator]
static GLOBAL_ALLOCATOR: WdkAllocator = WdkAllocator;

/// Main entry point for the KMDF driver.
///
/// # Panics
/// This function may panic if internal string conversions (e.g. `CString::new`)
/// fail due to invalid UTF-8 input. Such panics will trigger the kernel panic
/// handler provided by `wdk_panic`.
///
/// # Safety
/// This function is called directly by the Windows kernel. The pointers
/// `driver` and `registry_path` must be valid for the duration of the call.
/// The caller (the OS) guarantees these invariants. The function must not
/// assume any additional safety beyond what KMDF provides.
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

    let gamepad_model = GamepadModels::from_vid_and_pid(
        device_info.ProductID,
        device_info.VendorID,
    );

    info!("Device capabilities: {}", gamepad_model);

    STATUS_SUCCESS
}
