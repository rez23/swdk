//! # TODO
//! Auto-generate buildable WDF wrappers from bindgen metadata.
//! Hand-writing WDF_PNPPOWER_EVENT_CALLBACKS-like structs
//! does not scale.
//! # Notes
//! This is a provvisorial solution for having idiomatic struct pretty formatted without the need of specifying size
//!  Anyway, this is not the best solution. Think to port the huge amount of Buildable struct in WDF is an impossible task.
//! The solution is something like NTSTATUS.fmt_status(), a build script generator that auto creates the builders from the C original struct,
//! So less late or earlier I will implement something like that... but, for now, this is a little version needed for getting work the first examples
use core::ptr;

use wdk_sys::{
    ACCESS_MASK, PDEVICE_OBJECT, PFILE_OBJECT,
    PFN_WDF_DEVICE_D0_ENTRY,
    PFN_WDF_DEVICE_D0_ENTRY_POST_INTERRUPTS_ENABLED,
    PFN_WDF_DEVICE_D0_EXIT,
    PFN_WDF_DEVICE_D0_EXIT_PRE_INTERRUPTS_DISABLED,
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
    PFN_WDF_IO_TARGET_QUERY_REMOVE,
    PFN_WDF_IO_TARGET_REMOVE_CANCELED,
    PFN_WDF_IO_TARGET_REMOVE_COMPLETE, PLONGLONG, PVOID,
    UNICODE_STRING, WDF_IO_TARGET_OPEN_TYPE,
    WDF_PNPPOWER_EVENT_CALLBACKS, WDFIOTARGET,
};

use crate::ctx::WdfCtxNoneDesc;
use crate::op::{AsBuilder, AsCtxDescriptor, AsRaw};
use crate::rt::wdk_sys::{
    PCUNICODE_STRING, PCWDF_OBJECT_CONTEXT_TYPE_INFO,
    PFN_WDF_DRIVER_DEVICE_ADD, PFN_WDF_DRIVER_UNLOAD,
    PFN_WDF_OBJECT_CONTEXT_CLEANUP,
    PFN_WDF_OBJECT_CONTEXT_DESTROY, ULONG,
    WDF_DRIVER_CONFIG, WDF_EXECUTION_LEVEL,
    WDF_OBJECT_ATTRIBUTES, WDF_SYNCHRONIZATION_SCOPE,
    WDFOBJECT,
};
use crate::vals::{
    WdfExecutionLevel, WdfIoTargetOpenType, WdfSyncScope,
};
use crate::{Handle, const_size_to_ulong};

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
#[derive(Default)]
pub struct WdfDevicePropsSetup {
    pub(crate) is_filter: bool,
}
#[derive(Default)]
pub struct WdfDeviceConf {
    pub pnp_power_setup: WdfDevicePnpPowerSetup,
    pub props_setup: WdfDevicePropsSetup,
}
pub struct WdfObjAttrs<D: AsCtxDescriptor = WdfCtxNoneDesc>
{
    pub on_cleanup: PFN_WDF_OBJECT_CONTEXT_CLEANUP,
    pub on_destroy: PFN_WDF_OBJECT_CONTEXT_DESTROY,
    pub sync_scope: WdfSyncScope,
    pub execution_level: WdfExecutionLevel,
    pub parent_obj: Option<Handle<WDFOBJECT>>,
    _descriptor: D,
}
impl<D: AsCtxDescriptor> AsBuilder for WdfObjAttrs<D> {
    type Descriptor<'b>
        = WDF_OBJECT_ATTRIBUTES
    where
        Self: 'b;

    fn build(&self) -> Self::Descriptor<'_> {
        let parent = self
            .parent_obj
            .as_ref()
            .map_or(ptr::null_mut(), |ptr| ptr.raw());

        let unique: PCWDF_OBJECT_CONTEXT_TYPE_INFO =
            D::unique().unwrap_or(ptr::null());

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
            _descriptor: D::default(),
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
