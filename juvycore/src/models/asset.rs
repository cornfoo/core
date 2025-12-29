use primitives::{Asset, AssetId, AssetType, Chain};

pub type TinAsset = Asset;
pub type TinAssetId = AssetId;
pub type TinAssetType = AssetType;

#[allow(non_camel_case_types)]
#[uniffi::remote(Enum)]
pub enum TinAssetType {
    NATIVE,
    ERC20,
    SPL,
    SPL2022,
}

#[uniffi::remote(Record)]
pub struct TinAsset {
    pub id: TinAssetId,
    pub chain: Chain,
    pub token_id: Option<String>,
    pub name: String,
    pub symbol: String,
    pub decimals: i32,
    pub asset_type: TinAssetType,
}

pub fn get_default_rank(chain: Chain) -> i32 {
    chain.rank()
}

pub fn get_asset(chain: Chain) -> TinAsset {
    Asset::from_chain(chain)
}

#[uniffi::export]
pub fn asset_default_rank(chain: Chain) -> i32 {
    get_default_rank(chain)
}

#[uniffi::export]
pub fn asset_wrapper(chain: Chain) -> TinAsset {
    get_asset(chain)
}
