#![cfg_attr(feature = "kmdf-runtime", no_std)]
/*#![cfg_attr(
    feature = "unstable",
    feature(
        trait_alias,
        associated_type_defaults,
        min_specialization,
        type_alias_impl_trait,
        negative_impls,
        impl_trait_in_assoc_type,
    )
)]*/
#[cfg(all(
    not(feature = "test-runtime"),
    not(feature = "kmdf-runtime")
))]
compile_error!(
    "Multiple runtime behavior selected. Select only one! ('kmdf-runtime', 'test-runtime')"
);

#[cfg(all(
    feature = "test-runtime",
    feature = "kmdf-runtime"
))]
compile_error!(
    "Select a valid runtime behavior for swdk. ('kmdf-runtime', 'test-runtime')"
);
pub extern crate alloc;

mod runtime;
mod wdf;

pub mod rt {
    #[cfg(feature = "kmdf-runtime")]
    mod __kmdf_rt {
        pub use wdk;
        pub use wdk_alloc;
        pub use wdk_sys;

        pub extern crate wdk_panic;

        /// KMDFS internal callbacks
        pub use crate::runtime::kmdf as __cb;
        pub use crate::runtime::{logging, utils};
    }

    #[cfg(feature = "kmdf-runtime")]
    pub use __kmdf_rt::*;

    #[cfg(feature = "test-runtime")]
    pub use crate::runtime::test::*;
}

#[doc(hidden)]
pub use paste::paste as __swdk_paste;
#[cfg(feature = "kmdf-runtime")]
pub use wdk::println;

mod __public_api {
    /// Contains WDF builders
    pub use crate::wdf::builders as bd;

    /// Contains WDF context types
    pub use crate::wdf::context as ctx;
    pub use crate::wdf::handle::{Handle, HandleMut};
    pub use crate::wdf::ioctl;

    /// Contains WDF operators
    pub use crate::wdf::operators as op;
    /// Contains various WDF convertible vals
    pub use crate::wdf::values as vals;
}

pub use __public_api::*;

pub use crate::wdf::generators as gens;
