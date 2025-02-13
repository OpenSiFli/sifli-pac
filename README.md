# SiFli PAC

[![Crates.io][badge-license]][crates]
[![Crates.io][badge-version]][crates]
[![docs.rs][badge-docsrs]][docsrs]
[![Support status][badge-support-status]][githubrepo]

[badge-license]: https://img.shields.io/crates/l/sifli-pac?style=for-the-badge
[badge-version]: https://img.shields.io/crates/v/sifli-pac?style=for-the-badge
[badge-docsrs]: https://img.shields.io/docsrs/sifli-pac?style=for-the-badge
[badge-support-status]: https://img.shields.io/badge/Support_status-Community-yellow?style=for-the-badge
[crates]: https://crates.io/crates/sifli-pac
[docsrs]: https://docs.rs/sifli-pac
[githubrepo]: https://github.com/OpenSiFli/sifli-pac

Rust Peripheral Access Crate (PAC) for SiFli MCUs.

## Generation

To generate the PAC, you need to install [chiptool](https://github.com/embassy-rs/chiptool), [form](https://github.com/djmcgill/form) and [rustfmt](https://github.com/rust-lang/rustfmt):

```bash
cargo install form
rustup component add rustfmt
```

Before [#61](https://github.com/embassy-rs/chiptool/pull/61) is merged, you need to install a forked version of chiptool:

```bash
cargo install --git https://github.com/decaday/chiptool --branch dev --locked
```

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