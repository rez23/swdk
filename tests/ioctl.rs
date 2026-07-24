use swdk::ioctl::{
    IoCtlRequest,
    IoCtlResponse,
};
use swdk::op::AsBuilder;

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
        IoCtlResponse::<u32>::default();

    assert_eq!(*response, 0);
}

#[test]
fn response_into_inner_returns_inner() {
    let response =
        IoCtlResponse::<u32>::new(77);

    assert_eq!(response.into_inner(), 77);
}