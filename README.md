# Experimental Rust Safe Windows Driver Kit
> ###### This is a highly experimental project in a very early stage of development.
> ###### Contributions are welcome, but if you submit PR may be some time for the approval

**swdk** is middleware for the Windows Driver Framework (WDF) designed to provide a safe, idiomatic and expressive 
way to write Windows kernel drivers in Rust, with a current focus on KMDF.

**swdk** was built, for now, primary around KMDF, but is not designed around a specific WDF framework or other.  
Rather than wrapping WDF callbacks and framework features behind a higher-level, object-oriented abstraction, 
SWDK defines a compact set of Rust traits, types and capabilities that describe how WDF resources, operations and
relationships are represented within Rust. 

In this sense, *SWDK* does not merely wrap WDF handles in a Rust-friendly and idiomatic way.  
It shifts the focus from individual WDF callbacks to the **modeling of WDF relationships**.
Instead of treating framework operations as isolated function calls, SWDK exposes the relationships between WDF resources as Rust types, trait bounds and composable capabilities. 

This allows driver authors to express, at compile time, what WDF will do at runtime:

- a driver creates a device;
- a device owns queues and I/O targets;
- a WDF object carries typed context data;
- a descriptor configures how a framework object is created;
- a request flows through a queue and can be inspected, completed or forwarded.

Traits such as `AsKernelType`, `AsKernelType`, `AsCtxDescriptor` and `AsBuilder` are therefore not just helpers.

They are the formal operators of SWDK’s meta-WDF language: they describe how WDF resources relate to each other and expose those relationships to the Rust compiler.

# A brief example


```rust
#[unsafe(export_name = "DriverEntry")]
pub unsafe extern "system" fn driver_entry(
    driver_obj: PDRIVER_OBJECT,
    registry_path: PCUNICODE_STRING,
) -> NTSTATUS {
    unwrap_nt!(
        Handle::<WDFDRIVER__>::from_kernel(
            Handel::new(driver_obj.to_non_null()),
            WdfDriverConf {
                setup: WdfDriverSetup {
                    on_device_add: Some(on_device_add),
                    ..Default::default()
                },
                registry_path,
            },
            Some(WdfObjAttrs::<WdfCtxNoneDesc>::default()),
        )
    );

    STATUS_SUCCESS
}

unsafe extern "C" fn on_device_add(
    _driver: WDFDRIVER,
    device_init: PWDFDEVICE_INIT,
) -> NTSTATUS {
    unwrap_nt!(
        Handle::<WDFDEVICE__>::from_kernel(
            device_init.to_non_null(),
            Some(WdfObjAttrs::<WdfCtxNoneDesc>::default()),
        )
    );

    info!("Device added and loaded!");
    STATUS_SUCCESS
}
```

Although minimal, this example creates both the KMDF driver object and a device object. It demonstrates several of the framework's core concepts:

- `Handle<T>` as the primary abstraction
- typed WDF object creation
- driver configuration through data structures
- automatic descriptor initialization
- `NTSTATUS`-oriented error handling

For a more complete overview of ***swdk*** capabilities, [xmouseinput-sys](https://github.com/rez23/xmouseinput-sys) is a simple driver that 
registers itself as a filter and asks the device its capability via IOCTL, you can test it via Hyper-V and compile via MSVC-Clang

# Getting started

swdk is built on Microsoft's **windows-drivers-rs** ecosystem.

Building a driver requires WDF bindings generated from the official C headers. Microsoft's `wdk-build` crate configures this process, while the `cargo-wdk` tool provides the `cargo wdk` command. See [windows-drivers-rs](https://github.com/microsoft/windows-drivers-rs) for installation requirements and further details.

A typical driver project uses:

```toml
[dependencies]
swdk = "..."
```

The optional `swdk-macros` crate provides custom derives and attributes, such as `#[derive(CtxDescriptor)]`:

```toml
[dependencies]
swdk = "..."
swdk-macros = "..."
```

## Build your driver

Because swdk is built on WDF, it uses the bindings exposed by Microsoft's `wdk` crates. Your driver crate therefore needs a custom `build.rs`, driver-model metadata in `Cargo.toml`, and a working LLVM/Clang installation.

Add `wdk-build` as a build dependency:

```toml
[build-dependencies]
wdk-build = "..."
```

Create `build.rs`:

```rust
fn main() -> Result<(), wdk_build::ConfigError> {
    wdk_build::configure_wdf_binary_build();
    Ok(())
}
```

After installing the `cargo-wdk` command-line tool, build the driver with:

```bash
cargo wdk build
```



# Goals

This crate aims to give to Rust Windows kernel-driver developers a safe and ergonomic bridge between Rust 
and the Windows Driver Framework runtime. 

SWDK was designed trying to follow the next principals:
- Preserve the original KMDF architecture
- Apply Rust ownership semantics to WDF resources
- Reduce the boilerplate required for native WDF descriptors
- Improve `NTSTATUS` diagnostics and debugging
- Provide a runtime abstraction suitable for testing
- Let driver authors choose when to work with raw WDF types

Probably the primary objective of *swdk* is
> If you are already familiar with the Windows Driver Framework, swdk should feel familiar too.



# Philosophy

Fews key ideas lie at the heart of swdk:

1. `Handle` represent a generic handle to a runtime kernel resource described by WDF (eg: WDFDEVICE, WDFDRIVER, WDFDRIVER_INT, ecc).
2. `Handle` deal with resources that handle (allocation, deallocation, initialization, ecc).
3. `Handle` permit to extends any WDF resource without touching the raw WDF types that handle.

swdk implements these ideas through a deliberately simple default concept:

```rust
Handle<T>
```

`Handle<T>` packages a WDF kernel object or resource in a Rust type that can expose the corresponding raw WDF value when required.



### Data-oriented

Many frameworks or wrappers built around KMDF attempt to encapsulate devices and framework resources behind large object-oriented abstractions. swdk intentionally avoids this approach.

Instead of building an object hierarchy, swdk describes the following as composable pieces of data:

- ownership relationships
- object contexts
- descriptors
- object attributes
- framework callbacks
- I/O targets
- device capabilities

Traits then transform these descriptions into native KMDF structures and runtime objects.

For example, the following C code configures device initialization:

```c
WdfFdoInitSetFilter(DeviceInit);

WDF_PNPPOWER_EVENT_CALLBACKS callbacks;
WDF_PNPPOWER_EVENT_CALLBACKS_INIT(&callbacks);

callbacks.EvtDeviceD0Entry = OnD0Entry;

WdfDeviceInitSetPnpPowerEventCallbacks(
    DeviceInit,
    &callbacks
);
```

With swdk, the same intent can be expressed as a composition of behaviors:

```rust
fn initialize_device_init(device_init: PWDFDEVICE_INIT) {
    let device_init = Handle::new(device_init.to_non_null())
        .with_filter()
        .with_pnp_setup(WdfDevicePnpPowerSetup {
            on_device_d0_entry: Some(on_device_d0_entry),
            ..Default::default()
        });
}

fn on_device_d0_entry(...) {
    ...
}
```

The result remains recognizably KMDF, while shifting the focus from imperative descriptor manipulation to declarative composition.



### Zero-cost

swdk relies heavily on Rust generics, trait implementations and static dispatch. As a result, most abstractions provided by the framework exist only at compile time.

carry additional semantic information for the compiler while introducing little to no runtime overhead.

The framework avoids dynamic dispatch (`dyn Trait`) and delegates specialization to Rust's compile-time type-resolution system through generics and traits. This keeps the APIs expressive while producing code close to an equivalent hand-written KMDF driver.

In practice, most abstractions compile down to:

- direct function calls
- direct structure initialization
- direct pointer manipulation
- direct KMDF callback invocations

without introducing additional runtime layers.
# Additional resources

For more information about the Windows Driver Framework, KMDF and the official Rust WDK project, see:

- [windows-drivers-rs](https://github.com/microsoft/windows-drivers-rs)
- [swdk official repository](https://github.com/rez23/swdk)
- [Get started with WDF](https://learn.microsoft.com/windows-hardware/drivers/wdf/)
- [Windows Driver Kit API reference](https://learn.microsoft.com/windows-hardware/drivers/ddi/)
