fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Tell Cargo to tell rustc to link the system bzip2
    // shared library.
    tonic_build::compile_protos("src/proto/spin.proto")?;
    Ok(())
}