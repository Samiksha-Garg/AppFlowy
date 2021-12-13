use crate::{
    errors::UserError,
    services::user::ws_manager::{FlowyWebSocket, FlowyWsSender},
};
use lib_infra::future::FutureResult;
use lib_ws::{WsConnectState, WsMessage, WsMessageHandler};
use std::sync::Arc;
use tokio::sync::{broadcast, broadcast::Receiver};

pub(crate) struct LocalWebSocket {
    state_sender: broadcast::Sender<WsConnectState>,
    ws_sender: broadcast::Sender<WsMessage>,
}

impl std::default::Default for LocalWebSocket {
    fn default() -> Self {
        let (state_sender, _) = broadcast::channel(16);
        let (ws_sender, _) = broadcast::channel(16);
        LocalWebSocket {
            state_sender,
            ws_sender,
        }
    }
}

impl FlowyWebSocket for Arc<LocalWebSocket> {
    fn start_connect(&self, _addr: String) -> FutureResult<(), UserError> { FutureResult::new(async { Ok(()) }) }

    fn conn_state_subscribe(&self) -> Receiver<WsConnectState> { self.state_sender.subscribe() }

    fn reconnect(&self, _count: usize) -> FutureResult<(), UserError> { FutureResult::new(async { Ok(()) }) }

    fn add_handler(&self, _handler: Arc<dyn WsMessageHandler>) -> Result<(), UserError> { Ok(()) }

    fn ws_sender(&self) -> Result<Arc<dyn FlowyWsSender>, UserError> { Ok(Arc::new(self.ws_sender.clone())) }
}

impl FlowyWsSender for broadcast::Sender<WsMessage> {
    fn send(&self, msg: WsMessage) -> Result<(), UserError> {
        let _ = self.send(msg);
        Ok(())
    }
}
