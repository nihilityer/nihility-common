use async_trait::async_trait;
use tokio::spawn;
use tokio::sync::mpsc::Receiver;
use tokio_stream::StreamExt;
use tokio_stream::wrappers::ReceiverStream;
use tonic::Request;
use tonic::transport::Channel;
use tracing::error;

use crate::communicat::SendInstructOperate;
use crate::error::WrapResult;
use crate::instruct::instruct_client::InstructClient;
use crate::instruct::TextInstruct;
use crate::response_code::RespCode;

#[async_trait]
impl SendInstructOperate for InstructClient<Channel> {
    async fn send_text(&mut self, instruct: TextInstruct) -> WrapResult<RespCode> {
        Ok(self
            .send_text_instruct(Request::new(instruct))
            .await?
            .into_inner()
            .code())
    }

    async fn send_multiple_text(
        &mut self,
        instruct_stream: Receiver<TextInstruct>,
    ) -> WrapResult<Receiver<RespCode>> {
        let (tx, rx) = tokio::sync::mpsc::channel::<RespCode>(128);
        let mut resp_stream = self
            .send_multiple_text_instruct(ReceiverStream::new(instruct_stream))
            .await?
            .into_inner();
        spawn(async move {
            while let Some(result) = resp_stream.next().await {
                match result {
                    Ok(resp) => {
                        match tx.send(resp.code()).await {
                            Ok(_) => {}
                            Err(e) => {
                                error!("Instruct Grpc Client send_multiple_text Send To Core Error: {:?}", e);
                                break;
                            }
                        }
                    }
                    Err(status) => {
                        error!(
                            "Instruct Grpc Client send_multiple_text Send Error: {:?}",
                            &status
                        );
                        match tx.send(RespCode::UnknownError).await {
                            Ok(_) => {}
                            Err(e) => {
                                error!("Instruct Grpc Client send_multiple_text Send To Core Error: {:?}", e);
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
