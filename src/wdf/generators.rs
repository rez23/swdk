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

/// A macro to invoke a Windows Driver Framework (WDF) function, handling NTSTATUS
/// values and converting them into a result type.
///
/// This macro simplifies calling unsafe WDF functions by wrapping them with error
/// handling. It uses the `ntstatus_to_result` function to translate `NTSTATUS` return
/// values into `Result` for easier and safer error management.
///
/// # Usage
///
/// ```rust
/// call_ntstatus_wdf_unsafe_binding!(MyWdfFunction, arg1, arg2);
/// ```
///
/// - `WdfFunctionBinding`: The name of the WDF function to call.
/// - `arg1, arg2, arg3, ecc...`: The arguments required from the WDF function (if any).
///   These are variadic and optional but must match the signature of the WDF function being invoked.
///
/// # Parameters
///
/// - `$func`: Identifier of the WDF function.
/// - `$args...`: Comma-separated list of arguments to pass to the WDF function (optional).
///
/// # Behavior
///
/// 1. Invokes the desired unsafe WDF function using the `call_unsafe_wdf_function_binding!` macro.
/// 2. Converts the returned `[NTSTATUS]` value into a [`op::NtResult`] type using the
///    `ntstatus_to_result` function for error handling.
/// 3. Supports optional trailing commas in the argument list.
///
/// # Safety
///
/// Since this macro involves unsafe FFI calls to WDF functions, ensure:
/// - The function pointer and arguments are valid and properly initialized.
/// - The corresponding WDF environment is correctly configured in the runtime.
///
/// # Example
/// ```rust
/// let my_fn() -> NtResult<MyType>{
///     let result = call_ntstatus_wdf_unsafe_binding!(SomeWdfFunction, param1, param2)?;
/// }
/// ```
///
/// In the example above, the `SomeWdfFunction` is called with the provided parameters and
/// uses the usual `?` operator semantics to handle [`op::NtResult::Err<NTSTATUS>`] if any.
///
/// # Notes
///
/// - This macro depends on external components provided by the crate:
///   - [`call_unsafe_wdf_function_binding!`]: A macro for invoking the WDF function provided directly by MS [wdk crate](https://github.com/microsoft/windows-drivers-rs).
///
/// # See Also
/// - [`if_nterror_return`]: convert [`wdk_sys::NTSTATUS`] into [`op::NtResult`]
/// - [`crate::if_nterror_return_ntstatus`]
///
/// # External Resources
/// For have information about the available functions binding you can see on the ***huge***
/// Microsoft documentation pages:
/// - [KMDF Official MS Docs](https://learn.microsoft.com/windows-hardware/drivers/wdf/kernel-mode-driver-framework--kmdf-)
/// - [WDF Official MS Docs](https://learn.microsoft.com/windows-hardware/drivers/wdf/)
#[macro_export]
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
