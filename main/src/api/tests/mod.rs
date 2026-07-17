//! Scaffold for external tests of standalone `pub fn` items declared directly in `api/`.
//!
//! `edge-domain-event`'s `api/` layer exposes only trait contracts and value types — it has
//! no standalone `pub fn` items to test here. Trait method coverage lives in the crate-root
//! `tests/` directory per [`trait_has_test_file`] and [`source_module_has_test_file`].
