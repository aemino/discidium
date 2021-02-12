use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::models::prelude::*;

create_id!(pub TextChannelId {});

#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TextChannelData {
    #[serde(default)]
    last_message_id: Option<MessageId>,

    #[serde(rename = "last_pin_timestamp", default)]
    last_pin_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CreateMessageParams {
    content: String,
}

#[async_trait]
pub trait TextChannel {
    fn id(&self) -> TextChannelId;

    fn route(&self) -> Route {
        Api::route()
            .join("/channels")
            .join("/:channel_id")
            .with_var("channel_id", self.id().id)
    }

    async fn send_message(&self, ctx: &Context<'_>, content: String) -> anyhow::Result<Message> {
        self.route()
            .join("/messages")
            .post()
            .send(ctx, &CreateMessageParams { content })
            .await
    }
}

impl<T, U> TextChannel for T
where
    T: ResourceId<Id = U>,
    U: Into<TextChannelId> + Clone,
{
    fn id(&self) -> TextChannelId {
        self.id().clone().into()
    }
}
