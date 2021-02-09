use serde::{Deserialize, Serialize};

use crate::models::{Guild, GuildId};

// TODO: Once enum tagging with bools is added to serde, rename with boolean
// literals rather than strings.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "unavailable")]
#[non_exhaustive]
pub enum GuildCreate {
    #[serde(rename = "false")]
    Available(AvailableGuildCreate),

    #[serde(rename = "true")]
    Unavailable(UnavailableGuildCreate),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub struct AvailableGuildCreate {
    #[serde(flatten)]
    pub guild: Guild,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub struct UnavailableGuildCreate {
    pub id: GuildId,
}
