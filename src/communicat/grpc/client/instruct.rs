use async_trait::async_trait;
use tokio::spawn;
use tokio::sync::mpsc;
use tokio::sync::mpsc::Receiver;
use tokio_stream::StreamExt;
use tokio_stream::wrappers::ReceiverStream;
use tonic::Request;
use tracing::error;

use crate::communicat::SendInstructOperate;
use crate::entity::instruct::InstructEntity;
use crate::entity::response::ResponseCode;
use crate::error::WrapResult;
use crate::instruct::TextInstruct;

use super::GrpcClient;

const STREAM_BUFFER: usize = 12;

#[async_trait]
impl SendInstructOperate for GrpcClient {
    fn is_instruct_client_connected(&self) -> bool {
        if let None = self.instruct_client {
            return false;
        }
        true
    }

    async fn send_text_instruct(&self, instruct: InstructEntity) -> WrapResult<ResponseCode> {
        Ok(ResponseCode::from(self.instruct_client.clone().unwrap()
            .send_text_instruct(Request::new(instruct.try_into()?))
            .await?
            .into_inner()
            .code()))
    }

    async fn send_multiple_text_instruct(
        &self,
        mut instruct_stream: Receiver<InstructEntity>,
    ) -> WrapResult<Receiver<ResponseCode>> {
        let (req_tx, req_rx) = mpsc::channel::<TextInstruct>(STREAM_BUFFER);
        let (out_tx, out_rx) = mpsc::channel::<ResponseCode>(STREAM_BUFFER);
        spawn(async move {
            while let Some(instruct) = instruct_stream.recv().await {
                match <InstructEntity as TryInto<TextInstruct>>::try_into(instruct) {
                    Ok(text_instruct) => {
                        match req_tx.send(text_instruct).await {
                            Ok(_) => {}
                            Err(e) => {
                                error!("Grpc Client send_multiple_text_instruct Send To Stream Error: {:?}", e);
                                break;
                            }
                        }
                    }
                    Err(e) => {
                        error!("Grpc Client send_multiple_text_instruct Transform Error: {:?}", e);
                        break;
                    }
                }
            }
        });
        let mut resp_stream = self.instruct_client.clone().unwrap()
            .send_multiple_text_instruct(ReceiverStream::new(req_rx))
            .await?
            .into_inner();
        spawn(async move {
            while let Some(result) = resp_stream.next().await {
                match result {
                    Ok(resp) => {
                        match out_tx.send(ResponseCode::from(resp.code())).await {
                            Ok(_) => {}
                            Err(e) => {
                                error!("Instruct Grpc Client send_multiple_text_instruct Send To Core Error: {:?}", e);
                                break;
                            }
                        }
                    }
                    Err(status) => {
                        error!(
                            "Instruct Grpc Client send_multiple_text_instruct Send Error: {:?}",
                            &status
                        );
                        match out_tx.send(ResponseCode::UnknownError).await {
                            Ok(_) => {}
                            Err(e) => {
                                error!("Instruct Grpc Client send_multiple_text_instruct Send To Core Error: {:?}", e);
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
