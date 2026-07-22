
#[cfg(feature = "wdk-runtime")]
fn main() -> Result<(), wdk_build::ConfigError> {
    wdk_build::configure_wdk_library_build()
}

#[cfg(not(feature = "wdk-runtime"))]
fn main() {}