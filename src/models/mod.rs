use std::hash::Hash;

use chrono::{DateTime, Utc};
use serde::Serialize;

mod channel;
mod gateway;
mod guild;

mod macros {
    #[macro_export]
    macro_rules! create_id {
        ($vis:vis $name:ident { $($field:tt)* }) => {
            #[derive(
                Clone,
                Copy,
                Debug,
                PartialEq,
                Eq,
                Hash,
                PartialOrd,
                Ord,
                ::serde::Deserialize,
                ::serde::Serialize,
            )]
            $vis struct $name {
                pub(crate) id: Snowflake,
                $($field)*
            }

            impl ResourceId for $name {
                type Id = Self;

                fn id(&self) -> &Self::Id {
                    self
                }
            }
        };
    }

    #[macro_export]
    macro_rules! impl_resource {
        ($name:ident, $id:ident) => {
            impl ResourceId for $name {
                type Id = $id;

                fn id(&self) -> &Self::Id {
                    &self.id
                }
            }

            impl Resource for $name {
                fn received_at(&self) -> DateTime<Utc> {
                    self.received_at
                }
            }
        };
    }
}

pub(self) mod prelude {
    pub use crate::{
        client::{Client, Context},
        create_id,
        http::{Api, Route},
        impl_resource,
    };

    pub use super::*;
}

pub use channel::{message::*, text::*, *};
pub use gateway::*;
pub use guild::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
#[serde(transparent)]
pub struct Snowflake(u64);

impl ToString for Snowflake {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

pub trait ResourceId {
    type Id: Clone + Eq + Hash + Send + Sync;

    fn id(&self) -> &Self::Id;
}

pub trait Resource: ResourceId + Clone {
    fn received_at(&self) -> DateTime<Utc>;
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
