use std::{env, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);

    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("spin_descriptor.bin"))
        .compile(&["src/proto/spin.proto"], &["src/proto"])?;

    // Tell Cargo to tell rustc to link the system bzip2
    // shared library.
    tonic_build::compile_protos("src/proto/spin.proto")?;
    Ok(())
}