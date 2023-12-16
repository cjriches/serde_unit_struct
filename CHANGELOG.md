# Changelog

## v0.1.3
* Fixed the issue in `v0.1.2`; the crate now works with all `Deserializer`s
  while still being no-std compatible.
* Updated dependency syn from 1.0 to 2.0


## v0.1.2 (YANKED)
* Made the crate `no_std`-compatible.

### Known Issues
* Changing the `Deserialize` impl broke some `Deserializer` impls that
  produce strings in a non-borrowed form, notably `toml`.


## v0.1.1
* Fixed documentation.


## v0.1.0
* Initial release.
