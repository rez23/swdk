mod private {
    #[cfg(feature = "test-runtime")]
    use crate::rt::test_rt::*;

    use crate::op::NtResult;
    use wdk_sys::ntddk::KeQuerySystemTimePrecise;
    use wdk_sys::{LARGE_INTEGER, NTSTATUS};

    #[inline]
    pub fn ntstatus_to_result(status: NTSTATUS) -> NtResult {
        if status >= 0 { Ok(()) } else { Err(status) }
    }

    #[allow(dead_code)]
    pub fn remove_module_name(full_path: &str) -> &str {
        full_path
            .split_once("::")
            .map(|(_, path)| path)
            .unwrap_or("")
    }

    #[allow(dead_code)]
    pub fn file_name(full_path: &str) -> &str {
        full_path.rsplit(['/', '\\']).next().unwrap_or("??.rs")
    }

    pub fn timestamp() -> i64 {
        let mut ts = LARGE_INTEGER::default();

        unsafe {
            KeQuerySystemTimePrecise(&mut ts);
        }

        unsafe { ts.QuadPart }
    }

    #[must_use]
    #[allow(dead_code)]
    pub fn ntstatus_name(status: NTSTATUS) -> &'static str {
        match status {
            wdk_sys::STATUS_SUCCESS => "STATUS_SUCCESS",
            wdk_sys::STATUS_UNSUCCESSFUL => "STATUS_UNSUCCESSFUL",
            wdk_sys::STATUS_INVALID_PARAMETER => "STATUS_INVALID_PARAMETER",
            wdk_sys::STATUS_INVALID_DEVICE_REQUEST => "STATUS_INVALID_DEVICE_REQUEST",
            wdk_sys::STATUS_INSUFFICIENT_RESOURCES => "STATUS_INSUFFICIENT_RESOURCES",
            wdk_sys::STATUS_OBJECT_NAME_COLLISION => "STATUS_OBJECT_NAME_COLLISION",
            wdk_sys::STATUS_OBJECT_NAME_INVALID => "STATUS_OBJECT_NAME_INVALID",
            wdk_sys::STATUS_ACCESS_DENIED => "STATUS_ACCESS_DENIED",
            wdk_sys::STATUS_NOT_SUPPORTED => "STATUS_NOT_SUPPORTED",
            wdk_sys::STATUS_DEVICE_NOT_READY => "STATUS_DEVICE_NOT_READY",
            wdk_sys::STATUS_DELETE_PENDING => "STATUS_DELETE_PENDING",
            _ => "UNKNOWN_NTSTATUS",
        }
    }
}

#[macro_export]
macro_rules! logger_name {
    () => {{env!("CARGO_PKG_NAME")}};
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {{
        swdk::println!(
            "{}[{}][error]: {} in {}:{} at line {}",
            $crate::logger_name!(),
            $crate::rt::logging::timestamp(),
            format_args!($($arg)*),
            $crate::rt::logging::remove_module_name(core::module_path!()),
            $crate::rt::logging::file_name(core::file!()),
            core::line!()
        );
    }};
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {{
        swdk::println!(
            "{}[{}][info]: {}",
            $crate::logger_name!(),
            $crate::rt::logging::timestamp(),
            format_args!($($arg)*)
        );
    }};
}

#[cfg(debug_assertions)]
#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {{
        swdk::println!(
            "{}[{}][debug]: {}",
            $crate::logger_name!(),
            $crate::rt::logging::timestamp(),
            format_args!($($arg)*),
        );
    }};
}

#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {};
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {{
        wdk::println!(
            "{}[{}][warn]: {} in {}:{}",
            $crate::logger_name!(),
            $crate::rt::logging::timestamp(),
            alloc::format!($($arg)*),
            $crate::rt::logging::remove_module_name(core::module_path!()),
            $crate::rt::logging::file_name(core::file!()),
        );
    }};
}

#[macro_export]
macro_rules! xmi_windbg_msg_with_status {
    ($level_macro:ident, $status:expr, $message:literal) => {{
        let status = $status;

        $crate::$level_macro!("{} (STATUS=(%!STATUS)", $message,)
    }};
}

pub use private::*;
