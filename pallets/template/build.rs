//! Use *.wat file for human readable
use std::{env, fs, io::Result, path::Path};

fn main() -> Result<()> {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("flip.wasm");
    fs::write(&dest_path, fs::read("flip.wat")?)?;
    println!("cargo:rerun-if-changed=build.rs");
    Ok(())
}
