fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_prost_build::compile_protos("proto/streaming.proto")?;
    Ok(())
}

// find ../../target | grep streaming.rs
