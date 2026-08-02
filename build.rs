use std::collections::BTreeSet;
#[cfg(feature = "kmdf-runtime")]
use std::{fs, io, path::Path};

#[cfg(feature = "kmdf-runtime")]
use regex::Regex;

#[cfg(feature = "kmdf-runtime")]
struct NtStatusEntry {
    macro_name: String,
    value: String,
}
#[cfg(feature = "kmdf-runtime")]
pub fn generate_is_wdf_type_impls(
    types: &[String],
) -> String {
    use std::collections::BTreeSet;

    let mut generated = String::new();

    for name in
        types.iter().cloned().collect::<BTreeSet<_>>()
    {
        generated.push_str(&format!(
            "impl crate::op::marks::IsWdfType for wdk_sys::{name} {{}}\n"
        ));
    }

    generated
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

        let macro_name = captures.get(1).unwrap().as_str();

        let value = captures.get(2).unwrap().as_str();

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
        let raw = u32::from_str_radix(
            status.value.trim_start_matches("0x"),
            16,
        )
        .unwrap();

        let signed = raw as i32;
        out.push_str(&format!(
            "                    {} => \"{}\",\n",
            signed, status.macro_name,
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

    fs::write(out_dir.join("wdkgen.rs"), generated)
}
use std::sync::{Arc, Mutex};

use bindgen::callbacks::ParseCallbacks;

#[derive(Default, Debug)]
pub struct WdfTypeCollector {
    pub types: Arc<Mutex<BTreeSet<String>>>,
}

use bindgen::callbacks::ItemInfo;

impl ParseCallbacks for WdfTypeCollector {
    fn item_name(
        &self,
        item_info: ItemInfo<'_>,
    ) -> Option<String> {
        let name = item_info.name;

        if name.starts_with("WDF") && name.ends_with("__") {
            self.types
                .lock()
                .unwrap()
                .insert(name.to_string());
        }

        None
    }
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

            let collector = WdfTypeCollector::default();
            let collected = collector.types.clone();

            let header = config
                .bindgen_header_contents([
                    wdk_build::ApiSubset::Base,
                    wdk_build::ApiSubset::Wdf,
                ])
                .map_err(|_| {
                    wdk_build::ConfigError::WdkContentRootDetectionError
                })?;

            let mut builder = bindgen::Builder::default()
                .header_contents("wrapper.h", &header)
                .parse_callbacks(Box::new(collector));

            for path in config.include_paths()? {
                builder = builder.clang_arg(format!(
                    "-I{}",
                    path.display()
                ));
            }

            builder = builder.clang_args(
                wdk_build::Config::wdk_bindgen_compiler_flags(),
            );

            for (name, value) in
                config.preprocessor_definitions()
            {
                builder = match value {
                    Some(v) => builder
                        .clang_arg(format!("-D{name}={v}")),
                    None => builder
                        .clang_arg(format!("-D{name}")),
                };
            }

            let _bindings = builder
                .generate()
                .map_err(|e| {
                    println!("cargo:warning=bindgen error: {e}");
                    wdk_build::ConfigError::WdkContentRootDetectionError
                })?;

            let statuses =
                collect_ntstatus_values_from_wdf_header(&ntstatus)
                    .map_err(|_| {
                        wdk_build::ConfigError::WdkContentRootDetectionError
                    })?;

            println!(
                "cargo:warning=Collected {} NTSTATUS values from {}",
                statuses.len(),
                ntstatus.display()
            );

            let ntstatus_bindings =
                generate_nt_status_as_ntstatus_impl(
                    &statuses,
                );

            let wdf_types = collected.lock().unwrap();

            println!(
                "cargo:warning=Collected {} WDF types",
                wdf_types.len()
            );

            let wdf_types_bindings =
                generate_is_wdf_type_impls(
                    &wdf_types
                        .iter()
                        .cloned()
                        .collect::<Vec<_>>(),
                );

            let generated = format!(
                "{}\n{}\n",
                ntstatus_bindings, wdf_types_bindings
            );

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
