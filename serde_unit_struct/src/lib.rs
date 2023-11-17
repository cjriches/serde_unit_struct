//! Copyright (c) 2022 Chris Riches
//! (Licensed under MIT or Apache 2.0)
//!
//! (De)serialize a unit struct as its name.
//! ```
//! use serde_unit_struct::{Deserialize_unit_struct, Serialize_unit_struct};
//!
//! #[derive(Deserialize_unit_struct, Serialize_unit_struct)]
//! struct Foo;
//! ```

#![no_std]

pub use serde_unit_struct_derive::{Deserialize_unit_struct, Serialize_unit_struct};

#[cfg(test)]
mod tests {
    use super::*;

    use serde::{Deserialize, Serialize};

    #[derive(Serialize_unit_struct, Deserialize_unit_struct)]
    struct Foo;

    #[derive(Serialize_unit_struct, Deserialize_unit_struct)]
    struct Bar;

    #[derive(Serialize, Deserialize)]
    struct Config {
        foo: Foo,
        bar: Bar,
    }

    impl Config {
        pub fn new() -> Self {
            Self { foo: Foo, bar: Bar }
        }
    }

    #[test]
    fn serialize() {
        let foo = serde_json::to_string(&Foo).expect("Serializing Foo failed");
        assert_eq!(&foo, "\"Foo\"");

        let bar = serde_json::to_string(&Bar).expect("Serializing Bar failed");
        assert_eq!(&bar, "\"Bar\"");
    }

    #[test]
    fn deserialize() {
        let _foo = serde_json::from_str::<Foo>("\"Foo\"").expect("Deserializing Foo failed");
        let _bar = serde_json::from_str::<Bar>("\"Bar\"").expect("Deserializing Bar failed");
    }

    #[test]
    fn bad_deserialize() {
        let foo_err = serde_json::from_str::<Foo>("\"Bar\"");
        assert!(foo_err.is_err());
        let bar_err = serde_json::from_str::<Bar>("\"Foo\"");
        assert!(bar_err.is_err());
    }

    #[test]
    fn config_bson() {
        let conf = Config::new();
        let bsn = bson::to_raw_document_buf(&conf).expect("Serializing config to bson failed");
        bson::from_slice::<Config>(bsn.as_bytes()).expect("Deserializing config from bson failed");
    }

    #[test]
    fn config_csv() {
        let conf = Config::new();
        let mut buf = [0u8; 128];
        let mut writer = csv::Writer::from_writer(&mut buf[..]);
        writer
            .serialize(&conf)
            .expect("Serializing config to csv failed");
        drop(writer);
        let mut reader = csv::Reader::from_reader(&buf[..]).into_deserialize::<Config>();
        reader
            .next()
            .unwrap()
            .expect("Deserializing config from csv failed");
    }

    #[test]
    fn config_csv_bad() {
        let csv_str = "foo,bar\nBar,Foo";
        let mut reader = csv::Reader::from_reader(csv_str.as_bytes()).into_deserialize::<Config>();
        assert!(reader.next().unwrap().is_err());
    }

    #[test]
    fn config_postcard() {
        let conf = Config::new();
        let mut buf = [0u8; 128];
        let pc =
            postcard::to_slice(&conf, &mut buf).expect("Serializing config to postcard failed");
        postcard::from_bytes::<Config>(pc).expect("Deserializing config from postcard failed");
    }

    #[test]
    fn config_json() {
        let conf = Config::new();
        let json = serde_json::to_string(&conf).expect("Serializing config to json failed");
        serde_json::from_str::<Config>(&json).expect("Deserializing config from json failed");
    }

    #[test]
    fn config_json_bad() {
        let json = "{\"foo\": \"foo\", \"bar\": \"foo\"}";
        let err = serde_json::from_str::<Config>(json);
        assert!(err.is_err());
    }

    #[test]
    fn config_pickle() {
        let conf = Config::new();
        let ser_options = serde_pickle::SerOptions::new();
        let pickle =
            serde_pickle::to_vec(&conf, ser_options).expect("Serializing config to pickle failed");
        let de_options = serde_pickle::DeOptions::new();
        serde_pickle::from_slice::<Config>(&pickle, de_options)
            .expect("Deserializing config from pickle failed");
    }

    #[test]
    fn config_yaml() {
        let conf = Config::new();
        let yaml = serde_yaml::to_string(&conf).expect("Serializing config to yaml failed");
        serde_yaml::from_str::<Config>(&yaml).expect("Deserializing config from yaml failed");
    }

    #[test]
    fn config_yaml_bad() {
        let yaml = "foo: Foo\nbar: 3";
        let err = serde_yaml::from_str::<Config>(yaml);
        assert!(err.is_err());
    }

    #[test]
    fn config_toml() {
        let conf = Config::new();
        let tml = toml::to_string(&conf).expect("Serializing config to toml failed");
        toml::from_str::<Config>(&tml).expect("Deserializing config from toml failed");
    }

    #[test]
    fn config_toml_bad() {
        let tml = "foo = \"Apple\"\nbar = \"Banana\"";
        let err = toml::from_str::<Config>(tml);
        assert!(err.is_err());
    }
}
