use std::collections::HashMap;
use std::time::Duration;

use time::macros::format_description;
use time::UtcOffset;
use tokio::sync::mpsc;
use tracing::{info, Level};

use nihility_common::{
    set_core_public_key_path, set_submodule_name, ClientType, ConnParams, ConnectionType,
    GrpcClient, GrpcClientConfig, InstructData, InstructEntity, ManipulateData, ManipulateEntity,
    ModuleOperate, NihilityClient, OperateType, SubmoduleInfo,
};

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_client() {
    init_log();
    tokio::time::sleep(Duration::from_secs(3)).await;
    set_submodule_name(String::from("test"));
    set_core_public_key_path(String::from("./auth/id_rsa.pub"));
    test_grpc_submodule_operate_client().await;
    test_grpc_instruct_client().await;
    test_grpc_manipulate_client().await;
    tokio::time::sleep(Duration::from_secs(15)).await;
}

async fn test_grpc_submodule_operate_client() {
    info!("Sleep, Wait Server Start");
    let config = GrpcClientConfig::default();
    let mut client = GrpcClient::init(config);
    client.connection_submodule_operate_server().await.unwrap();
    info!("Connection Success!");
    client
        .register(SubmoduleInfo {
            default_instruct: vec![String::from("test_instruct")],
            conn_params: ConnParams {
                connection_type: ConnectionType::GrpcType,
                client_type: ClientType::NotReceiveType,
                conn_config: HashMap::new(),
            },
        })
        .await
        .unwrap();
    info!("register finish");
    client
        .update(SubmoduleInfo {
            default_instruct: vec![String::from("test_instruct")],
            conn_params: ConnParams {
                connection_type: ConnectionType::GrpcType,
                client_type: ClientType::NotReceiveType,
                conn_config: HashMap::new(),
            },
        })
        .await
        .unwrap();
    info!("update finish");
    let mut operate = ModuleOperate::default();
    operate.name = String::from("test");
    operate.operate_type = OperateType::Heartbeat;
    client.heartbeat().await.unwrap();
    info!("heartbeat finish");
    client
        .offline(SubmoduleInfo {
            default_instruct: vec![String::from("test_instruct")],
            conn_params: ConnParams {
                connection_type: ConnectionType::GrpcType,
                client_type: ClientType::NotReceiveType,
                conn_config: HashMap::new(),
            },
        })
        .await
        .unwrap();
    info!("heartbeat finish");
}

async fn test_grpc_instruct_client() {
    info!("Sleep, Wait Server Start");
    let config = GrpcClientConfig::default();
    let mut client = GrpcClient::init(config);
    client.connection_instruct_server().await.unwrap();
    info!("Connection Success!");
    let mut instruct = InstructEntity::default();
    instruct.instruct = InstructData::Text(String::from("test send instruct"));
    client.text_instruct(instruct).await.unwrap();
    info!("text_instruct finish");
    let (tx, rx) = mpsc::channel(1);
    let mut instruct = InstructEntity::default();
    instruct.instruct = InstructData::Text(String::from("test send instruct"));
    tx.send(instruct).await.unwrap();
    client.multiple_text_instruct(rx).await.unwrap();
    info!("multiple_text_instruct finish");
}

async fn test_grpc_manipulate_client() {
    info!("Sleep, Wait Server Start");
    let config = GrpcClientConfig::default();
    let mut client = GrpcClient::init(config);
    client.connection_manipulate_server().await.unwrap();
    info!("Connection Success!");
    let mut manipualte = ManipulateEntity::default();
    manipualte.manipulate = ManipulateData::Simple;
    client.simple_manipulate(manipualte).await.unwrap();
    info!("simple_manipulate finish");
    let mut manipualte = ManipulateEntity::default();
    manipualte.manipulate = ManipulateData::Text(String::from("text_display_manipulate"));
    client.text_display_manipulate(manipualte).await.unwrap();
    info!("text_display_manipulate finish");
    let (tx, rx) = mpsc::channel(1);
    let mut manipualte = ManipulateEntity::default();
    manipualte.manipulate = ManipulateData::Text(String::from("multiple_text_display_manipulate"));
    tx.send(manipualte).await.unwrap();
    client.multiple_text_display_manipulate(rx).await.unwrap();
    info!("multiple_text_display_manipulate finish");
    let mut manipualte = ManipulateEntity::default();
    manipualte.manipulate = ManipulateData::ConnectionParams(ConnParams {
        connection_type: ConnectionType::GrpcType,
        client_type: ClientType::NotReceiveType,
        conn_config: Default::default(),
    });
    client
        .direct_connection_manipulate(manipualte)
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
        .with_max_level(Level::DEBUG)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_target(true)
        .with_timer(timer)
        .finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();
    tracing::debug!("log subscriber init success");
}
