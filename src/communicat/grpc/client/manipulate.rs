use async_trait::async_trait;
use tokio::spawn;
use tokio::sync::mpsc;
use tokio::sync::mpsc::Receiver;
use tokio_stream::wrappers::ReceiverStream;
use tokio_stream::StreamExt;
use tonic::Request;
use tracing::error;

use crate::communicat::SendManipulateOperate;
use crate::entity::manipulate::ManipulateEntity;
use crate::entity::response::ResponseCode;
use crate::error::WrapResult;
use crate::manipulate::TextDisplayManipulate;

use super::GrpcClient;

const STREAM_BUFFER: usize = 12;

#[async_trait]
impl SendManipulateOperate for GrpcClient {
    fn is_manipulate_client_connected(&self) -> bool {
        if self.manipulate_client.is_none() {
            return false;
        }
        true
    }
    async fn send_simple_manipulate(
        &self,
        manipulate: ManipulateEntity,
    ) -> WrapResult<ResponseCode> {
        Ok(ResponseCode::from(
            self.manipulate_client
                .clone()
                .unwrap()
                .send_simple_manipulate(Request::new(manipulate.try_into()?))
                .await?
                .into_inner()
                .code(),
        ))
    }

    async fn send_text_display_manipulate(
        &self,
        manipulate: ManipulateEntity,
    ) -> WrapResult<ResponseCode> {
        Ok(ResponseCode::from(
            self.manipulate_client
                .clone()
                .unwrap()
                .send_text_display_manipulate(Request::new(manipulate.try_into()?))
                .await?
                .into_inner()
                .code(),
        ))
    }

    async fn send_multiple_text_display_manipulate(
        &self,
        mut manipulate_stream: Receiver<ManipulateEntity>,
    ) -> WrapResult<Receiver<ResponseCode>> {
        let (req_tx, req_rx) = mpsc::channel::<TextDisplayManipulate>(STREAM_BUFFER);
        let (out_tx, out_rx) = mpsc::channel::<ResponseCode>(STREAM_BUFFER);
        spawn(async move {
            while let Some(manipulate) = manipulate_stream.recv().await {
                match <ManipulateEntity as TryInto<TextDisplayManipulate>>::try_into(manipulate) {
                    Ok(text_display_manipulate) => {
                        match req_tx.send(text_display_manipulate).await {
                            Ok(_) => {}
                            Err(e) => {
                                error!("Grpc Client send_multiple_text_display_manipulate Send To Stream Error: {:?}", e);
                                break;
                            }
                        }
                    }
                    Err(e) => {
                        error!("Grpc Client send_multiple_text_display_manipulate Transform Error: {:?}", e);
                        break;
                    }
                }
            }
        });
        let mut resp_stream = self
            .manipulate_client
            .clone()
            .unwrap()
            .send_multiple_text_display_manipulate(ReceiverStream::new(req_rx))
            .await?
            .into_inner();
        spawn(async move {
            while let Some(result) = resp_stream.next().await {
                match result {
                    Ok(resp) => match out_tx.send(ResponseCode::from(resp.code())).await {
                        Ok(_) => {}
                        Err(e) => {
                            error!("Instruct Grpc Client send_multiple_text_display_manipulate Send To Core Error: {:?}", e);
                            break;
                        }
                    },
                    Err(status) => {
                        error!(
                            "Instruct Grpc Client send_multiple_text_display_manipulate Send Error: {:?}",
                            &status
                        );
                        match out_tx.send(ResponseCode::UnknownError).await {
                            Ok(_) => {}
                            Err(e) => {
                                error!("Instruct Grpc Client send_multiple_text_display_manipulate Send To Core Error: {:?}", e);
                                break;
                            }
                        }
                    }
                }
            }
        });
        Ok(out_rx)
    }
}
