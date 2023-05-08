use pin_project_lite::pin_project;
struct Struct<T, U> {
    pinned: T,
    unpinned: U,
}
#[doc(hidden)]
#[allow(dead_code)]
#[allow(single_use_lifetimes)]
#[allow(clippy::unknown_clippy_lints)]
#[allow(clippy::mut_mut)]
#[allow(clippy::redundant_pub_crate)]
#[allow(clippy::ref_option_ref)]
#[allow(clippy::type_repetition_in_bounds)]
struct StructProj<'__pin, T, U>
where
    Struct<T, U>: '__pin,
{
    pinned: ::core::pin::Pin<&'__pin mut (T)>,
    unpinned: &'__pin mut (U),
}
#[doc(hidden)]
#[allow(dead_code)]
#[allow(single_use_lifetimes)]
#[allow(clippy::unknown_clippy_lints)]
#[allow(clippy::mut_mut)]
#[allow(clippy::redundant_pub_crate)]
#[allow(clippy::ref_option_ref)]
#[allow(clippy::type_repetition_in_bounds)]
struct StructProjRef<'__pin, T, U>
where
    Struct<T, U>: '__pin,
{
    pinned: ::core::pin::Pin<&'__pin (T)>,
    unpinned: &'__pin (U),
}
#[doc(hidden)]
#[allow(dead_code)]
#[allow(single_use_lifetimes)]
#[allow(clippy::mut_mut)]
#[allow(clippy::redundant_pub_crate)]
#[allow(clippy::type_repetition_in_bounds)]
struct StructProjReplace<T, U> {
    pinned: ::core::marker::PhantomData<T>,
    unpinned: U,
}
#[allow(explicit_outlives_requirements)]
#[allow(single_use_lifetimes)]
#[allow(clippy::unknown_clippy_lints)]
#[allow(clippy::redundant_pub_crate)]
#[allow(clippy::used_underscore_binding)]
const _: () = {
    impl<T, U> Struct<T, U> {
        #[doc(hidden)]
        #[inline]
        fn project<'__pin>(
            self: ::core::pin::Pin<&'__pin mut Self>,
        ) -> StructProj<'__pin, T, U> {
            unsafe {
                let Self { pinned, unpinned } = self.get_unchecked_mut();
                StructProj {
                    pinned: ::core::pin::Pin::new_unchecked(pinned),
                    unpinned: unpinned,
                }
            }
        }
        #[doc(hidden)]
        #[inline]
        fn project_ref<'__pin>(
            self: ::core::pin::Pin<&'__pin Self>,
        ) -> StructProjRef<'__pin, T, U> {
            unsafe {
                let Self { pinned, unpinned } = self.get_ref();
                StructProjRef {
                    pinned: ::core::pin::Pin::new_unchecked(pinned),
                    unpinned: unpinned,
                }
            }
        }
        #[doc(hidden)]
        #[inline]
        fn project_replace(
            self: ::core::pin::Pin<&mut Self>,
            replacement: Self,
        ) -> StructProjReplace<T, U> {
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
                let Self { pinned, unpinned } = &mut *__self_ptr;
                let result = StructProjReplace {
                    pinned: ::core::marker::PhantomData,
                    unpinned: ::core::ptr::read(unpinned),
                };
                {
                    (__UnsafeDropInPlaceGuard(pinned), ());
                }
                result
            }
        }
    }
    struct __AlwaysUnpin<T: ?::core::marker::Sized>(::core::marker::PhantomData<T>);
    impl<T: ?::core::marker::Sized> ::core::marker::Unpin for __AlwaysUnpin<T> {}
    #[allow(non_snake_case)]
    struct __Origin<'__pin, T, U> {
        __dummy_lifetime: ::core::marker::PhantomData<&'__pin ()>,
        pinned: T,
        unpinned: __AlwaysUnpin<U>,
    }
    impl<'__pin, T, U> ::core::marker::Unpin for Struct<T, U>
    where
        __Origin<'__pin, T, U>: ::core::marker::Unpin,
    {}
    trait MustNotImplDrop {}
    #[allow(clippy::drop_bounds, drop_bounds)]
    impl<T: ::core::ops::Drop> MustNotImplDrop for T {}
    impl<T, U> MustNotImplDrop for Struct<T, U> {}
    #[forbid(unaligned_references, safe_packed_borrows)]
    fn __assert_not_repr_packed<T, U>(this: &Struct<T, U>) {
        let _ = &this.pinned;
        let _ = &this.unpinned;
    }
};
fn main() {}
