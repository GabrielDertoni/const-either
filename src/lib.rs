//! Some types to allow deciding at compile time if an option contains a value or which variant
//! from the either type is active. This might be useful when you have some const generic type that
//! should decide whether to use one datastructure or another, or no datastructure at all.
//!
//! # Syntax
//!
//! ```ignore
//! let _definetly_none = ConstOption::<String, false>::new();
//! let definetly_some = ConstOption::<String, true>::new("hello, world".to_string());
//!
//! // When there is definetly some value, the deref trait can be used.
//! println!("{}", &*definetly_some);
//!
//! // Obtain the string inside.
//! let contained_string = definetly_some.into_inner();
//!
//!
//! struct Container<T, const IS_UNIQUE: bool> {
//!     data: ConstEither<Vec<T>, HashSet<T>, UNIQUE>,
//! }
//!
//! impl<T> Container<T, false> {
//!     fn insert(&mut self, val: T) {
//!         /* ... */
//!     }
//! }
//!
//! impl<T: Eq + Hash> Container<T, true> {
//!     fn insert(&mut self, val: T) -> Result<(), T> {
//!         /* ... */
//!     }
//! }
//! ```
//!

#![allow(incomplete_features)]
#![feature(specialization, inherent_associated_types, never_type)]

use std::ops::{Deref, DerefMut};

pub struct ConstOption<T, const IS_SOME: bool>(
    <ConstOptionWrapper<IS_SOME> as ConstOptionStorage<T>>::Store,
);

struct ConstOptionWrapper<const B: bool>;

trait ConstOptionStorage<T> {
    type Store;
}

impl<T, const B: bool> ConstOptionStorage<T> for ConstOptionWrapper<B> {
    default type Store = !;
}

impl<T> ConstOptionStorage<T> for ConstOptionWrapper<false> {
    type Store = ();
}

impl<T> ConstOptionStorage<T> for ConstOptionWrapper<true> {
    type Store = T;
}

impl<T> ConstOption<T, false> {
    pub fn new() -> Self {
        ConstOption(())
    }
}

impl<T> ConstOption<T, true> {
    pub fn new(val: T) -> Self {
        ConstOption(val)
    }

    pub fn into_inner(self) -> T { self.0 }
}

impl<T> AsRef<T> for ConstOption<T, true> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<T> AsMut<T> for ConstOption<T, true> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T> Deref for ConstOption<T, true> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<T> DerefMut for ConstOption<T, true> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut()
    }
}

pub struct ConstEither<L, R, const IS_RIGHT: bool>(
    <ConstEitherWrapper<IS_RIGHT> as ConstEitherStorage<L, R>>::Store,
);

trait ConstEitherStorage<L, R> {
    type Store;
}

struct ConstEitherWrapper<const B: bool>;

impl<L, R, const B: bool> ConstEitherStorage<L, R> for ConstEitherWrapper<B> {
    default type Store = !;
}

impl<L, R> ConstEitherStorage<L, R> for ConstEitherWrapper<false> {
    type Store = L;
}

impl<L, R> ConstEitherStorage<L, R> for ConstEitherWrapper<true> {
    type Store = R;
}

impl<L, R> ConstEither<L, R, false> {
    pub fn new(left: L) -> Self {
        ConstEither(left)
    }

    pub fn into_inner(self) -> L {
        self.0
    }

    pub fn flip(self) -> ConstEither<R, L, true> {
        ConstEither::<R, L, true>::new(self.0)
    }
}

impl<L, R> ConstEither<L, R, true> {
    pub fn new(right: R) -> Self {
        ConstEither(right)
    }

    pub fn into_inner(self) -> R {
        self.0
    }

    pub fn flip(self) -> ConstEither<R, L, false> {
        ConstEither::<R, L, false>::new(self.0)
    }
}

impl<L, R> AsRef<L> for ConstEither<L, R, false> {
    fn as_ref(&self) -> &L {
        &self.0
    }
}

impl<L, R> AsRef<R> for ConstEither<L, R, true> {
    fn as_ref(&self) -> &R {
        &self.0
    }
}

impl<L, R> AsMut<L> for ConstEither<L, R, false> {
    fn as_mut(&mut self) -> &mut L {
        &mut self.0
    }
}

impl<L, R> AsMut<R> for ConstEither<L, R, true> {
    fn as_mut(&mut self) -> &mut R {
        &mut self.0
    }
}

impl<L, R> Deref for ConstEither<L, R, false> {
    type Target = L;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<L, R> Deref for ConstEither<L, R, true> {
    type Target = R;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<L, R> DerefMut for ConstEither<L, R, false> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut()
    }
}

impl<L, R> DerefMut for ConstEither<L, R, true> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut()
    }
}

#[cfg(test)]
mod tests {
    use std::convert::Infallible;

    use super::*;

    #[test]
    fn nothing() {
        let _none = ConstOption::<Infallible, false>::new();
        let mut right = ConstEither::<Infallible, usize, true>::new(1234);

        assert_eq!(*right, 1234);
        *right = 456;
        assert_eq!(right.into_inner(), 456);
    }

    #[test]
    fn something() {
        let some = ConstOption::<String, true>::new("Hello, world".to_string());
        assert_eq!(*some, "Hello, world");
    }
}
