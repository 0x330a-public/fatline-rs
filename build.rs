fn main() -> Result<(), Box<dyn std::error::Error>>{
    tonic_build::configure()
        .build_server(cfg!(server))
        .build_client(cfg!(client))
        .compile(
            &[
                "proto/protobufs/schemas/rpc.proto"
            ],
            &["proto/protobufs/schemas"]
        )?;

    Ok(())
}