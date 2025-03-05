#[cfg(test)]
mod tests {
    use rstest::rstest;

    #[test]
    fn one_plus_one() {
        assert_eq!(1 + 1, 2);
    }

    /// This test requires a `/tmp` directory that is writable. We use rstest to create two
    /// separate tests. This allows us to configure them differently in `cargo-maelstrom.toml`.
    #[rstest]
    #[case::tmpfs(())]
    #[case::writable_root(())]
    fn tempfile(#[case] _ignore: ()) {
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
