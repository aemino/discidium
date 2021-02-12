pub mod message;
pub mod text;

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr as DeserializeRepr, Serialize_repr as SerializeRepr};

use crate::models::prelude::*;

create_id!(pub ChannelId {});

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Channel {
    #[serde(rename = "0")]
    GuildText(GuildTextChannel),

    #[serde(rename = "1")]
    Direct,

    #[serde(rename = "2")]
    GuildVoice(GuildVoiceChannel),

    #[serde(rename = "3")]
    Group,

    #[serde(rename = "4")]
    GuildCategory(GuildCategoryChannel),

    #[serde(rename = "5")]
    GuildNews,

    #[serde(rename = "6")]
    GuildStore,
}

create_id!(pub GuildTextChannelId {});

impl From<GuildTextChannelId> for TextChannelId {
    fn from(this: GuildTextChannelId) -> Self {
        Self {
            id: this.id,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GuildTextChannel {
    #[serde(flatten)]
    id: GuildTextChannelId,

    #[serde(flatten)]
    guild_data: GuildChannelData,

    #[serde(flatten)]
    text_data: TextChannelData,
}


create_id!(pub GuildVoiceChannelId {});

#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GuildVoiceChannel {
    #[serde(flatten)]
    id: GuildVoiceChannelId,

    #[serde(flatten)]
    guild_data: GuildChannelData,

    #[serde(flatten)]
    text_data: TextChannelData,
}

create_id!(pub GuildCategoryChannelId {});

#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GuildCategoryChannel {
    #[serde(flatten)]
    id: GuildCategoryChannelId,

    #[serde(flatten)]
    guild_data: GuildChannelData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GuildChannelData {
    #[serde(flatten)]
    guild_id: GuildId,
    position: i16,
    name: String,

    #[serde(rename = "nsfw")]
    is_nsfw: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct VoiceChannelData {
    bitrate: u32,
    user_limit: u16,
}
