use std::net::IpAddr;
use std::str::FromStr;
use std::time::Duration;

use time::macros::format_description;
use time::UtcOffset;
use tokio::sync::mpsc;
use tokio::{join, spawn};
use tokio_util::sync::CancellationToken;
use tracing::{info, Level};

use nihility_common::{
    core_authentication_core_init, GrpcClientConfig, GrpcServer, GrpcServerConfig, NihilityServer,
};

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_server() {
    init_log();
    core_authentication_core_init("./auth").unwrap();
    join!(test_grpc_server(),);
    tokio::time::sleep(Duration::from_secs(30)).await;
}

async fn test_grpc_server() {
    let mut server_config = GrpcServerConfig::default();
    server_config.bind_ip = IpAddr::from_str("127.0.0.1").unwrap();
    let connection_params = server_config.create_connection_params(&"test".to_string());
    info!("connection_params: {:?}", &connection_params);
    let client_config = GrpcClientConfig::try_from(connection_params.clone()).unwrap();
    info!("client_config: {:?}", &client_config);
    let mut server = GrpcServer::init(server_config, CancellationToken::new());
    let (module_tx, mut module_rx) = mpsc::unbounded_channel();
    let (instruct_tx, mut instruct_rx) = mpsc::unbounded_channel();
    let (manipulate_tx, mut manipulate_rx) = mpsc::unbounded_channel();
    server.set_submodule_operate_sender(module_tx).unwrap();
    server.set_instruct_sender(instruct_tx).unwrap();
    server.set_manipulate_sender(manipulate_tx).unwrap();
    server.start().unwrap();
    tokio::time::sleep(Duration::from_secs(10)).await;
    info!("Start Receiver");
    spawn(async move {
        while let Some(operate) = module_rx.recv().await {
            info!("Module Operate: {:?}", operate);
        }
    });
    spawn(async move {
        while let Some(instruct) = instruct_rx.recv().await {
            info!("Instruct: {:?}", instruct);
        }
    });
    spawn(async move {
        while let Some(manipulate) = manipulate_rx.recv().await {
            info!("Manipulate: {:?}", manipulate);
        }
    });
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