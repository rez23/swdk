mod _concepts {
    use crate::rt::wdk_sys::NTSTATUS;

    pub type NtResult<T = ()> = Result<T, NTSTATUS>;
}

mod _operators {
    use alloc::string::String;
    use alloc::vec::Vec;
    use core::borrow::Borrow;
    use core::ffi::c_void;
    use core::ops::Deref;
    use core::ptr;
    use core::ptr::NonNull;

    use crate::bd::WdfObjAttrs;
    use crate::ctx::WdfCtxNoneDesc;
    use crate::op::_concepts::NtResult;
    use crate::rt::wdk_sys::{
        HANDLE, PCWDF_OBJECT_CONTEXT_TYPE_INFO, WDFOBJECT,
    };
    use crate::{Handle};

    /// A trait providing utility methods for working with raw pointers.
    ///
    /// The `AsPtr` trait extends the `AsRef` trait, allowing types to return raw pointers
    /// to their contained data and providing additional operations to work with these pointers.
    ///
    /// # Safety
    /// Methods in this trait involve the use of raw pointers, which are inherently unsafe in Rust.
    /// Proper care must be taken while dereferencing or accessing data through these pointers to
    /// prevent undefined behavior. Users are responsible for adhering to Rust's borrowing and
    /// ownership rules when using the methods from this trait.
    ///
    /// ## Primary Features
    /// - Convert a reference of the type to a raw pointer.
    /// - Temporary access to the raw pointer with the ability to perform user-defined operations.
    ///
    /// # Examples
    ///
    /// ```rust
    /// struct ExampleType(i32);
    ///
    /// impl AsRef<i32> for ExampleType {
    ///     fn as_ref(&self) -> &i32 {
    ///         &self.0
    ///     }
    /// }
    ///
    /// impl AsPtr<i32> for ExampleType {}
    ///
    /// let example = ExampleType(100);
    ///
    /// unsafe {
    ///     let ptr = example.as_ptr();
    ///     assert_eq!(*ptr, 100);
    /// }
    ///
    /// example.with_ptr(|ptr| {
    ///     // Access raw pointer temporarily
    ///     println!("Pointer value: {:?}", ptr);
    /// });
    /// ```
    pub trait AsPtr<T>: AsRef<T> {
        /// Returns a raw pointer to the value.
        ///
        /// This method converts the reference to a raw pointer of the same type. The pointer
        /// is valid for the lifetime of the original reference, but it comes with no guarantees
        /// regarding safety or ownership handling.
        ///
        /// # Safety
        ///
        /// This method is marked as `unsafe` because raw pointers do not enforce Rust's
        /// borrowing rules, and improper usage can lead to undefined behavior. It is the
        /// caller's responsibility to ensure:
        ///
        /// - The pointer is not dereferenced improperly.
        /// - The pointer does not outlive the lifetime of the referenced value.
        ///
        /// # Example
        ///
        /// ```
        /// # use std::ptr;
        ///
        /// let value = 42;
        /// let raw_ptr = unsafe { value.as_ptr() };
        ///
        /// unsafe {
        ///     assert_eq!(*raw_ptr, 42);
        /// }
        /// ```
        ///
        /// # Inline
        ///
        /// This method is marked `#[inline]` to suggest that the compiler should
        /// consider inlining it, improving performance in hot code paths.
        ///
        /// # Returns
        ///
        /// A raw pointer of type `*const T` to the value.
        ///
        /// # Notes
        ///
        /// This uses `ptr::from_ref()` for pointer conversion, ensuring the proper
        /// creation of a raw pointer from a reference.
        #[inline]
        fn as_ptr(&self) -> *const T {
            ptr::from_ref(self.as_ref())
        }

        /// Provides temporary access to the raw pointer of the contained data and applies a
        /// user-defined function `f` to it.
        ///
        /// # Type Parameters
        /// - `F`: A function or closure that takes a single argument of type `*const T`.
        ///
        /// # Parameters
        /// - `self`: A reference to the current instance of the type.
        /// - `f`: A function or closure to be executed, taking the raw pointer as its input.
        ///
        /// # Safety
        /// This method uses an unsafe block to obtain a raw pointer via `self.as_ptr()`.
        /// The caller must ensure that the provided closure or function does not
        /// dereference the raw pointer in an invalid or unsafe manner. The raw pointer
        /// should not outlive the temporary access provided within this method.
        ///
        /// # Examples
        /// ```rust
        /// let data = YourType::new();
        /// data.with_ptr(|ptr| {
        ///     // Perform operations with the raw pointer `ptr`
        /// });
        /// ```
        ///
        /// This method is marked as `#[inline]` to suggest that the compiler inlines
        /// the function for performance optimization in tight loops or frequently
        /// executed code paths.
        #[inline]
        fn with_ptr<F>(&self, f: F)
                       where
                           F: FnOnce(*const T),
        {
            f(unsafe { self.as_ptr() })
        }
    }

    pub trait AsWdfHandle<H: Copy> {
        fn as_wdf_handle(&self) -> NonNull<HANDLE>;
    }

    /// A trait that extends [`AsPtr`] and [`AsMut`] to provide functionality for obtaining
    /// and using mutable raw pointers to the underlying data.
    ///
    /// This trait defines methods for acquiring mutable raw pointers and performing
    /// operations on them. Since raw pointers bypass Rust's borrow checker, these methods
    /// are inherently unsafe and must be used with caution.
    ///
    /// # Safety
    /// Implementors of this trait must ensure that the returned raw pointers do not violate
    /// Rust's aliasing or memory safety rules. Users of this trait are responsible for
    /// adhering to the safety guarantees described in the documentation for each method.
    ///
    /// # Examples
    /// ```
    /// # use swdk::Handle;
    /// #
    /// struct MyStruct {
    ///     value: i32,
    /// }
    ///
    /// impl AsMut<MyStruct> for MyStruct {
    ///     fn as_mut(&mut self) -> &mut MyStruct {
    ///         &mut self.value
    ///     }
    /// }
    /// impl AsPtr<MyStruct> for MyStruct {}
    /// impl AsPtrMut<MyStruct> for MyStruct {}
    ///
    /// let mut instance = MyStruct { value: 10 };
    ///
    /// // Using `with_ptr_mut` to modify the value.
    /// instance.with_ptr_mut(|ptr| {
    ///     unsafe {
    ///         ptr.value = 42; // Change the value using the raw pointer.
    ///     }
    /// });
    ///
    /// assert_eq!(instance.value, 42);
    /// ```
    pub trait AsPtrMut<T>: AsPtr<T> + AsMut<T> {
        /// Returns a mutable raw pointer to `T`.
        ///
        /// # Safety
        ///
        /// The borrow checker does not track the returned raw pointer.
        ///
        /// The caller must ensure that:
        ///
        /// - The pointer does not outlive `self`;
        /// - No aliasing violations are introduced;
        /// - No other mutable access is performed while the pointer is in use;
        /// - The pointer is not dereferenced after the underlying object becomes invalid.
        ///
        /// Consider using [`with_ptr_mut`] whenever possible, as it confines the
        /// lifetime of the raw pointer to the provided closure.
        #[inline]
        unsafe fn as_ptr_mut(&mut self) -> *mut T {
            ptr::from_mut(self.as_mut())
        }

        /// Provides mutable access to the internal raw pointer by invoking a function `f`
        /// with the pointer as an argument. The raw pointer can be used to directly
        /// manipulate the underlying data.
        ///
        /// # Type Parameters
        /// - `F`: A function or closure that takes a mutable raw pointer (`*mut T`) as an argument
        ///        and performs some operation with it.
        ///
        /// # Parameters
        /// - `f`: The function or closure to invoke with the internal mutable raw pointer.
        ///
        /// # Safety
        /// This function internally calls an unsafe method (`self.as_ptr_mut()`), meaning that
        /// care must be taken to follow all safety requirements when dealing with raw pointers.
        /// The caller of the provided function `f` is responsible for ensuring any accesses
        /// or modifications to the data referenced by the raw pointer are safe.
        ///
        /// # Inline
        /// This method is marked with `#[inline]`, hinting to the compiler to attempt inlining
        /// the function to improve performance when possible.
        ///
        /// # Example
        /// ```
        /// let mut data = YourStruct::new();
        /// data.with_ptr_mut(|ptr| {
        ///     unsafe {
        ///         // Use the raw pointer for some low-level operation.
        ///         *ptr = 42; // Example of modifying the value.
        ///     }
        /// });
        /// ```
        #[inline]
        fn with_ptr_mut<F>(&mut self, f: F)
                           where
                               F: FnOnce(*mut T),
        {
            f(unsafe { self.as_ptr_mut() })
        }
    }

    pub trait AsRawWdf<T: Copy>: AsRef<T> {
        /// Converts the reference of the current object into its raw underlying value.
        ///
        /// # Returns
        ///
        /// A value of type `T` which is a copy of the underlying data referenced by `self`.
        ///
        /// # Example
        ///
        /// ```
        /// let obj = YourType::new();
        /// let raw_value = obj.as_raw();
        /// ```
        ///
        /// This method requires that `self` implements the `AsRef` trait, and it dereferences
        /// the value obtained from `as_ref()` in order to produce the raw value of type `T`.
        fn as_raw(&self) -> T {
            *self.as_ref()
        }
    }

    pub trait AsBuff<T>: AsRef<T> {
        /// Executes a provided closure with a reference to the underlying buffer.
        ///
        /// This method provides read-only access to the internal buffer of the object
        /// by passing a slice of the buffer (`&[u8]`) to the given closure. It is useful
        /// for performing operations on the buffer without directly exposing it.
        ///
        /// # Type Parameters
        /// - `F`: A closure that takes a reference to a slice of bytes (`&[u8]`) as input.
        ///
        /// # Parameters
        /// - `f`: The closure to execute. It receives the internal buffer (`&[u8]`) as its argument.
        ///
        /// # Examples
        /// ```rust
        /// let data = SomeStruct::new(); // Assume `SomeStruct` implements `as_buff()`.
        /// data.with_buff(|buff| {
        ///     println!("{:?}", buff);
        /// });
        /// ```
        ///
        /// # Notes
        /// The buffer is passed as a read-only slice, so the closure cannot modify its contents.
        ///
        /// # Inline
        /// The method is marked as `#[inline]` to suggest that the compiler should inline it
        /// for performance reasons, especially in scenarios where this method is called frequently.
        ///
        /// # Requirements
        /// - `self` must implement the `as_buff()` method, which retrieves the underlying buffer.
        ///
        /// # Safety
        /// Always ensure the buffer is in a valid state before this method is called to avoid undefined behavior.
        #[inline]
        fn with_buff<F>(&self, f: F)
                        where
                            F: FnOnce(&[u8]),
        {
            f(self.as_buff())
        }

        /// Converts the referenced object into a byte slice (`&[u8]`).
        ///
        /// # Safety
        /// This method uses unsafe code to reinterpret the memory of the object as a byte slice.
        /// Ensure that the object referenced by `self` is valid, properly aligned, and appropriately sized
        /// for the type `T` to avoid undefined behavior.
        ///
        /// # Returns
        /// A byte slice (`&[u8]`) representing the memory of the underlying object.
        ///
        /// # Implementation Details
        /// - Uses `core::slice::from_raw_parts` to create the byte slice from a raw pointer.
        /// - The raw pointer is obtained from the reference to `self`, cast to a pointer of `u8`.
        /// - The length of the slice is determined by the size of the type `T`.
        ///
        /// # Examples
        /// ```
        /// struct MyStruct {
        ///     value: u32,
        /// }
        ///
        /// let my_struct = MyStruct { value: 42 };
        /// let bytes = my_struct.as_buff();
        /// assert_eq!(bytes.len(), std::mem::size_of::<MyStruct>());
        /// ```
        #[inline]
        fn as_buff(&self) -> &[u8] {
            unsafe {
                core::slice::from_raw_parts(
                    ptr::from_ref(self.as_ref()).cast::<u8>(),
                    size_of::<T>(),
                )
            }
        }
    }

    pub trait AsBuffMut<T>: Default + AsMut<T> {
        /// Provides mutable access to the internal buffer by applying a closure.
        ///
        /// This method allows the caller to modify the underlying buffer of the object
        /// by passing a closure that operates on a mutable slice of bytes. The closure
        /// has temporary exclusive access to the buffer for the duration of its execution.
        ///
        /// # Type Parameters
        /// - `F`: A closure or function that takes a mutable reference to a byte slice (`&mut [u8]`)
        ///        and performs some operation on it. The closure is executed exactly once.
        ///
        /// # Parameters
        /// - `f`: The closure to be executed, which receives a mutable reference to the
        ///        internal buffer as its argument.
        ///
        /// # Examples
        /// ```rust
        /// let mut obj = YourType::new();
        /// obj.with_buff_mut(|buff| {
        ///     // Modify the buffer
        ///     buff[0] = 42;
        /// });
        /// ```
        ///
        /// # Notes
        /// This method is useful for safely encapsulating mutations to the internal buffer
        /// without exposing direct access to it. The exact behavior of `as_buff_mut` depends
        /// on the implementation of the enclosing object.
        fn with_buff_mut<F>(&mut self, f: F)
                            where
                                F: FnOnce(&mut [u8]),
        {
            f(self.as_buff_mut())
        }

        fn as_buff_mut(&mut self) -> &mut [u8] {
            unsafe {
                // SAFETY: buff cannot be null in this trait
                core::slice::from_raw_parts_mut(
                    ptr::from_mut(self.as_mut()).cast::<u8>(),
                    size_of::<T>(),
                )
            }
        }
    }

    pub trait AsBuilder {
        type Descriptor<'a>
            where
                Self: 'a;

        /// Constructs and returns a descriptor based on the current state of the builder.
        ///
        /// # Returns
        ///
        /// A `Descriptor` instance representing the result of the builder configuration.
        ///
        /// # Attributes
        ///
        /// - **`#[must_use]`**: Indicates that the return value of this function
        ///   should not be ignored, as it represents the constructed descriptor.
        ///
        /// # Examples
        ///
        /// ```
        /// let builder = SomeBuilder::new();
        /// let descriptor = builder.build();
        /// ```
        ///
        /// # Lifetimes
        ///
        /// - The returned `Descriptor` instance is tied to the lifetime of the builder object.
        #[must_use]
        fn build(&self) -> Self::Descriptor<'_>;
    }

    pub trait AsBuilderMut: AsBuilder {
        /// Constructs and returns a mutable descriptor of the current object.
        ///
        /// # Returns
        ///
        /// * `Self::Descriptor<'_>` - A mutable descriptor tied to the lifetime of the current object.
        ///
        /// # Examples
        ///
        /// ```rust
        /// let mut instance = MyStruct::new();
        /// let descriptor = instance.build_mut();
        /// // Use the mutable descriptor here
        /// ```
        ///
        /// # Notes
        ///
        /// The returned descriptor allows mutable access to the object's properties or associated data,
        /// with lifetimes ensuring that the object is still in scope while the descriptor is in use.
        fn build_mut(&mut self) -> Self::Descriptor<'_>;
    }

    pub trait AsMappableBuff<T>: AsBuff<T> {
        type Descriptor<'a>
            where
                Self: 'a;

        /// Applies a provided function `f` to each element of the underlying buffer,
        /// transforming the elements and returning a `Vec<U>` with the results.
        ///
        /// # Type Parameters
        /// - `U`: The type of the elements in the resulting vector.
        ///
        /// # Parameters
        /// - `f`: A closure or function that takes a reference to a `u8` and produces a value of type `U`.
        ///
        /// # Returns
        /// A `Vec<U>` containing the results of applying `f` to each element of the buffer.
        ///
        /// # Example
        /// ```
        /// let buffer = MyBuffer::new(vec![1, 2, 3]);
        /// let result = buffer.map(|&x| x as f64 * 1.5);
        /// assert_eq!(result, vec![1.5, 3.0, 4.5]);
        /// ```
        ///
        /// # Notes
        /// This function assumes the existence of an `as_buff()` method on the current type
        /// that returns a slice (`&[u8]`) of the buffer to iterate over.
        fn map<U>(
            &self,
            f: impl FnMut(&u8) -> U,
        ) -> Vec<U> {
            self.as_buff().iter().map(f).collect()
        }
    }

    pub trait AsOptionalBuff<T>: AsRef<Option<T>> {
        /// Executes a closure `f`, providing it with an optional borrowed buffer (`Option<&[u8]>`)
        /// obtained from the `as_buff` method.
        ///
        /// # Type Parameters
        /// - `F`: A closure or function that takes an `Option<&[u8]>` as its argument and returns nothing (`FnOnce(Option<&[u8]>)`).
        ///
        /// # Parameters
        /// - `f`: The closure or function to execute. It is called with the result of `self.as_buff()`.
        ///
        /// # Behavior
        /// The `with_buff` method is intended to abstract the process of working with an optional
        /// buffer, ensuring the consumer only needs to provide logic within the provided closure.
        ///
        /// # Inlining
        /// This method is marked with `#[inline]`, hinting to the compiler that it may be advantageous
        /// to inline this function to reduce overhead.
        ///
        /// # Example
        /// ```rust
        /// // Assuming `as_buff` returns an `Option<&[u8]>`...
        /// my_struct.with_buff(|buffer| {
        ///     if let Some(data) = buffer {
        ///         println!("Buffer data: {:?}", data);
        ///     } else {
        ///         println!("No buffer available.");
        ///     }
        /// });
        /// ```
        #[inline]
        fn with_buff<F>(&self, f: F)
                        where
                            F: FnOnce(Option<&[u8]>),
        {
            f(self.as_buff())
        }
        /// Converts the underlying value into a byte slice (`&[u8]`), if possible.
        ///
        /// This function attempts to represent the internal data of the type `T` as a
        /// byte slice. If `self` is `None`, the function returns `None`.
        ///
        /// # Safety
        ///
        /// This function uses `unsafe` to create a byte slice from the raw pointer
        /// of the value. Therefore, it relies on the following to avoid undefined behavior:
        /// - The value of `T` must not have any padding bytes that could lead to
        ///   undefined behavior when read from memory.
        /// - The type `T` must be properly aligned and have a stable memory layout for
        ///   its size and alignment criteria.
        ///
        /// # Returns
        ///
        /// - `Some(&[u8])` if the underlying value exists and can be represented as a byte slice.
        /// - `None` if `self` is `None`.
        ///
        /// # Example
        ///
        /// ```rust
        /// use core::mem::size_of;
        /// use core::ptr;
        ///
        /// struct MyStruct {
        ///     value: u32,
        /// }
        ///
        /// let my_struct = Some(MyStruct { value: 42 });
        /// let byte_slice = my_struct.as_buff();
        /// assert!(byte_slice.is_some());
        /// assert_eq!(byte_slice.unwrap().len(), size_of::<MyStruct>());
        /// ```
        ///
        /// # Note
        ///
        /// This method assumes that the type `T` is `Sized`.
        fn as_buff(&self) -> Option<&[u8]> {
            self.as_ref().as_ref().map(|value| unsafe {
                core::slice::from_raw_parts(
                    ptr::from_ref(value).cast::<u8>(),
                    size_of::<T>(),
                )
            })
        }
    }

    /// A trait that extends [`AsOptionalBuff`] and provides mutable access to an internal buffer.
    ///
    /// This trait builds upon the [`AsOptionalBuff`] trait and [`AsMut`] implementation
    /// to enable manipulating an optional internal buffer of type `T`. It includes utility
    /// methods to interact with the buffer in various ways, specifically focusing on
    /// mutable access.
    ///
    /// # Type Parameter
    ///
    /// - `T`: The type of the internal buffer.
    ///
    /// # Provided Methods
    ///
    /// ## `with_buff_mut`
    ///
    /// Executes a closure on a mutable view of the buffer, if it exists.
    ///
    /// ### Parameters
    /// - `f`: A closure of type `FnOnce(Option<&mut [u8]>)` that operates on the mutable
    ///   buffer. The closure receives either:
    ///   - `Some(&mut [u8])`: A mutable slice representing the buffer's data.
    ///   - `None`: If the buffer does not exist.
    ///
    /// ### Example
    /// ```rust
    /// use your_crate::AsOptionalBuffMut;
    ///
    /// struct YourStruct {
    ///     data: Option<Vec<u8>>,
    /// }
    ///
    /// impl AsOptionalBuffMut<u8> for YourStruct {
    ///     // Provide required implementations here.
    /// }
    ///
    /// let mut obj = YourStruct { data: Some(vec![1, 2, 3]) };
    /// obj.with_buff_mut(|buff| {
    ///     if let Some(buffer) = buff {
    ///         buffer[0] = 42; // Modify the first element in the buffer.
    ///     }
    /// });
    /// ```
    ///
    /// ### Notes
    /// - The actual mutable slice is derived using [`Self::as_buff_mut`] and provided to the closure.
    /// - The method is marked as `#[inline]` to suggest the compiler inline it,
    ///   which can improve performance.
    ///
    /// ## `as_buff_mut`
    ///
    /// Creates a mutable view of the underlying buffer, if it is accessible.
    ///
    /// ### Returns
    /// - `Some(&mut [u8])`: A mutable slice pointing to the bytes of the underlying buffer.
    /// - `None`: If the buffer is not available or inaccessible.
    ///
    /// ### Safety
    ///
    /// This method involves unsafe code to convert a raw pointer into a mutable slice.
    /// To ensure safe usage, the following constraints must be satisfied:
    /// - The underlying buffer (if it exists) must be properly aligned for the type `T`.
    /// - The memory allocation backing the buffer must be at least `size_of::<T>()` bytes in size.
    /// - The slice must not be accessed concurrently by any other part of the code to avoid
    ///   undefined behavior.
    ///
    /// ### Example
    /// ```rust
    /// use your_crate::AsOptionalBuffMut;
    ///
    /// struct YourStruct {
    ///     data: Option<u8>,
    /// }
    ///
    /// impl AsOptionalBuffMut<u8> for YourStruct {
    ///     fn as_buff_mut(&mut self) -> Option<&mut [u8]> {
    ///         self.data.as_mut().map(|value| unsafe {
    ///             core::slice::from_raw_parts_mut(
    ///                 core::ptr::addr_of_mut!(*value).cast::<u8>(),
    ///                 std::mem::size_of::<u8>(),
    ///             )
    ///         })
    ///     }
    /// }
    ///
    /// let mut obj = YourStruct { data: Some(42) };
    /// if let Some(slice) = obj.as_buff_mut() {
    ///     slice[0] = 99; // Update the buffer through the mutable slice.
    /// }
    /// ```
    ///
    /// ### Note
    /// The underlying type `T` must be compatible with the expected usage for the buffer, and the
    /// size of `T` must align with the logic of the memory access. Replace instances of `u8` in
    /// the examples with the actual type `T` you're using in your implementation.
    ///
    /// ---
    pub trait AsOptionalBuffMut<T>: AsOptionalBuff<T> + AsMut<Option<T>> {
        /// Provides mutable access to an internal buffer by applying a closure function.
        ///
        /// This method allows you to execute a closure (`f`) that operates on an
        /// optional mutable slice of the buffer (`Option<&mut [u8]>`).
        ///
        /// # Parameters
        /// - `f`: A closure of type `FnOnce(Option<&mut [u8]>)` that is applied
        ///   to the mutable buffer.
        ///
        /// # Example
        /// ```
        /// let mut obj = YourStruct::new();
        /// obj.with_buff_mut(|buff| {
        ///     if let Some(buffer) = buff {
        ///         buffer[0] = 42; // Modify the buffer
        ///     }
        /// });
        /// ```
        ///
        /// # Notes
        /// - The buffer is accessed via the `as_buff_mut()` method, which is not
        ///   explicitly shown here but expected to return `Option<&mut [u8]>`.
        /// - This utility is marked as `#[inline]` to hint the compiler to inline
        ///   the function for performance considerations.
        #[inline]
        fn with_buff_mut<F>(&mut self, f: F)
                            where
                                F: FnOnce(Option<&mut [u8]>),
        {
            f(self.as_buff_mut())
        }

        /// Provides a mutable buffer view into the underlying data structure if available.
        ///
        /// # Returns
        ///
        /// * `Some(&mut [u8])` - A mutable slice of bytes representing the underlying data.
        /// * `None` - If the underlying data is absent or inaccessible.
        ///
        /// # Safety
        ///
        /// This method uses unsafe code to create a mutable slice from a raw pointer.
        /// Users must ensure the following to avoid undefined behavior:
        /// - The underlying data must be properly aligned for type `T`.
        /// - The size of the allocated memory must be at least `size_of::<T>()` bytes.
        /// - The memory represented by the returned slice must not be accessed concurrently elsewhere.
        ///
        /// # Example
        ///
        /// ```rust
        /// let mut buffer: Option<MyType> = Some(MyType::new());
        /// if let Some(slice) = buffer.as_buff_mut() {
        ///     // Modify the data through the mutable slice.
        /// }
        /// ```
        ///
        /// Note: Replace `MyType` with the appropriate type T used in your implementation.
        fn as_buff_mut(&mut self) -> Option<&mut [u8]> {
            self.as_mut().as_mut().map(|value| unsafe {
                core::slice::from_raw_parts_mut(
                    ptr::from_mut(value).cast::<u8>(),
                    size_of::<T>(),
                )
            })
        }
    }

    /// A trait that combines the functionality of `AsPtr<T>` and `AsBuff<T>` to
    /// provide raw pointer access and safe buffer references.
    ///
    /// The `AsBuffPtr` trait is intended for types that represent or encapsulate
    /// a buffer of type `T`. It provides methods for accessing the buffer either
    /// as a raw pointer or as a safe reference.
    ///
    /// # Associated Types
    /// - `T`: The type of elements contained in the underlying buffer.
    ///
    /// # Implementors
    /// Types implementing this trait must also implement the `AsPtr<T>` and `AsBuff<T>` traits.
    /// This ensures that the type can provide raw pointer access and buffer-like behavior.
    ///
    /// # Provided Methods
    /// This trait provides two default methods:
    /// - [`as_raw_buff`](#tymethod.as_raw_buff): Returns a raw pointer to the buffer.
    /// - [`as_buff`](#tymethod.as_buff): Provides a safe reference to the buffer.
    ///
    /// ## Safety
    /// Special care must be taken while using raw pointers returned by `as_raw_buff`.
    /// Dereferencing such pointers must follow all the safety requirements outlined
    /// in Rust's pointer documentation to avoid undefined behavior.
    ///
    /// # Example
    /// ```rust
    /// // A hypothetical struct implementing `AsBuffPtr`.
    /// struct MyBuffer {
    ///     data: usize, // Represents some data buffer.
    /// }
    ///
    /// impl AsPtr<usize> for MyBuffer {
    ///     fn as_ptr(&self) -> *const usize {
    ///         &self.data as *const usize
    ///     }
    /// }
    ///
    /// impl AsBuff<usize> for MyBuffer {}
    ///
    /// impl AsBuffPtr<usize> for MyBuffer {}
    ///
    /// let my_buffer = MyBuffer { data: 42 };
    /// let raw_ptr = unsafe { my_buffer.as_raw_buff() }; // Unsafe pointer access.
    /// let safe_ref = my_buffer.as_buff(); // Safe reference access.
    ///
    /// assert_eq!(unsafe { *raw_ptr }, *safe_ref);
    /// assert_eq!(*safe_ref, 42);
    /// ```
    pub trait AsBuffPtr<T>: AsPtr<T> + AsBuff<T> {
        /// Returns a raw pointer (`*const T`) to the underlying buffer.
        ///
        /// # Safety
        /// This function is `unsafe` because it provides raw access to the buffer,
        /// bypassing Rust's safety guarantees. The caller must ensure the returned
        /// pointer is valid and properly aligned, and that it does not lead to any
        /// undefined behavior when dereferenced.
        ///
        /// # Notes
        /// - The function is marked with `#[inline]` to suggest inlining during
        ///   compilation, which may improve performance in tight loops or frequent
        ///   calls.
        /// - The `self.as_ptr()` method is expected to safely provide a raw pointer
        ///   to the buffer, but the underlying safety guarantees and checks remain
        ///   the caller's responsibility.
        ///
        /// # Example
        /// ```
        /// let buffer = ...; // Assume `buffer` is an instance supporting `as_raw_buff`.
        /// let raw_ptr = unsafe { buffer.as_raw_buff() };
        /// ```
        #[inline]
        unsafe fn as_raw_buff(&self) -> *const T {
            unsafe { self.as_ptr() }
        }

        /// Provides a safe reference to the underlying buffer of type `T`.
        ///
        /// # Safety
        /// This function internally uses an unsafe block to access the raw buffer
        /// through `self.as_raw_buff()`. It's assumed that the raw buffer is valid
        /// and properly aligned for type `T` at the time of usage.
        ///
        /// # Returns
        /// A reference to the buffer of type `T`.
        ///
        /// # Example
        /// ```
        /// let buffer = my_struct.as_buff();
        /// ```
        fn as_buff(&self) -> &T {
            unsafe { &*self.as_raw_buff() }
        }
    }

    /// A trait that provides mutable access to an internal buffer and its associated raw pointers.
    ///
    /// This trait extends the functionality of `AsBuffPtr<T>`, `AsBuffMut<T>`, and `AsPtrMut<T>`
    /// to include methods for mutable borrowing and raw pointer conversion.
    ///
    /// # Overview
    /// The `AsBuffPtrMut` trait is designed for types that encapsulate a buffer of type `T`
    /// and expose methods to mutate the buffer or work with its raw pointer in a safe and controlled way.
    ///
    /// # Safety
    /// Several methods in this trait use unsafe code internally to manipulate raw pointers.
    /// The caller must uphold the safety guarantees outlined in the documentation of each method.
    pub trait AsBuffPtrMut<T>: AsBuffPtr<T> + AsBuffMut<T> + AsPtrMut<T> {
        /// Provides mutable access to the internal buffer.
        ///
        /// This method returns a mutable reference to the underlying buffer of type `T`.
        ///
        /// # Safety
        /// This function uses an unsafe block to dereference the raw mutable pointer
        /// returned by `self.as_ptr_mut()`. The caller must ensure that:
        /// - The pointer is valid and properly aligned.
        /// - The pointer does not alias any existing references.
        ///
        /// # Inline Attribute
        /// The `#[inline]` annotation hints to the compiler that this function can
        /// be inlined to potentially improve performance.
        ///
        /// # Usage
        /// This method is commonly used to mutate the data in the internal buffer of the struct.
        ///
        /// # Example
        /// ```
        /// let mut instance = MyStruct::new();
        /// let buffer: &mut T = instance.as_buff_mut();
        /// buffer.modify_somehow();
        /// ```
        #[inline]
        fn as_buff_mut(&mut self) -> &mut T {
            unsafe { &mut *self.as_ptr_mut() }
        }

        /// Converts the current object into a raw mutable pointer of type `*mut T`.
        ///
        /// # Returns
        /// A mutable pointer to the underlying buffer of type `*mut T`.
        ///
        /// # Safety
        /// This function internally calls an unsafe function (`self.as_raw_buff()`)
        /// and then casts the resulting raw pointer to a mutable version.
        /// Ensure that the usage of the returned pointer upholds all safety guarantees
        /// and does not violate Rust's borrowing rules or cause undefined behavior.
        ///
        /// # Examples
        /// ```
        /// let mut obj = YourType::new();
        /// let raw_ptr = obj.as_raw_buff_mut();
        /// // Use `raw_ptr` safely, ensuring no data races or invalid access.
        /// ```
        fn as_raw_buff_mut(&mut self) -> *mut T {
            unsafe { self.as_raw_buff() }.cast_mut()
        }
    }

    pub unsafe trait AsCtxDescriptor: Sized + Default {
        /// A function that returns an optional `PCWDF_OBJECT_CONTEXT_TYPE_INFO`.
        ///
        /// This function is currently implemented to always return `None`.
        ///
        /// # Returns
        ///
        /// * `Option<PCWDF_OBJECT_CONTEXT_TYPE_INFO>` - Always returns `None`.
        ///
        /// # Example
        ///
        /// ```
        /// let result = unique();
        /// assert!(result.is_none());
        /// ```
        fn descriptor()
            -> Option<PCWDF_OBJECT_CONTEXT_TYPE_INFO> {
            None
        }

        /// Returns an optional static string slice representing the type name of a WDF (Widget Definition Framework).
        ///
        /// # Returns
        ///
        /// - `Option<&'static str>`: Currently always returns `None`, indicating that no type name is defined.
        ///
        /// # Examples
        ///
        /// ```rust
        /// let type_name = wdf_type_name();
        /// assert!(type_name.is_none());
        /// ```
        fn wdf_type_name() -> Option<&'static str> {
            None
        }

        /// Creates an `Option` containing a reference to a `HandleRef` of `Self`
        /// from a given object.
        ///
        /// This function currently does not have an implementation and always
        /// returns `None`. The `#[allow(unused_variables)]` attribute is applied
        /// to suppress warnings for the unused parameter.
        ///
        /// # Type Parameters
        /// - `O`: The type of the object passed as input.
        ///
        /// # Parameters
        /// - `obj`: A reference to an object of type `O`.
        ///
        /// # Notes
        /// - Currently, this function is effectively a placeholder and doesn't
        ///   utilize the passed parameter or perform any operations.
        #[allow(unused_variables)]
        fn from_kernel(
            obj: NonNull<c_void>,
        ) -> Option<NonNull<Self>> {
            None
        }

        fn initialize(obj: NonNull<c_void>) -> Option<()>{
            if let Some(mut ctx) = Self::from_kernel(NonNull::new(obj.as_ptr())?) {
                unsafe {
                    ptr::write(
                        ctx.as_mut(),
                        Self::default(),
                    )
                }
            };

            Some(())
        }
    }

    pub trait AsKernelType<O: Copy>: Sized + AsPtr<O> + AsRef<O> + Deref<Target=O> + AsWdfHandle<O> {}


    /// A trait usually represents a kernel object or resource that is allocable inside the kernel
    /// by a runtime framework (like WDF).
    ///
    /// If you need to represent something that depends of some resources allocated inside the kernel
    /// look at [`AsWdfOwned`].
    ///
    /// This trait extends the functionality of `AsPtr` and `AsRef` and provides methods to initialize objects
    /// with or without contextual attributes.
    ///
    /// # Associated Types
    /// - `Conf`: The type that defines the configuration parameters for object creation.
    /// - `Owned`: The type of the owned resource being managed.
    ///
    /// # Provided Methods
    ///
    /// ## from_owned_with_attrs
    ///
    /// Creates an instance of the implementing type from an owned object, a configuration object,
    /// and optional attributes related to the context descriptor.
    ///
    /// ### Type Parameters
    /// - `D`: A type that implements the `AsCtxDescriptor` trait, representing the context descriptor.
    ///
    /// ### Parameters
    /// - `owned`: The owned instance of type `Self::Owned` from which the new instance will be created.
    /// - `conf`: The configuration object of type `Self::Conf` to configure the new instance.
    /// - `attrs`: Optional attributes of type `WdfObjAttrs<D>` containing additional context information
    ///   related to the provided descriptor `D`.
    ///
    /// ### Returns
    /// - An `NtResult<Self>`, which is either the successfully created instance of the implementing type
    ///   or an error reflecting the failure of the creation process.
    ///
    /// ### Constraints
    /// - The type parameter `D` must implement the `AsCtxDescriptor` trait.
    ///
    /// ## from_owned
    /// Creates an instance of the implementing type from an owned resource and a configuration object.
    ///
    /// This method is a higher-level abstraction over `from_owned_with_attrs`. It internally delegates
    /// to `from_owned_with_attrs` using a default context descriptor (`WdfCtxNoneDesc`) and no additional attributes.
    ///
    /// ### Parameters
    /// - `owned`: The owned resource to initialize the type from.
    /// - `conf`: A configuration object that specifies how the type should be initialized.
    ///
    /// ### Returns
    /// An `NtResult` containing the newly created instance, or an error if initialization fails.
    ///
    /// ### Example
    /// ```rust
    /// let instance = MyType::from_owned(owned_resource, config_object)?;
    /// ```
    ///
    /// ### Notes
    /// Use this method for simpler use cases where no explicit attributes are required.
    ///
    /// ### Se Also
    /// - [`Handle`]
    /// - [`WdfObjAttrs`]
    /// - [`AsWdfOwned`]
    pub trait FromKernel<O: Copy>: AsWdfHandle<O>+ AsKernelType<O> {
        type Accessor;
        type Conf;

        #[inline]
        fn ctx<C: AsCtxDescriptor>(
            &self,
        ) -> Option<Handle<C>> {
            Some(Handle::new(C::from_kernel(self.as_wdf_handle().cast())?))
        }

        fn from_kernel_explicit<D>(
            accessor: NonNull<Self::Accessor>,
            conf: Option<Self::Conf>,
            attrs: Option<WdfObjAttrs<D>>,
        ) -> NtResult<Self> where
                D: AsCtxDescriptor;
    }

    pub trait FromKernelWithConf<O: Copy>: FromKernel<O> {
        /// Helpers to call [`AsWdfOwner::allocate_from_owned`] when no args are needed by the implementing type
        /// ### See Also
        /// - [`AsWdfOwner::allocate_from_owned`]
        fn from_kernel(
            owned: NonNull<Self::Accessor>,
            conf: Self::Conf,
        ) -> NtResult<Self> {
            Self::from_kernel_explicit::<WdfCtxNoneDesc>(
                owned,
                Some(conf),
                None,
            )
        }
    }

    pub trait FromKernelWithConfAndAttrs<O: Copy>: FromKernel<O> {
        fn from_kernel(
            owned: NonNull<Self::Accessor>,
            conf: Self::Conf,
            attrs: WdfObjAttrs,
        ) -> NtResult<Self> {
            Self::from_kernel_explicit::<WdfCtxNoneDesc>(
                owned,
                Some(conf),
                Some(attrs),
            )
        }
    }

    pub trait FromKernelWithAttrs<O: Copy>: FromKernel<O> {
        fn from_kernel(
            owned: NonNull<Self::Accessor>,
            attrs: WdfObjAttrs,
        ) -> NtResult<Self> {
            Self::from_kernel_explicit::<WdfCtxNoneDesc>(
                owned,
                None,
                Some(attrs),
            )
        }
    }

    /// A trait for types that wrap another value and allow retrieval of the inner value by consuming the wrapper.
    ///
    /// The `IntoInner` trait is useful for types that encapsulate or wrap an inner value of type `T`.
    /// It provides a mechanism to retrieve the inner value while consuming the wrapper.
    /// This trait also requires the implementing type to implement `AsRef<T>`,
    /// which allows borrowing the inner value without consuming the wrapper.
    ///
    /// # Type Parameters
    ///
    /// * `T`: The type of the inner value that is being wrapped.
    ///
    /// # Required Methods
    ///
    /// ## `into_inner`
    ///
    /// Consumes the wrapper and retrieves the inner value.
    ///
    /// This method moves the wrapper, extracting the contained value of type `T`.
    /// After calling `into_inner`, the wrapper can no longer be used.
    ///
    /// ### Returns
    ///
    /// The inner value of type `T` that was encapsulated by the wrapper.
    ///
    /// ### Examples
    ///
    /// ```
    /// struct SomeWrapper<T> {
    ///     value: T,
    /// }
    ///
    /// impl<T> SomeWrapper<T> {
    ///     fn new(value: T) -> Self {
    ///         Self { value }
    ///     }
    /// }
    ///
    /// impl<T> AsRef<T> for SomeWrapper<T> {
    ///     fn as_ref(&self) -> &T {
    ///         &self.value
    ///     }
    /// }
    ///
    /// impl<T> IntoInner<T> for SomeWrapper<T> {
    ///     fn into_inner(self) -> T {
    ///         self.value
    ///     }
    /// }
    ///
    /// let wrapped = SomeWrapper::new(42);
    /// let value = wrapped.into_inner();
    /// assert_eq!(value, 42);
    /// ```
    ///
    /// By using the `into_inner` method, you take ownership of the inner value `42`,
    /// and the wrapper `SomeWrapper` is consumed in the process.
    pub trait IntoInner<T>: AsRef<T> {
        /// Consumes the wrapper and returns the inner value.
        ///
        /// # Returns
        ///
        /// The inner value of type `T` that was wrapped by this instance.
        ///
        /// # Examples
        ///
        /// ```
        /// let wrapped = SomeWrapper::new(42);
        /// let value = wrapped.into_inner();
        /// assert_eq!(value, 42);
        /// ```
        ///
        /// By calling this method, you take ownership of the inner value,
        /// and the wrapper is consumed in the process.
        fn into_inner(self) -> T;
    }

    /// A trait that extends `AsRef` to provide an additional method for retrieving
    /// a raw, dereferenced value of type `T`.
    ///
    /// The `AsRaw` trait is designed for types that can reference an inner value of type `T`
    /// and wish to provide a convenient way to obtain the raw value by dereferencing that reference.
    ///
    /// # Type Parameters
    /// * `T`: The type of the raw value, which must implement the `Copy` trait to allow for safe dereferencing.
    ///
    /// # Inherits
    /// This trait inherits from `AsRef`, meaning any implementing type must also implement `AsRef<T>`.
    ///
    /// # Provided Methods
    /// ## `raw`
    /// A convenience method to fetch the raw, dereferenced value of type `T` from the reference obtained
    /// by the `as_ref` method.
    ///
    /// # Returns
    /// * `T`: The raw value represented by the implementing type.
    ///
    /// # Example
    /// ```
    /// struct Wrapper(i32);
    ///
    /// impl AsRef<i32> for Wrapper {
    ///     fn as_ref(&self) -> &i32 {
    ///         &self.0
    ///     }
    /// }
    ///
    /// impl AsRaw<i32> for Wrapper {}
    ///
    /// let wrapper = Wrapper(42);
    /// let raw_value = wrapper.raw();
    /// assert_eq!(raw_value, 42);
    /// ```
    ///
    /// # Notes
    /// * This trait requires the implementing type to support `as_ref()` to enable dereferencing and retrieving the raw value.
    ///
    /// # See Also
    /// Refer to the standard library's [`AsRef`](https://doc.rust-lang.org/std/convert/trait.AsRef.html) trait
    /// which serves as the base requirement for this trait.
    pub trait AsRaw<T: Copy>: AsRef<T> {
        /// Returns a raw value of type `T` by dereferencing the reference obtained
        /// from the `as_ref()` method.
        ///
        /// # Returns
        /// * `T` - The raw value represented by the implementing type.
        ///
        /// # Example
        /// ```
        /// let wrapper = Wrapper::new(42);
        /// let raw_value = wrapper.raw();
        /// assert_eq!(raw_value, 42);
        /// ```
        ///
        /// # Notes
        /// * This function requires the implementing type to support dereferencing through `as_ref()`.
        fn raw(&self) -> T {
            *self.as_ref()
        }
    }

    /// A trait that extends the `Borrow` trait to allow borrowing a value and returning a copy of it.
    ///
    /// This trait is designed to work with types that implement the `Copy` trait, ensuring that the
    /// borrowed value can be safely copied and returned without modifying the source object.
    ///
    /// # Type Parameters
    ///
    /// * `T` - The type of the value stored within the object, which must implement the `Copy` trait.
    ///
    /// # Example
    ///
    /// ```rust
    /// use std::borrow::Borrow;
    ///
    /// struct MyStruct {
    ///     value: i32,
    /// }
    ///
    /// impl Borrow<i32> for MyStruct {
    ///     fn borrow(&self) -> &i32 {
    ///         &self.value
    ///     }
    /// }
    ///
    /// impl AsRawWithBorrow<i32> for MyStruct {}
    ///
    /// let my_obj = MyStruct { value: 42 };
    /// let borrowed_value = my_obj.raw_with_borrow();
    /// assert_eq!(borrowed_value, 42);
    /// ```
    ///
    /// # Notes
    ///
    /// - This trait assumes that the type `T` is `Copy`, as the method `raw_with_borrow` will
    ///   dereference the borrowed value and return a copy of it.
    /// - Implementors of this trait should ensure that their borrowing mechanism is safe and does not
    ///   introduce runtime panics or violate Rust's borrowing rules.
    pub trait AsRawWithBorrow<T: Copy>: Borrow<T> {
        /// Retrieves the data stored within the current object by borrowing its value
        /// and returning a copy of the borrowed value.
        ///
        /// # Returns
        ///
        /// * `T` - A copy of the value obtained from borrowing the data.
        ///
        /// # Notes
        ///
        /// - This method assumes that the type `T` implements the `Copy` trait
        ///   since it dereferences the borrowed value and returns a copy.
        /// - Ensure the borrowing mechanism does not cause runtime panics
        ///   due to invalid access or mutability conflicts.
        fn raw_with_borrow(&self) -> T {
            *self.borrow()
        }
    }

    /// A trait that extends the functionality of the `AsUnique` trait to provide
    /// additional context related to a "None" or empty state.
    ///
    /// This trait can be implemented for types that already implement the `AsUnique`
    /// trait, allowing the object to represent a unique context with a specific
    /// behavior or state for handling "None" cases.
    ///
    /// # Notes
    /// - This is a marker trait and does not introduce any new methods or functionality,
    ///   but rather serves as a semantic extension to the `AsUnique` trait.
    ///
    /// # Example
    /// ```
    /// use your_crate::AsNoneCtxUnique;
    ///
    /// struct MyType;
    ///
    /// impl AsUnique for MyType {
    ///     // Implement the details for the `AsUnique` trait
    /// }
    ///
    /// impl AsNoneCtxUnique for MyType {}
    /// ```
    pub trait AsNoneCtxUnique: AsUnique {}

    /// A trait that extends the functionality of `AsCtxDescriptor` to represent
    /// a context that can optionally describe "none" or absence of a particular value.
    ///
    /// # Type Parameters
    /// - `O`: The output type for the context when it represents "none". Defaults to the unit type `()`.
    ///
    /// # Implementors
    /// Types implementing this trait must also implement the `AsCtxDescriptor` trait.
    ///
    /// # Example
    /// ```rust
    /// // Assuming `MyType` implements both `AsCtxDescriptor` and `AsNoneCtxDesc`
    /// let my_instance: MyType;
    /// // Use the functionalities provided by `AsNoneCtxDesc`
    /// ```
    ///
    /// # See Also
    /// - `AsCtxDescriptor`: The parent trait that must be implemented alongside this trait.
    pub trait AsNoneCtxDesc<O = ()>: AsCtxDescriptor {}

    /// Describe a generic data type that is able to expose an handle
    /// to a static instance of `WDF_OBJECT_CONTEXT_TYPE_INFO`
    /// # Note
    /// `WDF_OBJECT_CONTEXT_TYPE_INFO` is the data structure that WDM use internally
    /// to describe a context type object. WDM uses this struct to have the necessary
    /// information about the context type that needs to be allocated inside the kernel.
    ///
    /// # Safety
    /// This trait is concerned with implementing a handler to one of the static [`WDF_OBJECT_CONTEXT_TYPE_INFO`]
    /// instances that WDM internally uses to describe a kernel-allocated context object.
    /// Since the [`WDF_OBJECT_CONTEXT_TYPE_INFO`] need to be addressed as a static element,
    /// the type that implements this type needs to be sure that [`AsUnique::unique()`]
    /// point to a valid static instance of [`WDF_OBJECT_CONTEXT_TYPE_INFO`]
    ///
    /// # Implementors
    /// This trait is typically implemented by the [`swdf_declare_context_handle!`].
    /// See [`swdf_declare_context_handle!`] examples for more information
    pub unsafe trait AsUnique {
        /// Retrieves unique object context type information unsafely.
        ///
        /// # Safety
        /// This function is marked as `unsafe` because it assumes that the caller
        /// upholds certain invariants regarding the usage and validity of the returned
        /// `PCWDF_OBJECT_CONTEXT_TYPE_INFO` object. Improper usage may lead to undefined behavior.
        ///
        /// # Returns
        /// Returns an instance of `PCWDF_OBJECT_CONTEXT_TYPE_INFO` which holds the
        /// context type information of a unique object.
        ///
        /// # Note
        /// - The caller must ensure that the returned object context type information
        ///   is properly handled and does not violate memory safety constraints.
        /// - This function is typically used when low-level operations involving
        ///   object context types are required.
        ///
        /// # Examples
        /// ```rust
        /// // Example usage of unique()
        /// let context_type_info = some_instance.unique();
        /// ```
        unsafe fn unique(
            &self,
        ) -> PCWDF_OBJECT_CONTEXT_TYPE_INFO;
    }

    pub trait AsNtStatus: Sized {
        fn fmt_status(self) -> &'static str;

        fn fmt_hex(self) -> String;
    }

    pub trait IntoRaw<O: Copy>: Deref<Target=O> + Sized {
        fn into_raw(self) -> O {
            *self.deref()
        }
    }
}

pub use _concepts::NtResult;
pub use _operators::*;
