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

#[test]
fn state_conversion_started() {
    let raw = i32::from(WdfIoTargetState::Started);

    assert_eq!(
        WdfIoTargetState::from(raw),
        WdfIoTargetState::Started
    );
}

#[test]
fn state_conversion_closed() {
    let raw = i32::from(WdfIoTargetState::Closed);

    assert_eq!(
        WdfIoTargetState::from(raw),
        WdfIoTargetState::Closed
    );
}

#[test]
fn unknown_state_is_preserved() {
    assert_eq!(
        WdfIoTargetState::from(999),
        WdfIoTargetState::Unknown(999)
    );
}