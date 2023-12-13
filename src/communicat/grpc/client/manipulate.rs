use async_trait::async_trait;
use tokio::spawn;
use tokio::sync::mpsc::Receiver;
use tokio_stream::StreamExt;
use tokio_stream::wrappers::ReceiverStream;
use tonic::Request;
use tonic::transport::Channel;
use tracing::error;

use crate::communicat::SendManipulateOperate;
use crate::error::WrapResult;
use crate::manipulate::{SimpleManipulate, TextDisplayManipulate};
use crate::manipulate::manipulate_client::ManipulateClient;
use crate::response_code::RespCode;

#[async_trait]
impl SendManipulateOperate for ManipulateClient<Channel> {
    async fn send_simple(&mut self, manipulate: SimpleManipulate) -> WrapResult<RespCode> {
        Ok(self
            .send_simple_manipulate(Request::new(manipulate))
            .await?
            .into_inner()
            .code())
    }

    async fn send_text_display(&mut self, manipulate: TextDisplayManipulate) -> WrapResult<RespCode> {
        Ok(self
            .send_text_display_manipulate(Request::new(manipulate))
            .await?
            .into_inner()
            .code())
    }

    async fn send_multiple_text_display(
        &mut self,
        manipulate_stream: Receiver<TextDisplayManipulate>,
    ) -> WrapResult<Receiver<RespCode>> {
        let (tx, rx) = tokio::sync::mpsc::channel::<RespCode>(128);
        let mut resp_stream = self
            .send_multiple_text_display_manipulate(ReceiverStream::new(manipulate_stream))
            .await?
            .into_inner();
        spawn(async move {
            while let Some(result) = resp_stream.next().await {
                match result {
                    Ok(resp) => match tx.send(resp.code()).await {
                        Ok(_) => {}
                        Err(e) => {
                            error!("Manipulate Grpc Client send_multiple_text_display Send To Core Error: {:?}", e);
                            break;
                        }
                    },
                    Err(status) => {
                        error!(
                            "Manipulate Grpc Client send_multiple_text_display Send Error: {:?}",
                            &status
                        );
                        match tx.send(RespCode::UnknownError).await {
                            Ok(_) => {}
                            Err(e) => {
                                error!("Manipulate Grpc Client send_multiple_text_display Send To Core Error: {:?}", e);
                                break;
                            }
                        }
                    }
                }
            }
        });
        Ok(rx)
    }
}
