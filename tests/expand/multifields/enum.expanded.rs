use pin_project_lite::pin_project;
enum Enum<T, U> {
    Struct { pinned1: T, pinned2: T, unpinned1: U, unpinned2: U },
    Unit,
}
#[doc(hidden)]
#[allow(dead_code)]
#[allow(single_use_lifetimes)]
#[allow(clippy::mut_mut)]
#[allow(clippy::redundant_pub_crate)]
#[allow(clippy::type_repetition_in_bounds)]
enum EnumProjReplace<T, U> {
    Struct {
        pinned1: ::core::marker::PhantomData<T>,
        pinned2: ::core::marker::PhantomData<T>,
        unpinned1: U,
        unpinned2: U,
    },
    Unit,
}
#[allow(single_use_lifetimes)]
#[allow(clippy::unknown_clippy_lints)]
#[allow(clippy::used_underscore_binding)]
const _: () = {
    impl<T, U> Enum<T, U> {
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
                    Self::Struct { pinned1, pinned2, unpinned1, unpinned2 } => {
                        let result = EnumProjReplace::Struct {
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
        Struct: (T, T, __AlwaysUnpin<U>, __AlwaysUnpin<U>),
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
