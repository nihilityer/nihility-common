use std::net::IpAddr;
use std::str::FromStr;
use std::time::Duration;

use time::macros::format_description;
use time::UtcOffset;
use tokio::join;
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;
use tracing::{error, info, Level};

use nihility_common::{GrpcClient, GrpcClientConfig, GrpcServer, GrpcServerConfig, NihilityClient, NihilityServer};

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test() {
    init_log();
    join!(test_grpc_server(), test_grpc_client());
    tokio::time::sleep(Duration::from_secs(15)).await;
}

async fn test_grpc_server() {
    let mut server_config = GrpcServerConfig::default();
    server_config.bind_ip = IpAddr::from_str("127.0.0.1").unwrap();
    let mut server = GrpcServer::init(server_config, CancellationToken::new());
    let (tx, mut rx) = mpsc::unbounded_channel();
    server.set_submodule_operate_sender(tx).unwrap();
    server.start().unwrap();
    tokio::time::sleep(Duration::from_secs(10)).await;
    match rx.try_recv() {
        Ok(operate) => {
            info!("operate: {:?}", operate);
        }
        Err(e) => {
            error!("error: {}", e)
        }
    }
}

async fn test_grpc_client() {
    info!("Sleep, Wait Server Start");
    tokio::time::sleep(Duration::from_secs(5)).await;
    let config = GrpcClientConfig::default();
    let mut client = GrpcClient::init(config);
    client.connection_submodule_operate_server().await.unwrap();
    info!("Connection Success!");
}

fn init_log() {
    let subscriber = tracing_subscriber::fmt().compact();
    let timer = tracing_subscriber::fmt::time::OffsetTime::new(
        UtcOffset::from_hms(8, 0, 0).unwrap(),
        format_description!("[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]"),
    );
    let subscriber = subscriber
        .with_file(false)
        .with_max_level(Level::DEBUG)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_target(true)
        .with_timer(timer)
        .finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();
    tracing::debug!("log subscriber init success");
}