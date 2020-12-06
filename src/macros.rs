#[macro_export]
macro_rules! crate_relative {
    ($path:expr) => {
        concat!(env!("CARGO_MANIFEST_DIR"), "/", $path)
    };
}
