use std::{collections::HashMap, sync::Arc};

use anansi_core::{Message, Response};
use tokio::sync::RwLock;

use crate::{runner::Runner, Error};

pub type EventsMap = HashMap<Box<str>, Box<str>>;

#[derive(Clone)]
pub struct EventRegistry {
    // TODO: consider changing to dashmap
    inner: Arc<RwLock<EventsMap>>,
}

impl EventRegistry {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    #[inline(always)]
    async fn handle_send(&self, channel_name: &str) -> Response {
        let res = {
            let read_guard = self.inner.read().await;

            let Some(command_to_run) =
                read_guard.get(channel_name)
            else {
                return Response::ChannelNotFound;
            };

            Runner::run_to_completion(command_to_run).await
        };

        tracing::info!(
            "On handle_send: running command got `{res:?}`"
        );

        match res {
            Ok(()) => Response::Ok,
            Err(Error::ProcessFailed(_)) => {
                Response::ProcessFailed
            }
            Err(_) => {
                // TODO: embed error details?
                Response::InternalError
            }
        }
    }

    #[inline(always)]
    async fn handle_on(
        &self,
        channel_name: &str,
        command_to_run: &str,
    ) -> Response {
        let mut write_guard = self.inner.write().await;

        // TODO: notify if a channel gets its command rewritten?
        write_guard
            .insert(channel_name.into(), command_to_run.into());

        Response::Ok
    }

    pub async fn handle_message<'msg>(
        &self,
        msg: Message<'msg>,
    ) -> Response {
        match msg {
            Message::Send { channel_name } => {
                self.handle_send(channel_name).await
            }
            Message::On { channel_name, run } => {
                self.handle_on(channel_name, run).await
            }
            Message::Close => Response::Unimplemented,
        }
    }
}
