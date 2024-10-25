use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_files = vec![
        "proto/key_manager.proto",
        "proto/signer.proto"
    ];

    let proto_include_dirs = vec![
        PathBuf::from("proto"),
    ];

    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        // Optionally add custom configuration
        .out_dir("src/generated") // Optional: output generated code to a specific directory
        .compile(
            &proto_files,
            &proto_include_dirs,
        )?;

    // Ensure cargo tracks changes to proto files
    for proto_file in proto_files {
        println!("cargo:rerun-if-changed={}", proto_file);
    }

    Ok(())
}