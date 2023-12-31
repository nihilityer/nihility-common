use std::collections::HashMap;
use std::net::IpAddr;
use std::str::FromStr;
use std::time::Duration;

use time::macros::format_description;
use time::UtcOffset;
use tokio::join;
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;
use tracing::{error, info, Level};

use nihility_common::{
    ClientType, ConnParams, ConnectionType, GrpcClient, GrpcClientConfig, GrpcServer,
    GrpcServerConfig, InstructData, InstructEntity, InstructInfoEntity, ManipulateData,
    ManipulateEntity, ManipulateInfoEntity, ModuleOperate, NihilityClient, NihilityServer,
    OperateType, SubmoduleInfo,
};

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test() {
    init_log();
    join!(test_grpc_server(), test_grpc_client());
    tokio::time::sleep(Duration::from_secs(15)).await;
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
    match module_rx.try_recv() {
        Ok(operate) => {
            info!("Module Operate: {:?}", operate);
        }
        Err(e) => {
            error!("error: {}", e)
        }
    }
    match instruct_rx.try_recv() {
        Ok(instruct) => {
            info!("Instruct: {:?}", instruct);
        }
        Err(e) => {
            error!("error: {}", e)
        }
    }
    match manipulate_rx.try_recv() {
        Ok(manipulate) => {
            info!("Manipulate: {:?}", manipulate);
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
    client.connection_instruct_server().await.unwrap();
    client.connection_manipulate_server().await.unwrap();
    client
        .register(ModuleOperate {
            name: String::from("test"),
            info: Some(SubmoduleInfo {
                default_instruct: vec![String::from("test_instruct")],
                conn_params: ConnParams {
                    connection_type: ConnectionType::GrpcType,
                    client_type: ClientType::NotReceiveType,
                    conn_params: HashMap::new(),
                },
            }),
            operate_type: OperateType::Register,
        })
        .await
        .unwrap();
    client
        .text_instruct(InstructEntity {
            info: InstructInfoEntity::default(),
            instruct: InstructData::Text(String::from("test send instruct")),
        })
        .await
        .unwrap();
    client
        .simple_manipulate(ManipulateEntity {
            info: ManipulateInfoEntity::default(),
            manipulate: ManipulateData::Simple,
        })
        .await
        .unwrap();
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
