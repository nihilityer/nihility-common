use std::collections::HashMap;
use std::time::Duration;

use time::macros::format_description;
use time::UtcOffset;
use tokio::join;
use tokio::sync::mpsc;
use tracing::{info, Level};

use nihility_common::{
    set_entity_submodule_sign, ClientType, ConnParams, ConnectionType, GrpcClient,
    GrpcClientConfig, InstructData, InstructEntity, ManipulateData, ManipulateEntity,
    ModuleOperate, NihilityClient, OperateType, SubmoduleInfo,
};

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_client() {
    init_log();
    tokio::time::sleep(Duration::from_secs(3)).await;
    join!(
        test_grpc_submodule_operate_client(),
        test_grpc_instruct_client(),
        test_grpc_manipulate_client()
    );
    tokio::time::sleep(Duration::from_secs(15)).await;
}

async fn test_grpc_submodule_operate_client() {
    info!("Sleep, Wait Server Start");
    tokio::time::sleep(Duration::from_secs(5)).await;
    let config = GrpcClientConfig::default();
    let mut client = GrpcClient::init(config);
    client.connection_submodule_operate_server().await.unwrap();
    info!("Connection Success!");
    let mut operate = ModuleOperate::default();
    operate.name = String::from("test");
    operate.info = Some(SubmoduleInfo {
        default_instruct: vec![String::from("test_instruct")],
        conn_params: ConnParams {
            connection_type: ConnectionType::GrpcType,
            client_type: ClientType::NotReceiveType,
            conn_params: HashMap::new(),
        },
    });
    operate.operate_type = OperateType::Register;
    client
        .register(set_entity_submodule_sign(operate))
        .await
        .unwrap();
    info!("register finish");
    let mut operate = ModuleOperate::default();
    operate.name = String::from("test");
    operate.info = Some(SubmoduleInfo {
        default_instruct: vec![String::from("test_instruct")],
        conn_params: ConnParams {
            connection_type: ConnectionType::GrpcType,
            client_type: ClientType::NotReceiveType,
            conn_params: HashMap::new(),
        },
    });
    operate.operate_type = OperateType::Update;
    client
        .update(set_entity_submodule_sign(operate))
        .await
        .unwrap();
    info!("update finish");
    let mut operate = ModuleOperate::default();
    operate.name = String::from("test");
    operate.info = Some(SubmoduleInfo {
        default_instruct: vec![String::from("test_instruct")],
        conn_params: ConnParams {
            connection_type: ConnectionType::GrpcType,
            client_type: ClientType::NotReceiveType,
            conn_params: HashMap::new(),
        },
    });
    operate.operate_type = OperateType::Heartbeat;
    client
        .heartbeat(set_entity_submodule_sign(operate))
        .await
        .unwrap();
    info!("heartbeat finish");
    let mut operate = ModuleOperate::default();
    operate.name = String::from("test");
    operate.info = Some(SubmoduleInfo {
        default_instruct: vec![String::from("test_instruct")],
        conn_params: ConnParams {
            connection_type: ConnectionType::GrpcType,
            client_type: ClientType::NotReceiveType,
            conn_params: HashMap::new(),
        },
    });
    operate.operate_type = OperateType::Offline;
    client
        .offline(set_entity_submodule_sign(operate))
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
    let mut instruct = InstructEntity::default();
    instruct.instruct = InstructData::Text(String::from("test send instruct"));
    client
        .text_instruct(set_entity_submodule_sign(instruct))
        .await
        .unwrap();
    info!("text_instruct finish");
    let (tx, rx) = mpsc::channel(1);
    let mut instruct = InstructEntity::default();
    instruct.instruct = InstructData::Text(String::from("test send instruct"));
    tx.send(set_entity_submodule_sign(instruct)).await.unwrap();
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
    let mut manipualte = ManipulateEntity::default();
    manipualte.manipulate = ManipulateData::Simple;
    client
        .simple_manipulate(set_entity_submodule_sign(manipualte))
        .await
        .unwrap();
    info!("simple_manipulate finish");
    let mut manipualte = ManipulateEntity::default();
    manipualte.manipulate = ManipulateData::Text(String::from("text_display_manipulate"));
    client
        .text_display_manipulate(set_entity_submodule_sign(manipualte))
        .await
        .unwrap();
    info!("text_display_manipulate finish");
    let (tx, rx) = mpsc::channel(1);
    let mut manipualte = ManipulateEntity::default();
    manipualte.manipulate = ManipulateData::Text(String::from("multiple_text_display_manipulate"));
    tx.send(set_entity_submodule_sign(manipualte))
        .await
        .unwrap();
    client.multiple_text_display_manipulate(rx).await.unwrap();
    info!("multiple_text_display_manipulate finish");
    let mut manipualte = ManipulateEntity::default();
    manipualte.manipulate = ManipulateData::ConnectionParams(ConnParams {
        connection_type: ConnectionType::GrpcType,
        client_type: ClientType::NotReceiveType,
        conn_params: Default::default(),
    });
    client
        .direct_connection_manipulate(set_entity_submodule_sign(manipualte))
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
