use std::fmt;
use strum::{AsRefStr, EnumString};

#[derive(AsRefStr, EnumString)]
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
    pub fn rank(&self) -> i32 {
        match self {
            Self::Solana => 100,
            Self::Ethereum => 90,
            Self::Bitcoin => 80,
        }
    }
}
