extern crate mfight_sdk;
extern crate near_sdk;

use near_sdk::{
    AccountId,
    Balance,
    env,
    log,
    near_bindgen,
    PanicOnDefault,
    PromiseOrValue,
    BorshStorageKey,
};
use near_sdk::borsh::{ self, BorshDeserialize, BorshSerialize };
use near_sdk::collections::LazyOption;
use near_sdk::json_types::U128;
use mfight_sdk::referral::{ReferralFeature, ContractId, AccountContractId, InfluencerId, InfluencerProgramId, InfluencerRoyalty, ProgramId, ReferralInfo, ContractProgramId, ReferralProgramMetadata};
use near_sdk::collections::{ LookupMap, TreeMap, UnorderedSet };
use mfight_sdk::storage::StorageFeature;

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    Influencer,
    ContractReferrals,
    InfluencerReferrals,
    ProgramReferrals,
    ProgramRoyalty,
    ContractPrograms,
    InfluencersPrograms,
    CodeByProgram,
    InfoByCode,
    ReferralProgramMetadata,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    referral: ReferralFeature,
}

#[near_bindgen]
impl Contract {
    /// Initializes the contract with the given total supply owned by the given `owner_id` with
    /// default metadata (for example purposes only).
    #[init]
    pub fn new_default_meta(owner_id: AccountId) -> Self {
        Self::new(owner_id)
    }

    /// Initializes the contract with the given total supply owned by the given `owner_id` with
    /// the given fungible token metadata.
    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        assert!(!env::state_exists(), "Already initialized");

        let mut this = Self {
            referral: ReferralFeature::new(
                StorageKey::Influencer,
                StorageKey::ContractReferrals,
                StorageKey::InfluencerReferrals,
                StorageKey::ProgramReferrals,
                StorageKey::ProgramRoyalty,
                StorageKey::ContractPrograms,
                StorageKey::InfluencersPrograms,
                StorageKey::CodeByProgram,
                StorageKey::InfoByCode,
                StorageKey::ReferralProgramMetadata,
            ),
        };

        this
    }

    // pub fn test(&mut self) -> u128 {
    //   self.referral.calc_min_storage()
    // }

    #[init(ignore_state)]
    #[private]
    pub fn migrate() -> Self {
        #[derive(BorshDeserialize, BorshSerialize)]
        pub struct OldReferral {
            pub influencer_by_id: LookupMap<AccountContractId, InfluencerId>,
            pub referrals_by_contract: TreeMap<ContractId, UnorderedSet<AccountId>>,
            pub referrals_by_influencer: TreeMap<InfluencerId, UnorderedSet<AccountId>>,
            pub referrals_by_program: TreeMap<InfluencerProgramId, UnorderedSet<AccountId>>,

            pub royalty_by_program: LookupMap<InfluencerProgramId, InfluencerRoyalty>,
            pub metadata_by_program: LookupMap<ContractProgramId, ReferralProgramMetadata>,

            pub programs_by_contract: TreeMap<
                ContractId,
                TreeMap<InfluencerId, UnorderedSet<ProgramId>>
            >,
            pub programs_by_influencer: TreeMap<
                InfluencerId,
                TreeMap<ContractId, UnorderedSet<ProgramId>>
            >,

            pub code_by_program: LookupMap<InfluencerProgramId, String>,
            pub info_by_code: LookupMap<String, ReferralInfo>,
        }
        #[derive(BorshDeserialize)]
        struct Old {
            referral: OldReferral,
        }

        let old: Old = env::state_read().expect("Error");
        let referral = ReferralFeature {
            influencer_by_id: old.referral.influencer_by_id,
            referrals_by_contract: old.referral.referrals_by_contract,
            referrals_by_influencer: old.referral.referrals_by_influencer,
            referrals_by_program: old.referral.referrals_by_program,
            royalty_by_program: old.referral.royalty_by_program,
            programs_by_contract: old.referral.programs_by_contract,
            programs_by_influencer: old.referral.programs_by_influencer,
            code_by_program: old.referral.code_by_program,
            info_by_code: old.referral.info_by_code,
            metadata_by_program: old.referral.metadata_by_program
        };

        Self {
            referral,
        }
    }

    pub fn assert_access(&self) {}
}

mfight_sdk::impl_referral_core!(Contract, referral, assert_access);
mfight_sdk::impl_referral_enumeration!(Contract, referral);
mfight_sdk::impl_referral_code!(Contract, referral, assert_access);
