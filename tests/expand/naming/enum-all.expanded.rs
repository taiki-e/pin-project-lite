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
enum EnumProj<'__pin, T, U>
where
    Enum<T, U>: '__pin,
{
    Struct { pinned: ::core::pin::Pin<&'__pin mut (T)>, unpinned: &'__pin mut (U) },
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
#[doc(hidden)]
#[allow(dead_code)]
#[allow(single_use_lifetimes)]
#[allow(clippy::mut_mut)]
#[allow(clippy::redundant_pub_crate)]
#[allow(clippy::type_repetition_in_bounds)]
enum EnumProjReplace<T, U> {
    Struct { pinned: ::core::marker::PhantomData<T>, unpinned: U },
    Unit,
}
#[allow(single_use_lifetimes)]
#[allow(clippy::unknown_clippy_lints)]
#[allow(clippy::used_underscore_binding)]
const _: () = {
    impl<T, U> Enum<T, U> {
        #[doc(hidden)]
        #[inline]
        fn project<'__pin>(
            self: ::core::pin::Pin<&'__pin mut Self>,
        ) -> EnumProj<'__pin, T, U> {
            unsafe {
                match self.get_unchecked_mut() {
                    Self::Struct { pinned, unpinned } => {
                        EnumProj::Struct {
                            pinned: ::core::pin::Pin::new_unchecked(pinned),
                            unpinned: unpinned,
                        }
                    }
                    Self::Unit => EnumProj::Unit,
                }
            }
        }
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
        #[doc(hidden)]
        #[inline]
        fn project_replace(
            self: ::core::pin::Pin<&mut Self>,
            replacement: Self,
        ) -> EnumProjReplace<T, U> {
            struct __UnsafeDropInPlaceGuard<T: ?::core::marker::Sized>(*mut T);
            impl<T: ?::core::marker::Sized> ::core::ops::Drop
            for __UnsafeDropInPlaceGuard<T> {
                fn drop(&mut self) {
                    unsafe {
                        ::core::ptr::drop_in_place(self.0);
                    }
                }
            }
            struct __UnsafeOverwriteGuard<T> {
                target: *mut T,
                value: ::core::mem::ManuallyDrop<T>,
            }
            impl<T> __UnsafeOverwriteGuard<T> {
                unsafe fn new(target: *mut T, value: T) -> Self {
                    Self {
                        target,
                        value: ::core::mem::ManuallyDrop::new(value),
                    }
                }
            }
            impl<T> ::core::ops::Drop for __UnsafeOverwriteGuard<T> {
                fn drop(&mut self) {
                    unsafe {
                        ::core::ptr::write(self.target, ::core::ptr::read(&*self.value));
                    }
                }
            }
            unsafe {
                let __self_ptr: *mut Self = self.get_unchecked_mut();
                let __guard = __UnsafeOverwriteGuard::new(__self_ptr, replacement);
                match &mut *__self_ptr {
                    Self::Struct { pinned, unpinned } => {
                        let result = EnumProjReplace::Struct {
                            pinned: ::core::marker::PhantomData,
                            unpinned: ::core::ptr::read(unpinned),
                        };
                        {
                            (__UnsafeDropInPlaceGuard(pinned), ());
                        }
                        result
                    }
                    Self::Unit => EnumProjReplace::Unit,
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
