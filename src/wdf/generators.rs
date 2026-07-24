#[cfg(feature = "kmdf-runtime")]
pub mod __cmd {
    use core::ffi::c_void;
    use wdk_sys::{call_unsafe_wdf_function_binding, PCWDF_OBJECT_CONTEXT_TYPE_INFO, WDFOBJECT};

    #[inline]
    #[expect(clippy::missing_safety_doc, reason="This function only exist to expose this wdf binding to macros")]
    pub unsafe fn __wdf_object_typed_ctx_worker(
        wdf_obj: WDFOBJECT,
        p_type_info: PCWDF_OBJECT_CONTEXT_TYPE_INFO,
    ) -> *mut c_void {
        call_unsafe_wdf_function_binding!(
                WdfObjectGetTypedContextWorker,
                wdf_obj,
                p_type_info,
            )
    }
}

/// Map `$crate::rt::wdk_sys::call_unsafe_wdf_function_binding!` to a `Result`
/// This permits to use the macro in a `?` context
/// # Example
/// ```
/// unsafe {
///     call_ntstatus_wdf_binding!(
///         WdfDeviceCreate,
///         &raw mut device_init,
///         &raw mut attrs,
///         &raw mut device_handle)
/// }?;
/// ```
macro_rules! call_ntstatus_wdf_unsafe_binding {
    ($func:ident $(, $args:expr )* $(,)?) => {{
        $crate::rt::logging::ntstatus_to_result(
            $crate::rt::wdk_sys::call_unsafe_wdf_function_binding!(
                $func,
                $($args),*
            )
        )}};
    }
pub(crate) use call_ntstatus_wdf_unsafe_binding;
