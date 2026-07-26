use swdk::vals::{
    IoCtlTargetSendInfo,
    WdfIoTargetError,
    WdfIoTargetState,
};

#[test]
fn illegal_state_preserves_state() {
    let err =
        WdfIoTargetError::IllegalState(
            WdfIoTargetState::Closed,
        );

    match err {
        WdfIoTargetError::IllegalState(state) => {
            assert_eq!(
                state,
                WdfIoTargetState::Closed
            );
        }
        _ => panic!("unexpected error variant"),
    }
}

#[test]
fn ioctl_send_error_preserves_command() {
    let err =
        WdfIoTargetError::IoCtlTargetSendError(
            IoCtlTargetSendInfo {
                command: 0x1234,
                ntstatus: -1,
                request: vec![],
                byte_returned: 0,
            },
        );

    match err {
        WdfIoTargetError::IoCtlTargetSendError(info) => {
            assert_eq!(info.command, 0x1234);
        }
        _ => panic!("unexpected error variant"),
    }
}

#[test]
fn ioctl_send_error_preserves_ntstatus() {
    let err =
        WdfIoTargetError::IoCtlTargetSendError(
            IoCtlTargetSendInfo {
                command: 0x1234,
                ntstatus: -42,
                request: vec![],
                byte_returned: 0,
            },
        );

    match err {
        WdfIoTargetError::IoCtlTargetSendError(info) => {
            assert_eq!(info.ntstatus, -42);
        }
        _ => panic!("unexpected error variant"),
    }
}

#[test]
fn ioctl_send_error_preserves_request_buffer() {
    let err =
        WdfIoTargetError::IoCtlTargetSendError(
            IoCtlTargetSendInfo {
                command: 0x1234,
                ntstatus: -1,
                request: vec![1, 2, 3],
                byte_returned: 0,
            },
        );

    match err {
        WdfIoTargetError::IoCtlTargetSendError(info) => {
            assert_eq!(
                info.request,
                vec![1, 2, 3]
            );
        }
        _ => panic!("unexpected error variant"),
    }
}

#[test]
fn ioctl_send_error_preserves_bytes_returned() {
    let err =
        WdfIoTargetError::IoCtlTargetSendError(
            IoCtlTargetSendInfo {
                command: 0x1234,
                ntstatus: -1,
                request: vec![],
                byte_returned: 123,
            },
        );

    match err {
        WdfIoTargetError::IoCtlTargetSendError(info) => {
            assert_eq!(
                info.byte_returned,
                123
            );
        }
        _ => panic!("unexpected error variant"),
    }
}