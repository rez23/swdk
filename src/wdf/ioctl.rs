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
        #[cfg(feature = "test-runtime")]
        use crate::rt::test_rt::*;

        use crate::size_to_ulong;
        use core::ptr;
        use wdk_sys::{
            _WDF_MEMORY_DESCRIPTOR__bindgen_ty_1,
            _WDF_MEMORY_DESCRIPTOR__bindgen_ty_1__bindgen_ty_1,
            WDF_MEMORY_DESCRIPTOR, _WDF_MEMORY_DESCRIPTOR_TYPE,
        };

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

    /// IoCtlResponse<T> is a Rust handle to an ioctl response
    /// knows how to describe T as a WDF_MEMORY_DESCRIPTOR
    /// through `Self::build()`
    pub struct IoCtlResponse<T: Default>(T);

    /// An IOCTL request from an IO target buffer
    pub struct IoCtlRequest<T>(IoCtlCommand, T);

    mod _ioctl_resp_impls {
        #[cfg(feature = "test-runtime")]
        use crate::rt::test_rt::*;

        use crate::ioctl::private::{operations, IoCtlResponse};
        use crate::operators::{AsBuff, AsBuilder, AsBuilderMut};
        use core::ops::Deref;
        use wdk_sys::WDF_MEMORY_DESCRIPTOR;

        impl<T: Default> IoCtlResponse<T> {
            #[allow(dead_code)]
            pub fn new(data: T) -> Self {
                Self(data)
            }

            #[allow(dead_code)]
            pub fn into_inner(self) -> T {
                self.0
            }
        }
        impl<T: Default> AsRef<T> for IoCtlResponse<T> {
            fn as_ref(&self) -> &T {
                &self.0
            }
        }
        impl<T: Default> AsBuff<T> for IoCtlResponse<T> {}
        impl<T: Default> AsBuilder for IoCtlResponse<T> {
            type Descriptor<'a>
                = WDF_MEMORY_DESCRIPTOR
            where
                Self: 'a;

            #[inline]
            fn build(&self) -> Self::Descriptor<'_> {
                operations::build_for_data_type(self.as_ref())
            }
        }
        impl<T: Default> Default for IoCtlResponse<T> {
            fn default() -> Self {
                Self(T::default())
            }
        }
        impl<T: Default> AsMut<T> for IoCtlResponse<T> {
            fn as_mut(&mut self) -> &mut T {
                &mut self.0
            }
        }
        impl<T: Default> AsBuilderMut for IoCtlResponse<T> {
            #[inline]
            fn build_mut(&mut self) -> Self::Descriptor<'_> {
                operations::build_for_data_type(self.as_mut())
            }
        }
        impl<T: Default> Deref for IoCtlResponse<T> {
            type Target = T;

            fn deref(&self) -> &Self::Target {
                self.as_ref()
            }
        }
    }
    mod _ioctl_req_impls {
        #[cfg(feature = "test-runtime")]
        use crate::rt::test_rt::*;

        use crate::ioctl::private::commands::IoCtlCommand;
        use crate::ioctl::private::{operations, IoCtlRequest};
        use crate::operators::{AsBuilder, AsOptionalBuff};
        use wdk_sys::WDF_MEMORY_DESCRIPTOR;

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
        impl<T> AsOptionalBuff<T> for IoCtlRequest<Option<T>> {}
    }
}
pub use private::commands;
pub use private::{IoCtlRequest, IoCtlResponse};
