use primitives::Chain;

pub fn get_default_rank(chain: Chain) -> i32 {
    chain.rank()
}

#[uniffi::export]
pub fn asset_default_rank(chain: Chain) -> i32 {
    get_default_rank(chain)
}
