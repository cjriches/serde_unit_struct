/// (De)serialize a unit struct as its name.
/// ```
/// use serde_unit_struct::{Deserialize_unit_struct, Serialize_unit_struct};
///
/// #[derive(Deserialize_unit_struct, Serialize_unit_struct)]
/// struct Foo;
/// ```
pub use serde_unit_struct_derive::{Deserialize_unit_struct, Serialize_unit_struct};

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Serialize_unit_struct, Deserialize_unit_struct)]
    struct Foo;

    #[derive(Serialize_unit_struct, Deserialize_unit_struct)]
    struct Bar;

    #[test]
    fn serialize() {
        let foo = serde_json::to_string(&Foo).expect("Serializing Foo failed.");
        assert_eq!(&foo, "\"Foo\"");

        let bar = serde_json::to_string(&Bar).expect("Serializing Bar failed.");
        assert_eq!(&bar, "\"Bar\"");
    }

    #[test]
    fn deserialize() {
        let _foo = serde_json::from_str::<Foo>("\"Foo\"").expect("Deserializing Foo failed.");
        let _bar = serde_json::from_str::<Bar>("\"Bar\"").expect("Deserializing Bar failed.");
    }

    #[test]
    fn bad_deserialize() {
        let foo_err = serde_json::from_str::<Foo>("\"Bar\"");
        assert!(foo_err.is_err());
        let bar_err = serde_json::from_str::<Bar>("\"Foo\"");
        assert!(bar_err.is_err());
    }
}
