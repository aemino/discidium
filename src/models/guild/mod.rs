use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr as DeserializeRepr, Serialize_repr as SerializeRepr};

use crate::models::prelude::*;

create_id!(pub GuildId {});

#[derive(Debug, Clone, Deserialize, Serialize)]
#[non_exhaustive]
pub struct UnavailableGuild {
    #[serde(flatten)]
    pub id: GuildId,

    #[serde(rename = "_received_at", default = "Utc::now")]
    pub(crate) received_at: DateTime<Utc>,
}

impl_resource!(UnavailableGuild, GuildId);

#[derive(Debug, Clone, Deserialize, Serialize)]
#[non_exhaustive]
pub struct Guild {
    #[serde(flatten)]
    id: GuildId,

    #[serde(rename = "_received_at", default = "Utc::now")]
    pub(crate) received_at: DateTime<Utc>,

    pub name: String,
    pub region: String,
    pub preferred_locale: String,
    pub verification_level: VerificationLevel,
    pub default_message_notifications: MessageNotificationLevel,
    pub explicit_content_filter: ExplicitContentFilterLevel,
    pub mfa_level: MfaLevel,
    pub features: Vec<GuildFeature>,
    // TODO: application_id
    pub roles: Vec<PartialGuildRole>,

    // TODO: emoji
    #[serde(flatten)]
    pub afk_details: AfkDetails,

    #[serde(flatten)]
    pub premium: PremiumDetails,

    #[serde(flatten)]
    pub system_channel: SystemChannelDetails,

    #[serde(rename = "icon", default)]
    pub icon_hash: Option<String>,

    #[serde(rename = "splash", default)]
    pub splash_hash: Option<String>,

    #[serde(rename = "discovery_splash", default)]
    pub discovery_splash_hash: Option<String>,

    #[serde(rename = "banner", default)]
    pub banner_hash: Option<String>,

    #[serde(default)]
    pub vanity_url_code: Option<String>,

    #[serde(default)]
    pub description: Option<String>,
    // TODO: max_video_channel_users is undocumented
}

impl_resource!(Guild, GuildId);

#[derive(Debug, Clone, Deserialize, Serialize)]
#[non_exhaustive]
pub struct ExtendedGuild {
    #[serde(flatten)]
    pub partial: Guild,

    #[serde(rename = "owner", default)]
    pub is_owner: Option<bool>,

    #[serde(flatten)]
    pub embed: EmbedDetails,

    #[serde(flatten)]
    pub widget: WidgetDetails,

    #[serde(rename = "max_presences")]
    pub max_presence_count: Option<u64>,

    #[serde(rename = "max_members")]
    pub max_member_count: u64,

    #[serde(default)]
    pub approximate_member_count: Option<u64>,

    #[serde(default)]
    pub approximate_presence_count: Option<u64>,
}

create_id!(pub GuildRoleId {});

#[derive(Debug, Clone, Deserialize, Serialize)]
#[non_exhaustive]
pub struct PartialGuildRole {
    #[serde(flatten)]
    pub id: GuildRoleId,

    pub name: String,
    // TODO: Color stuct
    pub color: u32,
    pub position: u64,
    // TODO: Permissions bitfield struct
    pub permissions: u64,

    #[serde(rename = "hoist")]
    pub is_hoisted: bool,

    #[serde(rename = "managed")]
    pub is_managed: bool,

    #[serde(rename = "mentionable")]
    pub is_mentionable: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub struct AfkDetails {
    #[serde(rename = "afk_channel_id", default, flatten)]
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
    #[serde(rename = "embed_channel_id", default, flatten)]
    channel_id: Option<ChannelId>,

    #[serde(rename = "embed_enabled")]
    is_enabled: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub struct WidgetDetails {
    #[serde(rename = "widget_channel_id", default, flatten)]
    channel_id: Option<ChannelId>,

    #[serde(rename = "widget_enabled")]
    is_enabled: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub struct SystemChannelDetails {
    #[serde(rename = "system_channel_id", default, flatten)]
    id: Option<ChannelId>,

    // TODO: Create bitfield
    #[serde(rename = "system_channel_flags")]
    flags: u64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub struct RulesChannelDetails {
    #[serde(rename = "rules_channel_id", default, flatten)]
    id: Option<ChannelId>,
}

#[derive(Clone, Debug, DeserializeRepr, SerializeRepr)]
#[repr(u8)]
#[non_exhaustive]
pub enum VerificationLevel {
    None = 0,
    Low = 1,
    Medium = 2,
    High = 3,
    VeryHigh = 4,
}

#[derive(Clone, Debug, DeserializeRepr, SerializeRepr)]
#[repr(u8)]
#[non_exhaustive]
pub enum MessageNotificationLevel {
    All = 0,
    Mentions = 1,
}

#[derive(Clone, Debug, DeserializeRepr, SerializeRepr)]
#[repr(u8)]
#[non_exhaustive]
pub enum ExplicitContentFilterLevel {
    Disabled = 0,
    RolelessMembers = 1,
    AllMembers = 2,
}

#[derive(Clone, Debug, DeserializeRepr, SerializeRepr)]
#[repr(u8)]
#[non_exhaustive]
pub enum MfaLevel {
    None = 0,
    Elevated = 1,
}

#[derive(Clone, Debug, DeserializeRepr, SerializeRepr)]
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
