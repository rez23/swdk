#[cfg(feature = "test-runtime")]
use crate::rt::test_rt::*;

use crate::op::{AsCtxDescriptor, AsNoneCtxDesc, AsNoneCtxUnique, AsUnique};
use core::ptr;
use wdk_sys::{PCWDF_OBJECT_CONTEXT_TYPE_INFO, WDF_OBJECT_CONTEXT_TYPE_INFO};

#[allow(dead_code)]
pub mod operations {
    #[cfg(feature = "test-runtime")]
    use crate::rt::test_rt::*;

    use core::ptr::from_ref;
    use wdk_sys::{PWDF_OBJECT_CONTEXT_TYPE_INFO, WDFOBJECT, WDF_OBJECT_CONTEXT_TYPE_INFO};
    use crate::{const_size_to_ulong, size_to_ulong};

    pub unsafe fn build_for_data_type<T: 'static>(
        addrs_of: *const T,
        type_name: &str,
    ) -> WDF_OBJECT_CONTEXT_TYPE_INFO {
        WDF_OBJECT_CONTEXT_TYPE_INFO {
            Size: const_size_to_ulong!(WDF_OBJECT_CONTEXT_TYPE_INFO),
            ContextSize: size_to_ulong!(size_of::<T>()) as usize,
            ContextName: type_name.as_bytes().as_ptr().cast(),
            UniqueType: core::ptr::addr_of!(addrs_of).cast_mut().cast(),
            EvtDriverGetUniqueContextType: None,
        }
    }

    pub fn build_for_data_type_mut<T: 'static>(
        data: &mut T,
        type_name: &str,
    ) -> WDF_OBJECT_CONTEXT_TYPE_INFO {
        WDF_OBJECT_CONTEXT_TYPE_INFO {
            Size: const_size_to_ulong!(WDF_OBJECT_CONTEXT_TYPE_INFO),
            ContextSize: size_to_ulong!(size_of::<T>()) as usize,
            ContextName: type_name.as_bytes().as_ptr().cast(),
            UniqueType: from_ref(data).cast(),
            EvtDriverGetUniqueContextType: None,
        }
    }

    #[cfg(feature = "kmdf-runtime")]
    #[inline]
    pub unsafe fn get_context_device_ptr<T: 'static>(
        data: PWDF_OBJECT_CONTEXT_TYPE_INFO,
        handle: WDFOBJECT,
    ) -> *const T {
        wdk_sys::call_unsafe_wdf_function_binding!(WdfObjectGetTypedContextWorker, handle, data,)
            .cast::<T>()
    }

    #[cfg(feature = "kmdf-runtime")]
    #[inline]
    pub unsafe fn get_context_device_ptr_mut<T: 'static>(
        data: PWDF_OBJECT_CONTEXT_TYPE_INFO,
        handle: WDFOBJECT,
    ) -> *mut T {
        wdk_sys::call_unsafe_wdf_function_binding!(WdfObjectGetTypedContextWorker, handle, data,)
            .cast()
    }
}

#[repr(transparent)]
pub struct WdfObjCtxTypeInfo(WDF_OBJECT_CONTEXT_TYPE_INFO);
unsafe impl Sync for WdfObjCtxTypeInfo {}
impl WdfObjCtxTypeInfo {
    pub const fn new(inner: WDF_OBJECT_CONTEXT_TYPE_INFO) -> Self {
        Self(inner)
    }
}

unsafe impl AsUnique for WdfObjCtxTypeInfo {
    unsafe fn unique(&self) -> PCWDF_OBJECT_CONTEXT_TYPE_INFO {
        let inner = core::ptr::from_ref::<Self>(self).cast::<WDF_OBJECT_CONTEXT_TYPE_INFO>();
        // SAFETY: This dereference is sound since the underlying
        // WDF_OBJECT_CONTEXT_TYPE_INFO is guaranteed to have the same memory
        // layout as WDFObjectContextTypeInfo since WDFObjectContextTypeInfo is
        // declared as repr(transparent)
        unsafe { *inner }.UniqueType
    }
}

/// Represents the absence of the context unique type for a [`IsWdfHandle`] type
#[derive(Default)]
pub struct WdfCtxNull;
unsafe impl AsUnique for WdfCtxNull {

    /// Returns a null pointer when no ctx object
    unsafe fn unique(&self) -> PCWDF_OBJECT_CONTEXT_TYPE_INFO {
        ptr::null()
    }
}
impl AsNoneCtxUnique for WdfCtxNull {}

/// Represents the absence of the context descriptor for a [`IsWdfHandle`] type
#[derive(Default)]
pub struct WdfCtxNoneDesc;
impl AsCtxDescriptor for WdfCtxNoneDesc {}
impl AsNoneCtxDesc for WdfCtxNoneDesc {}