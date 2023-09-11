## Rust Docs

Documentation [link](https://rust-lang.github.io/api-guidelines/documentation.html)

Several important points:
– Cargo.toml should include all the necessary metadata
– It is possible to use md-alike links to other documentation pages
– Error conditions should be included into documentation, ofc only those, which not expected by default, such as Error type, if someone is passing `str` instead of `i32`,
those kind of errors should not be listed in the documentation
– Examples should use a question mark – `?`, not `try!`/`unwrap`:
```rust-lang
/// ```rust
/// # use std::error::Error;
/// #
/// # fn main() -> Result<(), Box<dyn Error>> {
/// your;
/// example?;
/// code;
/// #
/// #     Ok(())
/// # }
/// ```
```

– All release notes document all significant changes
– Rustdoc should just enough users needs to use the crate, and nothing more. That means some underhood logic and its docs should be hidden.
