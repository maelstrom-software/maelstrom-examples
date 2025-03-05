This project is an example of a cargo [workspace with a root package]
(https://doc.rust-lang.org/cargo/reference/workspaces.html#root-package).

Filter patterns can use the `package`
[selector](https://maelstrom-software.com/doc/book/latest/cargo-maelstrom/filter.html#compound-selectors)
when necessary to distinguish between packages in the workspace. The root
package isn't handled any differently. The name defined in the top-level
[`Cargo.toml`](Cargo.toml) is used.
