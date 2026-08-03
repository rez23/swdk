use swdk::descriptors::{
    Describe,
};
use swdk::op::{AsBuilder, AsBuilderMut};
#[test]
fn response_default_allocates_default_value() {
    let response =
        Describe::<u32>::default();

    assert_eq!(*response, 0);
}

#[test]
fn response_into_inner_returns_inner() {
    let response =
        Describe::<u32>::new(77);

    assert_eq!(response.into_inner(), 77);
}

#[test]
fn response_build_does_not_modify_value() {
    let response = Describe::new(77u32);

    let _ = response.build();

    assert_eq!(*response, 77);
}

#[test]
fn response_build_mut_does_not_modify_value() {
    let mut response = Describe::new(77u32);

    let _ = response.build_mut();

    assert_eq!(*response, 77);
}

#[test]
fn response_build_and_build_mut_report_same_size() {
    let mut response =
        Describe::<u32>::default();

    let read_desc = response.build();
    let write_desc = response.build_mut();

    assert_eq!(
        unsafe { read_desc.u.BufferType.Length },
        unsafe { write_desc.u.BufferType.Length }
    );
}

#[test]
fn request_with_payload_builds_non_empty_descriptor() {
    let req = Describe::new(0x1234);

    let desc = req.build();

    assert_ne!(
        unsafe { desc.u.BufferType.Length },
        0
    );
}

#[test]
fn response_build_produces_non_empty_descriptor() {
    let response =
        Describe::<u32>::default();

    let desc = response.build();

    assert_ne!(
        unsafe { desc.u.BufferType.Length },
        0
    );
}

#[test]
fn request_with_payload_builds_non_null_buffer() {
    let request = Describe::new(0x1234);
    let desc = request.build();

    assert!(!unsafe { desc.u.BufferType.Buffer }.is_null());
}

#[test]
fn response_build_produces_non_null_buffer() {
    let response = Describe::<u32>::default();
    let desc = response.build();

    assert!(!unsafe { desc.u.BufferType.Buffer }.is_null());
}

#[test]
fn response_build_mut_produces_non_null_buffer() {
    let mut response = Describe::<u32>::default();
    let desc = response.build_mut();

    assert!(!unsafe { desc.u.BufferType.Buffer }.is_null());
}