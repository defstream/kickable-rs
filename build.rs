fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Use a vendored protoc so no system protobuf-compiler is required in any
    // build environment (local, CI, or the cross-rs containers). This avoids a
    // Cross.toml `pre-build` step, which would force a custom-image `docker
    // build` against cross-rs's amd64-only images and fail on arm64 hosts.
    std::env::set_var("PROTOC", protoc_bin_vendored::protoc_bin_path()?);
    tonic_prost_build::compile_protos("proto/kickable/kickable.proto")?;
    Ok(())
}
