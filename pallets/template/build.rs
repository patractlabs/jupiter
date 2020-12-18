//! Use *.wat file for human readable
use std::{
    env, fs,
    io::{Error, Result},
    path::Path,
};

fn main() -> Result<()> {
    println!("cargo:rerun-if-changed=./groth16.wat");
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("groth16.wasm");
    fs::write(
        &dest_path,
        wat::parse_file("groth16.wat").map_err(|_| Error::last_os_error())?,
    )?;
    println!("cargo:rerun-if-changed=build.rs");
    Ok(())
}
