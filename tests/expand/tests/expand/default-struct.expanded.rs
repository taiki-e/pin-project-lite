use pin_project_lite::pin_project;
struct Struct<T, U> {
    pinned: T,
    unpinned: U,
}
#[allow(single_use_lifetimes)]
#[allow(clippy::used_underscore_binding)]
const _: () = {
    #[allow(dead_code)]
    #[allow(clippy::mut_mut)]
    #[allow(clippy::type_repetition_in_bounds)]
    struct Projection<'__pin, T, U>
    where
        Struct<T, U>: '__pin,
    {
        pinned: ::pin_project_lite::__private::Pin<&'__pin mut (T)>,
        unpinned: &'__pin mut (U),
    }
    #[allow(dead_code)]
    #[allow(clippy::type_repetition_in_bounds)]
    struct ProjectionRef<'__pin, T, U>
    where
        Struct<T, U>: '__pin,
    {
        pinned: ::pin_project_lite::__private::Pin<&'__pin (T)>,
        unpinned: &'__pin (U),
    }
    impl<T, U> Struct<T, U> {
        fn project<'__pin>(
            self: ::pin_project_lite::__private::Pin<&'__pin mut Self>,
        ) -> Projection<'__pin, T, U> {
            unsafe {
                let this = self.get_unchecked_mut();
                Projection {
                    pinned: ::pin_project_lite::__private::Pin::new_unchecked(&mut this.pinned),
                    unpinned: &mut this.unpinned,
                }
            }
        }
        fn project_ref<'__pin>(
            self: ::pin_project_lite::__private::Pin<&'__pin Self>,
        ) -> ProjectionRef<'__pin, T, U> {
            unsafe {
                let this = self.get_ref();
                ProjectionRef {
                    pinned: ::pin_project_lite::__private::Pin::new_unchecked(&this.pinned),
                    unpinned: &this.unpinned,
                }
            }
        }
    }
    struct __Origin<'__pin, T, U> {
        __dummy_lifetime: ::pin_project_lite::__private::PhantomData<&'__pin ()>,
        pinned: T,
        unpinned: ::pin_project_lite::__private::AlwaysUnpin<U>,
    }
    impl<'__pin, T, U> ::pin_project_lite::__private::Unpin for Struct<T, U> where
        __Origin<'__pin, T, U>: ::pin_project_lite::__private::Unpin
    {
    }
    trait MustNotImplDrop {}
    #[allow(clippy::drop_bounds)]
    impl<T: ::pin_project_lite::__private::Drop> MustNotImplDrop for T {}
    impl<T, U> MustNotImplDrop for Struct<T, U> {}
    #[deny(safe_packed_borrows)]
    fn __assert_not_repr_packed<T, U>(this: &Struct<T, U>) {
        &this.pinned;
        &this.unpinned;
    }
};
fn main() {}