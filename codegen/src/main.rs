fn main() {
    tonic_build::configure()
        .out_dir("substreams-starknet/src/protobuf")
        .compile(&["proto/zklend.starknet.type.v1.proto"], &["proto"])
        .expect("Failed to compile Firehose Starknet proto");
}
