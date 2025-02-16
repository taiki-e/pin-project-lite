// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Check interoperability with rustc and clippy lints.

#![forbid(unsafe_code)]
#![allow(unknown_lints)] // for old compilers
#![warn(nonstandard_style, rust_2018_idioms, unused, deprecated_safe)]
// Note: This does not guarantee compatibility with forbidding these lints in the future.
// If rustc adds a new lint, we may not be able to keep this.
#![forbid(
    future_incompatible,
    rust_2018_compatibility,
    rust_2021_compatibility,
    rust_2024_compatibility
)]
// lints forbidden as a part of future_incompatible, rust_2018_compatibility, rust_2021_compatibility, rust_2024_compatibility are not included in the list below.
// elided_lifetimes_in_paths, explicit_outlives_requirements, unused_extern_crates: included as a part of rust_2018_idioms
// non_exhaustive_omitted_patterns, multiple_supertrait_upcastable: unstable
// unstable_features: no way to generate #![feature(..)] by macros, expect for unstable inner attribute. and this lint is deprecated: https://doc.rust-lang.org/rustc/lints/listing/allowed-by-default.html#unstable-features
// unused_crate_dependencies, must_not_suspend: unrelated
// unsafe_code: checked in forbid_unsafe module
// TODO: Add warn-by-default lints for old versions?
#![warn(
    ambiguous_negative_literals,
    closure_returning_async_block,
    deprecated_in_future,
    dereferencing_mut_binding,
    fuzzy_provenance_casts,
    impl_trait_redundant_captures,
    invalid_reference_casting,
    let_underscore_drop,
    lossy_provenance_casts,
    macro_use_extern_crate,
    meta_variable_misuse,
    missing_abi,
    missing_copy_implementations,
    missing_debug_implementations,
    // missing_docs, // TODO: https://github.com/taiki-e/pin-project-lite/issues/3#issuecomment-703534472
    non_ascii_idents, // TODO: add test case
    non_local_definitions,
    noop_method_call,
    private_bounds,
    private_interfaces,
    redundant_imports,
    redundant_lifetimes,
    single_use_lifetimes,
    supertrait_item_shadowing_definition,
    supertrait_item_shadowing_usage,
    trivial_casts,
    trivial_numeric_casts,
    unit_bindings,
    unnameable_types,
    unqualified_local_imports,
    unreachable_pub,
    unused_import_braces,
    unused_lifetimes,
    unused_qualifications,
    unused_results,
    variant_size_differences
)]
#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::restriction)]
#![allow(clippy::blanket_clippy_restriction_lints)] // this is a test, so enable all restriction lints intentionally.
#![allow(
    clippy::allow_attributes,
    clippy::allow_attributes_without_reason,
    clippy::arbitrary_source_item_ordering
)] // TODO

/// Test for basic cases.
pub mod basic {
    include!("../include/basic.rs");
}

/// Test for `explicit_outlives_requirements` lint.
pub mod explicit_outlives_requirements {
    use pin_project_lite::pin_project;

    pin_project! {
        /// Testing struct.
        #[allow(clippy::exhaustive_structs, clippy::single_char_lifetime_names)] // for the type itself
        #[derive(Debug)]
        pub struct Struct<'a, T, U>
        where
            T: ?Sized,
            U: ?Sized,
        {
            #[pin]
            pub pinned: &'a mut T,
            pub unpinned: &'a mut U,
        }
    }

    pin_project! {
        /// Testing enum.
        #[project = EnumProj]
        #[project_ref = EnumProjRef]
        #[project(!Unpin)]
        #[allow(clippy::exhaustive_enums, clippy::single_char_lifetime_names)] // for the type itself
        #[derive(Debug)]
        pub enum Enum<'a, T, U>
        where
            T: ?Sized,
            U: ?Sized,
        {
            /// Struct variant.
            Struct {
                #[pin]
                pinned: &'a mut T,
                unpinned: &'a mut U,
            },
            /// Unit variant.
            Unit,
        }
    }
}

/// Test for `variant_size_differences` and `clippy::large_enum_variant` lints.
pub mod variant_size_differences {
    use pin_project_lite::pin_project;

    pin_project! {
        /// Testing enum.
        #[project = EnumProj]
        #[project_ref = EnumProjRef]
        #[project(!Unpin)]
        #[allow(missing_debug_implementations, missing_copy_implementations)] // https://github.com/rust-lang/rust/pull/74060
        #[allow(variant_size_differences, clippy::exhaustive_enums, clippy::large_enum_variant, clippy::min_ident_chars)] // for the type itself
        pub enum Enum {
            /// Small variant size.
            V1 { f: u8 },
            /// Huge variant size.
            V2 { f: [u8; 1024] },
        }
    }
}

/// Test for `clippy::mut_mut` lint.
pub mod clippy_mut_mut {
    use pin_project_lite::pin_project;

    pin_project! {
        /// Testing struct.
        #[allow(clippy::exhaustive_structs, clippy::single_char_lifetime_names)] // for the type itself
        #[derive(Debug)]
        pub struct Struct<'a, T, U> {
            #[pin]
            pub pinned: &'a mut T,
            pub unpinned: &'a mut U,
        }
    }

    pin_project! {
        /// Testing enum.
        #[project = EnumProj]
        #[project_ref = EnumProjRef]
        #[project(!Unpin)]
        #[allow(clippy::exhaustive_enums, clippy::single_char_lifetime_names)] // for the type itself
        #[derive(Debug)]
        pub enum Enum<'a, T, U> {
            /// Struct variant.
            Struct {
                #[pin]
                pinned: &'a mut T,
                unpinned: &'a mut U,
            },
            /// Unit variant.
            Unit,
        }
    }
}

/// Test for `clippy::redundant_pub_crate` lint.
#[allow(unreachable_pub)]
mod clippy_redundant_pub_crate {
    use pin_project_lite::pin_project;

    pin_project! {
        /// Testing struct.
        pub struct Struct<T, U> {
            #[pin]
            pub pinned: T,
            pub unpinned: U,
        }
    }

    pin_project! {
        /// Testing enum.
        #[project = EnumProj]
        #[project_ref = EnumProjRef]
        #[project(!Unpin)]
        pub enum Enum<T, U> {
            /// Struct variant.
            Struct {
                #[pin]
                pinned: T,
                unpinned: U,
            },
            /// Unit variant.
            Unit,
        }
    }
}

/// Test for `clippy::type_repetition_in_bounds` lint.
#[allow(clippy::use_self)]
pub mod clippy_type_repetition_in_bounds {
    use pin_project_lite::pin_project;

    pin_project! {
        /// Testing struct.
        #[allow(clippy::exhaustive_structs)] // for the type itself
        pub struct Struct<T, U>
        where
            Struct<T, U>: Sized,
        {
            #[pin]
            pub pinned: T,
            pub unpinned: U,
        }
    }

    pin_project! {
        /// Testing enum.
        #[allow(clippy::exhaustive_enums)] // for the type itself
        #[project = EnumProj]
        #[project_ref = EnumProjRef]
        #[project(!Unpin)]
        pub enum Enum<T, U>
        where
            Enum<T, U>: Sized,
        {
            /// Struct variant.
            Struct {
                #[pin]
                pinned: T,
                unpinned: U,
            },
            /// Unit variant.
            Unit,
        }
    }
}

/// Test for `clippy::used_underscore_binding` lint.
pub mod clippy_used_underscore_binding {
    use pin_project_lite::pin_project;

    pin_project! {
        /// Testing struct.
        #[allow(clippy::exhaustive_structs, clippy::pub_underscore_fields)] // for the type itself
        pub struct Struct<T, U> {
            #[pin]
            pub _pinned: T,
            pub _unpinned: U,
        }
    }

    pin_project! {
        /// Testing enum.
        #[allow(clippy::exhaustive_enums)] // for the type itself
        #[project = EnumProj]
        #[project_ref = EnumProjRef]
        #[project(!Unpin)]
        pub enum Enum<T, U> {
            /// Variant.
            Struct {
                #[pin]
                _pinned: T,
                _unpinned: U,
            },
        }
    }
}

/// Test for `clippy::ref_option_ref` lint.
pub mod clippy_ref_option_ref {
    use pin_project_lite::pin_project;

    pin_project! {
        /// Testing struct.
        #[allow(clippy::exhaustive_structs, clippy::pub_underscore_fields, clippy::single_char_lifetime_names)] // for the type itself
        pub struct Struct<'a> {
            #[pin]
            pub _pinned: Option<&'a ()>,
            pub _unpinned: Option<&'a ()>,
        }
    }

    pin_project! {
        /// Testing enum.
        #[allow(clippy::exhaustive_enums, clippy::single_char_lifetime_names)] // for the type itself
        #[project = EnumProj]
        #[project_ref = EnumProjRef]
        #[project(!Unpin)]
        pub enum Enum<'a> {
            /// Variant.
            Struct {
                #[pin]
                _pinned: Option<&'a ()>,
                _unpinned: Option<&'a ()>,
            },
        }
    }
}
