use std::time::Duration;
use time::macros::format_description;
use time::UtcOffset;

use tokio::sync::mpsc;
use tracing::{error, info, Level};

use nihility_common::{GrpcServer, GrpcServerConfig, NihilityServer};

#[tokio::test]
async fn test_server() {
    init_log();
    let server_config = GrpcServerConfig::default();
    let mut server = GrpcServer::init(server_config).unwrap();
    let (tx, mut rx) = mpsc::unbounded_channel();
    server.set_submodule_operate_sender(tx).unwrap();
    server.start().unwrap();
    tokio::time::sleep(Duration::from_secs(10)).await;
    match rx.try_recv() {
        Ok(operate) => {
            info!("operate: {:?}", operate);
        }
        Err(e) => {
            error!("error: {:?}", e)
        }
    }
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