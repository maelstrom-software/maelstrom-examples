#[cfg(test)]
mod tests {
    /// This test requires a `/tmp` directory that is writable. This is configured in
    /// `cargo-maelstrom.toml`.
    #[test]
    fn tempfile() {
        use ::tempfile::tempfile;
        use std::io::{Read as _, Seek as _, Write as _};

        let mut file = tempfile().unwrap();
        file.write_all(b"Hello, World!").unwrap();
        file.rewind().unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        assert_eq!(contents, "Hello, World!");
    }
}
