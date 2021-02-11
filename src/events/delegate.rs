use anyhow::Result;
use async_trait::async_trait;
use log::warn;

use super::payload::*;

#[async_trait]
#[allow(unused_variables)]
pub trait PayloadDelegate {
    async fn delegate_payload(&mut self, payload: &Payload) -> Result<()> {
        match payload {
            Payload::Dispatch(data) => self.dispatch(data).await,
            Payload::Heartbeat { data } => self.heartbeat_req(data).await,
            Payload::Hello { data } => self.hello(data).await,
            Payload::HeartbeatAck => self.heartbeat_ack().await,

            Payload::Unknown => {
                warn!("[PayloadDelegate] Unknown payload received {:?}", payload);

                Ok(())
            }

            _ => Ok(()),
        }
    }

    async fn dispatch(&mut self, data: &Dispatch) -> Result<()> {
        Ok(())
    }

    async fn heartbeat_req(&mut self, data: &Heartbeat) -> Result<()> {
        Ok(())
    }

    async fn hello(&mut self, data: &Hello) -> Result<()> {
        Ok(())
    }

    async fn heartbeat_ack(&mut self) -> Result<()> {
        Ok(())
    }
}
