use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::prelude::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GuildId {
    id: Snowflake,
}

impl ResourceId for GuildId {
    fn id(&self) -> &Snowflake {
        &self.id
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub struct AfkDetails {
    #[serde(rename = "afk_channel_id", default)]
    channel_id: Option<ChannelId>,

    #[serde(rename = "afk_timeout")]
    timeout: u64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub struct PremiumDetails {
    #[serde(rename = "premium_tier")]
    tier: PremiumTierLevel,

    // TODO: Determine when this field isn't sent
    #[serde(rename = "premium_subscription_count")]
    subscription_count: Option<u64>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub struct EmbedDetails {
    #[serde(rename = "embed_channel_id", default)]
    channel_id: Option<ChannelId>,

    #[serde(rename = "embed_enabled")]
    is_enabled: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub struct WidgetDetails {
    #[serde(rename = "widget_channel_id", default)]
    channel_id: Option<ChannelId>,

    #[serde(rename = "widget_enabled")]
    is_enabled: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub struct SystemChannelDetails {
    #[serde(rename = "system_channel_id", default)]
    id: Option<ChannelId>,

    // TODO: Create bitfield
    #[serde(rename = "system_channel_flags")]
    flags: u64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub struct RulesChannelDetails {
    #[serde(rename = "rules_channel_id", default)]
    id: Option<ChannelId>,
}

#[derive(Clone, Debug, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
#[non_exhaustive]
pub enum VerificationLevel {
    None = 0,
    Low = 1,
    Medium = 2,
    High = 3,
    VeryHigh = 4,
}

#[derive(Clone, Debug, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
#[non_exhaustive]
pub enum MessageNotificationLevel {
    All = 0,
    Mentions = 1,
}

#[derive(Clone, Debug, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
#[non_exhaustive]
pub enum ExplicitContentFilterLevel {
    Disabled = 0,
    RolelessMembers = 1,
    AllMembers = 2,
}

#[derive(Clone, Debug, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
#[non_exhaustive]
pub enum MfaLevel {
    None = 0,
    Elevated = 1,
}

#[derive(Clone, Debug, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
#[non_exhaustive]
pub enum PremiumTierLevel {
    None = 0,
    Tier1 = 1,
    Tier2 = 2,
    Tier3 = 3,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
pub enum GuildFeature {
    InviteSplash,
    VipRegions,
    VanityUrl,
    Verified,
    Partnered,
    Public,
    Commerce,
    News,
    Discoverable,
    Featurable,
    AnimatedIcon,
    Banner,
    PublicDisabled,
    WelcomeScreenEnabled,
}
