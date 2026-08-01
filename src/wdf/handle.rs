mod private {
    use core::ptr::NonNull;

    use crate::rt::wdk_sys::HANDLE;

    /// Encapsulates a kernel object or resource [`HANDLE`] of type `H`.
    ///
    /// # About [`Handle`]
    /// [`HANDLE`] is one of the core concepts in the Windows Driver Framework (WDF) and represents
    /// a handle to a kernel resource or object.
    /// Handles are used extensively in WDF to manage various system resources,
    /// such as device objects, file objects, and other kernel objects. This is the Rust version.
    /// There are 3 common forms of `Handle<T>`, exposed through 3 main aliases of [`Handle`]:
    /// - [`Handle`]: Simply a `Handle<T>`. Typically a raw pointer to a kernel object.
    /// - [`HandleRef`]: `Handle<&'a T>`, typically a reference to a raw pointer that points to a kernel object.
    /// - [`HandleMut`]: `Handle<&'a mut T>`, typically a mutable reference to a raw pointer that points to a kernel object.
    ///
    /// # Type Parameters
    /// - `H`: The handle type being wrapped. Defaults to `HANDLE` if not explicitly specified.
    ///
    /// # Traits
    /// - `Clone`: Allows `Handle` to be cloned, creating a duplicate with the same underlying handle.
    /// - `Debug`: Enables formatting of `Handle` for debugging purposes.
    ///
    /// # Example
    /// `Handle` is the basic building block of `swdk`.
    /// You can use it to implement your own functions for raw WDF kernel types.
    /// For example, the `swdk` library implements [`Handle::read_status()`] for [`WDFIOTARGET`]
    /// exactly in this way:
    /// ```rust
    /// impl swdk::Handle<swdk::rt::wdk_sys::WDFIOTARGET> {
    ///     pub fn read_status(&self) -> swdk::val::WdfIoTargetState {
    ///         swdk::val::WdfIoTargetState::from(unsafe {
    ///             ...
    /// ```
    ///
    /// `swdk` is still in development, anyway, you can already declare a full Rust driver
    /// in just a few lines using `Handle`:
    /// ```rust
    /// use swdk::rt::wdk_sys::{WDFDEVICE, WDFDRIVER, PWDFDEVICE_INIT, STATUS_SUCCESS};
    /// use swdk::Handle;
    /// use swdk::unwrap_nt;
    /// use swdk::bd::{WdfDriverConf, WdfDriverSetup, WdfObjAttrs};
    /// use swdk::println;
    ///
    /// type HandleDevice = Handle<WDFDEVICE>;
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
    pub struct Handle<H = HANDLE>(NonNull<H>);

    mod impls {
        use core::ops::Deref;
        use core::ptr::NonNull;

        use wdk_sys::HANDLE;

        use crate::Handle;
        use crate::op::{
            AsKernelType, AsPtr, AsRaw, AsWdfHandle,
            IntoInner, IntoRaw,
        };

        impl<H> Handle<H> {
            pub fn new(raw: NonNull<H>) -> Self {
                Self(raw)
            }
        }

        impl<H> Handle<H> {
            pub fn non_null(&self) -> NonNull<H> {
                self.0
            }
        }

        impl<H: Copy> AsWdfHandle<H> for Handle<H> {
            fn as_wdf_handle(&self) -> NonNull<HANDLE> {
                self.0.cast()
            }
        }
        impl<H: Copy> AsKernelType<H> for Handle<H> {}
        impl<H: Copy> IntoInner<H> for Handle<H> {
            fn into_inner(self) -> H {
                unsafe { *self.0.as_ptr() }
            }
        }
        impl<H> Deref for Handle<H> {
            type Target = H;
            fn deref(&self) -> &Self::Target {
                unsafe { self.0.as_ref() }
            }
        }
        impl<H> AsRef<H> for Handle<H> {
            fn as_ref(&self) -> &H {
                unsafe { self.0.as_ref() }
            }
        }
        impl<H> AsPtr<H> for Handle<H> {}
        unsafe impl<H> Send for Handle<H> {}
        unsafe impl<H> Sync for Handle<H> {}
        impl<H: Copy> IntoRaw<H> for Handle<H> {}
        impl<H: Copy> AsRaw<H> for Handle<H> {}

        #[cfg(feature = "minimal-runtime")]
        mod kmdf {
            mod object {
                use core::ptr;
                use core::ptr::NonNull;

                use wdk_sys::STATUS_INTERNAL_ERROR;

                use crate::Handle;
                use crate::bd::WdfObjAttrs;
                use crate::op::{
                    AsBuilder, AsCtxDescriptor, FromKernel,
                    NtResult,
                };
                #[cfg(feature = "kmdf-runtime")]
                use crate::rt::__cb;
                use crate::rt::wdk_sys::{
                    WDF_NO_HANDLE, WDFOBJECT,
                };

                impl Handle<WDFOBJECT> {
                    pub fn allocate(
                        attrs: Option<WdfObjAttrs>,
                    ) -> NtResult<Self>
                    {
                        Self::from_kernel_explicit(
                            NonNull::dangling(),
                            None,
                            attrs,
                        )
                    }
                }

                impl FromKernel<WDFOBJECT> for Handle<WDFOBJECT> {
                    type Accessor = ();
                    type Conf = ();

                    fn from_kernel_explicit<D>(
                        _: NonNull<Self::Accessor>,
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

                use wdk_sys::WDFDEVICE_INIT;

                use crate::Handle;
                use crate::bd::WdfDevicePnpPowerSetup;
                use crate::op::{AsBuilder, AsPtr};
                #[cfg(feature = "kmdf-runtime")]
                use crate::rt::__cb;

                impl Handle<WDFDEVICE_INIT> {
                    #[inline]
                    #[cfg(feature = "kmdf-runtime")]
                    pub fn with_filter(self) -> Self {
                        unsafe {
                            __cb::wdf_f_do_init_set_filter(
                                self.0.as_ptr().cast(),
                            )
                        };
                        self
                    }

                    #[inline]
                    #[cfg(feature = "kmdf-runtime")]
                    pub fn with_pnp_setup(
                        self,
                        setup: WdfDevicePnpPowerSetup,
                    ) -> Self {
                        let pnp_setup = setup.build();
                        unsafe {
                            __cb::wdf_device_init_set_pnp_power_event_callbacks(
                                self.as_ptr().cast_mut(),
                                ptr::from_ref(&pnp_setup).cast_mut(),
                            )
                        };
                        self
                    }
                }
            }
            mod device {
                use core::ptr;
                use core::ptr::NonNull;

                use wdk_sys::{
                    STATUS_INTERNAL_ERROR, WDFDEVICE__,
                    WDFDEVICE_INIT, WDFIOTARGET__,
                    WDFQUEUE__,
                };

                use crate::Handle;
                use crate::bd::WdfObjAttrs;
                use crate::op::{
                    AsBuilder, AsCtxDescriptor, AsPtr,
                    FromKernel, NtResult,
                };
                #[cfg(feature = "kmdf-runtime")]
                use crate::rt::__cb;
                use crate::rt::wdk_sys::{
                    WDF_NO_HANDLE, WDFDEVICE,
                };

                impl FromKernel<WDFDEVICE__> for Handle<WDFDEVICE__> {
                    type Accessor = WDFDEVICE_INIT;
                    type Conf = ();

                    fn from_kernel_explicit<D>(
                        owned: NonNull<Self::Accessor>,
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
                            owned.as_ptr();

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

                impl Handle<WDFDEVICE__> {
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
                        owner: NonNull<WDFDEVICE_INIT>,
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
                        let device = NonNull::new(
                            unsafe {
                                __cb::wdf_io_queue_get_device(
                                queue.as_ptr(),
                            )
                            },
                        )?;

                        Some(Self::new(device))
                    }
                }
                impl Handle<WDFDEVICE__> {
                    pub fn get_io_target(
                        &self,
                    ) -> Option<Handle<WDFIOTARGET__>>
                    {
                        Handle::<WDFIOTARGET__>::from_device(
                            NonNull::new(
                                self.as_ptr().cast_mut(),
                            )?,
                        )
                    }
                }
            }
            mod driver {
                use core::ptr;
                use core::ptr::NonNull;

                use wdk_sys::{
                    DRIVER_OBJECT, STATUS_INTERNAL_ERROR,
                    WDFDEVICE__, WDFDRIVER__,
                };

                use crate::Handle;
                use crate::bd::{
                    WdfDriverConf, WdfObjAttrs,
                };
                use crate::op::{AsBuilder, AsCtxDescriptor, FromKernel, FromKernelWithConf, FromKernelWithConfAndAttrs, NtResult};
                #[cfg(feature = "kmdf-runtime")]
                use crate::rt::__cb;
                use crate::rt::wdk_sys::{
                    STATUS_INVALID_PARAMETER,
                    WDF_NO_HANDLE, WDFDRIVER,
                };

                impl FromKernel<WDFDRIVER__> for Handle<WDFDRIVER__> {
                    type Accessor = DRIVER_OBJECT;
                    type Conf = WdfDriverConf;

                    fn from_kernel_explicit<D>(
                        accessor: NonNull<Self::Accessor>,
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

                        let driver_obj_ptr =
                            accessor.as_ptr();

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
                                driver_obj_ptr,
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

                impl FromKernelWithConfAndAttrs<WDFDRIVER__>
                    for Handle<WDFDRIVER__> {}
            }
            mod io_target {
                #[cfg(feature = "kmdf-runtime")]
                use core::ptr;
                use core::ptr::NonNull;

                use wdk_sys::{
                    WDFDEVICE__, WDFIOTARGET__,
                    WDFREQUEST__,
                };

                use crate::Handle;
                use crate::bd::{
                    WdfObjAttrs, WdfRequestSendOption,
                };
                use crate::ioctl::{
                    IoCtlRequest, IoCtlResponse,
                };
                #[cfg(feature = "kmdf-runtime")]
                use crate::op::AsOptionalBuff;
                use crate::op::{
                    AsBuilder, AsBuilderMut,
                    AsCtxDescriptor, AsPtr, FromKernel,
                    NtResult,
                };
                #[cfg(feature = "kmdf-runtime")]
                use crate::rt::__cb;
                use crate::rt::wdk_sys::{
                    STATUS_INVALID_PARAMETER, ULONG_PTR,
                    WDF_NO_HANDLE, WDFIOTARGET,
                };
                #[cfg(feature = "kmdf-runtime")]
                use crate::runtime::kmdf::{
                    wdf_target_io_get_state,
                    wdf_target_io_send_ioctl_sync,
                };
                use crate::runtime::utils::from_option_to_ptr;
                use crate::vals::WdfIoTargetError::IllegalState;
                #[cfg(feature = "kmdf-runtime")]
                use crate::vals::WdfIoTargetError::IoCtlTargetSendError;
                use crate::vals::{
                    IoCtlTargetSendInfo, WdfIoTargetError,
                    WdfIoTargetState,
                };

                impl Handle<WDFIOTARGET__> {
                    pub fn from_device(
                        owner: NonNull<WDFDEVICE__>,
                    ) -> Option<Self> {
                        #[cfg(feature = "kmdf-runtime")]
                        {
                            let device = owner.as_ptr();

                            let io_target =
                                NonNull::new(unsafe {
                                    __cb::wdf_target_io_get(
                                        device,
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
                impl FromKernel<WDFIOTARGET__> for Handle<WDFIOTARGET__> {
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
                        accessor: NonNull<Self::Accessor>,
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

                        let device_ptr = accessor.as_ptr();

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
                impl Handle<WDFIOTARGET__> {
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
                    pub fn read_status(
                        &self,
                    ) -> WdfIoTargetState
                    {
                        #[cfg(feature = "kmdf-runtime")]
                        {
                            WdfIoTargetState::from(unsafe {
                                wdf_target_io_get_state(
                                    self.as_ptr()
                                        .cast_mut(),
                                )
                            })
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
                    /// * [`Ok(IoCtlResponse<R>)`]: On success, returns a populated [`IoCtlResponse`] of type [`R`] which holds
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
                    /// * [`IoCtlResponse`]: Represents the response from the IOCTL request.
                    /// * [`WdfIoTargetError`]: Error types related to WDF IO targets.
                    /// * KMDF APIs (if the ` kmdf-runtime ` feature is enabled).
                    ///
                    /// # Notes
                    /// * Ensure you manage the lifetime and validity of `self` and buffers when interacting
                    ///   with the KMDF runtime, as improper handling can lead to undefined behavior in unsafe blocks.
                    pub fn send_ioctl_sync<R: Default>(
                        &self,
                        request: IoCtlRequest<Option<R>>,
                    ) -> Result<
                        IoCtlResponse<R>,
                        WdfIoTargetError,
                    > {
                        // if device is not started, return error
                        let device_status =
                            self.read_status();
                        let WdfIoTargetState::Started =
                            device_status
                        else {
                            return Err(IllegalState(
                                device_status,
                            ));
                        };

                        // build input ioctl output buffer only if request has data to send
                        let request_desc = request.build();

                        #[cfg_attr(
                            feature = "test-runtime",
                            expect(
                                unused_variables,
                                reason = "Unused because of test-runtime"
                            )
                        )]
                        let request_desc_ptr =
                            from_option_to_ptr(
                                request_desc.as_ref(),
                            );

                        // build Ioctl output buffer ptr
                        let mut response =
                            IoCtlResponse::<R>::default();

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
                                self.as_ptr().cast_mut(),
                                request.command(),
                                ptr::null_mut(),
                                request_desc_ptr.cast_mut(),
                                &raw mut response_desc,
                                ptr::null_mut(),
                                &raw mut bytes_returned,
                            )
                        }.map_err(|ntstatus| {
                            IoCtlTargetSendError(IoCtlTargetSendInfo {
                                command: request.command(),
                                ntstatus,
                                request: request.as_buff().map(|buff| buff.to_vec()).unwrap_or_default(),
                                byte_returned: bytes_returned as usize,
                            })
                        })?;

                        Ok(response)
                    }

                    pub fn send_wdf(
                        &self,
                        req: NonNull<WDFREQUEST__>,
                        options: WdfRequestSendOption,
                    ) -> Option<()> {
                        let mut conf = options.build();

                        unsafe {
                            __cb::wdf_request_send(
                                self.as_ptr().cast_mut(),
                                &raw mut conf,
                                req.as_ptr(),
                            )
                            .then_some(())
                        }
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
                    AsBuilder, AsCtxDescriptor, AsPtr,
                    FromKernel, FromKernelWithConfAndAttrs,
                    NtResult,
                };
                #[cfg(feature = "kmdf-runtime")]
                use crate::rt::__cb;
                use crate::rt::wdk_sys::{
                    STATUS_INVALID_PARAMETER, WDF_NO_HANDLE,
                };
                use crate::wdf::handle::private::Handle;

                impl FromKernel<WDFQUEUE__> for Handle<WDFQUEUE__> {
                    type Accessor = WDFDEVICE__;
                    type Conf = WdfIoQueueConfig;

                    fn from_kernel_explicit<D>(
                        accessor: NonNull<Self::Accessor>,
                        conf: Option<Self::Conf>,
                        attrs: Option<WdfObjAttrs<D>>,
                    ) -> NtResult<Self>
                    where
                        D: AsCtxDescriptor,
                    {
                        let mut queue =
                            WDF_NO_HANDLE.cast();
                        let device = accessor.as_ptr();

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
                impl FromKernelWithConfAndAttrs<WDFQUEUE__>
                    for Handle<WDFQUEUE__>
                {
                }
                impl Handle<WDFQUEUE__> {
                    pub fn get_device(
                        &self,
                    ) -> Option<Handle<WDFDEVICE__>>
                    {
                        Handle::<WDFDEVICE__>::from_queue(
                            NonNull::new(
                                self.as_ptr().cast_mut(),
                            )?,
                        )
                    }
                }
            }
            mod w_request {
                use wdk_sys::WDFREQUEST__;

                use crate::Handle;
                use crate::op::AsPtr;
                #[cfg(feature = "kmdf-runtime")]
                use crate::rt::__cb;
                use crate::rt::wdk_sys::NTSTATUS;

                impl Handle<WDFREQUEST__> {
                    pub fn complete(
                        self,
                        status: NTSTATUS,
                    ) -> Option<Self> {
                        #[cfg(feature = "kmdf-runtime")]
                        unsafe {
                            __cb::wdf_request_complete(
                                self.as_ptr().cast_mut(),
                                status,
                            )
                        };

                        Some(self)
                    }

                    pub fn format_using_current_type(
                        self,
                    ) -> Self {
                        unsafe {
                            __cb::wdf_request_format_using_current_type(
                                self.as_ptr().cast_mut(),
                            )
                        };

                        self
                    }
                }
            }
        }
    }
}

#[allow(unused)]
pub use private::Handle;
