use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
fn main() {
    if env::var_os("CARGO_FEATURE_RT").is_some() {
        let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
        if env::var_os("CARGO_FEATURE_SF32LB52X").is_some() {
            File::create(out.join("device.x"))
                .unwrap()
                .write_all(include_bytes!("devices/sf32lb52x/device.x"))
                .unwrap();
        }

        println!("cargo:rustc-link-search={}", out.display());
        println!("cargo:rerun-if-changed=device.x");
    }
    println!("cargo:rerun-if-changed=build.rs");
}
