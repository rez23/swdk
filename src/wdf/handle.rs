mod private {
    use wdk_sys::HANDLE;

    use crate::op::marks::IsWdfType;
    #[cfg(feature = "test-runtime")]
    use crate::rt::wdk_sys;
    impl IsWdfType for HANDLE {}

    /// # Example
    /// `Handle` is the basic building block of `swdk`.
    /// You can use it to implement your own functions for raw WDF kernel types.
    /// For example, the `swdk` library implements [`Handle::read_status()`] for [`WDFIOTARGET`]
    /// exactly in this way:
    ///```rust
    /// impl swdk::HandleBasimpl swdk::__Handle<swdk::rt::wdk_sys::WDFIOTARGET> {
    ///     pub fn read_status(&self) -> swdk::val::WdfIoTargetState {
    ///         swdk::val::WdfIoTargetState::from(unsafe {
    ///             ...
    /// ```
    ///
    /// `swdk` is still in development, anyway, you can already declare a full Rust driver
    /// in just a few lines using `Handle`:
    /// ```rust
    /// use swdk::rt::wdk_sys::{WDFDEVICE, WDFDRIVER, PWDFDEVICE_INIT, STATUS_SUCCESS};
    /// use swdk::__Handle;
    /// use swdk::unwrap_nt;
    /// use swdk::bd::{WdfDriverConf, WdfDriverSetup, WdfObjAttrs};
    /// use swdk::println;
    ///
    /// type HandleDevice = __Handle<WDFDEVICE>;
    ///
    ///#[unsafe(export_name = "DriverEntry")]
    /// pub unsafe extern "system" fn driver_entry(
    ///     driver_obj: PDRIVER_OBJECT,
    ///     registry_path: PCUNICODE_STRING,
    /// ) -> NTSTATUS {
    ///     debug!("DriverEntry launched from WDF");
    ///     unwrap_nt!(
    ///         Handle::<WDFDRIVER>::allocate_from_owned(
    ///             driver_obj,
    ///             WdfDriverConf {
    ///                 setup: WdfDriverSetup {
    ///                     on_driver_unload: Some(
    ///                         on_driver_unload
    ///                     ),
    ///                     on_device_add: Some(
    ///                         on_driver_device_add
    ///                     ),
    ///                     ..WdfDriverSetup::default()
    ///                 },
    ///                 registry_path,
    ///             },
    ///             Some(WdfObjAttrs::<WdfCtxNoneDesc>::default())
    ///         )
    ///     );
    ///     STATUS_SUCCESS
    /// }
    ///
    /// #[unsafe(link_section = "PAGE")]
    /// unsafe extern "C" fn on_driver_device_add(
    ///     _driver: WDFDRIVER,
    ///     device_init: PWDFDEVICE_INIT,
    /// ) -> NTSTATUS {
    ///     println!("Hello world")
    /// }
    ///
    /// #[unsafe(link_section = "PAGE")]
    /// unsafe extern "C" fn on_driver_unload(
    ///     _driver: WDFDRIVER,
    ///     device_init: PWDFDEVICE_INIT,
    /// ) -> NTSTATUS {
    ///     println!("Goodbye")
    /// }
    /// ```
    /// Above, an example of a potential driver that writes "Hello world" to WinDbg buffer
    /// when its device is added, and "Goodbye" when it unloads.
    /// # See Also
    /// - [swdk official repo](https://github.com/rez23/swdk)
    /// - [WDF get started](https://learn.microsoft.com/windows-hardware/drivers/wdf/)
    /// - [API reference documentation for Windows Driver Kit](https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/)
    #[derive(Clone, Debug)]
    pub struct Handle<'a, H: IsWdfType>(&'a H);
    #[derive(Debug)]
    pub struct HandleMut<'a, H: IsWdfType>(&'a mut H);

    impl<'a, H: IsWdfType> From<HandleMut<'a, H>> for Handle<'a, H> {
        fn from(value: HandleMut<'a, H>) -> Self {
            Self {
                0: value.0,
            }
        }
    }
    impl<T: IsWdfType + ?Sized> IsWdfType for &T {}
    impl<T: IsWdfType + ?Sized> IsWdfType for &mut T {}

    mod impls {
        use core::borrow::Borrow;
        use core::ops::{Deref, DerefMut};
        use core::ptr;
        use core::ptr::NonNull;

        use crate::op::marks::IsWdfType;
        use crate::op::{
            AsNonNull, AsNonNullCVoid, AsRaw,
            AsRawWithBorrow, IntoRaw, ToNonNull,
        };
        #[cfg(feature = "test-runtime")]
        use crate::rt::wdk_sys;
        use crate::wdf::handle::private::{
            Handle, HandleMut,
        };

        impl<'a, H: IsWdfType> Handle<'a, H> {
            pub fn new(raw: NonNull<H>) -> Self {
                // SAFETY: this is safe because HANDLE
                //         types are passed by WDF
                Self(unsafe { raw.as_ref() })
            }
        }

        impl<'a, H: IsWdfType> HandleMut<'a, H> {
            pub fn new(mut raw: NonNull<H>) -> Self {
                Self(unsafe { raw.as_mut() })
            }
        }
        impl<'a, H: IsWdfType> Handle<'a, H> {
            pub fn into_inner(self) -> Option<NonNull<H>> {
                NonNull::new(
                    ptr::from_ref(self.0).cast_mut(),
                )
            }
        }

        impl<'a, H: IsWdfType + Copy> AsNonNull<H>
            for HandleMut<'a, H>
        {
        }
        impl<'a, H: IsWdfType + Copy> ToNonNull<H>
            for HandleMut<'a, H>
        {
        }
        impl<'a, H: IsWdfType + Copy> AsNonNullCVoid<H>
            for HandleMut<'a, H>
        {
        }
        impl<'a, H: IsWdfType + Copy> AsNonNull<H>
            for Handle<'a, H>
        {
        }
        impl<'a, H: IsWdfType + Copy> ToNonNull<H>
            for Handle<'a, H>
        {
        }
        impl<'a, H: IsWdfType + Copy> AsNonNullCVoid<H>
            for Handle<'a, H>
        {
        }
        impl<'a, H: IsWdfType> Deref for Handle<'a, H> {
            type Target = H;
            fn deref(&self) -> &Self::Target {
                self.0
            }
        }
        impl<'a, H: IsWdfType> Deref for HandleMut<'a, H> {
            type Target = H;
            fn deref(&self) -> &Self::Target {
                self.0
            }
        }

        impl<'a, T: IsWdfType> DerefMut for HandleMut<'a, T> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                self.0
            }
        }

        impl<'a, H: IsWdfType> AsRef<H> for Handle<'a, H> {
            fn as_ref(&self) -> &H {
                self.0
            }
        }
        impl<'a, H: IsWdfType> AsRef<H> for HandleMut<'a, H> {
            fn as_ref(&self) -> &H {
                self.0
            }
        }
        impl<'a, H: IsWdfType + Copy> IntoRaw<H> for Handle<'a, H> {}
        impl<'a, H: IsWdfType + Copy> AsRaw<H>
            for HandleMut<'a, H>
        {
        }
        impl<'a, H: IsWdfType + Copy> Borrow<H> for Handle<'a, H> {
            fn borrow(&self) -> &H {
                self.0
            }
        }
        impl<'a, H: IsWdfType + Copy> AsRawWithBorrow<H>
            for Handle<'a, H>
        {
        }
        impl<'a, H: IsWdfType + Copy> Handle<'a, H> {
            pub fn raw(&self) -> H {
                *self.0
            }
        }

        #[cfg(feature = "minimal-runtime")]
        mod kmdf {
            mod object {
                use core::ptr;
                use core::ptr::NonNull;

                use wdk_sys::STATUS_INTERNAL_ERROR;

                use crate::bd::WdfObjAttrs;
                use crate::op::{
                    AsBuilder, AsCtxDescriptor, FromKernel,
                    NtResult,
                };
                #[cfg(feature = "kmdf-runtime")]
                use crate::rt::__cb;
                #[cfg(feature = "test-runtime")]
                use crate::rt::wdk_sys;
                use crate::rt::wdk_sys::{
                    WDF_NO_HANDLE, WDFOBJECT,
                };
                use crate::wdf::handle::private::Handle;

                impl<'a> Handle<'a, WDFOBJECT> {
                    pub fn allocate(
                        attrs: Option<WdfObjAttrs>,
                    ) -> NtResult<Self>
                    {
                        Self::from_kernel_explicit(
                            Handle::new(NonNull::dangling()),
                            None,
                            attrs,
                        )
                    }
                }
                impl<'a> FromKernel<WDFOBJECT> for Handle<'a, WDFOBJECT> {
                    type Accessor = ();
                    type Conf = ();

                    fn from_kernel_explicit<D>(
                        _: Handle<Self::Accessor>,
                        _: Option<Self::Conf>,
                        attrs: Option<WdfObjAttrs<D>>,
                    ) -> NtResult<Self>
                    where
                        D: AsCtxDescriptor,
                    {
                        let mut obj = WDF_NO_HANDLE.cast();
                        let mut attrs =
                            attrs.map(|a| a.build());
                        let attrs_ptr =
                            attrs.as_mut().map_or(
                                ptr::null_mut(),
                                ptr::from_mut,
                            );

                        #[cfg(feature = "kmdf-runtime")]
                        unsafe {
                            __cb::wdf_object_create(
                                attrs_ptr,
                                &raw mut obj,
                            )
                        }?;

                        Ok(Handle::new(
                            NonNull::new(&raw mut obj)
                                .ok_or(
                                    STATUS_INTERNAL_ERROR,
                                )?,
                        ))
                    }
                }
            }
            mod device_init {
                use core::ptr;

                use wdk_sys::{
                    STATUS_INVALID_PARAMETER,
                    WDFDEVICE_INIT,
                };

                use crate::bd::{
                    WdfDevicePnpPowerSetup,
                    WdfFileObjectConfig, WdfObjAttrs,
                };
                use crate::op::marks::IsWdfType;
                use crate::op::{
                    AsBuilder, AsNonNull, NtResult,
                };
                #[cfg(feature = "kmdf-runtime")]
                use crate::rt::__cb;
                #[cfg(feature = "test-runtime")]
                use crate::rt::wdk_sys;
                use crate::wdf::handle::private::Handle;
                impl IsWdfType for WDFDEVICE_INIT {}

                impl<'a> Handle<'a, WDFDEVICE_INIT> {
                    pub fn with_filter(
                        self,
                    ) -> Option<Self>
                    {
                        #[cfg(feature = "kmdf-runtime")]
                        unsafe {
                            __cb::wdf_f_do_init_set_filter(
                                self.as_non_null()?.as_ptr().cast(),
                            )
                        };

                        Some(self)
                    }

                    pub fn with_pnp_setup(
                        self,
                        setup: WdfDevicePnpPowerSetup,
                    ) -> Option<Self>
                    {
                        #[cfg(feature = "kmdf-runtime")]
                        let pnp_setup = setup.build();

                        #[cfg(feature = "kmdf-runtime")]
                        unsafe {
                            __cb::wdf_device_init_set_pnp_power_event_callbacks(
                                self.as_non_null()?.as_ptr(),
                                ptr::from_ref(&pnp_setup).cast_mut(),
                            )
                        };
                        Some(self)
                    }

                    #[cfg(feature = "kmdf-runtime")]
                    pub fn with_file_object(
                        self,
                        conf: WdfFileObjectConfig,
                        attrs: Option<WdfObjAttrs>,
                    ) -> NtResult<Self>
                    {
                        let mut attrs = attrs
                            .map(|attrs| attrs.build());

                        let conf = conf.build();
                        let p_attrs =
                            attrs.as_mut().map_or(
                                ptr::null_mut(),
                                ptr::from_mut,
                            );

                        unsafe {
                            __cb::wdf_device_init_set_file_object_config(
                                self.as_non_null().ok_or(STATUS_INVALID_PARAMETER)?.as_ptr(),
                                ptr::from_ref(&conf).cast_mut(),
                                p_attrs,
                            )
                        };
                        Ok(self)
                    }
                }
            }
            mod device {
                use core::ptr;
                use core::ptr::NonNull;

                use wdk_sys::{
                    STATUS_INTERNAL_ERROR,
                    STATUS_INVALID_PARAMETER, WDFDEVICE__,
                    WDFDEVICE_INIT, WDFIOTARGET__,
                    WDFQUEUE__,
                };

                use crate::bd::WdfObjAttrs;
                use crate::op::{
                    AsBuilder, AsCtxDescriptor, AsNonNull,
                    FromKernel, NtResult,
                };
                #[cfg(feature = "kmdf-runtime")]
                use crate::rt::__cb;
                #[cfg(feature = "test-runtime")]
                use crate::rt::wdk_sys;
                use crate::rt::wdk_sys::{
                    WDF_NO_HANDLE, WDFDEVICE,
                };
                use crate::wdf::handle::private::Handle;

                impl<'a> FromKernel<WDFDEVICE__>
                    for Handle<'a, WDFDEVICE__>
                {
                    type Accessor = WDFDEVICE_INIT;
                    type Conf = ();

                    fn from_kernel_explicit<D>(
                        accessor: Handle<Self::Accessor>,
                        _: Option<Self::Conf>,
                        attrs: Option<WdfObjAttrs<D>>,
                    ) -> NtResult<Self>
                    where
                        D: AsCtxDescriptor,
                    {
                        let mut p_device: WDFDEVICE =
                            WDF_NO_HANDLE.cast();
                        let mut attrs =
                            attrs.map(|a| a.build());

                        let mut p_device_initializer =
                            accessor.as_non_null().ok_or(STATUS_INVALID_PARAMETER)?.as_ptr();

                        let attrs_ptr =
                            attrs.as_mut().map_or(
                                ptr::null_mut(),
                                ptr::from_mut,
                            );

                        #[cfg(feature = "kmdf-runtime")]
                        // SAFETY: `driver_init` is safe because is passed by WDF
                        unsafe {
                            __cb::wdf_device_create(
                                &raw mut p_device_initializer,
                                attrs_ptr,
                                &raw mut p_device,
                            )
                        }?;

                        let p_device = NonNull::new(
                            p_device,
                        )
                        .ok_or(STATUS_INTERNAL_ERROR)?;

                        // If CtxDescriptor exists, allocate and initialize
                        D::initialize(p_device.cast());
                        Ok(Self::new(p_device))
                    }
                }

                impl<'a> Handle<'a, WDFDEVICE__> {
                    /// Creates a new instance of the object from an owned `PWDFDEVICE_INIT` with optional attributes.
                    ///
                    /// This method initializes the object using a provided `PWDFDEVICE_INIT` that is owned by the caller
                    /// and applies optional object attributes if specified. It delegates the actual creation to the method
                    /// `from_owned_with_attrs`.
                    ///
                    /// # Type Parameters
                    ///
                    /// - [`D`]: A type that implements `AsCtxDescriptor`. This is used to describe the context associated
                    ///   with the object being created.
                    ///
                    /// # Parameters
                    ///
                    /// - [`owner`]: A `PWDFDEVICE_INIT` structure that must be owned by the caller. This structure is utilized
                    ///   for the initialization process of the object.
                    /// - [`attrs`]: An optional `WdfObjAttrs` descriptor that configures the attributes of the object. Pass
                    ///   `None` if no additional attributes are required.
                    ///
                    /// # Returns
                    ///
                    /// Returns an [`NtResult`] containing the newly created object wrapped in `Self` on success. If an
                    /// error occurs during initialization, the result contains the corresponding [`NTSTATUS`] failure code.
                    ///
                    /// # Examples
                    ///
                    /// ```rust
                    /// let device_init: PWDFDEVICE_INIT = ...;
                    /// let attrs = Some(WdfObjAttrs::new(...));
                    ///
                    /// let result = MyObject::from_owned(device_init, attrs);
                    /// match result {
                    ///     Ok(obj) => {
                    ///         // Successfully created the object
                    ///     }
                    ///     Err(status) => {
                    ///         // Handle the error
                    ///     }
                    /// }
                    /// ```
                    ///
                    /// # Notes
                    ///
                    /// - This method is typically used when the caller explicitly owns the [`PWDFDEVICE_INIT`] handle,
                    ///   ensuring its proper management during the creation process.
                    /// - If additional context or attributes need to be applied beyond the basics, use the [`attrs`] parameter
                    ///   to specify them.
                    #[inline]
                    pub fn allocate<D>(
                        owner: Handle<WDFDEVICE_INIT>,
                        attrs: Option<WdfObjAttrs<D>>,
                    ) -> NtResult<Self>
                    where
                        D: AsCtxDescriptor,
                    {
                        Self::from_kernel_explicit::<D>(
                            owner, None, attrs,
                        )
                    }

                    #[inline]
                    pub fn from_queue(
                        queue: NonNull<WDFQUEUE__>,
                    ) -> Option<Self> {
                        #[cfg(feature = "kmdf-runtime")]
                        let device = NonNull::new(
                            unsafe {
                                __cb::wdf_io_queue_get_device(
                                    queue.as_ptr(),
                                )
                            },
                        )?;
                        #[cfg(feature = "test-runtime")]
                        let device = NonNull::dangling();

                        Some(Self::new(device))
                    }
                }
                impl<'a> Handle<'a, WDFDEVICE__> {
                    pub fn get_io_target(
                        self,
                    ) -> Option<Handle<'a, WDFIOTARGET__>>
                    {
                        Handle::from_device(self)
                    }
                }
            }
            mod driver {
                use core::ptr;
                use core::ptr::NonNull;

                use wdk_sys::{
                    DRIVER_OBJECT, STATUS_INTERNAL_ERROR,
                    WDFDRIVER__,
                };

                use crate::bd::{
                    WdfDriverConf, WdfObjAttrs,
                };
                use crate::op::marks::IsWdfType;
                use crate::op::{
                    AsBuilder, AsCtxDescriptor, AsNonNull,
                    FromKernel, FromKernelWithConfAndAttrs,
                    NtResult,
                };
                #[cfg(feature = "kmdf-runtime")]
                use crate::rt::__cb;
                #[cfg(feature = "test-runtime")]
                use crate::rt::wdk_sys;
                use crate::rt::wdk_sys::{
                    STATUS_INVALID_PARAMETER,
                    WDF_NO_HANDLE, WDFDRIVER,
                };
                use crate::wdf::handle::private::Handle;

                impl IsWdfType for DRIVER_OBJECT {}
                impl<'a> FromKernel<WDFDRIVER__>
                    for Handle<'a, WDFDRIVER__>
                {
                    type Accessor = DRIVER_OBJECT;
                    type Conf = WdfDriverConf;

                    fn from_kernel_explicit<D>(
                        accessor: Handle<Self::Accessor>,
                        conf: Option<Self::Conf>,
                        attrs: Option<WdfObjAttrs<D>>,
                    ) -> NtResult<Self>
                    where
                        D: AsCtxDescriptor,
                    {
                        let mut driver: WDFDRIVER =
                            WDF_NO_HANDLE.cast();
                        let mut attrs =
                            attrs.map(|a| a.build());
                        let conf = conf.ok_or({
                            STATUS_INVALID_PARAMETER
                        })?;
                        let mut config = conf.build();

                        let p_driver_obj = accessor
                            .as_non_null()
                            .ok_or(
                                STATUS_INVALID_PARAMETER,
                            )?
                            .as_ptr();

                        let attrs_ptr =
                            attrs.as_mut().map_or(
                                ptr::null_mut(),
                                ptr::from_mut,
                            );

                        let registry_path =
                            conf.registry_path;
                        let config_ptr = &raw mut config;
                        let driver_ptr = &raw mut driver;

                        #[cfg(feature = "kmdf-runtime")]
                        // SAFETY: `driver_init` is safe because is passed by WDF
                        unsafe {
                            __cb::wdf_driver_create(
                                p_driver_obj,
                                registry_path,
                                attrs_ptr,
                                config_ptr,
                                driver_ptr,
                            )
                        }?;

                        Ok(Self::new(
                            NonNull::new(driver).ok_or(
                                STATUS_INTERNAL_ERROR,
                            )?,
                        ))
                    }
                }

                impl<'a>
                    FromKernelWithConfAndAttrs<WDFDRIVER__>
                    for Handle<'a, WDFDRIVER__>
                {
                }
            }
            mod io_target {
                #[cfg(feature = "kmdf-runtime")]
                use core::ptr;
                use core::ptr::NonNull;

                use wdk_sys::{
                    WDFDEVICE__, WDFIOTARGET__,
                    WDFREQUEST__,
                };

                use crate::bd::{
                    WdfObjAttrs, WdfRequestSendOption,
                };
                use crate::ioctl::IoBuffer;
                use crate::ioctl::commands::IoCtlCommand;
                use crate::op::AsNonNull;
                #[cfg(feature = "kmdf-runtime")]
                use crate::op::{
                    AsBuilder, AsBuilderMut,
                    AsCtxDescriptor, FromKernel, NtResult,
                    ToNonNull,
                };
                #[cfg(feature = "kmdf-runtime")]
                use crate::rt::__cb;
                #[cfg(feature = "test-runtime")]
                use crate::rt::wdk_sys;
                use crate::rt::wdk_sys::{
                    STATUS_INVALID_PARAMETER, ULONG_PTR,
                    WDF_NO_HANDLE, WDFIOTARGET,
                };
                #[cfg(feature = "kmdf-runtime")]
                use crate::runtime::kmdf::{
                    wdf_target_io_get_state,
                    wdf_target_io_send_ioctl_sync,
                };
                #[cfg(feature = "kmdf-runtime")]
                use crate::vals::WdfIoTargetError::IoCtlTargetSendError;
                use crate::vals::WdfIoTargetError::{
                    DeviceHasNoIoTarget, IllegalState,
                };
                use crate::vals::{
                    IoCtlTargetSendInfo, WdfIoTargetError,
                    WdfIoTargetState,
                };
                use crate::wdf::handle::private::Handle;

                impl<'a> Handle<'a, WDFIOTARGET__> {
                    pub fn from_device(
                        owner: Handle<WDFDEVICE__>,
                    ) -> Option<Self> {
                        #[cfg(feature = "kmdf-runtime")]
                        {
                            let device =
                                owner.to_non_null()?;
                            let io_target =
                                NonNull::new(unsafe {
                                    __cb::wdf_target_io_get(
                                        device.as_ptr(),
                                    )
                                })?;

                            Some(Self::new(io_target))
                        }
                        #[cfg(feature = "test-runtime")]
                        {
                            unimplemented!(
                                "Cannot create WDFIOTARGET handle in test-runtime"
                            )
                        }
                    }
                }

                #[cfg(feature = "kmdf-runtime")]
                impl<'a> FromKernel<WDFIOTARGET__>
                    for Handle<'a, WDFIOTARGET__>
                {
                    type Accessor = WDFDEVICE__;
                    type Conf = ();

                    #[cfg_attr(
                        feature = "test-runtime",
                        expect(
                            unused_variables,
                            reason = "Unused because of test-runtime"
                        )
                    )]
                    fn from_kernel_explicit<D>(
                        accessor: Handle<Self::Accessor>,
                        _: Option<Self::Conf>,
                        attrs: Option<WdfObjAttrs<D>>,
                    ) -> NtResult<Self>
                    where
                        D: AsCtxDescriptor,
                    {
                        let mut io_target: WDFIOTARGET =
                            WDF_NO_HANDLE.cast();
                        let mut attrs =
                            attrs.map(|a| a.build());

                        let device_ptr = accessor
                            .as_non_null()
                            .ok_or(
                                STATUS_INVALID_PARAMETER,
                            )?
                            .as_ptr();

                        let attrs_ptr =
                            attrs.as_mut().map_or(
                                ptr::null_mut(),
                                ptr::from_mut,
                            );

                        #[cfg(feature = "kmdf-runtime")]
                        // SAFETY: `device_ptr` is safe because is passed by WDF
                        unsafe {
                            __cb::wdf_target_io_create(
                                device_ptr,
                                attrs_ptr,
                                &raw mut io_target,
                            )
                        }?;

                        Ok(Self::new(
                            NonNull::new(io_target).ok_or(
                                STATUS_INVALID_PARAMETER,
                            )?,
                        ))
                    }
                }
                impl<'a> Handle<'a, WDFIOTARGET__> {
                    /// Reads and returns the status of the I/O target.
                    ///
                    /// This method determines the current state of the I/O target and returns
                    /// it as a `WdfIoTargetState` enum. The returned state depends on the
                    /// active runtime feature.
                    ///
                    /// # Runtime Features
                    /// - **kmdf-runtime**:
                    ///   - If this feature is enabled, the state is retrieved by calling the
                    ///     underlying KMDF-specific function `wdf_get_targetio_state` on the
                    ///     raw handle returned by the `self.raw()` method.
                    ///   - Note: The call to `wdf_get_targetio_state` is marked `unsafe`
                    ///     because it interacts directly with a lower-level API.
                    /// - **test-runtime**:
                    ///   - If this feature is enabled, the method will always return
                    ///     `WdfIoTargetState::Started`, simulating a consistent state for
                    ///     testing purposes.
                    ///
                    /// # Returns
                    /// A `WdfIoTargetState` indicating the current status of the I/O target.
                    ///
                    /// # Safety
                    /// When the `kmdf-runtime` feature is enabled, this method uses an
                    /// `unsafe` block to call `wdf_get_targetio_state`. Ensure that the
                    /// environment and underlying abstractions are correctly initialized and
                    /// managed to avoid undefined behavior.
                    #[inline]
                    pub fn read_status(
                        &self,
                    ) -> Option<WdfIoTargetState>
                    {
                        #[cfg(feature = "kmdf-runtime")]
                        {
                            Some(WdfIoTargetState::from(
                                unsafe {
                                    wdf_target_io_get_state(
                                    self.as_non_null()?
                                        .as_ptr()
                                )
                                },
                            ))
                        }
                        #[cfg(feature = "test-runtime")]
                        {
                            WdfIoTargetState::Started
                        }
                    }

                    /// Sends an IOCTL (Input/Output Control) request to the device and waits for a response.
                    ///
                    /// This function is designed to send IOCTL requests when the device is in a
                    /// "Started" state. It constructs the input and output buffers for the IOCTL operation
                    /// and interacts with the underlying KMDF runtime (if enabled) using the appropriate
                    /// APIs. It returns the response obtained from the device if the operation is successful
                    /// or an error if the operation fails.
                    ///
                    /// # Type Parameters
                    /// * [`R`]: The type of the expected response from the IOCTL operation. Must implement [`Default`].
                    ///
                    /// # Parameters
                    /// * [`request`]: An [`IoCtlRequest`] containing the IOCTL command and optional data to send to the device.
                    ///
                    /// # Returns
                    /// * [`Ok(IoCtlResponse<R>)`]: On success, returns a populated [`IoBuffer`] of type [`R`] which holds
                    ///   the response to the request.
                    /// * [`Err(WdfIoTargetError)`]: On failure, returns a [`WdfIoTargetError`] describing the nature of the error.
                    ///
                    /// # Errors
                    /// * This function will return:
                    ///   * [`WdfIoTargetState::IllegalState`]: If the device is not in a "Started" state.
                    ///   * [`WdfIoTargetState::IoCtlTargetSendError`]: If sending the IOCTL request failed, providing detailed information
                    ///     such as the command, NTSTATUS code, any optional input buffer, and bytes returned.
                    ///
                    /// # Key Operations
                    /// 1. Validates that the device is in the "Started" state.
                    /// 2. Builds the input and output buffers for the IOCTL operation.
                    /// 3. Interacts with the device driver stack (e.g., via [`WdfIoTargetSendIoctlSynchronously`] in KMDF mode).
                    /// 4. Processes the response or handles any errors arising from the operation.
                    ///
                    /// # Safety
                    /// * Unsafe blocks are used for interacting with the KMDF runtime. These blocks ensure that the
                    ///   required pointers for buffers are valid and the KMDF callback for sending IOCTL requests correctly
                    ///   handles the data. Developer attention is required when working in "kmdf-runtime" mode.
                    ///
                    /// # Features
                    /// This function includes conditional compilation specifically for the `test-runtime` feature:
                    /// * Suppresses warnings for unused variables where certain parameters are not required during
                    ///   testing scenarios.
                    ///
                    /// # Example
                    /// ```rust
                    /// // Example usage:
                    /// let ioctl_request = IoCtlRequest::new(REQUEST_CODE, Some(input_data));
                    /// let ioctl_response: Result<IoCtlResponse<ResponseData>, WdfIoTargetError> = device.send_ioctl(ioctl_request);
                    ///
                    /// match ioctl_response {
                    ///     Ok(response) => println!("Response received: {:?}", response.data()),
                    ///     Err(e) => eprintln!("Failed to send IOCTL request: {:?}", e),
                    /// }
                    /// ```
                    ///
                    /// # Dependencies
                    /// This function relies on the following external components:
                    /// * [`IoCtlRequest`]: Represents the IOCTL request, including the command and optional data.
                    /// * [`IoBuffer`]: Represents the response from the IOCTL request.
                    /// * [`WdfIoTargetError`]: Error types related to WDF IO targets.
                    /// * KMDF APIs (if the ` kmdf-runtime ` feature is enabled).
                    ///
                    /// # Notes
                    /// * Ensure you manage the lifetime and validity of `self` and buffers when interacting
                    ///   with the KMDF runtime, as improper handling can lead to undefined behavior in unsafe blocks.
                    pub fn send_ioctl_sync<
                        R: Default + Copy,
                    >(
                        &self,
                        io_request: IoCtlCommand,
                        input: Option<IoBuffer<R>>,
                        //wdf_request: Option<HandleRef<WDFREQUEST__>>
                    ) -> Result<IoBuffer<R>, WdfIoTargetError>
                    {
                        // if device is not started, return error
                        let device_status =
                            self.read_status().ok_or(DeviceHasNoIoTarget)?;
                        let WdfIoTargetState::Started =
                            device_status
                        else {
                            return Err(IllegalState(
                                device_status,
                            ));
                        };

                        // build input ioctl output buffer only if request has data to send
                        let mut request_desc =
                            input.map(|data| data.build());

                        let request_desc_ptr =
                            request_desc.as_mut().map_or(
                                ptr::null_mut(),
                                ptr::from_mut,
                            );

                        #[cfg_attr(
                            feature = "test-runtime",
                            expect(
                                unused_variables,
                                reason = "Unused because of test-runtime"
                            )
                        )]
                        // build Ioctl output buffer ptr
                        let mut response =
                            IoBuffer::<R>::default();

                        #[cfg_attr(
                            feature = "test-runtime",
                            expect(
                                unused_mut,
                                unused_variables,
                                reason = "Unused because of test-runtime"
                            )
                        )]
                        let mut response_desc =
                            response.build_mut();

                        #[cfg_attr(
                            feature = "test-runtime",
                            expect(unused_mut,
                                unused_variables,
                                reason = "Unused because of test-runtime")
                        )] let mut bytes_returned: ULONG_PTR = 0;

                        #[cfg(feature = "kmdf-runtime")]
                        // SAFETY: Is iot safe since output_desc and bytes_returned are valid pointers
                        //         and `WdfIoTargetSendIoctlSynchronously` KMDF callback handles it correctly
                        unsafe {
                            // Retrieve the general collection info (including the required preparsed descriptor size)
                            wdf_target_io_send_ioctl_sync(
                                self.as_non_null()
                                    .ok_or(
                                        DeviceHasNoIoTarget,
                                    )?
                                    .as_ptr(),
                                io_request,
                                ptr::null_mut(),
                                request_desc_ptr,
                                &raw mut response_desc,
                                ptr::null_mut(),
                                &raw mut bytes_returned,
                            )
                        }
                        .map_err(
                            |ntstatus| {
                                IoCtlTargetSendError(
                                    IoCtlTargetSendInfo {
                                        command: io_request,
                                        ntstatus,
                                        byte_returned:
                                            bytes_returned
                                                as usize,
                                    },
                                )
                            },
                        )?;

                        Ok(response)
                    }

                    pub fn send_wdf(
                        &self,
                        req: NonNull<WDFREQUEST__>,
                        options: WdfRequestSendOption,
                    ) -> Option<()> {
                        let mut conf = options.build();

                        #[cfg(feature = "kmdf-runtime")]
                        unsafe {
                            __cb::wdf_request_send(
                                self.as_non_null()?
                                    .as_ptr(),
                                &raw mut conf,
                                req.as_ptr(),
                            )
                            .then_some(())
                        }

                        #[cfg(feature = "test-runtime")]
                        Some(())
                    }
                }
            }
            mod queue {
                use core::ptr;
                use core::ptr::NonNull;

                use wdk_sys::{
                    STATUS_INTERNAL_ERROR, WDFDEVICE__,
                    WDFQUEUE__,
                };

                use crate::bd::{
                    WdfIoQueueConfig, WdfObjAttrs,
                };
                use crate::op::{
                    AsBuilder, AsCtxDescriptor, AsNonNull,
                    FromKernel, FromKernelWithConfAndAttrs,
                    NtResult,
                };
                #[cfg(feature = "kmdf-runtime")]
                use crate::rt::__cb;
                #[cfg(feature = "test-runtime")]
                use crate::rt::wdk_sys;
                use crate::rt::wdk_sys::{
                    STATUS_INVALID_PARAMETER, WDF_NO_HANDLE,
                };
                use crate::wdf::handle::private::Handle;

                impl<'a> FromKernel<WDFQUEUE__> for Handle<'a, WDFQUEUE__> {
                    type Accessor = WDFDEVICE__;
                    type Conf = WdfIoQueueConfig;

                    fn from_kernel_explicit<D>(
                        accessor: Handle<Self::Accessor>,
                        conf: Option<Self::Conf>,
                        attrs: Option<WdfObjAttrs<D>>,
                    ) -> NtResult<Self>
                    where
                        D: AsCtxDescriptor,
                    {
                        let mut queue =
                            WDF_NO_HANDLE.cast();
                        let device = accessor
                            .as_non_null()
                            .ok_or(
                                STATUS_INVALID_PARAMETER,
                            )?
                            .as_ptr();

                        let mut config =
                            conf.map(|c| c.build()).ok_or(
                                STATUS_INVALID_PARAMETER,
                            )?;
                        let mut attrs =
                            attrs.map(|a| a.build());

                        let attrs_ptr =
                            attrs.as_mut().map_or(
                                ptr::null_mut(),
                                ptr::from_mut,
                            );

                        #[cfg(feature = "kmdf-runtime")]
                        // SAFETY: `device` is safe because cannot be null
                        //          and is passed directly from wdf
                        unsafe {
                            __cb::wdf_io_queue_create(
                                device,
                                &raw mut config,
                                attrs_ptr,
                                &raw mut queue,
                            )
                        }?;

                        Ok(Self::new(
                            NonNull::new(queue).ok_or(
                                STATUS_INTERNAL_ERROR,
                            )?,
                        ))
                    }
                }
                impl<'a>
                    FromKernelWithConfAndAttrs<WDFQUEUE__>
                    for Handle<'a, WDFQUEUE__>
                {
                }
                impl<'a> Handle<'a, WDFQUEUE__> {
                    pub fn get_device(
                        &self,
                    ) -> Option<Handle<'a, WDFDEVICE__>>
                    {
                        Handle::<WDFDEVICE__>::from_queue(
                            NonNull::new(
                                self.as_non_null()?
                                    .as_ptr(),
                            )?,
                        )
                    }
                }
            }
            mod w_request {
                use wdk_sys::{
                    STATUS_INVALID_PARAMETER, WDFREQUEST__,
                };

                use crate::op::{AsNonNull, NtResult};
                #[cfg(feature = "kmdf-runtime")]
                use crate::rt::__cb;
                #[cfg(feature = "test-runtime")]
                use crate::rt::wdk_sys;
                use crate::rt::wdk_sys::NTSTATUS;
                use crate::wdf::handle::private::Handle;

                impl<'a> Handle<'a, WDFREQUEST__> {
                    pub fn complete(
                        self,
                        status: NTSTATUS,
                    ) -> Option<Self> {
                        #[cfg(feature = "kmdf-runtime")]
                        unsafe {
                            __cb::wdf_request_complete(
                                self.as_non_null()?
                                    .as_ptr(),
                                status,
                            )
                        };

                        Some(self)
                    }

                    pub fn format_using_current_type(
                        &mut self,
                    ) -> NtResult<&Self>
                    {
                        #[cfg(feature = "kmdf-runtime")]
                        unsafe {
                            __cb::wdf_request_format_using_current_type(
                                self.as_non_null().ok_or(STATUS_INVALID_PARAMETER)?.as_ptr(),
                            )
                        };

                        Ok(self)
                    }
                }
            }
        }
    }
}

#[allow(unused)]
pub use private::{Handle, HandleMut};
