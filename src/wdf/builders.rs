//! # TODO
//! Auto-generate buildable WDF wrappers from bindgen metadata.
//! Hand-writing WDF_PNPPOWER_EVENT_CALLBACKS-like structs
//! does not scale.
//! # Notes
//! This is a provvisorial solution for having idiomatic struct pretty formatted without the need of specifying size
//!  Anyway, this is not the best solution. Think to port the huge amount of Buildable struct in WDF is an impossible task.
//! The solution is something like NTSTATUS.fmt_status(), a build script generator that auto creates the builders from the C original struct,
//! So less late or earlier I will implement something like that... but, for now, this is a little version needed for getting work the first examples

use core::ffi::{c_int, c_ulonglong, c_void};
use core::marker::PhantomData;
use core::ptr;
use core::ptr::NonNull;

use wdk_sys::{
    _WDF_IO_QUEUE_CONFIG__bindgen_ty_1, ACCESS_MASK,
    BOOLEAN, LONGLONG, PCUNICODE_STRING,
    PCWDF_OBJECT_CONTEXT_TYPE_INFO, PDEVICE_OBJECT,
    PFILE_OBJECT, PFN_WDF_DEVICE_D0_ENTRY,
    PFN_WDF_DEVICE_D0_ENTRY_POST_INTERRUPTS_ENABLED,
    PFN_WDF_DEVICE_D0_EXIT,
    PFN_WDF_DEVICE_D0_EXIT_PRE_INTERRUPTS_DISABLED,
    PFN_WDF_DEVICE_FILE_CREATE,
    PFN_WDF_DEVICE_PREPARE_HARDWARE,
    PFN_WDF_DEVICE_QUERY_REMOVE, PFN_WDF_DEVICE_QUERY_STOP,
    PFN_WDF_DEVICE_RELATIONS_QUERY,
    PFN_WDF_DEVICE_RELEASE_HARDWARE,
    PFN_WDF_DEVICE_SELF_MANAGED_IO_CLEANUP,
    PFN_WDF_DEVICE_SELF_MANAGED_IO_FLUSH,
    PFN_WDF_DEVICE_SELF_MANAGED_IO_INIT,
    PFN_WDF_DEVICE_SELF_MANAGED_IO_RESTART,
    PFN_WDF_DEVICE_SELF_MANAGED_IO_SUSPEND,
    PFN_WDF_DEVICE_SURPRISE_REMOVAL,
    PFN_WDF_DEVICE_USAGE_NOTIFICATION,
    PFN_WDF_DEVICE_USAGE_NOTIFICATION_EX,
    PFN_WDF_DRIVER_DEVICE_ADD, PFN_WDF_DRIVER_UNLOAD,
    PFN_WDF_FILE_CLEANUP, PFN_WDF_FILE_CLOSE,
    PFN_WDF_IO_QUEUE_IO_CANCELED_ON_QUEUE,
    PFN_WDF_IO_QUEUE_IO_DEFAULT,
    PFN_WDF_IO_QUEUE_IO_DEVICE_CONTROL,
    PFN_WDF_IO_QUEUE_IO_INTERNAL_DEVICE_CONTROL,
    PFN_WDF_IO_QUEUE_IO_READ, PFN_WDF_IO_QUEUE_IO_RESUME,
    PFN_WDF_IO_QUEUE_IO_STOP, PFN_WDF_IO_QUEUE_IO_WRITE,
    PFN_WDF_IO_TARGET_QUERY_REMOVE,
    PFN_WDF_IO_TARGET_REMOVE_CANCELED,
    PFN_WDF_IO_TARGET_REMOVE_COMPLETE,
    PFN_WDF_OBJECT_CONTEXT_CLEANUP,
    PFN_WDF_OBJECT_CONTEXT_DESTROY, PLONGLONG, PVOID,
    ULONG, UNICODE_STRING, WDF_DRIVER_CONFIG,
    WDF_EXECUTION_LEVEL, WDF_FILEOBJECT_CONFIG,
    WDF_IO_QUEUE_CONFIG, WDF_NO_HANDLE,
    WDF_OBJECT_ATTRIBUTES, WDF_PNPPOWER_EVENT_CALLBACKS,
    WDF_REQUEST_SEND_OPTIONS, WDF_SYNCHRONIZATION_SCOPE,
    WDFDRIVER,
};

use crate::const_size_to_ulong;
use crate::ctx::WdfCtxNoneDesc;
use crate::op::{AsBuilder, AsCtxDescriptor};
#[cfg(feature = "test-runtime")]
use crate::rt::wdk_sys;
use crate::vals::{WdfExecutionLevel, WdfFileObjClass, WdfIoQueueDispatchType, WdfIoTargetOpenType, WdfRequestSendOptionsFlags, WdfSyncScope, WdfTriState};

#[derive(Default)]
pub struct WdfDriverSetup {
    pub on_device_add: PFN_WDF_DRIVER_DEVICE_ADD,
    pub on_driver_unload: PFN_WDF_DRIVER_UNLOAD,
    pub init_flags: ULONG,
    pub pool_tag: ULONG,
}
pub struct WdfIoTargetOpenParams {
    open_type: WdfIoTargetOpenType,
    on_query_remove: PFN_WDF_IO_TARGET_QUERY_REMOVE,
    on_remove_canceled: PFN_WDF_IO_TARGET_REMOVE_CANCELED,
    on_remove_complete: PFN_WDF_IO_TARGET_REMOVE_COMPLETE,

    #[cfg(feature = "kmdf-runtime")]
    target_device_obj: Option<PDEVICE_OBJECT>,

    #[cfg(feature = "kmdf-runtime")]
    p_file_object: PFILE_OBJECT,

    target_device_name: Option<UNICODE_STRING>,
    desired_access: Option<ACCESS_MASK>,
    share_access: Option<ULONG>,
    file_attrs: Option<ULONG>,
    create_disposition: Option<ULONG>,
    create_options: Option<ULONG>,
    ea_buffer: Option<PVOID>,
    ea_buffer_length: Option<ULONG>,
    allocation_size: Option<PLONGLONG>,
    file_info: Option<ULONG>,
    file_name: Option<UNICODE_STRING>,
}
pub struct WdfDriverConf {
    pub setup: WdfDriverSetup,
    pub registry_path: PCUNICODE_STRING,
}

#[derive(Default)]
pub struct WdfDevicePnpPowerSetup {
    pub on_device_hw_prepare:
        PFN_WDF_DEVICE_PREPARE_HARDWARE,
    pub on_device_hw_release:
        PFN_WDF_DEVICE_RELEASE_HARDWARE,

    pub on_device_d0_entry: PFN_WDF_DEVICE_D0_ENTRY,
    pub on_device_d0_entry_post_interrupts_enabled:
        PFN_WDF_DEVICE_D0_ENTRY_POST_INTERRUPTS_ENABLED,

    pub on_device_d0_exit: PFN_WDF_DEVICE_D0_EXIT,
    pub on_device_d0_exit_pre_interrupts_disabled:
        PFN_WDF_DEVICE_D0_EXIT_PRE_INTERRUPTS_DISABLED,

    pub on_device_self_managed_io_init:
        PFN_WDF_DEVICE_SELF_MANAGED_IO_INIT,

    pub on_device_self_managed_io_flush:
        PFN_WDF_DEVICE_SELF_MANAGED_IO_FLUSH,

    pub on_device_self_managed_io_cleanup:
        PFN_WDF_DEVICE_SELF_MANAGED_IO_CLEANUP,

    pub on_device_self_managed_io_suspend:
        PFN_WDF_DEVICE_SELF_MANAGED_IO_SUSPEND,

    pub on_device_self_managed_io_restart:
        PFN_WDF_DEVICE_SELF_MANAGED_IO_RESTART,

    pub on_device_surprise_removal:
        PFN_WDF_DEVICE_SURPRISE_REMOVAL,

    pub on_device_query_remove: PFN_WDF_DEVICE_QUERY_REMOVE,

    pub on_device_query_stop: PFN_WDF_DEVICE_QUERY_STOP,

    pub on_device_usage_notification:
        PFN_WDF_DEVICE_USAGE_NOTIFICATION,

    pub on_device_usage_notification_ex:
        PFN_WDF_DEVICE_USAGE_NOTIFICATION_EX,

    pub on_device_relations_query:
        PFN_WDF_DEVICE_RELATIONS_QUERY,
}

pub struct WdfIoQueueConfig {
    pub dispatch_type: WdfIoQueueDispatchType,
    pub power_managed: WdfTriState,
    pub allow_zero_length_requests: bool,
    pub default_queue: bool,
    pub on_io_default: PFN_WDF_IO_QUEUE_IO_DEFAULT,
    pub on_io_read: PFN_WDF_IO_QUEUE_IO_READ,
    pub on_io_write: PFN_WDF_IO_QUEUE_IO_WRITE,
    pub on_io_device_control:
        PFN_WDF_IO_QUEUE_IO_DEVICE_CONTROL,
    pub on_io_internal_device_control:
        PFN_WDF_IO_QUEUE_IO_INTERNAL_DEVICE_CONTROL,
    pub on_io_stop: PFN_WDF_IO_QUEUE_IO_STOP,
    pub on_io_resume: PFN_WDF_IO_QUEUE_IO_RESUME,
    pub on_io_canceled_on_queue:
        PFN_WDF_IO_QUEUE_IO_CANCELED_ON_QUEUE,
    pub settings: _WDF_IO_QUEUE_CONFIG__bindgen_ty_1,
    pub driver: WDFDRIVER,
}

impl Default for WdfIoQueueConfig {
    fn default() -> Self {
        Self {
            dispatch_type:
                WdfIoQueueDispatchType::Sequential,
            power_managed: Default::default(),
            allow_zero_length_requests: false,
            default_queue: false,
            on_io_default: None,
            on_io_read: None,
            on_io_write: None,
            on_io_device_control: None,
            on_io_internal_device_control: None,
            on_io_stop: None,
            on_io_resume: None,
            on_io_canceled_on_queue: None,
            settings: Default::default(),
            driver: WDF_NO_HANDLE.cast(),
        }
    }
}

impl AsBuilder for WdfIoQueueConfig {
    type Descriptor<'a>
        = WDF_IO_QUEUE_CONFIG
    where
        Self: 'a;

    fn build(&self) -> Self::Descriptor<'_> {
        WDF_IO_QUEUE_CONFIG {
            Size: const_size_to_ulong!(WDF_IO_QUEUE_CONFIG),
            DispatchType: self.dispatch_type.into(),
            PowerManaged: self.power_managed.into(),
            AllowZeroLengthRequests: self
                .allow_zero_length_requests
                as BOOLEAN,
            DefaultQueue: self.default_queue as BOOLEAN,
            EvtIoDefault: self.on_io_default,
            EvtIoRead: self.on_io_read,
            EvtIoWrite: self.on_io_write,
            EvtIoDeviceControl: self.on_io_device_control,
            EvtIoInternalDeviceControl: self
                .on_io_internal_device_control,
            EvtIoStop: self.on_io_stop,
            EvtIoResume: self.on_io_resume,
            EvtIoCanceledOnQueue: self
                .on_io_canceled_on_queue,
            Settings: self.settings,
            Driver: self.driver,
        }
    }
}

pub struct WdfObjAttrs<D: AsCtxDescriptor = WdfCtxNoneDesc>
{
    pub on_cleanup: PFN_WDF_OBJECT_CONTEXT_CLEANUP,
    pub on_destroy: PFN_WDF_OBJECT_CONTEXT_DESTROY,
    pub sync_scope: WdfSyncScope,
    pub execution_level: WdfExecutionLevel,
    pub parent_obj: Option<NonNull<c_void>>,
    _descriptor: PhantomData<D>,
}
impl<D: AsCtxDescriptor> AsBuilder for WdfObjAttrs<D> {
    type Descriptor<'b>
        = WDF_OBJECT_ATTRIBUTES
    where
        Self: 'b;

    fn build(&self) -> Self::Descriptor<'_> {
        let parent = self
            .parent_obj
            .map_or(ptr::null_mut(), |ptr| ptr.as_ptr());

        let unique: PCWDF_OBJECT_CONTEXT_TYPE_INFO =
            D::descriptor().unwrap_or(ptr::null_mut());

        WDF_OBJECT_ATTRIBUTES {
            Size: const_size_to_ulong!(
                WDF_OBJECT_ATTRIBUTES
            ),
            ExecutionLevel: self.execution_level.clone()
                as WDF_EXECUTION_LEVEL,
            SynchronizationScope: self.sync_scope.clone()
                as WDF_SYNCHRONIZATION_SCOPE,
            EvtCleanupCallback: self.on_cleanup,
            EvtDestroyCallback: self.on_destroy,
            ParentObject: parent,
            ContextTypeInfo: unique,
            ..WDF_OBJECT_ATTRIBUTES::default()
        }
    }
}
impl<D: AsCtxDescriptor> Default for WdfObjAttrs<D> {
    fn default() -> Self {
        Self {
            sync_scope: WdfSyncScope::None,
            execution_level: WdfExecutionLevel::Inherit,
            parent_obj: None,
            on_cleanup: None,
            on_destroy: None,
            _descriptor: PhantomData,
        }
    }
}
impl<D: AsCtxDescriptor> WdfObjAttrs<D> {
    #[must_use]
    pub fn with_sync_scope(
        mut self,
        sync_scope: WdfSyncScope,
    ) -> Self {
        self.sync_scope = sync_scope;
        self
    }

    #[must_use]
    pub fn with_execution_level(
        mut self,
        execution_level: WdfExecutionLevel,
    ) -> Self {
        self.execution_level = execution_level;
        self
    }

    #[must_use]
    pub fn with_on_destroy(
        mut self,
        on_destroy: PFN_WDF_OBJECT_CONTEXT_DESTROY,
    ) -> Self {
        self.on_destroy = on_destroy;
        self
    }

    #[must_use]
    pub fn with_on_cleanup(
        mut self,
        on_cleanup: PFN_WDF_OBJECT_CONTEXT_CLEANUP,
    ) -> Self {
        self.on_cleanup = on_cleanup;
        self
    }
}
impl WdfDriverConf {
    pub fn from_registry_path(
        registry_path: PCUNICODE_STRING,
    ) -> Self {
        Self {
            setup: WdfDriverSetup::default(),
            registry_path,
        }
    }
}
impl AsBuilder for WdfDriverConf {
    type Descriptor<'a> = WDF_DRIVER_CONFIG;

    fn build(&self) -> Self::Descriptor<'_> {
        WDF_DRIVER_CONFIG {
            Size: const_size_to_ulong!(WDF_DRIVER_CONFIG),
            EvtDriverDeviceAdd: self.setup.on_device_add,
            EvtDriverUnload: self.setup.on_driver_unload,
            DriverInitFlags: self.setup.init_flags,
            DriverPoolTag: self.setup.pool_tag,
        }
    }
}

impl AsBuilder for WdfDevicePnpPowerSetup {
    type Descriptor<'a> = WDF_PNPPOWER_EVENT_CALLBACKS;

    #[inline]
    fn build(&self) -> Self::Descriptor<'_> {
        WDF_PNPPOWER_EVENT_CALLBACKS {
            Size: const_size_to_ulong!(
                WDF_PNPPOWER_EVENT_CALLBACKS
            ),
            EvtDevicePrepareHardware: self
                .on_device_hw_prepare,
            EvtDeviceReleaseHardware: self
                .on_device_hw_release,
            EvtDeviceD0Entry: self.on_device_d0_entry,
            EvtDeviceD0EntryPostInterruptsEnabled: self
                .on_device_d0_entry_post_interrupts_enabled,
            EvtDeviceD0Exit: self.on_device_d0_exit,
            EvtDeviceD0ExitPreInterruptsDisabled: self
                .on_device_d0_exit_pre_interrupts_disabled,
            EvtDeviceSelfManagedIoInit: self
                .on_device_self_managed_io_init,
            EvtDeviceSelfManagedIoFlush: self
                .on_device_self_managed_io_flush,
            EvtDeviceSelfManagedIoCleanup: self
                .on_device_self_managed_io_cleanup,
            EvtDeviceSelfManagedIoSuspend: self
                .on_device_self_managed_io_suspend,
            EvtDeviceSelfManagedIoRestart: self
                .on_device_self_managed_io_restart,
            EvtDeviceSurpriseRemoval: self
                .on_device_surprise_removal,
            EvtDeviceQueryRemove: self
                .on_device_query_remove,
            EvtDeviceQueryStop: self.on_device_query_stop,
            EvtDeviceUsageNotification: self
                .on_device_usage_notification,
            EvtDeviceRelationsQuery: self
                .on_device_relations_query,
            EvtDeviceUsageNotificationEx: self
                .on_device_usage_notification_ex,
        }
    }
}

pub struct WdfRequestSendOption {
    pub flags: WdfRequestSendOptionsFlags,
    pub timeout: c_ulonglong,
}
pub type IoCtlCommand = u32;

impl Default for WdfRequestSendOption {
    fn default() -> Self {
        Self {
            flags: WdfRequestSendOptionsFlags::Synchronous,
            timeout: 0,
        }
    }
}

impl AsBuilder for WdfRequestSendOption {
    type Descriptor<'a> = WDF_REQUEST_SEND_OPTIONS;

    fn build(&self) -> Self::Descriptor<'_> {
        WDF_REQUEST_SEND_OPTIONS {
            Size: const_size_to_ulong!(
                WDF_REQUEST_SEND_OPTIONS
            ),
            Flags: self.flags as ULONG,
            Timeout: self.timeout as LONGLONG,
        }
    }
}

pub struct WdfFileObjectConfig {
    on_device_file_create: PFN_WDF_DEVICE_FILE_CREATE,
    on_file_close: PFN_WDF_FILE_CLOSE,
    on_file_cleanup: PFN_WDF_FILE_CLEANUP,
    auto_forward_cleanup_close: c_int,
    file_object_class: WdfFileObjClass,
}

impl Default for WdfFileObjectConfig {
    fn default() -> Self {
        Self {
            on_device_file_create: None,
            on_file_close: None,
            on_file_cleanup: None,
            auto_forward_cleanup_close: Default::default(),
            file_object_class:
                WdfFileObjClass::CanBeOptional.into(),
        }
    }
}
impl AsBuilder for WdfFileObjectConfig {
    type Descriptor<'a> = WDF_FILEOBJECT_CONFIG;

    #[inline]
    fn build(&self) -> Self::Descriptor<'_> {
        WDF_FILEOBJECT_CONFIG {
            Size: const_size_to_ulong!(
                WDF_FILEOBJECT_CONFIG
            ),
            EvtDeviceFileCreate: self.on_device_file_create,
            EvtFileClose: self.on_file_close,
            EvtFileCleanup: self.on_file_cleanup,
            AutoForwardCleanupClose: self
                .auto_forward_cleanup_close,
            FileObjectClass: self.file_object_class.into(),
        }
    }
}
