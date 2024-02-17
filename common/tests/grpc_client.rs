use std::collections::HashMap;
use std::time::Duration;

use tokio::sync::mpsc;
use tracing::info;

use nihility_common::{
    set_core_public_key_path, set_default_receiver_submodule, set_submodule_name, ClientType,
    ConnParams, ConnectionType, GrpcClient, GrpcClientConfig, InstructEntity, Log, LogConfig,
    ManipulateEntity, ModuleOperate, NihilityClient, OperateType, SubmoduleInfo,
};

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_client() {
    Log::init(&vec![LogConfig::default()]).unwrap();
    tokio::time::sleep(Duration::from_secs(3)).await;
    set_submodule_name("test");
    set_core_public_key_path("./auth/id_rsa.pub");
    set_default_receiver_submodule("test");
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
    info!("offline finish");
}

async fn test_grpc_instruct_client() {
    info!("Sleep, Wait Server Start");
    let config = GrpcClientConfig::default();
    let mut client = GrpcClient::init(config);
    client.connection_instruct_server().await.unwrap();
    info!("Connection Success!");
    let instruct = InstructEntity::new_text(String::from("test send instruct"));
    client.text_instruct(instruct).await.unwrap();
    info!("text_instruct finish");
    let (tx, rx) = mpsc::channel(1);
    let instruct = InstructEntity::new_text(String::from("test send instruct"));
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
    let manipulate = ManipulateEntity::new_simple();
    client.simple_manipulate(manipulate).await.unwrap();
    info!("simple_manipulate finish");
    let manipulate = ManipulateEntity::new_text(String::from("text_display_manipulate"));
    client.text_display_manipulate(manipulate).await.unwrap();
    info!("text_display_manipulate finish");
    let (tx, rx) = mpsc::channel(1);
    let manipulate = ManipulateEntity::new_text(String::from("multiple_text_display_manipulate"));
    tx.send(manipulate).await.unwrap();
    client.multiple_text_display_manipulate(rx).await.unwrap();
    info!("multiple_text_display_manipulate finish");
    let manipulate = ManipulateEntity::new_connection_params(ConnParams {
        connection_type: ConnectionType::GrpcType,
        client_type: ClientType::NotReceiveType,
        conn_config: Default::default(),
    });
    client
        .direct_connection_manipulate(manipulate)
        .await
        .unwrap();
    info!("direct_connection_manipulate finish");
}
