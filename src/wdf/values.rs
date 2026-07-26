mod _values {
    use alloc::vec::Vec;

    use crate::ioctl::commands::IoCtlCommand;
    use crate::rt::wdk_sys::{
        NTSTATUS, WDF_IO_TARGET_STATE,
    };

    #[derive(Debug)]
    pub struct IoCtlTargetSendInfo {
        pub command: IoCtlCommand,
        pub ntstatus: NTSTATUS,
        pub request: Vec<u8>,
        pub byte_returned: usize,
    }
    #[derive(Clone)]
    pub enum WdfSyncScope {
        Invalid = 0,
        Inherit = 1,
        Device = 2,
        Queue = 3,
        None = 4,
    }
    #[derive(Clone)]
    pub enum WdfExecutionLevel {
        Invalid = 0,
        Inherit = 1,
        Passive = 2,
        Dispatch = 3,
    }
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum WdfIoTargetState {
        Started,
        Stopped,
        Closed,
        Deleted,
        Purged,
        ClosedForQueryRemove,
        Unknown(WDF_IO_TARGET_STATE),
    }
    #[derive(Debug)]
    #[allow(dead_code)]
    pub enum WdfIoTargetError {
        DeviceHasNoIoTarget,
        IllegalState(WdfIoTargetState),
        IoCtlTargetSendError(IoCtlTargetSendInfo),
    }

    mod _impls {
        use crate::rt::wdk_sys::_WDF_IO_TARGET_STATE::{
            WdfIoTargetClosed,
            WdfIoTargetClosedForQueryRemove,
            WdfIoTargetDeleted, WdfIoTargetPurged,
            WdfIoTargetStarted, WdfIoTargetStopped,
        };
        use crate::rt::wdk_sys::WDF_IO_TARGET_STATE;
        use crate::vals::_values::WdfIoTargetState;

        impl From<WdfIoTargetState> for WDF_IO_TARGET_STATE {
            fn from(
                state: WdfIoTargetState,
            ) -> WDF_IO_TARGET_STATE {
                match state {
                    WdfIoTargetState::Started => WdfIoTargetStarted,
                    WdfIoTargetState::Stopped => WdfIoTargetStopped,
                    WdfIoTargetState::Closed => WdfIoTargetClosed,
                    WdfIoTargetState::Deleted => WdfIoTargetDeleted,
                    WdfIoTargetState::Purged => WdfIoTargetPurged,
                    WdfIoTargetState::ClosedForQueryRemove => {
                        WdfIoTargetClosedForQueryRemove
                    }
                    WdfIoTargetState::Unknown(value) => value,
                }
            }
        }
        impl From<WDF_IO_TARGET_STATE> for WdfIoTargetState {
            #[allow(non_upper_case_globals)]
            fn from(value: WDF_IO_TARGET_STATE) -> Self {
                match value {
                    WdfIoTargetStarted => Self::Started,
                    WdfIoTargetStopped => Self::Stopped,
                    WdfIoTargetClosed => Self::Closed,
                    WdfIoTargetDeleted => Self::Deleted,
                    WdfIoTargetPurged => Self::Purged,
                    WdfIoTargetClosedForQueryRemove => {
                        Self::ClosedForQueryRemove
                    }
                    _ => Self::Unknown(value),
                }
            }
        }
    }
}

pub use _values::*;
