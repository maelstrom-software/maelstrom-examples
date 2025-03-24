use std::{env, process::Command, str};

/// The use of [`Command`]'s `output` or `spawn` requires three things from the Maelstrom container:
///   - If any of stdin, stdout, and stderr are [`std::process::Stdio::null()`], which is the
///     default, then there needs to be a [`/dev/null`] file that always returns EOF. The default
///     configuration contains this.
///   - The program to be executed must exist in the container.
///   - If the program to be executed has shared library dependencies, then the dependencies must
///     also exist in the container.
#[test]
fn echo() {
    let output = Command::new(env!("CARGO_BIN_EXE_say-hello"))
        .output()
        .unwrap_or_else(|err| panic!("error executing say-hello: {err:?}"));
    assert_eq!(str::from_utf8(&output.stderr).unwrap(), "");
    assert!(output.status.success());
    assert_eq!(str::from_utf8(&output.stdout).unwrap(), "Hello, World!\n");
}
