use std::hash::Hash;

use serde::Serialize;

mod channel;
mod gateway;
mod guild;

pub(self) mod prelude {
    pub use crate::{
        client::{Client, Context},
        http::{Api, Route},
    };

    pub use super::*;
}

pub use channel::*;
pub use gateway::*;
pub use guild::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
#[serde(transparent)]
pub struct Snowflake(u64);

pub trait ResourceId: Copy + Hash + Ord {
    fn id(&self) -> Snowflake;
}

pub trait Resource: Clone {
    type Id: Eq + Hash + Send + Sync;

    fn id(&self) -> Self::Id;
}

// TODO: Custom Serialize derivation using `Serializer::is_human_readable`
mod snowflake {
    use std::fmt;

    use serde::de::{self, Deserialize, Deserializer, Visitor};

    use super::Snowflake;

    impl<'de> Deserialize<'de> for Snowflake {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            struct SnowflakeVisitor;

            impl<'de> Visitor<'de> for SnowflakeVisitor {
                type Value = Snowflake;

                fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    f.write_str("snowflake as a number or string")
                }

                fn visit_u64<E>(self, id: u64) -> Result<Self::Value, E>
                where
                    E: de::Error,
                {
                    Ok(Snowflake(id))
                }

                fn visit_str<E>(self, id: &str) -> Result<Self::Value, E>
                where
                    E: de::Error,
                {
                    id.parse().map(Snowflake).map_err(de::Error::custom)
                }
            }

            deserializer.deserialize_any(SnowflakeVisitor)
        }
    }
}
