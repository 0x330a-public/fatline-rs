fn main() -> Result<(), Box<dyn std::error::Error>> {

    tonic_build::configure()
        .build_server(false)
        .build_client(cfg!(feature = "client"))
        .compile(
            &[
                "proto/protobufs/schemas/rpc.proto",
            ],
            &["proto/protobufs/schemas"]
        )?;

    Ok(())
}