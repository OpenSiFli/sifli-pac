# sifli-pac

Rust Peripheral Access Crate (PAC) for SiFli MCUs.

## Usage

To generate the PAC, you need to install [chiptool](https://github.com/embassy-rs/chiptool) ,[form](https://github.com/djmcgill/form) and [rustfmt](https://github.com/rust-lang/rustfmt):

```bash
cargo install --git https://github.com/embassy-rs/chiptool --locked
cargo install form
rustup component add rustfmt
```

If you have an older version of chiptool installed, you may need to update it.

Then run the following in Bash or PowerShell:

```
./update
```

This crate is inspired by the project structure of [rp-pac](https://github.com/embassy-rs/rp-pac).

## License

This project is licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT licensWe ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.