use swdk::ioctl::{
    IoCtlRequest,
    IoBuffer,
};
use swdk::op::{AsBuilder, AsBuilderMut};

#[test]
fn request_keeps_command() {
    let req = IoCtlRequest::new(0x1234, 42u32);

    assert_eq!(req.command(), 0x1234);
}

#[test]
fn request_without_payload_builds_none() {
    let req =
        IoCtlRequest::<Option<u32>>::with_command(0x1234);

    assert!(req.build().is_none());
}

#[test]
fn response_default_allocates_default_value() {
    let response =
        IoBuffer::<u32>::default();

    assert_eq!(*response, 0);
}

#[test]
fn response_into_inner_returns_inner() {
    let response =
        IoBuffer::<u32>::new(77);

    assert_eq!(response.into_inner(), 77);
}

#[test]
fn request_build_keeps_payload_alive() {
    let req = IoCtlRequest::new(0x1234, 42u32);

    let _ = req.build();

    assert_eq!(
        req.as_ref(),
        &Some(42)
    );
}

#[test]
fn response_build_does_not_modify_value() {
    let response = IoBuffer::new(77u32);

    let _ = response.build();

    assert_eq!(*response, 77);
}

#[test]
fn response_build_mut_does_not_modify_value() {
    let mut response = IoBuffer::new(77u32);

    let _ = response.build_mut();

    assert_eq!(*response, 77);
}

#[test]
fn response_build_and_build_mut_report_same_size() {
    let mut response =
        IoBuffer::<u32>::default();

    let read_desc = response.build();
    let write_desc = response.build_mut();

    assert_eq!(
        unsafe { read_desc.u.BufferType.Length },
        unsafe { write_desc.u.BufferType.Length }
    );
}

#[test]
fn request_with_payload_builds_non_empty_descriptor() {
    let req = IoCtlRequest::new(0x1234, 42u32);

    let desc = req.build().unwrap();

    assert_ne!(
        unsafe { desc.u.BufferType.Length },
        0
    );
}

#[test]
fn response_build_produces_non_empty_descriptor() {
    let response =
        IoBuffer::<u32>::default();

    let desc = response.build();

    assert_ne!(
        unsafe { desc.u.BufferType.Length },
        0
    );
}

#[test]
fn request_with_payload_builds_non_null_buffer() {
    let request = IoCtlRequest::new(0x1234, 42u32);
    let desc = request.build().unwrap();

    assert!(!unsafe { desc.u.BufferType.Buffer }.is_null());
}

#[test]
fn response_build_produces_non_null_buffer() {
    let response = IoBuffer::<u32>::default();
    let desc = response.build();

    assert!(!unsafe { desc.u.BufferType.Buffer }.is_null());
}

#[test]
fn response_build_mut_produces_non_null_buffer() {
    let mut response = IoBuffer::<u32>::default();
    let desc = response.build_mut();

    assert!(!unsafe { desc.u.BufferType.Buffer }.is_null());
}