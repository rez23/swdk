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
            value = value
                .checked_mul(16)?
                .checked_add(u16::from(digit))?;
        }

        Some(value)
    }

    #[inline]
    #[allow(dead_code)]
    pub fn from_option_to_mut_ptr<T>(
        value: Option<&mut T>,
    ) -> *mut T {
        value.map_or(ptr::null_mut(), ptr::from_mut)
    }

    #[inline]
    #[allow(dead_code)]
    pub fn from_option_to_ptr<T>(
        value: Option<&T>,
    ) -> *const T {
        value.map_or(ptr::null(), ptr::from_ref)
    }
}

#[cfg(feature = "kmdf-runtime")]
mod __ntstatus {
    include!(concat!(env!("OUT_DIR"), "/wdkgen.rs"));
}

pub use private::*;