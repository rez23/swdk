#[cfg(not(feature = "wdk-runtime"))]
mod __runtime {
    #[cfg(feature = "test-runtime")]
    #[allow(
        non_camel_case_types,
        non_snake_case,
        non_upper_case_globals,
        reason = "This crate emulates Windows kernel runtime"
    )]
    pub mod test_runtime {
        pub mod wdk_sys {
            use core::ffi::c_void;

            //
            // Base types
            //

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
            pub struct _DRIVER_OBJECT {
                _priv: [u8; 0],
            }

            #[repr(C)]
            pub struct _DEVICE_OBJECT {
                _priv: [u8; 0],
            }

            #[repr(C)]
            pub struct _IRP {
                _priv: [u8; 0],
            }

            pub type PDRIVER_OBJECT = *mut _DRIVER_OBJECT;
            pub type PDEVICE_OBJECT = *mut _DEVICE_OBJECT;
            pub type PIRP = *mut _IRP;

            //
            // DEVICE_INIT
            //

            #[repr(C)]
            pub struct DEVICE_INIT {
                _priv: [u8; 0],
            }

            pub type PWDFDEVICE_INIT = *mut DEVICE_INIT;

            //
            // WDF Handles
            //

            #[repr(C)]
            pub struct _WDFOBJECT {
                _priv: [u8; 0],
            }

            #[repr(C)]
            pub struct _WDFDEVICE {
                _priv: [u8; 0],
            }

            #[repr(C)]
            pub struct _WDFDRIVER {
                _priv: [u8; 0],
            }

            #[repr(C)]
            pub struct _WDFMEMORY {
                _priv: [u8; 0],
            }

            #[repr(C)]
            pub struct _WDFREQUEST {
                _priv: [u8; 0],
            }

            #[repr(C)]
            pub struct _WDFQUEUE {
                _priv: [u8; 0],
            }

            #[repr(C)]
            pub struct _WDFIOTARGET {
                _priv: [u8; 0],
            }

            #[repr(C)]
            pub struct _WDFCOLLECTION {
                _priv: [u8; 0],
            }

            #[repr(C)]
            pub struct _WDFSTRING {
                _priv: [u8; 0],
            }

            pub type WDFOBJECT = *mut _WDFOBJECT;
            pub type WDFDEVICE = *mut _WDFDEVICE;
            pub type WDFDRIVER = *mut _WDFDRIVER;
            pub type WDFMEMORY = *mut _WDFMEMORY;
            pub type WDFREQUEST = *mut _WDFREQUEST;
            pub type WDFQUEUE = *mut _WDFQUEUE;
            pub type WDFIOTARGET = *mut _WDFIOTARGET;
            pub type WDFCOLLECTION = *mut _WDFCOLLECTION;
            pub type WDFSTRING = *mut _WDFSTRING;

            pub const WDF_NO_HANDLE: HANDLE = core::ptr::null_mut();

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

            pub type PCUNICODE_STRING = *const UNICODE_STRING;

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

            pub type PFN_WDF_DRIVER_DEVICE_ADD =
            Option<
                unsafe extern "C" fn(
                    WDFDRIVER,
                    PWDFDEVICE_INIT,
                ) -> NTSTATUS,
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

                pub ExecutionLevel:
                    WDF_EXECUTION_LEVEL,

                pub SynchronizationScope:
                    WDF_SYNCHRONIZATION_SCOPE,

                pub ParentObject:
                    WDFOBJECT,

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

                pub EvtDriverUnload:
                    PFN_WDF_DRIVER_UNLOAD,

                pub DriverInitFlags:
                    ULONG,

                pub DriverPoolTag:
                    ULONG,
            }

            //
            // IO Target State
            //

            pub type WDF_IO_TARGET_STATE = u32;

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
            pub struct _WDF_MEMORY_DESCRIPTOR__bindgen_ty_1__bindgen_ty_1 {
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
                use super::WDF_IO_TARGET_STATE;

                pub const WdfIoTargetStateUndefined: WDF_IO_TARGET_STATE = 0;
                pub const WdfIoTargetStarted: WDF_IO_TARGET_STATE = 1;
                pub const WdfIoTargetStopped: WDF_IO_TARGET_STATE = 2;
                pub const WdfIoTargetClosedForQueryRemove: WDF_IO_TARGET_STATE = 3;
                pub const WdfIoTargetClosed: WDF_IO_TARGET_STATE = 4;
                pub const WdfIoTargetDeleted: WDF_IO_TARGET_STATE = 5;
                pub const WdfIoTargetPurged: WDF_IO_TARGET_STATE = 6;
            }
        }

    }
}

pub mod generators;
#[cfg(not(feature = "test-runtime"))]
pub mod logging;
pub mod utils;

#[cfg(feature = "test-runtime")]
pub use __rt::test_runtime;
