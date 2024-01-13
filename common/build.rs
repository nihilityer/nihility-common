fn main() {
    tonic_build::configure()
        .compile(
            &[
                "proto/manipulate.proto",
                "proto/instruct.proto",
                "proto/submodule.proto",
                "proto/response_code.proto",
            ],
            &["proto"],
        )
        .expect("failed to compile protos");
}
