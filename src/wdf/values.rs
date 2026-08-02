mod _values {
    use core::ffi::c_int;

    use num_enum::{IntoPrimitive, TryFromPrimitive};

    use crate::ioctl::commands::IoCtlCommand;
    use crate::op::marks::IsWdfType;
    use crate::rt::wdk_sys::{
        NTSTATUS, WDF_IO_TARGET_STATE,
    };

    #[repr(i32)]
    #[derive(
        Debug,
        Copy,
        Clone,
        Eq,
        PartialEq,
        IntoPrimitive,
        TryFromPrimitive,
    )]
    pub enum WdfIoQueueDispatchType {
        Invalid = 0,
        Sequential = 1,
        Parallel = 2,
        Manual = 3,
        Max = 4,
    }
    #[repr(i32)]
    #[derive(
        Debug,
        Copy,
        Clone,
        Eq,
        PartialEq,
        IntoPrimitive,
        TryFromPrimitive,
    )]
    pub enum WdfIoQueueState {
        AcceptRequests = 1,
        DispatchRequests = 2,
        NoRequests = 4,
        DriverNoRequests = 8,
        PnpHeld = 16,
    }
    #[repr(i32)]
    #[derive(
        Debug,
        Copy,
        Clone,
        Eq,
        PartialEq,
        IntoPrimitive,
        TryFromPrimitive,
    )]
    pub enum WdfTriState {
        False = 0,
        True = 1,
        Default = 2,
    }

    #[repr(i32)]
    #[derive(
        Debug,
        Copy,
        Clone,
        Eq,
        PartialEq,
        IntoPrimitive,
        TryFromPrimitive,
    )]
    pub enum WdfFileObjClass {
        Invalid = 0,
        NotRequired = 1,
        WdfCanUseFsContext = 2,
        WdfCanUseFsContext2 = 3,
        WdfCannotUseFsContexts = 4,
        CanBeOptional = -2147483648,
    }
    impl Default for WdfTriState {
        fn default() -> Self {
            Self::Default
        }
    }

    #[derive(Debug)]
    pub struct IoCtlTargetSendInfo {
        pub command: IoCtlCommand,
        pub ntstatus: NTSTATUS,
        pub byte_returned: usize,
    }

    #[repr(i32)]
    #[derive(
        Debug,
        Copy,
        Clone,
        Eq,
        PartialEq,
        IntoPrimitive,
        TryFromPrimitive,
    )]
    pub enum WdfSyncScope {
        Invalid = 0,
        Inherit = 1,
        Device = 2,
        Queue = 3,
        None = 4,
    }
    #[repr(i32)]
    #[derive(
        Debug,
        Copy,
        Clone,
        Eq,
        PartialEq,
        IntoPrimitive,
        TryFromPrimitive,
    )]
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
        InvalidIoDescriptor,
    }

    #[derive(Debug)]
    pub enum WdfIoTargetOpenType {
        Undefined,
        UseExistingDevice,
        ByName,
        Reopen,
        LocalTargetByFile,
        Unknown(c_int),
    }

    mod _impls {
        use core::ffi::c_int;

        use crate::rt::wdk_sys::_WDF_IO_TARGET_STATE::{
            WdfIoTargetClosed,
            WdfIoTargetClosedForQueryRemove,
            WdfIoTargetDeleted, WdfIoTargetPurged,
            WdfIoTargetStarted, WdfIoTargetStopped,
        };
        use crate::rt::wdk_sys::{
            _WDF_IO_TARGET_OPEN_TYPE, WDF_IO_TARGET_STATE,
        };
        use crate::vals::_values::WdfIoTargetState;
        use crate::vals::WdfIoTargetOpenType;

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

        impl From<c_int> for WdfIoTargetOpenType {
            fn from(value: c_int) -> Self {
                match value {
                    _WDF_IO_TARGET_OPEN_TYPE::WdfIoTargetOpenUndefined => Self::Undefined,
                    _WDF_IO_TARGET_OPEN_TYPE::WdfIoTargetOpenByName => Self::ByName,
                    _WDF_IO_TARGET_OPEN_TYPE::WdfIoTargetOpenReopen => Self::Reopen,
                    _WDF_IO_TARGET_OPEN_TYPE::WdfIoTargetOpenLocalTargetByFile => Self::LocalTargetByFile,
                    _WDF_IO_TARGET_OPEN_TYPE::WdfIoTargetOpenUseExistingDevice => Self::UseExistingDevice,
                    _ => Self::Unknown(value),
                }
            }
        }
        impl From<WdfIoTargetOpenType> for c_int {
            fn from(value: WdfIoTargetOpenType) -> Self {
                match value {
                    WdfIoTargetOpenType::Undefined => _WDF_IO_TARGET_OPEN_TYPE::WdfIoTargetOpenUndefined,
                    WdfIoTargetOpenType::ByName => _WDF_IO_TARGET_OPEN_TYPE::WdfIoTargetOpenByName,
                    WdfIoTargetOpenType::Reopen => _WDF_IO_TARGET_OPEN_TYPE::WdfIoTargetOpenReopen,
                    WdfIoTargetOpenType::LocalTargetByFile => _WDF_IO_TARGET_OPEN_TYPE::WdfIoTargetOpenLocalTargetByFile,
                    WdfIoTargetOpenType::UseExistingDevice => _WDF_IO_TARGET_OPEN_TYPE::WdfIoTargetOpenUseExistingDevice,
                    WdfIoTargetOpenType::Unknown(val) => val,
                }
            }
        }
    }

    pub type WdfTypeAccessorNotNeeded = ();
    impl IsWdfType for WdfTypeAccessorNotNeeded {}
}

pub use _values::*;
