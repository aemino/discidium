use serde::{Serialize, Deserialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::models::prelude::*;

create_id!(pub MessageId {
    channel_id: Snowflake,
});

impl MessageId {
    pub fn channel_id(&self) -> TextChannelId {
        TextChannelId { id: self.channel_id }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[non_exhaustive]
pub struct Message {
    #[serde(flatten)]
    id: MessageId,

    #[serde(rename = "_received_at", default = "Utc::now")]
    pub(crate) received_at: DateTime<Utc>,

    // #[serde(rename = "type")]
    // pub kind: MessageKind,

    pub content: String,

    #[serde(rename = "timestamp")]
    pub created_at: DateTime<Utc>,

    #[serde(rename = "edited_timestamp")]
    pub edited_at: Option<DateTime<Utc>>,
}

impl_resource!(Message, MessageId);

impl Message {
    pub fn channel_id(&self) -> TextChannelId {
        self.id().channel_id()
    }
}

#[derive(Clone, Debug, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
#[non_exhaustive]
pub enum MessageKind {
    Default = 0,
    RecipientAdd = 1,
    RecipientRemove = 2,
    Call = 3,
    ChannelNameChange = 4,
    ChannelIconChange = 5,
    ChannelPinnedMessage = 6,
    GuildMemberJoin = 7,
    UserPremiumGuildSubscription = 8,
    UserPremiumGuildSubscriptionTier1 = 9,
    UserPremiumGuildSubscriptionTier2 = 10,
    UserPremiumGuildSubscriptionTier3 = 11,
    ChannelFollowAdd = 12,
    GuildDiscoveryDisqualified = 14,
    GuildDiscoveryRequalified = 15,
    Reply = 19,
    ApplicationCommand = 20,
}
