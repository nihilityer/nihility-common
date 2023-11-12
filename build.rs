fn main() {
    tonic_build::configure()
        .compile(
            &[
                "proto/manipulate.proto",
                "proto/instruct.proto",
                "proto/sub_module.proto",
                "proto/response_code.proto",
            ],
            &["proto"],
        )
        .expect("failed to compile protos");
}
