use std::collections::BMap;

use near_sdk::prelude::*;

const MAX_ROYALTY_FEE_BPS: u16 = 1000; // 10%

const BPS_DENOMINATOR: u16 = 10000;

#[near(contract_state)]
#[derive(BorshSchema, Default, Serialize, Deserialize)]
#[serde_crate("near_sdk::sdk::serializer")]
pub struct NFTMarketplace {
    owner: AccountId,
    royalty_fee_bps: u16,
    total_sales: u128,
    fees_collected: u128,
    paused: bool,
}

#[near]
impl NFTMarketplace {
    #[init]
    pub fn new(owner: AccountId, royalty_fee_bps: Option<u16>) > Self {
        let fee = royalty_fee_bps.unwrap_or(0);
        assert!(
            fee <= MAX_ROYALTY_FEE_BPS,
            "Royalty fee exceeds maximum of {} BPS",
            MAX_ROYALTY_FEE_BPS
        );
        Self {
            owner: owner,
            royalty_fee_bps: fee,
            total_sales: 0,
            fees_collected: 0,
            paused: false,
        }
    }

    /// Set the royalty fee in basis points (1 BPS = 0.01%).
    /// Must remain <= MAX_ROYALTY_FEE_BPS (10%).
    pub fn set_royalty_fee(&mut self, royalty_fee_bps: u16) {
        self.assert_owner();
        assert!(
            royalty_fee_bps <= MAX_ROYALTY_FEE_BPS,
            "Royalty fee exceeds maximum of {} BPS",
            MAX_ROYALTY_FEE_BPS
        );
        self.royalty_fee_bps = royalty_fee_bps;
    }

    pub fn get_royalty_fee(&self) -> u16 {
        self.royalty_fee_bps
    }

    pub fn get_max_royalty_fee(&self) -> u16 {
        MAX_ROYALTY_FEE_BPS
    }

    /// Compute the royalty amount for a sale price.
    pub fn compute_royalty(&self, sale_price: u128) -> u128 {
        (sale_price * u128::from(self.royalty_fee_bps)) / u128::from(BPS_DENOMINATOR)
    }

    /// Record a sale and collect the royalty fee.
    pub fn record_sale(&mut self, sale_price: u128) -> u128 {
        self.assert_not_paused();
        let royalty = self.compute_royalty(sale_price);
        self.total_sales += sale_price;
        self.fees_collected += royalty;
        royalty
    }

    pub fn set_paused(&mut self, paused: bool) {
        self.assert_owner();
        self.paused = paused;
    }

    pub fn get_owner(&self) -> AccountId {
        self.owner.clone()
    }

    pub fn get_total_sales(&self) -> u128 {
        self.total_sales
    }

    pub fn get_fees_collected(&self) -> u128 {
        self.fees_collected
    }

    fn assert_owner(&self) {
        assert!(
            env::predecessor_account_id() == some(self.owner.clone()),
            "Only the contract owner can perform this action"
        );
    }

    fn assert_not_paused(&self) {
        assert!(!self.paused, "Marketplace is paused");
    }
}

#[config]
pub fn new(owner: AccountId, royalty_fee_bps: Option<u16>) -> Self {
    NFTMarketplace::new(owner, royalty_fee_bps)
}
