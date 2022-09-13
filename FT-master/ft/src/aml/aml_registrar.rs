use crate::*;

use near_sdk::{collections::UnorderedMap, serde::{Serialize, Deserialize}};

// #[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, PartialEq, Debug)]
// #[serde(crate = "near_sdk::serde")]
// pub enum Category {
//     // for all unspecified categories
//     All,
//     None,
//     // Wallet service - custodial or mixed wallets
//     WalletService,
//     // Merchant service
//     MerchantService,
//     // Mining pool
//     MiningPool,
//     // Low risk exchange - Exchange with high KYC standards
//     LowRiskExchange,
//     // Medium eisk exchange
//     MediumRiskExchange,
//     // DeFi application
//     DeFi,
//     // OTC Broker
//     OTCBroker,
//     // Cryptocurrency ATM
//     ATM,
//     // Gambling
//     Gambling,
//     // Illicit organization
//     IllicitOrganization,
//     // Mixer
//     Mixer,
//     // Darknet market or service
//     DarknetService,
//     // Scam
//     Scam,
//     // Ransomware
//     Ransomware,
//     // Theft - stolen funds
//     Theft,
//     // Counterfeit - fake assets
//     Counterfeit,
//     // Terrorist financing
//     TerroristFinancing,
//     // Sanctions
//     Sanctions,
//     // Child abuse and porn materials
//     ChildAbuse,
// }

pub type Category = String;
pub const INITIAL_MAX_RISK_LEVEL: u8 = 10;

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct AML {
    pub account_id: AccountId,
    pub aml_conditions: UnorderedMap<Category, u8>,
}

pub trait AMl {
    fn get_aml(&self) -> (&AccountId, Vec<(Category, u8)>);

    fn update_aml_account_id(&mut self, aml_account_id: AccountId);

    fn update_aml_category(&mut self, category: Category, accepted_risk_score: u8);
    fn remove_aml_category(&mut self, category: Category);
}

#[near_bindgen]
impl AMl for Contract {
    fn get_aml(&self) -> (&AccountId, Vec<(Category, u8)>) {
        (&self.aml.account_id, self.aml.aml_conditions.iter().map(|(id, acc)| (id, acc)).collect())
    }

    fn update_aml_account_id(&mut self, aml_account_id: AccountId) {
        self.assert_owner();
        self.aml.set_account_id(aml_account_id);
    }

    fn update_aml_category(&mut self, category: Category, accepted_risk_score: u8) {
        self.assert_owner();
        assert!(accepted_risk_score <= INITIAL_MAX_RISK_LEVEL, "ERR_RISK_SCORE_IS_INVALID");
        assert!(accepted_risk_score > 0, "ERR_RISK_SCORE_IS_INVALID");
        self.aml.set_category(category, accepted_risk_score);
    }

    fn remove_aml_category(&mut self, category: Category) {
        self.assert_owner();
        self.aml.remove_category(category);
    }
}

impl AML {
    pub fn new(account_id: AccountId, category: Category, accepted_risk_score: u8) -> AML {
        let mut aml_conditions = UnorderedMap::new(b'c');
        aml_conditions.insert(&category, &accepted_risk_score);
        Self { account_id, aml_conditions }
    }

    pub fn set_account_id(&mut self, account_id: AccountId) {
        self.account_id = account_id;
    }

    pub fn set_category(&mut self, category: Category, accepted_risk_score: u8) {
        self.aml_conditions.insert(&category, &accepted_risk_score);
    }

    pub fn remove_category(&mut self, category: Category) {
        self.aml_conditions.remove(&category);
    }
}
