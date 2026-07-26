use swdk::Handle;
use swdk::ioctl::IoCtlRequest;
use swdk::op::AsWdfOwned;
use swdk::vals::WdfIoTargetState;

use swdk::rt::wdk_sys::{
    WDFDEVICE,
    WDFIOTARGET,
};


#[derive(Default)]
struct EchoRequest {
    value: u32,
}

#[derive(Default)]
struct EchoResponse {
    value: u32,
}

#[unsafe(no_mangle)]
static mut WDFDEVICE: WDFIOTARGET =
    core::ptr::dangling_mut();
#[test]
fn read_status_returns_started() {
    let target =
        Handle::<WDFIOTARGET>::new(core::ptr::dangling_mut());

    assert_eq!(
        target.read_status(),
        WdfIoTargetState::Started
    );
}

#[test]
#[should_panic(expected = "Cannot create WDFIOTARGET")]
fn cannot_create_iotarget_from_device() {
    let device: WDFDEVICE =
        core::ptr::dangling_mut();

    let _ =
        Handle::<WDFIOTARGET>::from_owner(&device);
}

#[test]
fn send_ioctl_returns_ok() {
    let target =
        Handle::<WDFIOTARGET>::new(core::ptr::dangling_mut());

    let request =
        IoCtlRequest::new(0x1234, EchoRequest::default());

    let result =
        target.send_ioctl_sync(request);

    assert!(result.is_ok());
}

#[test]
fn send_ioctl_returns_default_response() {
    let target =
        Handle::<WDFIOTARGET>::new(core::ptr::dangling_mut());

    let request =
        IoCtlRequest::new(0x1234, EchoRequest::default());

    let response =
        target
            .send_ioctl_sync(request)
            .unwrap();

    assert_eq!(response.value, 0);
}