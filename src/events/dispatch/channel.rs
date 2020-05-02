use serde::{Deserialize, Serialize};

use crate::models::ChannelId;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[repr(u8)]
#[non_exhaustive]
pub enum ChannelVariant {
    GuildText = 0,
    Direct = 1,
    GuildVoice = 2,
    Group = 3,
    GuildCategory = 4,
    GuildNews = 5,
    GuildStore = 6,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub struct UntaggedChannel {
    pub id: ChannelId,

    #[serde(rename = "type")]
    pub variant: ChannelVariant,
}
