
#[cfg(feature = "kmdf-runtime")]
fn main() -> Result<(), wdk_build::ConfigError> {
    wdk_build::configure_wdk_library_build()
}

#[cfg(feature = "test-runtime")]
fn main() {}