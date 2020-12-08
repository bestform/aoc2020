#[macro_export]
macro_rules! rd {
    ($path:expr) => {
        ::std::fs::read_to_string($path)
        .expect("Something went wrong reading the file");
    }
}
