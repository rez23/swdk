mod private {
    use core::ptr;

    #[must_use]
    #[allow(dead_code)]
    pub fn parse_hex_u16(s: &str) -> Option<u16> {
        let mut value: u16 = 0;

        for c in s.bytes() {
            let digit = match c {
                b'0'..=b'9' => c - b'0',
                b'a'..=b'f' => c - b'a' + 10,
                b'A'..=b'F' => c - b'A' + 10,
                _ => return None, // Invalid character
            };

            // avoids overflow: u16 max = 0xFFFF
            value = value.checked_mul(16)?
                .checked_add(u16::from(digit))?;
        }

        Some(value)
    }

    #[inline]
    #[allow(dead_code)]
    pub fn from_option_to_mut_ptr<T>(value: Option<&mut T>) -> *mut T {
        value.map_or(ptr::null_mut(), ptr::from_mut)
    }

    #[inline]
    #[allow(dead_code)]
    pub fn from_option_to_ptr<T>(value: Option<&T>) -> *const T {
        value.map_or(ptr::null(), ptr::from_ref)
    }
}

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
        const _: () = assert!(SIZE <= wdk_sys::ULONG::MAX as usize);

        #[allow(clippy::cast_possible_truncation)]
        {
            SIZE as wdk_sys::ULONG
        }
    }};
}

#[macro_export]
macro_rules! size_to_ulong {
    ($val:expr) => {{
       // Use a standard runtime let binding to support local variables
        let value: usize = $val;
        
        // This assertion is evaluated in debug builds.
        // In release builds, LLVM is extremely smart and will completely optimize
        // this assertion away into zero CPU instructions if the value is known to be safe.
        debug_assert!(value <= wdk_sys::ULONG::MAX as usize);

        #[allow(clippy::cast_possible_truncation)]
        {
            value as wdk_sys::ULONG
        }
    }};
}

#[macro_export]
macro_rules! if_ntstatus_error_return {
    ($status:expr) => {{
        let status = $status;

        if status < 0 {
            return status;
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
            return status;
        }

        status
    }};
}

#[macro_export]
macro_rules! if_nterror_return_result {
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

pub use private::*;
