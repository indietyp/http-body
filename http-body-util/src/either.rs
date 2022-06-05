use std::pin::Pin;

use crate::either::proj::EitherProj;

pub enum Either<L, R> {
    Left(L),
    Right(R),
}

impl<L, R> Either<L, R> {
    pub(crate) fn project(self: Pin<&mut Self>) -> EitherProj<L, R> {
        unsafe {
            match self.get_unchecked_mut() {
                Self::Left(left) => EitherProj::Left {
                    left: Pin::new_unchecked(left),
                },
                Self::Right(right) => EitherProj::Right {
                    right: Pin::new_unchecked(right),
                },
            }
        }
    }
}

pub(crate) mod proj {
    //! This code is the (cleaned output) generated by [pin-project-lite], as it
    //! does not support tuple variants.
    //!
    //! This is the altered expansion from the following snippet, expanded by `cargo-expand`:
    //! ```rust
    //! use pin_project_lite::pin_project;
    //!
    //! pin_project! {
    //!     #[project = EitherProj]
    //!     pub enum Either<L, R> {
    //!         Left {#[pin] left: L},
    //!         Right {#[pin] right: R}
    //!     }
    //! }
    //! ```  
    //!
    //! [pin-project-lite]: https://docs.rs/pin-project-lite/latest/pin_project_lite/
    use std::marker::PhantomData;

    use super::Either;

    #[allow(dead_code)]
    #[allow(single_use_lifetimes)]
    #[allow(unknown_lints)]
    #[allow(clippy::mut_mut)]
    #[allow(clippy::redundant_pub_crate)]
    #[allow(clippy::ref_option_ref)]
    #[allow(clippy::type_repetition_in_bounds)]
    pub(crate) enum EitherProj<'__pin, L, R>
    where
        Either<L, R>: '__pin,
    {
        Left {
            left: ::pin_project_lite::__private::Pin<&'__pin mut (L)>,
        },
        Right {
            right: ::pin_project_lite::__private::Pin<&'__pin mut (R)>,
        },
    }

    #[allow(single_use_lifetimes)]
    #[allow(unknown_lints)]
    #[allow(clippy::used_underscore_binding)]
    #[allow(missing_debug_implementations)]
    const _: () = {
        #[allow(non_snake_case)]
        pub struct __Origin<'__pin, L, R> {
            __dummy_lifetime: PhantomData<&'__pin ()>,
            Left: L,
            Right: R,
        }
        impl<'__pin, L, R> Unpin for Either<L, R> where __Origin<'__pin, L, R>: Unpin {}

        trait MustNotImplDrop {}
        #[allow(drop_bounds)]
        impl<T: Drop> MustNotImplDrop for T {}
        impl<L, R> MustNotImplDrop for Either<L, R> {}
    };
}
