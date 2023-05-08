use pin_project_lite::pin_project;
enum Enum<T, U> {
    Struct { pinned: T, unpinned: U },
    Unit,
}
#[doc(hidden)]
#[allow(dead_code)]
#[allow(single_use_lifetimes)]
#[allow(clippy::unknown_clippy_lints)]
#[allow(clippy::mut_mut)]
#[allow(clippy::redundant_pub_crate)]
#[allow(clippy::ref_option_ref)]
#[allow(clippy::type_repetition_in_bounds)]
enum EnumProjRef<'__pin, T, U>
where
    Enum<T, U>: '__pin,
{
    Struct { pinned: ::core::pin::Pin<&'__pin (T)>, unpinned: &'__pin (U) },
    Unit,
}
#[allow(single_use_lifetimes)]
#[allow(clippy::unknown_clippy_lints)]
#[allow(clippy::used_underscore_binding)]
const _: () = {
    impl<T, U> Enum<T, U> {
        #[doc(hidden)]
        #[inline]
        fn project_ref<'__pin>(
            self: ::core::pin::Pin<&'__pin Self>,
        ) -> EnumProjRef<'__pin, T, U> {
            unsafe {
                match self.get_ref() {
                    Self::Struct { pinned, unpinned } => {
                        EnumProjRef::Struct {
                            pinned: ::core::pin::Pin::new_unchecked(pinned),
                            unpinned: unpinned,
                        }
                    }
                    Self::Unit => EnumProjRef::Unit,
                }
            }
        }
    }
    struct __AlwaysUnpin<T: ?::core::marker::Sized>(::core::marker::PhantomData<T>);
    impl<T: ?::core::marker::Sized> ::core::marker::Unpin for __AlwaysUnpin<T> {}
    #[allow(non_snake_case)]
    struct __Origin<'__pin, T, U> {
        __dummy_lifetime: ::core::marker::PhantomData<&'__pin ()>,
        Struct: (T, __AlwaysUnpin<U>),
        Unit: (),
    }
    impl<'__pin, T, U> ::core::marker::Unpin for Enum<T, U>
    where
        __Origin<'__pin, T, U>: ::core::marker::Unpin,
    {}
    trait MustNotImplDrop {}
    #[allow(clippy::drop_bounds, drop_bounds)]
    impl<T: ::core::ops::Drop> MustNotImplDrop for T {}
    impl<T, U> MustNotImplDrop for Enum<T, U> {}
};
fn main() {}
