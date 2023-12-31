use std::collections::HashMap;
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
    ClientType, ConnParams, ConnectionType, GrpcClient, GrpcClientConfig, GrpcServer,
    GrpcServerConfig, InstructData, InstructEntity, InstructInfoEntity, ManipulateData,
    ManipulateEntity, ManipulateInfoEntity, ModuleOperate, NihilityClient, NihilityServer,
    OperateType, SubmoduleInfo,
};

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test() {
    init_log();
    join!(
        test_grpc_server(),
        test_grpc_submodule_operate_client(),
        test_grpc_instruct_client(),
        test_grpc_manipulate_client()
    );
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

async fn test_grpc_submodule_operate_client() {
    info!("Sleep, Wait Server Start");
    tokio::time::sleep(Duration::from_secs(5)).await;
    let config = GrpcClientConfig::default();
    let mut client = GrpcClient::init(config);
    client.connection_submodule_operate_server().await.unwrap();
    info!("Connection Success!");
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
    info!("register finish");
    client
        .update(ModuleOperate {
            name: String::from("test"),
            info: Some(SubmoduleInfo {
                default_instruct: vec![String::from("test_instruct")],
                conn_params: ConnParams {
                    connection_type: ConnectionType::GrpcType,
                    client_type: ClientType::NotReceiveType,
                    conn_params: HashMap::new(),
                },
            }),
            operate_type: OperateType::Update,
        })
        .await
        .unwrap();
    info!("update finish");
    client
        .heartbeat(ModuleOperate {
            name: String::from("test"),
            info: Some(SubmoduleInfo {
                default_instruct: vec![String::from("test_instruct")],
                conn_params: ConnParams {
                    connection_type: ConnectionType::GrpcType,
                    client_type: ClientType::NotReceiveType,
                    conn_params: HashMap::new(),
                },
            }),
            operate_type: OperateType::Heartbeat,
        })
        .await
        .unwrap();
    info!("heartbeat finish");
    client
        .offline(ModuleOperate {
            name: String::from("test"),
            info: Some(SubmoduleInfo {
                default_instruct: vec![String::from("test_instruct")],
                conn_params: ConnParams {
                    connection_type: ConnectionType::GrpcType,
                    client_type: ClientType::NotReceiveType,
                    conn_params: HashMap::new(),
                },
            }),
            operate_type: OperateType::Offline,
        })
        .await
        .unwrap();
    info!("heartbeat finish");
}

async fn test_grpc_instruct_client() {
    info!("Sleep, Wait Server Start");
    tokio::time::sleep(Duration::from_secs(5)).await;
    let config = GrpcClientConfig::default();
    let mut client = GrpcClient::init(config);
    client.connection_instruct_server().await.unwrap();
    info!("Connection Success!");
    client
        .text_instruct(InstructEntity {
            info: InstructInfoEntity::default(),
            instruct: InstructData::Text(String::from("test send instruct")),
        })
        .await
        .unwrap();
    info!("text_instruct finish");
    let (tx, rx) = mpsc::channel(1);
    tx.send(InstructEntity {
        info: InstructInfoEntity::default(),
        instruct: InstructData::Text(String::from("test send instruct")),
    })
    .await
    .unwrap();
    client.multiple_text_instruct(rx).await.unwrap();
    info!("multiple_text_instruct finish");
}

async fn test_grpc_manipulate_client() {
    info!("Sleep, Wait Server Start");
    tokio::time::sleep(Duration::from_secs(5)).await;
    let config = GrpcClientConfig::default();
    let mut client = GrpcClient::init(config);
    client.connection_manipulate_server().await.unwrap();
    info!("Connection Success!");
    client
        .simple_manipulate(ManipulateEntity {
            info: ManipulateInfoEntity::default(),
            manipulate: ManipulateData::Simple,
        })
        .await
        .unwrap();
    info!("simple_manipulate finish");
    client
        .text_display_manipulate(ManipulateEntity {
            info: ManipulateInfoEntity::default(),
            manipulate: ManipulateData::Text(String::from("text_display_manipulate")),
        })
        .await
        .unwrap();
    info!("text_display_manipulate finish");
    let (tx, rx) = mpsc::channel(1);
    tx.send(ManipulateEntity {
        info: ManipulateInfoEntity::default(),
        manipulate: ManipulateData::Text(String::from("multiple_text_display_manipulate")),
    })
    .await
    .unwrap();
    client.multiple_text_display_manipulate(rx).await.unwrap();
    info!("multiple_text_display_manipulate finish");
    client
        .direct_connection_manipulate(ManipulateEntity {
            info: ManipulateInfoEntity::default(),
            manipulate: ManipulateData::ConnectionParams(Box::new(ConnParams {
                connection_type: ConnectionType::GrpcType,
                client_type: ClientType::NotReceiveType,
                conn_params: Default::default(),
            })),
        })
        .await
        .unwrap();
    info!("direct_connection_manipulate finish");
}

fn init_log() {
    let subscriber = tracing_subscriber::fmt().compact();
    let timer = tracing_subscriber::fmt::time::OffsetTime::new(
        UtcOffset::from_hms(8, 0, 0).unwrap(),
        format_description!("[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]"),
    );
    let subscriber = subscriber
        .with_file(false)
        .with_max_level(Level::INFO)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_target(true)
        .with_timer(timer)
        .finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();
    tracing::debug!("log subscriber init success");
}
