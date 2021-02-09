use serde::{Deserialize, Serialize};

use super::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Deserialize, Serialize)]
pub struct ChannelId {
    id: Snowflake,
}

impl ResourceId for ChannelId {
    fn id(&self) -> Snowflake {
        self.id
    }
}
