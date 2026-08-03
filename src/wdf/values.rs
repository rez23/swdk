mod _values {
    use core::ffi::c_int;

    use num_enum::{IntoPrimitive, TryFromPrimitive};
    use crate::bd::IoCtlCommand;
    use crate::NtStatus;
    use crate::op::IsWdfType;
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
        pub ntstatus: NtStatus,
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
    pub enum WdfRequestType {
        Create = 0,
        CreateNamedPipe = 1,
        Close = 2,
        Read = 3,
        Write = 4,
        QueryInformation = 5,
        SetInformation = 6,
        QueryEA = 7,
        SetEA = 8,
        FlushBuffers = 9,
        QueryVolumeInformation = 10,
        SetVolumeInformation = 11,
        DirectoryControl = 12,
        FileSystemControl = 13,
        DeviceControl = 14,
        DeviceControlInternal = 15,
        Shutdown = 16,
        LockControl = 17,
        Cleanup = 18,
        CreateMailSlot = 19,
        QuerySecurity = 20,
        SetSecurity = 21,
        Power = 22,
        SystemControl = 23,
        DeviceChange = 24,
        QueryQuota = 25,
        SetQuota = 26,
        Pnp = 27,
        Other = 28,
        Usb = 64,
        NoFormat = 255,
        Max = 256,
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
    pub enum WdfRequestReuseFlags {
        NoFlags = 0,
        SetNewIrp = 1,
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
    pub enum WdfRequestStopActionFlags {
        Invalid = 0,
        Suspend = 1,
        Purge = 2,
        RequestCancelable = 268435456,
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
    pub enum WdfRequestSendOptionsFlags {
        Timeout = 1,
        Synchronous = 2,
        IgnoreTargetState = 4,
        SendAndForget = 8,
        ImpersonateClient = 65536,
        ImpersonationIgnoreFailure = 131072,
    }

    pub enum DeviceTargetBuffer {
        Input,
        Output,
    }
    pub type NtResult<T = ()> = Result<T, NtStatus>;
}

pub use _values::*;
