#[cfg(test)]
mod tests {
    #[test]
    fn subprocess() {
        use std::{process::Command, str};
        let output = Command::new("echo")
            .arg("Hello,")
            .arg("World!")
            .output()
            .unwrap_or_else(|err| panic!("error executing subprocess: {err:?}"));
        assert_eq!(str::from_utf8(&output.stderr).unwrap(), "");
        assert!(output.status.success());
        assert_eq!(str::from_utf8(&output.stdout).unwrap(), "Hello, World!\n");
    }
}
