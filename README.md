# SiFli PAC

[![Crates.io][badge-license]][crates]
[![Crates.io][badge-version]][crates]
[![docs.rs][badge-docsrs]][docsrs]

[badge-license]: https://img.shields.io/crates/l/sifli-pac?style=for-the-badge
[badge-version]: https://img.shields.io/crates/v/sifli-pac?style=for-the-badge
[badge-docsrs]: https://img.shields.io/docsrs/sifli-pac?style=for-the-badge
[crates]: https://crates.io/crates/sifli-pac
[docsrs]: https://docs.rs/sifli-pac

Rust Peripheral Access Crate (PAC) for SiFli MCUs.

## Generation

To generate the PAC, you need to install [chiptool](https://github.com/embassy-rs/chiptool), [form](https://github.com/djmcgill/form) and [rustfmt](https://github.com/rust-lang/rustfmt):

```bash
cargo install --git https://github.com/embassy-rs/chiptool --locked
cargo install form
rustup component add rustfmt
```

If you have an older version of chiptool installed, you may need to update it (2025.1.22 and later).

Then run the following in Bash or PowerShell:

```
./update
```

This crate is inspired by the project structure of [rp-pac](https://github.com/embassy-rs/rp-pac).

## License

This project is licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.