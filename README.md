Serde Unit Struct Derive
========================

This crate provides derive macros for Serde's `Serialize` and `Deserialize` traits on unit structs, such that the unit struct is represented by its name as a string.
This is useful if you wish to maintain type information, i.e. differentiate between different unit structs.

Without `serde_unit_struct`:
```rust
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct Foo;

#[derive(Deserialize, Serialize)]
struct Bar;

fn main() {
    // Normally, unit structs serialize to null.
    let json = serde_json::to_string(&Foo).unwrap();
    assert_eq!(json, "null");

    // We can successfully deserialize them, but...
    let foo: Foo = serde_json::from_str(&json).unwrap();
    assert_eq!(foo, Foo);

    // ...this also works; the type information is lost.
    let bar: Bar = serde_json::from_str(&json).unwrap();
    assert_eq!(bar, Bar);
}
```

With `serde_unit_struct`:
```rust
use serde_unit_struct::{Deserialize_unit_struct, Serialize_unit_struct};

#[derive(Deserialize_unit_struct, Serialize_unit_struct)]
struct Foo;

#[derive(Deserialize_unit_struct, Serialize_unit_struct)]
struct Bar;

fn main() {
    // Now, unit structs serialise to their name as a string.
    let json = serde_json::to_string(&Foo).unwrap();
    assert_eq!(json, "\"Foo\"");

    // We can successfully deserialize them.
    let foo: Foo = serde_json::from_str(&json).unwrap();
    assert_eq!(foo, Foo);

    // Type information is maintained.
    let bar: Result<Bar, _> = serde_json::from_str(&json);
    assert!(bar.is_err());
}
```

## License
Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT) at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this crate by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
