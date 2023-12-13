use std::sync::OnceLock;
use tokio_util::sync::CancellationToken;

pub mod communicat;
mod entity;
mod error;

pub(crate) static CANCELLATION_TOKEN: OnceLock<CancellationToken> = OnceLock::new();

pub(crate) mod manipulate {
    tonic::include_proto!("manipulate");
}

pub(crate) mod instruct {
    tonic::include_proto!("instruct");
}

pub(crate) mod submodule {
    tonic::include_proto!("submodule");
}

pub(crate) mod response_code {
    tonic::include_proto!("response_code");
}
