# Changelog


## v0.1.2 (YANKED)
* Made the crate `no_std`-compatible.

### Known Issues
* Changing the `Deserialize` impl broke some `Deserializer` impls that
  produce strings in a non-borrowed form, notably `toml`.


## v0.1.1
* Fixed documentation.


## v0.1.0
* Initial release.
