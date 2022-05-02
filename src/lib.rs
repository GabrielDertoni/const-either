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
//! # Drawbacks
//!
//! Because of the current state of rust, the type `ConstEither<L, R>` **will have the size and
//! alignment of the largest** from `L` and `R`.
//!

use std::{mem::ManuallyDrop, ops::{Deref, DerefMut}};

/// An `Option` type that is known to have or not the value at compile time.
pub struct ConstOption<T, const IS_SOME: bool>(ConstOptionInner<T, IS_SOME>);

union ConstOptionInner<T, const IS_SOME: bool> {
    none: (),
    some: ManuallyDrop<T>,
}

impl<T> ConstOption<T, false> {
    pub fn new() -> Self {
        ConstOption(ConstOptionInner { none: () })
    }
}

impl<T> ConstOption<T, true> {
    pub fn new(val: T) -> Self {
        ConstOption(ConstOptionInner { some: ManuallyDrop::new(val) })
    }

    pub fn into_inner(mut self) -> T {
        unsafe { ManuallyDrop::take(&mut self.0.some) }
    }
}

impl<T, const IS_SOME: bool> Drop for ConstOption<T, IS_SOME> {
    fn drop(&mut self) {
        unsafe {
            if IS_SOME {
                drop(ManuallyDrop::take(&mut self.0.some))
            }
        }
    }
}

impl<T> AsRef<T> for ConstOption<T, true> {
    fn as_ref(&self) -> &T {
        unsafe { &self.0.some }
    }
}

impl<T> AsMut<T> for ConstOption<T, true> {
    fn as_mut(&mut self) -> &mut T {
        unsafe { &mut self.0.some }
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

pub struct ConstEither<L, R, const IS_RIGHT: bool>(ConstEitherInner<L, R, IS_RIGHT>);

union ConstEitherInner<L, R, const IS_RIGHT: bool> {
    left: ManuallyDrop<L>,
    right: ManuallyDrop<R>,
}


impl<L, R> ConstEither<L, R, false> {
    pub fn new(left: L) -> Self {
        ConstEither(ConstEitherInner { left: ManuallyDrop::new(left) })
    }

    pub fn into_inner(mut self) -> L {
        unsafe { ManuallyDrop::take(&mut self.0.left) }
    }

    pub fn flip(self) -> ConstEither<R, L, true> {
        let val = self.into_inner();
        ConstEither::<R, L, true>::new(val)
    }
}

impl<L, R> ConstEither<L, R, true> {
    pub fn new(right: R) -> Self {
        ConstEither(ConstEitherInner { right: ManuallyDrop::new(right) })
    }

    pub fn into_inner(mut self) -> R {
        unsafe { ManuallyDrop::take(&mut self.0.right) }
    }

    pub fn flip(self) -> ConstEither<R, L, false> {
        let val = self.into_inner();
        ConstEither::<R, L, false>::new(val)
    }
}

impl<L, R> AsRef<L> for ConstEither<L, R, false> {
    fn as_ref(&self) -> &L {
        unsafe { &self.0.left }
    }
}

impl<L, R> AsRef<R> for ConstEither<L, R, true> {
    fn as_ref(&self) -> &R {
        unsafe { &self.0.right }
    }
}

impl<L, R> AsMut<L> for ConstEither<L, R, false> {
    fn as_mut(&mut self) -> &mut L {
        unsafe { &mut self.0.left }
    }
}

impl<L, R> AsMut<R> for ConstEither<L, R, true> {
    fn as_mut(&mut self) -> &mut R {
        unsafe { &mut self.0.right }
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

impl<L, R, const IS_RIGHT: bool> Drop for ConstEither<L, R, IS_RIGHT> {
    fn drop(&mut self) {
        unsafe {
            if IS_RIGHT {
                drop(ManuallyDrop::take(&mut self.0.right));
            } else {
                drop(ManuallyDrop::take(&mut self.0.left));
            }
        }
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
