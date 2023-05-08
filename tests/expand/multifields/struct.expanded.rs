use pin_project_lite::pin_project;
struct Struct<T, U> {
    pinned1: T,
    pinned2: T,
    unpinned1: U,
    unpinned2: U,
}
#[doc(hidden)]
#[allow(dead_code)]
#[allow(single_use_lifetimes)]
#[allow(clippy::mut_mut)]
#[allow(clippy::redundant_pub_crate)]
#[allow(clippy::type_repetition_in_bounds)]
struct StructProjReplace<T, U> {
    pinned1: ::core::marker::PhantomData<T>,
    pinned2: ::core::marker::PhantomData<T>,
    unpinned1: U,
    unpinned2: U,
}
#[allow(explicit_outlives_requirements)]
#[allow(single_use_lifetimes)]
#[allow(clippy::unknown_clippy_lints)]
#[allow(clippy::redundant_pub_crate)]
#[allow(clippy::used_underscore_binding)]
const _: () = {
    #[doc(hidden)]
    #[allow(dead_code)]
    #[allow(single_use_lifetimes)]
    #[allow(clippy::unknown_clippy_lints)]
    #[allow(clippy::mut_mut)]
    #[allow(clippy::redundant_pub_crate)]
    #[allow(clippy::ref_option_ref)]
    #[allow(clippy::type_repetition_in_bounds)]
    struct Projection<'__pin, T, U>
    where
        Struct<T, U>: '__pin,
    {
        pinned1: ::core::pin::Pin<&'__pin mut (T)>,
        pinned2: ::core::pin::Pin<&'__pin mut (T)>,
        unpinned1: &'__pin mut (U),
        unpinned2: &'__pin mut (U),
    }
    #[doc(hidden)]
    #[allow(dead_code)]
    #[allow(single_use_lifetimes)]
    #[allow(clippy::unknown_clippy_lints)]
    #[allow(clippy::mut_mut)]
    #[allow(clippy::redundant_pub_crate)]
    #[allow(clippy::ref_option_ref)]
    #[allow(clippy::type_repetition_in_bounds)]
    struct ProjectionRef<'__pin, T, U>
    where
        Struct<T, U>: '__pin,
    {
        pinned1: ::core::pin::Pin<&'__pin (T)>,
        pinned2: ::core::pin::Pin<&'__pin (T)>,
        unpinned1: &'__pin (U),
        unpinned2: &'__pin (U),
    }
    impl<T, U> Struct<T, U> {
        #[doc(hidden)]
        #[inline]
        fn project<'__pin>(
            self: ::core::pin::Pin<&'__pin mut Self>,
        ) -> Projection<'__pin, T, U> {
            unsafe {
                let Self { pinned1, pinned2, unpinned1, unpinned2 } = self
                    .get_unchecked_mut();
                Projection {
                    pinned1: ::core::pin::Pin::new_unchecked(pinned1),
                    pinned2: ::core::pin::Pin::new_unchecked(pinned2),
                    unpinned1: unpinned1,
                    unpinned2: unpinned2,
                }
            }
        }
        #[doc(hidden)]
        #[inline]
        fn project_ref<'__pin>(
            self: ::core::pin::Pin<&'__pin Self>,
        ) -> ProjectionRef<'__pin, T, U> {
            unsafe {
                let Self { pinned1, pinned2, unpinned1, unpinned2 } = self.get_ref();
                ProjectionRef {
                    pinned1: ::core::pin::Pin::new_unchecked(pinned1),
                    pinned2: ::core::pin::Pin::new_unchecked(pinned2),
                    unpinned1: unpinned1,
                    unpinned2: unpinned2,
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
                let Self { pinned1, pinned2, unpinned1, unpinned2 } = &mut *__self_ptr;
                let result = StructProjReplace {
                    pinned1: ::core::marker::PhantomData,
                    pinned2: ::core::marker::PhantomData,
                    unpinned1: ::core::ptr::read(unpinned1),
                    unpinned2: ::core::ptr::read(unpinned2),
                };
                {
                    (
                        __UnsafeDropInPlaceGuard(pinned1),
                        __UnsafeDropInPlaceGuard(pinned2),
                        (),
                        (),
                    );
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
        pinned1: T,
        pinned2: T,
        unpinned1: __AlwaysUnpin<U>,
        unpinned2: __AlwaysUnpin<U>,
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
        let _ = &this.pinned1;
        let _ = &this.pinned2;
        let _ = &this.unpinned1;
        let _ = &this.unpinned2;
    }
};
fn main() {}
