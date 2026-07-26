use core::ptr;

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
use crate::vals::{WdfExecutionLevel, WdfSyncScope};
use crate::{Handle, const_size_to_ulong};

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

#[derive(Default)]
pub struct WdfDriverSetup {
    pub on_device_add: PFN_WDF_DRIVER_DEVICE_ADD,
    pub on_driver_unload: PFN_WDF_DRIVER_UNLOAD,
    pub init_flags: ULONG,
    pub pool_tag: ULONG,
}

pub struct WdfDriverConf {
    pub setup: WdfDriverSetup,
    pub registry_path: PCUNICODE_STRING,
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
