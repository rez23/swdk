# SWDF - Safe WDF for Rust

**SWDF** is a strongly-typed WDF/KMDF wrapper for Windows kernel driver development.

Its design is centered around a single capability type, `Handle<T>`, which enables zero-cost object derivation, compile-time polymorphic behavior, typed context management, and explicit modeling of kernel object ownership and relationships through Rust's type system.

SWDF does not try to hide WDF concepts. Instead, it exposes them through a strongly-typed API that makes object creation, derivation, context access, and IOCTL communication explicit and verifiable at compile time.

---

## Features

- Strongly typed WDF object handles
- Zero-cost object derivation
- Compile-time ownership and object relationship modeling
- Typed WDF context management
- Typed IOCTL request/response workflow
- Generic WDF builders
- No runtime abstraction overhead
- Minimal `unsafe` in user code

---

# Core Concept: `Handle<T>`

The entire library is built around a single type:

```rust
pub struct Handle<T>(T);
```

Every WDF object is represented through a specialized `Handle<T>`.

Examples:

```rust
Handle<WDFDRIVER>
Handle<WDFDEVICE>
Handle<WDFIOTARGET>
```

Special semantic aliases are also provided:

```rust
type HandleRef<'a, T>    = Handle<&'a T>;
type HandleMut<'a, T>    = Handle<&'a mut T>;

type HandleRaw<T>        = Handle<*const T>;
type HandleRawMut<T>     = Handle<*mut T>;
```

These aliases do not introduce additional runtime types. They simply provide a richer vocabulary for expressing handle semantics.

---

# WDF Object Relationships

SWDF models two fundamental WDF concepts:

## Owner Objects

Objects created from external resources.

Examples:

```text
PDRIVER_OBJECT  -> WDFDRIVER
PWDFDEVICE_INIT -> WDFDEVICE
```

Modeled through:

```rust
AsWdfOwner<T>
```

Example:

```rust
Handle::<WDFDEVICE>::from_owned(...)
Handle::<WDFDRIVER>::from_owned(...)
```

---

## Derived Objects

Objects obtained from existing WDF objects.

Examples:

```text
WDFDEVICE -> WDFIOTARGET
```

Modeled through:

```rust
AsWdfOwned<T>
```

Example:

```rust
Handle::<WDFIOTARGET>::from_owner(&device)
```

This directly describes the underlying WDF relationship.

---

# Context Management

SWDF provides a typed wrapper around WDF context objects.

A context descriptor can be generated using:

```rust
swdf_declare_context_handle!(GamepadData);
```

This macro automatically generates:

- WDF context type metadata
- Typed context accessors
- Static WDF descriptors
- Context retrieval helpers

Example:

```rust
#[derive(Default)]
pub struct GamepadData {
    connected: bool,
}

swdf_declare_context_handle!(GamepadData);
```

Attach a context at object creation:

```rust
let device = Handle::<WDFDEVICE>::from_owned(
    device_init,
    Some(WdfObjAttrs::<GamepadData>::default()),
)?;
```

Retrieve it later:

```rust
let ctx = GamepadData::wdf_get(&device);
```

---

# Builders

SWDF uses builder objects to describe WDF configuration structures.

## Object Attributes

```rust
WdfObjAttrs<T>
```

Example:

```rust
let attrs =
    WdfObjAttrs::<GamepadData>::default()
        .with_sync_scope(WdfSyncScope::Device);
```

---

## Driver Configuration

```rust
WdfDriverConf
```

```rust
let conf = WdfDriverConf {
    setup: WdfDriverSetup {
        on_driver_unload: Some(on_driver_unload),
        on_device_add: Some(on_driver_device_add),
        ..Default::default()
    },
    registry_path,
};
```

---

# Typed IOCTL Layer

SWDF provides strongly typed IOCTL requests and responses.

## Request

```rust
IoCtlRequest<T>
```

Create an empty request:

```rust
IoCtlRequest::with_command(
    IOCTL_HID_GET_COLLECTION_INFORMATION
)
```

Create a request with payload:

```rust
IoCtlRequest::new(
    MY_IOCTL,
    my_request_data
)
```

---

## Response

```rust
IoCtlResponse<T>
```

Responses automatically know how to build the appropriate WDF memory descriptor.

Example:

```rust
let info: IoCtlResponse<HID_COLLECTION_INFORMATION> =
    io_target.send_ioctl(
        IoCtlRequest::with_command(
            IOCTL_HID_GET_COLLECTION_INFORMATION
        )
    )?;
```

Access the response as:

```rust
info.ProductID
info.VendorID
```

through `Deref`.

---

# Raw Access

Some WDF APIs still require raw handles.

SWDF exposes them explicitly:

```rust
handle.raw()
```

Example:

```rust
unsafe {
    WdfIoTargetGetState(
        io_target.raw()
    )
};
```

Borrowed handles can materialize their underlying value through:

```rust
raw_with_borrow()
```

Example:

```rust
Handle::new(device)
    .raw_with_borrow()
```

---

# Example Driver

The following example shows the complete creation flow of:

```text
PDRIVER_OBJECT
      ↓
WDFDRIVER
      ↓
PWDFDEVICE_INIT
      ↓
WDFDEVICE
      ↓
WDFIOTARGET
      ↓
IOCTL Communication
```

```rust
#[unsafe(export_name = "DriverEntry")]
pub unsafe extern "system" fn driver_entry(
    driver_obj: PDRIVER_OBJECT,
    registry_path: PCUNICODE_STRING,
) -> NTSTATUS {

    Handle::<WDFDRIVER>::from_owned_with_attrs(
        driver_obj,
        WdfDriverConf {
            setup: WdfDriverSetup {
                on_driver_unload: Some(on_driver_unload),
                on_device_add: Some(on_driver_device_add),
                ..Default::default()
            },
            registry_path,
        },
        Some(
            WdfObjAttrs::<WdfCtxNoneDesc>::default()
        ),
    )?;

    STATUS_SUCCESS
}

unsafe extern "C" fn on_driver_device_add(
    _driver: WDFDRIVER,
    device_init: PWDFDEVICE_INIT,
) -> NTSTATUS {

    let device =
        Handle::<WDFDEVICE>::from_owned(
            device_init,
            Some(
                WdfObjAttrs::<GamepadData>::default()
            ),
        )?;

    let io_target =
        Handle::<WDFIOTARGET>::from_owner(
            &device
        )?;

    let caps: IoCtlResponse<HID_COLLECTION_INFORMATION> =
        io_target.send_ioctl(
            IoCtlRequest::with_command(
                IOCTL_HID_GET_COLLECTION_INFORMATION
            ),
        )?;

    STATUS_SUCCESS
}
```

---

# Design Philosophy

SWDF is not a traditional wrapper that attempts to hide WDF.

Instead, it models WDF concepts directly through the type system.

Relationships such as:

```text
PWDFDEVICE_INIT -> WDFDEVICE

WDFDEVICE -> WDFIOTARGET

WDFOBJECT -> Context<T>
```

become explicit type-level operations.

The result is a driver implementation that describes object relationships and creation flows rather than manually orchestrating raw WDF calls.

---

# Status

SWDF is currently focused on:

- WDF object creation
- Object derivation
- Typed contexts
- Typed IOCTL workflows
- Compile-time object relationship modeling

Additional WDF abstractions will continue to be built around the same fundamental building block:

```rust
Handle<T>
```