use std::ptr::{dangling, NonNull};
use swdk::rt::wdk_sys::WDFIOTARGET__;
use swdk::Handle;
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
        Handle::<WDFIOTARGET__>::new(NonNull::dangling());

    assert_eq!(
        target.read_status(),
        Some(WdfIoTargetState::Started)
    );
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