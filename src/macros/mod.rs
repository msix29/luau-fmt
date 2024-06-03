//! Includes helper macros for the crate.

#[macro_export]
/// Panics when an `ERROR` variant is sent to be formatted (should never happen).
macro_rules! panic_for_error_variant {
    () => {
        panic!("Attempt to format an error variant.")
    };
}
