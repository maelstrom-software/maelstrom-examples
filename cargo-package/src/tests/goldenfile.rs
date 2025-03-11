use goldenfile::Mint;
use std::io::Write as _;

/// This is an example of using the goldenfile crate. See
/// [`cargo-maelstrom.toml`](../../cargo-maelstrom.toml) for how we configure the container. The
/// crate requires the goldenfiles to be on the container as well as /tmp to be writable.
#[test]
fn goldenfile() {
    let mut mint = Mint::new("tests/goldenfiles");
    let mut file1 = mint.new_goldenfile("file1.txt").unwrap();
    let mut file2 = mint.new_goldenfile("file2.txt").unwrap();

    writeln!(file1, "Hello world!").unwrap();
    writeln!(file2, "Foo bar!").unwrap();
}
