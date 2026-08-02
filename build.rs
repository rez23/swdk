use std::collections::BTreeSet;
#[cfg(feature = "kmdf-runtime")]
use std::{fs, io, path::Path};
use std::fmt::format;
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

    for name in types.iter().cloned().collect::<BTreeSet<_>>() {
        generated.push_str(&format!(
            "impl crate::op::IsWdfType for wdk_sys::{name} {{}}\n"
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
    ).unwrap();

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

fn generate_single_match_branch(left: &str, right: &str) -> String {
    format!("    {left} => {right},\n")
}

#[cfg(feature = "kmdf-runtime")]
fn generate_nt_status_as_ntstatus_impl(
    statuses: &[NtStatusEntry],
) -> String {
    let nt_status_enum_decl_open = String::from("\n\
    #[derive(\n\
        Debug,\n\
        Copy,\n\
        Clone,\n\
        Eq,\n\
        PartialEq,\n\
    )]\n\
    pub enum NtStatus {\n");
    let nts_status_enum_decl_close = String::from("\
       StatusNotWdfError((wdk_sys::NTSTATUS, core::option::Option<&'static str>)),\n\
    }\n\
    ");

    let from_nt_status_for_nt_open = String::from("\n\
    impl From<NtStatus> for wdk_sys::NTSTATUS {\n\
        fn from(status: NtStatus) -> Self {\n\
            match status {\n\
                NtStatus::StatusNotWdfError((status, _)) => status,\n\
    ");

    let match_fn_declaration_with_impl_close = String::from("\n\
            }\n\
        }\n\
    }\n\
    ");

    let from_nt_to_nt_status_open = String::from("\n\
    impl From<wdk_sys::NTSTATUS> for NtStatus {\n\
        fn from(status: wdk_sys::NTSTATUS) -> Self {\n\
            match status {\n\
                _ => NtStatus::StatusNotWdfError((status, core::option::Option::None)),\n\
    ");

    let format_functions_impls = String::from("\
    impl crate::op::AsNtStatus for NtStatus { }\n\
    impl PartialEq<NtStatus> for wdk_sys::NTSTATUS {\n\
        fn eq(&self, other: &NtStatus) -> bool {\n\
            *self == wdk_sys::NTSTATUS::from(*other)
        }\n\
    }\n\
    impl PartialEq<wdk_sys::NTSTATUS> for NtStatus {\n\
        fn eq(&self, other: &wdk_sys::NTSTATUS) -> bool {\n\
            wdk_sys::NTSTATUS::from(*self) == *other\n\
        }\n\
    }\n\
    impl PartialOrd<wdk_sys::NTSTATUS> for NtStatus {\n\
        fn partial_cmp(\n\
            &self,\n\
            other: &wdk_sys::NTSTATUS,\n\
        ) -> Option<core::cmp::Ordering> {\n\
            let lhs = wdk_sys::NTSTATUS::from(*self);\n\
            lhs.partial_cmp(other)\n\
        }\n\
    }\n\
    ");

    let with_info_fn_def_open = String::from("\
        pub fn with_info(self, info: &'static str) -> Self {\n\
            match self {\n\
                  NtStatus::StatusNotWdfError((ntstatus, _)) => NtStatus::StatusNotWdfError((\
                      ntstatus, core::option::Option::Some(info)\
                  )),\n"
    );
    let match_fn_close = String::from("\
        }\n\
    }\n\
    ");

    let from_nt_with_info_fn_def = String::from("\
        pub fn from_nt_with_info(ntstatus: wdk_sys::NTSTATUS, info: &'static str) -> Self {\n\
            NtStatus::from(ntstatus).with_info(info)\n\
        }\n\
        ");


    let ntstatus_impl_open = String::from("\
        impl NtStatus {\n"
    );
    let enum_nt_vals = generate_enum_def(statuses);
    let from_nt_to_nt_status = generate_matches(statuses, |value, name| {
        (value, format!("NtStatus::{}(core::option::Option::None)", name.to_pascal_case()))
    });
    let from_nt_status_for_nt = generate_matches(statuses, |value, name| {
        (format!("NtStatus::{}(_)", name.to_pascal_case()), value)
    });
    let with_info_match = generate_matches(statuses, |value, name| {
        (format!("NtStatus::{}(_)", name.to_pascal_case()), format!("NtStatus::{}(core::option::Option::Some(info))", name.to_pascal_case()))
    });
    let ntstatus_impl_close = String::from("}\n");

    format!("\
        /// NT status errors generated from ntstatus.h\n\
        {nt_status_enum_decl_open}\
        {enum_nt_vals}\
        {nts_status_enum_decl_close}\
        \n\
        {from_nt_status_for_nt_open}\
        {from_nt_status_for_nt}\
        {match_fn_declaration_with_impl_close}\
        \n\
        {from_nt_to_nt_status_open}\
        {from_nt_to_nt_status}\
        {match_fn_declaration_with_impl_close}\
        \n\
        {format_functions_impls}\n\
        \n\
        {ntstatus_impl_open}\n\
        {with_info_fn_def_open}\n\
        {with_info_match}\n\
        {match_fn_close}\n\
        {from_nt_with_info_fn_def}\n\
        {ntstatus_impl_close}\n\
    ")
}

fn generate_enum_def(statuses: &[NtStatusEntry]) -> String {
    statuses.iter().map(|status| {
        // Enum
        format!(
            "    {}(core::option::Option<&'static str>),\n",
            status.macro_name.to_pascal_case(),
        )
    }).collect()
}
fn generate_matches(statuses: &[NtStatusEntry], op: impl Fn(String, String) -> (String, String)) -> String {
    statuses.iter().map(|status| {
        let (name, value) = op(status.value.clone(), status.macro_name.clone());
        generate_single_match_branch(name.as_str(), value.as_str())
    }).collect()
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
use heck::ToPascalCase;

impl ParseCallbacks for WdfTypeCollector {
    fn item_name(
        &self,
        item_info: ItemInfo<'_>,
    ) -> Option<String> {
        let name = item_info.name;

        if name.starts_with("WDF") && name.ends_with("__") {
            self.types.lock().unwrap().insert(name.to_string());
        }

        None
    }
}

#[cfg(feature = "kmdf-runtime")]
fn main() -> Result<(), wdk_build::ConfigError> {
    wdk_build::configure_wdk_library_build_and_then(
        |config| {
            let ntstatus = config.include_paths()?.find_map(|path| {
                let candidate = path.join("ntstatus.h");
                candidate.exists().then_some(candidate)
            }).ok_or(
                wdk_build::ConfigError::WdkContentRootDetectionError,
            )?;

            let collector = WdfTypeCollector::default();
            let collected = collector.types.clone();

            let header = config.bindgen_header_contents([
                wdk_build::ApiSubset::Base,
                wdk_build::ApiSubset::Wdf,
            ]).map_err(|_| {
                wdk_build::ConfigError::WdkContentRootDetectionError
            })?;

            let mut builder = bindgen::Builder::default().header_contents("wrapper.h", &header).parse_callbacks(Box::new(collector));

            for path in config.include_paths()? {
                builder = builder.clang_arg(format!(
                    "-I{}",
                    path.display()
                ));
            }

            builder = builder.clang_args(
                wdk_build::Config::wdk_bindgen_compiler_flags(),
            );

            for (name, value) in config.preprocessor_definitions() {
                builder = match value {
                    Some(v) => builder.clang_arg(format!("-D{name}={v}")),
                    None => builder.clang_arg(format!("-D{name}")),
                };
            }

            let _bindings = builder.generate().map_err(|e| {
                println!("cargo:warning=bindgen error: {e}");
                wdk_build::ConfigError::WdkContentRootDetectionError
            })?;

            let statuses = collect_ntstatus_values_from_wdf_header(&ntstatus).map_err(|_| {
                wdk_build::ConfigError::WdkContentRootDetectionError
            })?;

            println!(
                "cargo:warning=Collected {} NTSTATUS values from {}",
                statuses.len(),
                ntstatus.display()
            );

            let ntstatus_bindings = generate_nt_status_as_ntstatus_impl(
                &statuses,
            );

            let wdf_types = collected.lock().unwrap();

            println!(
                "cargo:warning=Collected {} WDF types",
                wdf_types.len()
            );

            let wdf_types_bindings = generate_is_wdf_type_impls(
                &wdf_types.iter().cloned().collect::<Vec<_>>(),
            );

            let generated = format!("\
            {ntstatus_bindings}\n\
            {wdf_types_bindings}\n\
            ");

            write_generated_file(&generated).map_err(|_| {
                wdk_build::ConfigError::WdkContentRootDetectionError
            })?;
            Ok(())
        },
    )
}

#[cfg(feature = "test-runtime")]
fn main() {}
