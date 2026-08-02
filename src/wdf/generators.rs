/// Compile-time safe conversion of a type's size into a ULONG.
/// This avoids runtime panics and ensures KMDF ABI compatibility.
///
/// The macro performs:
/// - A `size_of::<T>()`
/// - A compile-time assertion that the size fits into ULONG
/// - A cast to ULONG with clippy suppression
///
/// This is the correct approach for kernel-mode Rust, where runtime panics
/// must be strictly avoided.
#[macro_export]
macro_rules! const_size_to_ulong {
    ($t:ty) => {{
            const SIZE: usize = core::mem::size_of::<$t>();

            // Compile-time check: if KMDF ever breaks ABI, the driver won't compile.
            const _: () = assert!(SIZE <= $crate::rt::wdk_sys::ULONG::MAX as usize);

            #[allow(clippy::cast_possible_truncation)]
            {
                SIZE as $crate::rt::wdk_sys::ULONG
            }
        }};
}

/// A macro to safely cast a `usize` value to an `ULONG`.
///
/// This macro takes an expression, evaluates it as a `usize`, and attempts to cast it to
/// the `ULONG` type from the `wdk_sys` module. It ensures type safety and prevents potential
/// overflows in debug builds.
///
/// # Parameters
/// - `$val:expr` - An expression that evaluates to a `usize` value to be cast to `ULONG`.
///
/// # Behavior
/// - A let binding is used to store the input value locally in order to support intermediate
///   local variables.
/// - A debug assertion is included to ensure that the input value does not exceed the maximum
///   value of `ULONG`. This assertion is only active in debug builds, allowing for extra
///   safety during testing while preserving runtime performance in release builds.
/// - After the check, the value is cast to an `ULONG` type. This cast allows potential
///   truncation of the value, but it is considered safe due to the preceding assertion.
///
/// # Notes
/// - This macro depends on the `wdk_sys` module and the `ULONG` type defined within it.
/// - The macro is designed to perform efficiently, with any debug-specific validation
///   optimized out in release builds.
///
/// # Example
/// ```rust
/// use my_crate::size_to_ulong;
///
/// let size: usize = 42;
/// let ulong_value = size_to_ulong!(size);
/// assert_eq!(ulong_value as usize, size);
/// ```
///
/// # Debug Assertion
/// In debug builds, if the input value exceeds the maximum value of `ULONG`, the macro will
/// panic. Ensure that `$val` is always within the valid range to avoid runtime errors.
///
/// # Clippy Allowance
/// A `clippy::cast_possible_truncation` lint is suppressed within the macro. This is because
/// the truncation risk is explicitly mitigated by the debug assertion.
#[macro_export]
macro_rules! size_to_ulong {
    ($val:expr) => {{
           // Use a standard runtime let binding to support local variables
            let value: usize = $val;

            // This assertion is evaluated in debug builds.
            // In release builds, LLVM is extremely smart and will completely optimize
            // this assertion away into zero CPU instructions if the value is known to be safe.
            debug_assert!(value <= $crate::rt::wdk_sys::ULONG::MAX as usize);

            #[allow(clippy::cast_possible_truncation)]
            {
                value as $crate::rt::wdk_sys::ULONG
            }
    }};
}

#[macro_export]
macro_rules! function_name {
    () => {{
        fn f() {}
        let name = core::any::type_name_of_val(&f);

        &name[..name.len() - 3]
    }};
}

#[macro_export]
macro_rules! unwrap_nt {
    ($expr:expr) => {{
        match $expr {
            Ok(value) => value,
            Err(ntstatus) => return ntstatus,
        }
    }};

    ($result:expr,as_error,msg = $msg:literal) => {{
        match $result {
            Ok(value) => value,
            Err(ntstatus) => {
                let as_status = ntstatus.fmt_status();
                let as_hex = ntstatus.fmt_hex();

                $crate::error!(
                    "Failure happens in '{}' with status '{}({})': '{}'",
                    $crate::function_name!(),
                    as_status,
                    as_hex,
                    $msg,
                );
                return ntstatus;
            }
        }
    }};

    ($result:expr,as_error) => {{
        match $result {
            Ok(value) => value,
            Err(ntstatus) => {
                let as_status = ntstatus.fmt_status();
                let as_hex = ntstatus.fmt_hex();

                $crate::error!(
                    "Failure happens in '{}' with status '{}({})'",
                    $crate::function_name!(),
                    as_status,
                    as_hex,
                );
                return ntstatus;
            }
        }
    }};

    ($result:expr, on_failure=$on_failure:expr) => {{
        match $result {
            Ok(value) => value,
            Err(ntstatus) => {
                return $on_failure(ntstatus);
            }
        }
    }};
}

#[macro_export]
macro_rules! ok_or_nt {
    ($expr:expr) => {{
        $crate::unwrap_nt!($expr.ok_or(::swdk::rt::wdk_sys::STATUS_INTERNAL_ERROR))
    }};
    ($expr:expr, status_err=$status:ident) => {{
        $crate::unwrap_nt!($expr.ok_or($status))
    }};
    ($expr:expr, on_failure=$on_failure:expr) => {{
        $crate::unwrap_nt!($expr.ok_or(::swdk::rt::wdk_sys::STATUS_INTERNAL_ERROR), on_failure=|_| $on_failure())
    }};
    ($expr:expr, status_err=$status:ident, on_failure=$on_failure:expr) => {{
        $crate::unwrap_nt!($expr.ok_or(::swdk::rt::wdk_sys::STATUS_INTERNAL_ERROR), on_failure=|_| $on_failure())
    }};
    ($expr:expr, as_error) => {{
        $crate::unwrap_nt!($expr.ok_or(::swdk::rt::wdk_sys::STATUS_INTERNAL_ERROR), as_error)
    }};
    ($expr:expr, as_error, msg=$msg:literal) => {{
        $crate::unwrap_nt!($expr.ok_or(::swdk::rt::wdk_sys::STATUS_INTERNAL_ERROR), as_error, msg=$msg)
    }};
    ($expr:expr, status_err=$status:ident, as_error) => {{
        $crate::unwrap_nt!($expr.ok_or($status), as_error)
    }};
    ($expr:expr, status_err=$status:ident, as_error, msg=$msg:literal) => {{
        $crate::unwrap_nt!($expr.ok_or($status), as_error, msg=$msg)
    }};
}

/// A macro to simplify converting NTSTATUS values into `Result` types.
///
/// This macro evaluates the given NTSTATUS value and determines whether it represents
/// a success or an error. NTSTATUS values less than 0 typically indicate an error.
/// In case of a failure, the macro optionally allows logging an error message
/// or name associated with the received NTSTATUS value.
///
/// # Usage
/// - **Single argument (`$status`)**: Returns an `Ok(status)` if the status indicates success, or directly returns the failing `status`.
/// - **Two arguments (`$status`, `$message`)**: Logs an error message and returns an `Err(status)` if it indicates failure.
///   Otherwise, returns `Ok(status)`.
///
/// # Parameters
/// - `$status`: An expression representing an NTSTATUS value to evaluate.
/// - `$message`: A string literal message (optional) to be logged when the status indicates an error.
///
/// # Return Value
/// - When `$message` is not provided:
///   - Returns an `Ok(status)` if `$status` indicates success (`status >= 0`).
///   - Returns the `status` directly if it indicates failure (`status < 0`).
/// - When `$message` is provided:
///   - Logs the provided `$message` along with the NTSTATUS name and hex value, and returns an `Err(status)` if it indicates failure.
///   - Returns `Ok(status)` if `$status` indicates success.
///
/// # Examples
///
/// ## Example 1: Without logging
/// ```rust
/// let nt_status = some_function_that_returns_ntstatus();
/// let result = from_ntstatus_to_ntresult!(nt_status);
/// match result {
///     Ok(status) => println!("Operation succeeded with status: {:#x}", status),
///     Err(status) => println!("Operation failed with status: {:#x}", status),
/// }
/// ```
///
/// ## Example 2: With logging
/// ```rust
/// let nt_status = some_function_that_returns_ntstatus();
/// let result = from_ntstatus_to_ntresult!(nt_status, "Operation failed");
/// if let Err(err) = result {
///     println!("Logged error with NTSTATUS: {:#x}", err);
/// }
/// ```
///
/// # Notes
/// - The macro assumes the availability of `$crate:error!` for logging and `$crate::utils::helpers::ntstatus_name(status)`
///   for NTSTATUS name conversion. Ensure these utilities are correctly defined in the crate where the macro is used.
#[macro_export]
macro_rules! from_ntstatus_to_ntresult {
    ($status:expr) => {{
        let status = $status;

        if status < 0 {
            return Ok(status);
        }

        status
    }};
    ($status:expr, $message:literal) => {{
        let status = $status;

        if status < 0 {
            $crate:error!(
                "{}: {} ({:#x})",
                $message,
                $crate::utils::helpers::ntstatus_name(status),
                status
            );
            return Err(status);
        }

        Ok(status)
    }};
}

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
/// - [`crate::unwrap_nt`]
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
