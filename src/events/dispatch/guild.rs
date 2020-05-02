use serde::{Deserialize, Serialize};

use crate::models::{
    AfkDetails, EmbedDetails, ExplicitContentFilterLevel, GuildFeature, GuildId,
    MessageNotificationLevel, MfaLevel, PremiumDetails, SystemChannelDetails, VerificationLevel,
    WidgetDetails,
};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[non_exhaustive]
pub struct PartialGuild {
    #[serde(flatten)]
    pub id: GuildId,
    pub unavailable: bool,
    pub name: String,
    pub region: String,
    pub preferred_locale: String,
    pub verification_level: VerificationLevel,
    pub default_message_notifications: MessageNotificationLevel,
    pub explicit_content_filter: ExplicitContentFilterLevel,
    pub mfa_level: MfaLevel,
    pub features: Vec<GuildFeature>,
    // TODO: application_id
    pub system_channel: SystemChannelDetails,

    // TODO: roles, emoji

    #[serde(flatten)]
    pub afk_details: AfkDetails,

    #[serde(flatten)]
    pub premium: PremiumDetails,

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

#[derive(Debug, Clone, Deserialize, Serialize)]
#[non_exhaustive]
pub struct GuildRole {
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

#[derive(Debug, Clone, Deserialize, Serialize)]
#[non_exhaustive]
pub struct FetchedGuild {
    pub partial: PartialGuild,

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

// TODO: Once enum tagging with bools is added to serde, `unavailable` can act as a discriminator.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
#[non_exhaustive]
pub enum GuildCreate {
    Available(AvailableGuildCreate),
    Unavailable(UnavailableGuildCreate),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub struct AvailableGuildCreate {
    #[serde(flatten)]
    pub partial: PartialGuild,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub struct UnavailableGuildCreate {
    #[serde(flatten)]
    pub id: GuildId,
    pub unavailable: bool,
}
