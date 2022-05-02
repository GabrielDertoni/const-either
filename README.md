# Const Enum

Some types to allow deciding at compile time if an option contains a value or which variant
from the either type is active. This might be useful when you have some const generic type that
should decide whether to use one datastructure or another, or no datastructure at all.

## Syntax

```rust
let _definetly_none = ConstOption::<String, false>::new();
let definetly_some = ConstOption::<String, true>::new("hello, world".to_string());

// When there is definetly some value, the deref trait can be used.
println!("{}", &*definetly_some);

// Obtain the string inside.
let contained_string = definetly_some.into_inner();


struct Container<T, const IS_UNIQUE: bool> {
    data: ConstEither<Vec<T>, HashSet<T>, UNIQUE>,
}

impl<T> Container<T, false> {
    fn insert(&mut self, val: T) {
        /* ... */
    }
}

impl<T: Eq + Hash> Container<T, true> {
    fn insert(&mut self, val: T) -> Result<(), T> {
        /* ... */
    }
}
```

## Drawbacks

Because of the current state of rust, the type `ConstEither<L, R>` **will have the size and
alignment of the largest** from `L` and `R`.

