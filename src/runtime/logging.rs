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
    const WINDOWS_TO_UNIX_EPOCH_100NS: i64 = 116_444_736_000_000_000;
    const HUNDRED_NS_PER_SEC: i64 = 10_000_000;
    const HUNDRED_NS_PER_MILLI: i64 = 10_000;
    const SECS_PER_DAY: i64 = 86_400;

    #[derive(Copy, Clone, Debug)]
    pub struct UtcDateTime {
        pub year: i32,
        pub month: u8,
        pub day: u8,
        pub hour: u8,
        pub minute: u8,
        pub second: u8,
        pub millis: u16,
    }

    impl core::fmt::Display for UtcDateTime {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(
                f,
                "{:04}-{:02}-{:02} {:02}:{:02}:{:02}.{:03} UTC",
                self.year,
                self.month,
                self.day,
                self.hour,
                self.minute,
                self.second,
                self.millis
            )
        }
    }

    #[inline]
    pub fn filetime_100ns_to_unix_parts(ts100ns: i64) -> (i64, u16) {
        let unix_100ns = ts100ns - WINDOWS_TO_UNIX_EPOCH_100NS;
        let secs = unix_100ns / HUNDRED_NS_PER_SEC;
        let millis = ((unix_100ns % HUNDRED_NS_PER_SEC) / HUNDRED_NS_PER_MILLI) as u16;
        (secs, millis)
    }

    #[inline]
    fn civil_from_days(days_since_unix_epoch: i64) -> (i32, u8, u8) {
        let z = days_since_unix_epoch + 719_468;
        let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
        let doe = z - era * 146_097;
        let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
        let y = yoe + era * 400;
        let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
        let mp = (5 * doy + 2) / 153;
        let d = doy - (153 * mp + 2) / 5 + 1;
        let m = mp + if mp < 10 { 3 } else { -9 };
        let year = y + if m <= 2 { 1 } else { 0 };

        (year as i32, m as u8, d as u8)
    }

    #[inline]
    pub fn unix_secs_to_utc(secs: i64, millis: u16) -> UtcDateTime {
        let days = secs.div_euclid(SECS_PER_DAY);
        let sod = secs.rem_euclid(SECS_PER_DAY);

        let (year, month, day) = civil_from_days(days);
        let hour = (sod / 3600) as u8;
        let minute = ((sod % 3600) / 60) as u8;
        let second = (sod % 60) as u8;

        UtcDateTime {
            year,
            month,
            day,
            hour,
            minute,
            second,
            millis,
        }
    }

    #[inline]
    pub fn timestamp_utc() -> UtcDateTime {
        let (secs, millis) = filetime_100ns_to_unix_parts(timestamp());
        unix_secs_to_utc(secs, millis)
    }
}

#[macro_export]
macro_rules! logger_name {
    () => {{env!("CARGO_PKG_NAME")}};
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {{
        ::swdk::println!(
            "{}[{}][error]: {} in {}:{} at line {}",
            $crate::logger_name!(),
            ::swdk::rt::logging::timestamp_utc(),
            format_args!($($arg)*),
            ::swdk::rt::logging::remove_module_name(core::module_path!()),
            ::swdk::rt::logging::file_name(core::file!()),
            core::line!()
        );
    }};
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {{
        ::swdk::println!(
            "{}[{}][info]: {}",
            $crate::logger_name!(),
            $crate::rt::logging::timestamp_utc(),
            format_args!($($arg)*)
        );
    }};
}

#[cfg(debug_assertions)]
#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {{
        ::swdk::println!(
            "{}[{}][debug]: {}",
            $crate::logger_name!(),
            ::swdk::rt::logging::timestamp_utc(),
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
        ::swdk::println!(
            "{}[{}][warn]: {} in {}:{}",
            $crate::logger_name!(),
            ::swdk::rt::logging::timestamp_utc(),
            alloc::format!($($arg)*),
            ::swdk::rt::logging::remove_module_name(core::module_path!()),
            ::swdk::rt::logging::file_name(core::file!()),
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
