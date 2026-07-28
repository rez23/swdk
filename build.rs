#[cfg(feature = "kmdf-runtime")]
use regex::Regex;

#[cfg(feature = "kmdf-runtime")]
struct NtStatusEntry {
    macro_name: String,
    value: String,
}

#[cfg(feature = "kmdf-runtime")]
fn collect_ntstatus_values_from_wdf_header(
    wdf_header_path: &std::path::Path,
) -> std::io::Result<Vec<NtStatusEntry>> {
    let content = std::fs::read_to_string(wdf_header_path)?;

    let regex = Regex::new(
        r#"^#define\s+(STATUS_[A-Z0-9_]+)\s+\(\(NTSTATUS\)(0x[0-9A-F]+)L\)"#
    )
        .unwrap();

    let mut entries = Vec::new();

    for line in content.lines() {
        let line = line.trim();

        let Some(captures) = regex.captures(line) else {
            continue;
        };

        let macro_name =
            captures.get(1).unwrap().as_str();

        let value =
            captures.get(2).unwrap().as_str();

        entries.push(NtStatusEntry {
            macro_name: macro_name.to_string(),
            value: value.to_string(),
        });
    }

    Ok(entries)
}


#[cfg(feature = "kmdf-runtime")]
fn generate_nt_status_as_ntstatus_impl(
    statuses: &[NtStatusEntry],
) -> String {
    let mut out = String::new();

    out.push_str(
        "
         use crate::alloc::format;\n\
         use crate::alloc::string::String;\n\
         impl crate::op::AsNtStatus for wdk_sys::NTSTATUS {\n\
             fn fmt_status(self) -> &'static str {\n\
                 match self {\n",
    );

    for status in statuses {
        let raw =
            u32::from_str_radix(
                status.value.trim_start_matches("0x"),
                16,
            )
                .unwrap();

        let signed = raw as i32;
        out.push_str(&format!(
            "                    {} => \"{}\",\n",
            signed,
            status.macro_name,
        ));
    }

    out.push_str(
        "                    _ => \"STATUS_UNKNOWN\",\n\
                 }\n\
             }\n\n\
             fn fmt_hex(self) -> String {\n\
                 format!(\"0x{:08X}\", self as u32)\n\
             }\n\
         }\n",
    );

    out
}

#[cfg(feature = "kmdf-runtime")]
fn write_generated_file(
    generated: &str,
) -> std::io::Result<()> {
    let out_dir = std::path::PathBuf::from(
        std::env::var("OUT_DIR").unwrap(),
    );

    std::fs::write(
        out_dir.join("ntstatus.rs"),
        generated,
    )
}
#[cfg(feature = "kmdf-runtime")]
fn main() -> Result<(), wdk_build::ConfigError> {
    wdk_build::configure_wdk_library_build_and_then(
        |config| {
            let ntstatus = config
                .include_paths()?
                .find_map(|path| {
                    let candidate = path.join("ntstatus.h");
                    candidate.exists().then_some(candidate)
                })
                .ok_or(
                    wdk_build::ConfigError::WdkContentRootDetectionError,
                )?;

            println!(
                "cargo:warning=Using {}",
                ntstatus.display()
            );

            let statuses =
                collect_ntstatus_values_from_wdf_header(&ntstatus)
                    .map_err(|_| {
                        wdk_build::ConfigError::WdkContentRootDetectionError
                    })?;

            println!(
                "cargo:warning=Collected {} NTSTATUS values",
                statuses.len()
            );

            let generated =
                generate_nt_status_as_ntstatus_impl(&statuses);

            write_generated_file(&generated)
                .map_err(|_| {
                    wdk_build::ConfigError::WdkContentRootDetectionError
                })?;

            Ok(())
        },
    )
}

#[cfg(feature = "test-runtime")]
fn main() {}