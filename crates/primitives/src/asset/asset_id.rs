use crate::Chain;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AssetId {
    pub chain: Chain,
    pub token_id: Option<String>,
}

impl Serialize for AssetId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for AssetId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        AssetId::new(&s).ok_or_else(|| de::Error::custom("Invalid AssetId"))
    }
}

impl fmt::Display for AssetId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let str = match &self.token_id {
            Some(token_id) => {
                format!("{}_{}", self.chain.as_ref(), token_id)
            }
            None => self.chain.as_ref().to_owned(),
        };
        write!(f, "{str}")
    }
}

impl AssetId {
    pub fn new(asset_id: &str) -> Option<Self> {
        let split: Vec<&str> = asset_id.split('_').collect();
        if split.len() == 1 {
            if let Ok(chain) = asset_id.parse::<Chain>() {
                return Some(AssetId { chain, token_id: None });
            }
        } else if split.len() >= 2
            && let Ok(chain) = split[0].parse::<Chain>()
        {
            return Some(AssetId {
                chain,
                token_id: Some(split[1..split.len()].join("_")),
            });
        }
        None
    }

    pub fn from_chain(chain: Chain) -> AssetId {
        AssetId { chain, token_id: None }
    }
}
