mod __runtime {
    use wdk_sys::NTSTATUS;

    #[cfg(feature = "test-runtime")]
    #[allow(
        non_camel_case_types,
        non_snake_case,
        non_upper_case_globals,
        reason = "This crate emulates Windows kernel runtime"
    )]
    pub mod test {
        pub use crate::runtime::utils;

        pub mod wdk_sys {
            use core::ffi::c_void;

            pub type LONGLONG = i64;
            #[repr(C)]
            #[derive(Debug, Default, Copy, Clone)]
            pub struct WDF_REQUEST_SEND_OPTIONS {
                pub Size: crate::rt::wdk_sys::ULONG,
                pub Flags: crate::rt::wdk_sys::ULONG,
                pub Timeout: crate::rt::wdk_sys::LONGLONG,
            }
            #[repr(C)]
            #[derive(Default, Copy, Clone)]
            pub struct WDF_PNPPOWER_EVENT_CALLBACKS {
                pub Size: ULONG,

                pub EvtDevicePrepareHardware:
                    PFN_WDF_DEVICE_PREPARE_HARDWARE,

                pub EvtDeviceReleaseHardware:
                    PFN_WDF_DEVICE_RELEASE_HARDWARE,

                pub EvtDeviceD0Entry:
                    PFN_WDF_DEVICE_D0_ENTRY,

                pub EvtDeviceD0EntryPostInterruptsEnabled:
                    PFN_WDF_DEVICE_D0_ENTRY_POST_INTERRUPTS_ENABLED,

                pub EvtDeviceD0Exit:
                    PFN_WDF_DEVICE_D0_EXIT,

                pub EvtDeviceD0ExitPreInterruptsDisabled:
                    PFN_WDF_DEVICE_D0_EXIT_PRE_INTERRUPTS_DISABLED,

                pub EvtDeviceSelfManagedIoInit:
                    PFN_WDF_DEVICE_SELF_MANAGED_IO_INIT,

                pub EvtDeviceSelfManagedIoSuspend:
                    PFN_WDF_DEVICE_SELF_MANAGED_IO_SUSPEND,

                pub EvtDeviceSelfManagedIoRestart:
                    PFN_WDF_DEVICE_SELF_MANAGED_IO_RESTART,

                pub EvtDeviceSelfManagedIoFlush:
                    PFN_WDF_DEVICE_SELF_MANAGED_IO_FLUSH,

                pub EvtDeviceSelfManagedIoCleanup:
                    PFN_WDF_DEVICE_SELF_MANAGED_IO_CLEANUP,

                pub EvtDeviceSurpriseRemoval:
                    PFN_WDF_DEVICE_SURPRISE_REMOVAL,

                pub EvtDeviceQueryRemove:
                    PFN_WDF_DEVICE_QUERY_REMOVE,

                pub EvtDeviceQueryStop:
                    PFN_WDF_DEVICE_QUERY_STOP,

                pub EvtDeviceUsageNotification:
                    PFN_WDF_DEVICE_USAGE_NOTIFICATION,

                pub EvtDeviceRelationsQuery:
                    PFN_WDF_DEVICE_RELATIONS_QUERY,

                pub EvtDeviceUsageNotificationEx:
                    PFN_WDF_DEVICE_USAGE_NOTIFICATION_EX,
            }
            //
            // I/O queue
            //

            pub type WDF_IO_QUEUE_DISPATCH_TYPE =
                core::ffi::c_int;
            pub type WDF_TRI_STATE = core::ffi::c_int;

            pub mod _WDF_IO_QUEUE_DISPATCH_TYPE {
                pub type Type = core::ffi::c_int;

                pub const WdfIoQueueDispatchInvalid: Type =
                    0;
                pub const WdfIoQueueDispatchSequential:
                    Type = 1;
                pub const WdfIoQueueDispatchParallel: Type =
                    2;
                pub const WdfIoQueueDispatchManual: Type =
                    3;
            }

            pub mod _WDF_TRI_STATE {
                pub type Type = core::ffi::c_int;

                pub const WdfFalse: Type = 0;
                pub const WdfTrue: Type = 1;
                pub const WdfUseDefault: Type = 2;
            }

            pub type PFN_WDF_IO_QUEUE_IO_DEFAULT = Option<
                unsafe extern "C" fn(WDFQUEUE, WDFREQUEST),
            >;

            pub type PFN_WDF_IO_QUEUE_IO_READ = Option<
                unsafe extern "C" fn(
                    WDFQUEUE,
                    WDFREQUEST,
                    usize,
                ),
            >;

            pub type PFN_WDF_IO_QUEUE_IO_WRITE = Option<
                unsafe extern "C" fn(
                    WDFQUEUE,
                    WDFREQUEST,
                    usize,
                ),
            >;

            pub type PFN_WDF_IO_QUEUE_IO_DEVICE_CONTROL =
                Option<
                    unsafe extern "C" fn(
                        WDFQUEUE,
                        WDFREQUEST,
                        usize,
                        usize,
                        ULONG,
                    ),
                >;

            pub type PFN_WDF_IO_QUEUE_IO_INTERNAL_DEVICE_CONTROL =
                Option<
                    unsafe extern "C" fn(
                        WDFQUEUE,
                        WDFREQUEST,
                        usize,
                        usize,
                        ULONG,
                    ),
                >;

            #[repr(C)]
            #[derive(Copy, Clone)]
            pub struct WDF_IO_QUEUE_CONFIG {
                pub Size: ULONG,

                pub DispatchType: WDF_IO_QUEUE_DISPATCH_TYPE,

                pub PowerManaged: WDF_TRI_STATE,

                pub AllowZeroLengthRequests: BOOLEAN,

                pub DefaultQueue: BOOLEAN,

                pub EvtIoDefault:
                    PFN_WDF_IO_QUEUE_IO_DEFAULT,

                pub EvtIoRead:
                    PFN_WDF_IO_QUEUE_IO_READ,

                pub EvtIoWrite:
                    PFN_WDF_IO_QUEUE_IO_WRITE,

                pub EvtIoDeviceControl:
                    PFN_WDF_IO_QUEUE_IO_DEVICE_CONTROL,

                pub EvtIoInternalDeviceControl:
                    PFN_WDF_IO_QUEUE_IO_INTERNAL_DEVICE_CONTROL,

                pub EvtIoStop:
                    PFN_WDF_IO_QUEUE_IO_STOP,

                pub EvtIoResume:
                    PFN_WDF_IO_QUEUE_IO_RESUME,

                pub EvtIoCanceledOnQueue:
                    PFN_WDF_IO_QUEUE_IO_CANCELED_ON_QUEUE,

                pub Settings: _WDF_IO_QUEUE_CONFIG__bindgen_ty_1,

                pub Driver: WDFDRIVER,
            }
            #[repr(C)]
            #[derive(Copy, Clone)]
            pub union _WDF_IO_QUEUE_CONFIG__bindgen_ty_1 {
                pub Parallel: ULONG,
                pub Sequential: ULONG,
                pub Manual: ULONG,
            }

            pub type PFN_WDF_IO_TARGET_QUERY_REMOVE =
                Option<
                    unsafe extern "C" fn(
                        WDFIOTARGET,
                    )
                        -> NTSTATUS,
                >;

            pub type PFN_WDF_IO_TARGET_REMOVE_CANCELED =
                Option<unsafe extern "C" fn(WDFIOTARGET)>;

            pub type PFN_WDF_IO_TARGET_REMOVE_COMPLETE =
                Option<unsafe extern "C" fn(WDFIOTARGET)>;

            impl Default for _WDF_IO_QUEUE_CONFIG__bindgen_ty_1 {
                fn default() -> Self {
                    Self { Parallel: 0 }
                }
            }
            impl Default for WDF_IO_QUEUE_CONFIG {
                fn default() -> Self {
                    Self {
                        Size: 0,

                        DispatchType:
                        _WDF_IO_QUEUE_DISPATCH_TYPE::WdfIoQueueDispatchInvalid,

                        PowerManaged:
                        _WDF_TRI_STATE::WdfUseDefault,

                        AllowZeroLengthRequests: 0,

                        DefaultQueue: 0,

                        EvtIoDefault: None,
                        EvtIoRead: None,
                        EvtIoWrite: None,
                        EvtIoDeviceControl: None,
                        EvtIoInternalDeviceControl: None,
                        EvtIoStop: None,
                        EvtIoResume: None,
                        EvtIoCanceledOnQueue: None,

                        Settings: Default::default(),

                        Driver: core::ptr::null_mut(),
                    }
                }
            }
            pub type PFN_WDF_IO_QUEUE_IO_STOP = Option<
                unsafe extern "C" fn(
                    WDFQUEUE,
                    WDFREQUEST,
                    ULONG,
                ),
            >;

            pub type PFN_WDF_IO_QUEUE_IO_RESUME = Option<
                unsafe extern "C" fn(WDFQUEUE, WDFREQUEST),
            >;

            pub type PFN_WDF_IO_QUEUE_IO_CANCELED_ON_QUEUE =
                Option<
                    unsafe extern "C" fn(
                        WDFQUEUE,
                        WDFREQUEST,
                    ),
                >;

            //
            // Common scalar / pointer aliases
            //

            pub type BOOLEAN = u8;
            pub type ACCESS_MASK = u32;
            pub type PLONGLONG = *mut i64;

            #[repr(C)]
            #[derive(Default, Copy, Clone)]
            pub struct _FILE_OBJECT {
                _priv: [u8; 0],
            }

            #[repr(C)]
            #[derive(Default, Copy, Clone)]
            pub struct _WDFDEVICE_INIT {
                _priv: [u8; 0],
            }

            #[repr(C)]
            #[derive(Default, Copy, Clone)]
            pub struct WDFDEVICE_INIT {
                _priv: [u8; 0],
            }

            #[repr(C)]
            #[derive(Default, Copy, Clone)]
            pub struct _DRIVER_OBJECT {
                _priv: [u8; 0],
            }

            pub type DRIVER_OBJECT = *mut _DRIVER_OBJECT;
            //
            // Power / PnP
            //

            pub type WDF_POWER_DEVICE_STATE =
                core::ffi::c_int;

            pub type PFN_WDF_DEVICE_PREPARE_HARDWARE =
                Option<
                    unsafe extern "C" fn(
                        WDFDEVICE,
                        PVOID,
                        PVOID,
                    )
                        -> NTSTATUS,
                >;

            pub type PFN_WDF_DEVICE_RELEASE_HARDWARE =
                Option<
                    unsafe extern "C" fn(
                        WDFDEVICE,
                        PVOID,
                    )
                        -> NTSTATUS,
                >;

            pub type PFN_WDF_DEVICE_D0_ENTRY = Option<
                unsafe extern "C" fn(
                    WDFDEVICE,
                    WDF_POWER_DEVICE_STATE,
                )
                    -> NTSTATUS,
            >;

            pub type PFN_WDF_DEVICE_D0_ENTRY_POST_INTERRUPTS_ENABLED =
                Option<
                    unsafe extern "C" fn(
                        WDFDEVICE,
                        WDF_POWER_DEVICE_STATE,
                    )
                        -> NTSTATUS,
                >;

            pub type PFN_WDF_DEVICE_D0_EXIT = Option<
                unsafe extern "C" fn(
                    WDFDEVICE,
                    WDF_POWER_DEVICE_STATE,
                )
                    -> NTSTATUS,
            >;

            pub type PFN_WDF_DEVICE_D0_EXIT_PRE_INTERRUPTS_DISABLED =
                Option<
                    unsafe extern "C" fn(
                        WDFDEVICE,
                        WDF_POWER_DEVICE_STATE,
                    )
                        -> NTSTATUS,
                >;

            pub type PFN_WDF_DEVICE_SELF_MANAGED_IO_INIT =
                Option<
                    unsafe extern "C" fn(
                        WDFDEVICE,
                    )
                        -> NTSTATUS,
                >;

            pub type PFN_WDF_DEVICE_SELF_MANAGED_IO_FLUSH =
                Option<unsafe extern "C" fn(WDFDEVICE)>;

            pub type PFN_WDF_DEVICE_SELF_MANAGED_IO_CLEANUP =
                Option<unsafe extern "C" fn(WDFDEVICE)>;

            pub type PFN_WDF_DEVICE_SELF_MANAGED_IO_SUSPEND =
                Option<
                    unsafe extern "C" fn(
                        WDFDEVICE,
                    )
                        -> NTSTATUS,
                >;

            pub type PFN_WDF_DEVICE_SELF_MANAGED_IO_RESTART =
                Option<
                    unsafe extern "C" fn(
                        WDFDEVICE,
                    )
                        -> NTSTATUS,
                >;

            pub type PFN_WDF_DEVICE_SURPRISE_REMOVAL =
                Option<unsafe extern "C" fn(WDFDEVICE)>;

            pub type PFN_WDF_DEVICE_QUERY_REMOVE = Option<
                unsafe extern "C" fn(WDFDEVICE) -> NTSTATUS,
            >;

            pub type PFN_WDF_DEVICE_QUERY_STOP = Option<
                unsafe extern "C" fn(WDFDEVICE) -> NTSTATUS,
            >;

            pub type PFN_WDF_DEVICE_USAGE_NOTIFICATION =
                Option<
                    unsafe extern "C" fn(
                        WDFDEVICE,
                        core::ffi::c_int,
                        BOOLEAN,
                    )
                        -> NTSTATUS,
                >;

            pub type PFN_WDF_DEVICE_USAGE_NOTIFICATION_EX =
                Option<
                    unsafe extern "C" fn(
                        WDFDEVICE,
                        core::ffi::c_int,
                        BOOLEAN,
                    )
                        -> NTSTATUS,
                >;

            pub type PFN_WDF_DEVICE_RELATIONS_QUERY =
                Option<
                    unsafe extern "C" fn(
                        WDFDEVICE,
                        core::ffi::c_int,
                    ),
                >;
            pub type PFILE_OBJECT = *mut _FILE_OBJECT;
            #[macro_export]
            macro_rules! call_unsafe_wdf_function_binding {
                ($func:ident $(, $args:expr)* $(,)?) => {{}};
            }
            pub(crate) use call_unsafe_wdf_function_binding;
            //
            // Base types
            //

            pub(crate) const STATUS_INVALID_PARAMETER:
                NTSTATUS = 0xC000000Du32 as i32;
            pub(crate) const STATUS_INTERNAL_ERROR:
                NTSTATUS = 0xC000000Eu32 as i32;
            pub(crate) type PWDF_IO_TARGET_OPEN_PARAMS =
                *mut c_void;
            pub type HANDLE = *mut c_void;
            pub type PVOID = *mut c_void;

            pub type NTSTATUS = i32;
            pub type ULONG = u32;
            pub type ULONG_PTR = usize;

            pub const STATUS_SUCCESS: NTSTATUS = 0;
            pub const STATUS_UNSUCCESSFUL: NTSTATUS = -1;

            //
            // Opaque kernel objects
            //

            #[repr(C)]
            #[derive(Default, Copy, Clone)]
            pub struct DRIVER_OBJECT__ {
                _priv: [u8; 0],
            }

            #[repr(C)]
            #[derive(Default, Copy, Clone)]
            pub struct DEVICE_OBJECT__ {
                _priv: [u8; 0],
            }

            #[repr(C)]
            #[derive(Default, Copy, Clone)]
            pub struct IRP__ {
                _priv: [u8; 0],
            }

            pub type PDRIVER_OBJECT = *mut DRIVER_OBJECT__;
            pub type PDEVICE_OBJECT = *mut DEVICE_OBJECT__;
            pub type PIRP = *mut IRP__;

            //
            // DEVICE_INIT
            //

            #[repr(C)]
            #[derive(Default, Copy, Clone)]
            pub struct DEVICE_INIT {
                _priv: [u8; 0],
            }

            pub type PWDFDEVICE_INIT = *mut DEVICE_INIT;

            //
            // WDF Handles
            //

            #[repr(C)]
            #[derive(Default, Copy, Clone)]
            pub struct WDFOBJECT__ {
                _priv: [u8; 0],
            }

            #[repr(C)]
            #[derive(Default, Copy, Clone)]
            pub struct WDFDEVICE__ {
                _priv: [u8; 0],
            }

            #[repr(C)]
            #[derive(Default, Copy, Clone)]
            pub struct WDFDRIVER__ {
                _priv: [u8; 0],
            }

            #[repr(C)]
            #[derive(Default, Copy, Clone)]
            pub struct WDFMEMORY__ {
                _priv: [u8; 0],
            }

            #[repr(C)]
            #[derive(Default, Copy, Clone)]
            pub struct WDFREQUEST__ {
                _priv: [u8; 0],
            }

            #[repr(C)]
            #[derive(Default, Copy, Clone)]
            pub struct WDFQUEUE__ {
                _priv: [u8; 0],
            }

            #[repr(C)]
            #[derive(Default, Copy, Clone)]
            pub struct WDFIOTARGET__ {
                _priv: [u8; 0],
            }

            #[repr(C)]
            #[derive(Default, Copy, Clone)]
            pub struct WDFCOLLECTION__ {
                _priv: [u8; 0],
            }

            #[repr(C)]
            #[derive(Default, Copy, Clone)]
            pub struct WDFSTRING__ {
                _priv: [u8; 0],
            }

            pub type WDFOBJECT = *mut WDFOBJECT__;
            pub type WDFDEVICE = *mut WDFDEVICE__;
            pub type WDFDRIVER = *mut WDFDRIVER__;
            pub type WDFMEMORY = *mut WDFMEMORY__;
            pub type WDFREQUEST = *mut WDFREQUEST__;
            pub type WDFQUEUE = *mut WDFQUEUE__;
            pub type WDFIOTARGET = *mut WDFIOTARGET__;
            pub type WDFCOLLECTION = *mut WDFCOLLECTION__;
            pub type WDFSTRING = *mut WDFSTRING__;

            pub const WDF_NO_HANDLE: HANDLE =
                core::ptr::null_mut();

            //
            // Unicode string
            //

            #[repr(C)]
            #[derive(Default, Copy, Clone)]
            pub struct UNICODE_STRING {
                pub Length: u16,
                pub MaximumLength: u16,
                pub Buffer: *mut u16,
            }

            pub type PCUNICODE_STRING =
                *const UNICODE_STRING;

            //
            // Context descriptors
            //

            #[repr(C)]
            #[derive(Copy, Clone)]
            pub struct WDF_OBJECT_CONTEXT_TYPE_INFO {
                pub Size: ULONG,

                pub ContextSize: usize,

                pub ContextName: *const c_void,

                pub UniqueType: PCWDF_OBJECT_CONTEXT_TYPE_INFO,

                pub EvtDriverGetUniqueContextType:
                    Option<unsafe extern "C" fn() -> PCWDF_OBJECT_CONTEXT_TYPE_INFO>,
            }

            pub type PWDF_OBJECT_CONTEXT_TYPE_INFO =
                *mut WDF_OBJECT_CONTEXT_TYPE_INFO;

            pub type PCWDF_OBJECT_CONTEXT_TYPE_INFO =
                *const WDF_OBJECT_CONTEXT_TYPE_INFO;

            //
            // Execution/synchronization enums
            //

            pub type WDF_EXECUTION_LEVEL = u32;
            pub type WDF_SYNCHRONIZATION_SCOPE = u32;

            //
            // Object callbacks
            //

            pub type PFN_WDF_OBJECT_CONTEXT_CLEANUP =
                Option<unsafe extern "C" fn(WDFOBJECT)>;

            pub type PFN_WDF_OBJECT_CONTEXT_DESTROY =
                Option<unsafe extern "C" fn(WDFOBJECT)>;

            //
            // Driver callbacks
            //

            pub type PFN_WDF_DRIVER_DEVICE_ADD = Option<
                unsafe extern "C" fn(
                    WDFDRIVER,
                    PWDFDEVICE_INIT,
                )
                    -> NTSTATUS,
            >;

            pub type PFN_WDF_DRIVER_UNLOAD =
                Option<unsafe extern "C" fn(WDFDRIVER)>;

            //
            // Object attributes
            //

            #[repr(C)]
            #[derive(Default, Copy, Clone)]
            pub struct WDF_OBJECT_ATTRIBUTES {
                pub Size: ULONG,

                pub EvtCleanupCallback:
                    PFN_WDF_OBJECT_CONTEXT_CLEANUP,

                pub EvtDestroyCallback:
                    PFN_WDF_OBJECT_CONTEXT_DESTROY,

                pub ExecutionLevel: WDF_EXECUTION_LEVEL,

                pub SynchronizationScope:
                    WDF_SYNCHRONIZATION_SCOPE,

                pub ParentObject: WDFOBJECT,

                pub ContextTypeInfo:
                    PCWDF_OBJECT_CONTEXT_TYPE_INFO,
            }

            //
            // Driver config
            //

            #[repr(C)]
            #[derive(Default, Copy, Clone)]
            pub struct WDF_DRIVER_CONFIG {
                pub Size: ULONG,

                pub EvtDriverDeviceAdd:
                    PFN_WDF_DRIVER_DEVICE_ADD,

                pub EvtDriverUnload: PFN_WDF_DRIVER_UNLOAD,

                pub DriverInitFlags: ULONG,

                pub DriverPoolTag: ULONG,
            }

            //
            // IO Target State
            //

            pub type WDF_IO_TARGET_STATE = core::ffi::c_int;

            //
            // WDF_MEMORY_DESCRIPTOR_TYPE
            //

            #[repr(u32)]
            #[derive(Copy, Clone)]
            pub enum _WDF_MEMORY_DESCRIPTOR_TYPE {
                WdfMemoryDescriptorTypeInvalid = 0,
                WdfMemoryDescriptorTypeBuffer = 1,
                WdfMemoryDescriptorTypeMdl = 2,
                WdfMemoryDescriptorTypeHandle = 3,
            }

            //
            // Buffer descriptor
            //

            #[repr(C)]
            #[derive(Copy, Clone)]
            pub struct _WDF_MEMORY_DESCRIPTOR__bindgen_ty_1__bindgen_ty_1
            {
                pub Buffer: *mut c_void,
                pub Length: ULONG,
            }

            //
            // Union wrapper
            //

            #[repr(C)]
            #[derive(Copy, Clone)]
            pub union _WDF_MEMORY_DESCRIPTOR__bindgen_ty_1 {
                pub BufferType:
                    _WDF_MEMORY_DESCRIPTOR__bindgen_ty_1__bindgen_ty_1,
            }

            impl Default for _WDF_MEMORY_DESCRIPTOR__bindgen_ty_1 {
                fn default() -> Self {
                    Self {
                        BufferType:
                        _WDF_MEMORY_DESCRIPTOR__bindgen_ty_1__bindgen_ty_1 {
                            Buffer: core::ptr::null_mut(),
                            Length: 0,
                        },
                    }
                }
            }

            //
            // WDF_MEMORY_DESCRIPTOR
            //

            #[repr(C)]
            #[derive(Copy, Clone)]
            pub struct WDF_MEMORY_DESCRIPTOR {
                pub Type: _WDF_MEMORY_DESCRIPTOR_TYPE,
                pub u: _WDF_MEMORY_DESCRIPTOR__bindgen_ty_1,
            }

            impl Default for WDF_MEMORY_DESCRIPTOR {
                fn default() -> Self {
                    Self {
                        Type:
                        _WDF_MEMORY_DESCRIPTOR_TYPE::WdfMemoryDescriptorTypeInvalid,
                        u: Default::default(),
                    }
                }
            }

            pub mod _WDF_IO_TARGET_STATE {
                pub type Type = core::ffi::c_int;
                pub const WdfIoTargetStateUndefined: Type =
                    0;
                pub const WdfIoTargetStarted: Type = 1;
                pub const WdfIoTargetStopped: Type = 2;
                pub const WdfIoTargetClosedForQueryRemove: Type = 3;
                pub const WdfIoTargetClosed: Type = 4;
                pub const WdfIoTargetDeleted: Type = 5;
                pub const WdfIoTargetPurged: Type = 6;
            }
            pub type PWDF_IO_TARGET_STATE =
                *mut _WDF_IO_TARGET_STATE::Type;
            pub mod _WDF_IO_TARGET_OPEN_TYPE {
                pub type Type = core::ffi::c_int;

                pub const WdfIoTargetOpenUndefined: Type =
                    0;
                pub const WdfIoTargetOpenUseExistingDevice: Type = 1;
                pub const WdfIoTargetOpenByName: Type = 2;
                pub const WdfIoTargetOpenReopen: Type = 3;
                pub const WdfIoTargetOpenLocalTargetByFile: Type = 4;
            }
            pub use self::_WDF_IO_TARGET_OPEN_TYPE::Type as WDF_IO_TARGET_OPEN_TYPE;
            pub mod _WDF_IO_TARGET_SENT_IO_ACTION {
                pub type Type = ::core::ffi::c_int;
                pub const WdfIoTargetSentIoUndefined: Type =
                    0;
                pub const WdfIoTargetCancelSentIo: Type = 1;
                pub const WdfIoTargetWaitForSentIoToComplete: Type = 2;
                pub const WdfIoTargetLeaveSentIoPending:
                    Type = 3;
            }
            pub use self::_WDF_IO_TARGET_SENT_IO_ACTION::Type as WDF_IO_TARGET_SENT_IO_ACTION;
            pub mod _WDF_IO_TARGET_PURGE_IO_ACTION {
                pub type Type = ::core::ffi::c_int;
                pub const WdfIoTargetPurgeIoUndefined:
                    Type = 0;
                pub const WdfIoTargetPurgeIoAndWait: Type =
                    1;
                pub const WdfIoTargetPurgeIo: Type = 2;
            }
        }
    }
    
    #[cfg(feature = "kmdf-runtime")]
    #[expect(
        clippy::missing_safety_doc,
        reason = "This is a binding to the Windows Driver Framework (WDF).\
                  It is only for internal crate use."
    )]
    pub mod kmdf {
        use core::ffi::c_void;

        use wdk_sys::{BOOLEAN, NTSTATUS, PCUNICODE_STRING, PCWDF_OBJECT_CONTEXT_TYPE_INFO, PDRIVER_OBJECT, PULONG_PTR, PWDF_FILEOBJECT_CONFIG, PWDF_IO_QUEUE_CONFIG, PWDF_IO_TARGET_OPEN_PARAMS, PWDF_MEMORY_DESCRIPTOR, PWDF_OBJECT_ATTRIBUTES, PWDF_PNPPOWER_EVENT_CALLBACKS, PWDF_REQUEST_SEND_OPTIONS, PWDFDEVICE_INIT, WDF_DRIVER_CONFIG, WDF_IO_TARGET_STATE, WDF_OBJECT_ATTRIBUTES, WDF_REQUEST_SEND_OPTIONS, WDFDEVICE, WDFDEVICE_INIT, WDFDRIVER, WDFIOTARGET, WDFOBJECT, WDFQUEUE, WDFREQUEST, call_unsafe_wdf_function_binding, WDFREQUEST__, ULONG_PTR};

        use crate::bd::WdfFileObjectConfig;
        use crate::call_ntstatus_wdf_unsafe_binding;
        use crate::ioctl::commands::IoCtlCommand;
        use crate::vals::NtResult;

        #[inline]
        pub unsafe fn wdf_driver_create(
            p_dr_obj: PDRIVER_OBJECT,
            registry_path: PCUNICODE_STRING,
            p_attrs: *mut WDF_OBJECT_ATTRIBUTES,
            p_config: *mut WDF_DRIVER_CONFIG,
            p_driver: *mut WDFDRIVER,
        ) -> NtResult {
            call_ntstatus_wdf_unsafe_binding!(
                WdfDriverCreate,
                p_dr_obj,
                registry_path,
                p_attrs,
                p_config,
                p_driver
            )
        }
        #[inline]
        pub unsafe fn wdf_device_create(
            p_dev_init: *mut PWDFDEVICE_INIT,
            p_attrs: *mut WDF_OBJECT_ATTRIBUTES,
            p_device: *mut WDFDEVICE,
        ) -> NtResult {
            call_ntstatus_wdf_unsafe_binding!(
                WdfDeviceCreate,
                p_dev_init,
                p_attrs,
                p_device
            )
        }

        #[inline]
        pub unsafe fn wdf_target_io_get(
            device: WDFDEVICE,
        ) -> WDFIOTARGET {
            call_unsafe_wdf_function_binding!(
                WdfDeviceGetIoTarget,
                device,
            )
        }

        #[inline]
        pub unsafe fn wdf_target_io_get_state(
            io_target: WDFIOTARGET,
        ) -> WDF_IO_TARGET_STATE {
            call_unsafe_wdf_function_binding!(
                WdfIoTargetGetState,
                io_target,
            )
        }

        #[inline]
        pub unsafe fn wdf_target_io_send_ioctl_sync(
            io_target: WDFIOTARGET,
            io_ctl_command: IoCtlCommand,
            wdf_request: WDFREQUEST,
            p_request_desc: PWDF_MEMORY_DESCRIPTOR,
            p_response_desc: PWDF_MEMORY_DESCRIPTOR,
            send_options: PWDF_REQUEST_SEND_OPTIONS,
            p_bytes_returned: PULONG_PTR,
        ) -> NtResult {
            // Retrieve the general collection info (including the required preparsed descriptor size)
            call_ntstatus_wdf_unsafe_binding!(
                WdfIoTargetSendIoctlSynchronously,
                io_target,
                wdf_request,
                io_ctl_command,
                p_request_desc,
                p_response_desc,
                send_options,
                p_bytes_returned
            )
        }

        #[inline]
        pub unsafe fn wdf_object_create(
            p_attrs: PWDF_OBJECT_ATTRIBUTES,
            p_obj: *mut WDFOBJECT,
        ) -> NtResult {
            call_ntstatus_wdf_unsafe_binding!(
                WdfObjectCreate,
                p_attrs,
                p_obj,
            )
        }

        #[inline]
        pub unsafe fn wdf_object_typed_ctx_worker(
            wdf_obj: WDFOBJECT,
            p_type_info: PCWDF_OBJECT_CONTEXT_TYPE_INFO,
        ) -> *mut c_void {
            call_unsafe_wdf_function_binding!(
                WdfObjectGetTypedContextWorker,
                wdf_obj,
                p_type_info,
            )
        }

        #[inline]
        pub unsafe fn wdf_target_io_create(
            device: WDFDEVICE,
            attrs: PWDF_OBJECT_ATTRIBUTES,
            io_target: *mut WDFIOTARGET,
        ) -> NtResult {
            call_ntstatus_wdf_unsafe_binding!(
                WdfIoTargetCreate,
                device,
                attrs,
                io_target,
            )
        }

        #[inline]
        pub unsafe fn wdf_target_io_open(
            io_target: WDFIOTARGET,
            open_params: PWDF_IO_TARGET_OPEN_PARAMS,
        ) -> NtResult {
            call_ntstatus_wdf_unsafe_binding!(
                WdfIoTargetOpen,
                io_target,
                open_params,
            )
        }

        #[inline]
        pub unsafe fn wdf_f_do_init_set_filter(
            device_init: PWDFDEVICE_INIT,
        ) {
            call_unsafe_wdf_function_binding!(
                WdfFdoInitSetFilter,
                device_init,
            );
        }

        #[inline]
        pub unsafe fn wdf_device_init_set_pnp_power_event_callbacks(
            device_init: PWDFDEVICE_INIT,
            callbacks: PWDF_PNPPOWER_EVENT_CALLBACKS,
        ) {
            call_unsafe_wdf_function_binding!(
                WdfDeviceInitSetPnpPowerEventCallbacks,
                device_init,
                callbacks,
            );
        }
        #[inline]
        pub unsafe fn wdf_io_queue_get_device(
            queue: WDFQUEUE,
        ) -> WDFDEVICE {
            call_unsafe_wdf_function_binding!(
                WdfIoQueueGetDevice,
                queue,
            )
        }

        #[inline]
        pub unsafe fn wdf_request_complete(
            request: WDFREQUEST,
            status: NTSTATUS,
        ) {
            call_unsafe_wdf_function_binding!(
                WdfRequestComplete,
                request,
                status,
            )
        }

        #[inline]
        pub unsafe fn wdf_io_queue_create(
            device: WDFDEVICE,
            config: PWDF_IO_QUEUE_CONFIG,
            attrs: PWDF_OBJECT_ATTRIBUTES,
            queue: *mut WDFQUEUE,
        ) -> NtResult {
            call_ntstatus_wdf_unsafe_binding!(
                WdfIoQueueCreate,
                device,
                config,
                attrs,
                queue,
            )
        }

        pub unsafe fn wdf_device_get_io_queue(
            queue: WDFQUEUE,
        ) -> WDFDEVICE {
            call_unsafe_wdf_function_binding!(
                WdfIoQueueGetDevice,
                queue,
            )
        }

        #[inline]
        pub unsafe fn wdf_request_send(
            target: WDFIOTARGET,
            options: PWDF_REQUEST_SEND_OPTIONS,
            request: WDFREQUEST,
        ) -> bool {
            call_unsafe_wdf_function_binding!(
                WdfRequestSend,
                request,
                target,
                options
            ) != 0
        }

        #[inline]
        pub unsafe fn wdf_request_get_status(
            request: WDFREQUEST,
        ) -> NtResult {
            call_ntstatus_wdf_unsafe_binding!(
                WdfRequestGetStatus,
                request,
            )
        }

        pub unsafe fn wdf_request_format_using_current_type(
            request: WDFREQUEST,
        ) {
            call_unsafe_wdf_function_binding!(
                WdfRequestFormatRequestUsingCurrentType,
                request,
            )
        }

        #[inline]
        pub unsafe fn wdf_device_init_set_file_object_config(
            p_init: PWDFDEVICE_INIT,
            p_config: PWDF_FILEOBJECT_CONFIG,
            attrs: PWDF_OBJECT_ATTRIBUTES,
        ) {
            call_unsafe_wdf_function_binding!(
                WdfDeviceInitSetFileObjectConfig,
                p_init,
                p_config,
                attrs
            )
        }

        #[inline]
        pub unsafe fn wdf_request_get_input_buffer(
            request: WDFREQUEST,
            size: usize,
            in_buff: PWDF_MEMORY_DESCRIPTOR,
            read_len: *mut usize
        ) -> NtResult {
            call_ntstatus_wdf_unsafe_binding!(
                WdfRequestRetrieveInputBuffer,
                request,
                size,
                in_buff.cast(),
                read_len
            )
        }
    }
}
pub mod utils;

#[cfg(feature = "wdk-default")]
pub mod logging;

#[cfg(feature = "kmdf-runtime")]
pub use __runtime::kmdf;
#[cfg(feature = "test-runtime")]
pub use __runtime::test;
