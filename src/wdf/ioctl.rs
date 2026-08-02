mod private {
    use crate::ioctl::private::commands::IoCtlCommand;

    pub mod commands {

        pub type IoCtlCommand = u32;

        #[allow(dead_code)]
        pub const IOCTL_HID_GET_COLLECTION_INFORMATION: IoCtlCommand =
            0x000B_01A8; // Method Neither (106)

        #[allow(dead_code)]
        pub const IOCTL_HID_GET_PREPARSED_DATA: IoCtlCommand = 0x000B_001C; // Method Buffered (7)
    }
    pub mod operations {
        use core::ptr;
        use crate::rt::wdk_sys::{
            _WDF_MEMORY_DESCRIPTOR__bindgen_ty_1,
            _WDF_MEMORY_DESCRIPTOR__bindgen_ty_1__bindgen_ty_1,
            WDF_MEMORY_DESCRIPTOR, _WDF_MEMORY_DESCRIPTOR_TYPE,
        };
        use crate::size_to_ulong;

        #[allow(dead_code)]
        pub fn build_for_data_type<T>(elem: &T) -> WDF_MEMORY_DESCRIPTOR {
            WDF_MEMORY_DESCRIPTOR {
                Type:
                    _WDF_MEMORY_DESCRIPTOR_TYPE::WdfMemoryDescriptorTypeBuffer,
                u: _WDF_MEMORY_DESCRIPTOR__bindgen_ty_1 {
                    BufferType:
                        _WDF_MEMORY_DESCRIPTOR__bindgen_ty_1__bindgen_ty_1 {
                            Buffer: ptr::from_ref(elem).cast_mut().cast(),
                            Length: size_to_ulong!(size_of::<T>()),
                        },
                },
            }
        }

        pub fn build_for_data_type_mut<T>(
            elem: &mut T,
        ) -> WDF_MEMORY_DESCRIPTOR {
            WDF_MEMORY_DESCRIPTOR {
                Type:
                    _WDF_MEMORY_DESCRIPTOR_TYPE::WdfMemoryDescriptorTypeBuffer,
                u: _WDF_MEMORY_DESCRIPTOR__bindgen_ty_1 {
                    BufferType:
                        _WDF_MEMORY_DESCRIPTOR__bindgen_ty_1__bindgen_ty_1 {
                            Buffer: ptr::from_mut(elem).cast(),
                            Length: size_to_ulong!(size_of::<T>()),
                        },
                },
            }
        }
    }

    /// A data type that is able to represent a fillable IOCTL WDF Response
    ///
    /// # Type Parameters
    /// - [`T`]: The type of the response payload, which must implement the [`Default`] trait.
    ///
    /// # Usage
    /// This struct is typically used to encapsulate the response data returned from ioctl operations
    /// in a structured and type-safe manner. The wrapped type `T` must have a `Default` implementation
    /// to ensure it can be initialized with default values.
    pub struct IoBuffer<T: Default>(T);

    /// A structure representing an I/O control (ioctl) request.
    ///
    /// `IoCtlRequest` encapsulates an ioctl operation, which consists of a command
    /// and associated data.
    ///
    /// # Type Parameters
    /// - [`T`]: The type of the data associated with the ioctl command.
    ///
    /// # Examples
    /// ```rust
    /// // Example usage of IoCtlRequest
    /// let command = IoCtlCommand::SomeCommand;
    /// let data = SomeDataStruct { value: 42 };
    /// let request = IoCtlRequest(command, data);
    /// // Now `request` can be passed to an ioctl handler.
    /// ```
    ///
    /// This structure provides a lightweight, type-safe way to encapsulate ioctl
    /// requests.
    pub struct IoCtlRequest<T>(IoCtlCommand, T);

    mod buff_io_impls {
        use crate::ioctl::private::{operations, IoBuffer};
        use crate::op::{AsBuff, AsBuilder, AsBuilderMut, AsNonNull, AsNonNullBuff, AsRawBuff, ToNonNull};
        use core::ops::Deref;
        use crate::rt::wdk_sys::WDF_MEMORY_DESCRIPTOR;

        impl<T: Default> IoBuffer<T> {
            #[allow(dead_code)]
            pub fn new(data: T) -> Self {
                Self(data)
            }

            #[allow(dead_code)]
            pub fn into_inner(self) -> T {
                self.0
            }
        }

        impl<T: Default> ToNonNull<T> for IoBuffer<T> {}

        impl<T: Default+Copy> AsNonNull<T> for IoBuffer<T> {}

        impl<T: Default+Copy> AsNonNullBuff<T> for IoBuffer<T> {}

        impl<T: Default> AsRef<T> for IoBuffer<T> {
            fn as_ref(&self) -> &T {
                &self.0
            }
        }

        impl<T: Default> AsRawBuff<T> for IoBuffer<T> {}
        impl<T: Default> AsBuilder for IoBuffer<T> {
            type Descriptor<'a>
                = WDF_MEMORY_DESCRIPTOR
            where
                Self: 'a;

            #[inline]
            fn build(&self) -> Self::Descriptor<'_> {
                operations::build_for_data_type(self.as_ref())
            }
        }
        impl<T: Default> Default for IoBuffer<T> {
            fn default() -> Self {
                Self(T::default())
            }
        }
        impl<T: Default> AsMut<T> for IoBuffer<T> {
            fn as_mut(&mut self) -> &mut T {
                &mut self.0
            }
        }
        impl<T: Default> AsBuilderMut for IoBuffer<T> {
            #[inline]
            fn build_mut(&mut self) -> Self::Descriptor<'_> {
                operations::build_for_data_type(self.as_mut())
            }
        }
        impl<T: Default> Deref for IoBuffer<T> {
            type Target = T;

            fn deref(&self) -> &Self::Target {
                self.as_ref()
            }
        }

    }
    mod _ioctl_req_impls {
        use crate::ioctl::private::commands::IoCtlCommand;
        use crate::ioctl::private::{operations, IoCtlRequest};
        use crate::op::{AsBuilder};
        use crate::rt::wdk_sys::WDF_MEMORY_DESCRIPTOR;

        #[allow(dead_code)]
        impl<T> IoCtlRequest<Option<T>> {
            pub fn new(command: IoCtlCommand, request: T) -> Self {
                Self(command, Some(request))
            }

            pub fn with_command(command: IoCtlCommand) -> Self {
                Self(command, None)
            }

            pub fn command(&self) -> IoCtlCommand {
                self.0
            }
        }

        /// An IOCTL request from an IO target buffer
        impl<T> AsRef<Option<T>> for IoCtlRequest<Option<T>> {
            fn as_ref(&self) -> &Option<T> {
                &self.1
            }
        }
        impl<T> AsBuilder for IoCtlRequest<Option<T>> {
            type Descriptor<'b>
                = Option<WDF_MEMORY_DESCRIPTOR>
            where
                Self: 'b;

            fn build(&self) -> Self::Descriptor<'_> {
                self.as_ref().as_ref().map(operations::build_for_data_type)
            }
        }
    }
}
pub use private::commands;
pub use private::{IoCtlRequest, IoBuffer};
