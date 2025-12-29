use serde::{Deserialize, Serialize};
use std::fmt;
use strum::{AsRefStr, EnumString};

use crate::AssetId;

#[derive(Copy, Clone, Debug, Serialize, Deserialize, AsRefStr, EnumString, PartialEq, Eq, Hash)]
#[strum(serialize_all = "lowercase")]
pub enum Chain {
    Bitcoin,
    Ethereum,
    Solana,
}

impl fmt::Display for Chain {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

impl Chain {
    pub fn as_asset_id(&self) -> AssetId {
        AssetId::from_chain(*self)
    }

    pub fn rank(&self) -> i32 {
        match self {
            Self::Solana => 100,
            Self::Ethereum => 90,
            Self::Bitcoin => 80,
        }
    }
}
