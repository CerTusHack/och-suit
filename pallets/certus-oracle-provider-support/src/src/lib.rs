#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::sp_runtime::Percent;
use frame_support::traits::ConstU32;
use frame_support::weights::Weight;
use frame_support::{BoundedVec, RuntimeDebug};
use lite_json::JsonValue;
use scale_info::TypeInfo;
use sp_std::vec::Vec;
use sp_std::fmt::Debug;
use frame_support::traits::tokens::Balance;
use sp_runtime::traits::AtLeast32BitUnsigned;
use sp_std::convert::TryInto;

pub type MaximumDataKey = ConstU32<15>;
pub type MaximumDataValue = ConstU32<15>;
pub type MaximumPreCheckListSize = ConstU32<500>;
pub type MaximumRequestBaseUrlSize =  ConstU32<500>;
pub type MaximumCertusOracleAuthoritieSize =  ConstU32<500>;
pub type MaximumPoolSize = ConstU32<1000>;

pub type DataKey = BoundedVec<u8, MaximumDataKey>;
pub type DataValue = BoundedVec<u8, MaximumDataValue>;

pub type RawDataKeys = BoundedVec<(DataKey, DataValue), MaximumPoolSize>;

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct PreCheckStruct {
    pub data_key: DataKey,
    pub data_value: JsonDataValue,
    pub timestamp: u64,
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub enum PreCheckStatus {
    Review,
    Prohibit,
    Pass,
}

impl Default for PreCheckStatus {
    fn default() -> Self {
        Self::Prohibit
    }
}

/// Pre-checked state configuration data, which is saved on-chain
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug,  TypeInfo)]
pub struct PreCheckTaskConfig {
    /// List of data keys to check.
    pub check_data_list: BoundedVec<DataKey, MaximumPoolSize>,
}

impl Default for PreCheckTaskConfig {
    fn default() -> Self {
        Self {
            check_data_list: Default::default(),
        }
    }
}

/// `Pre-Check`trait
pub trait ICertusOraclePreCheck<AccountId, AuthorityId, BlockNumber> {
    fn has_pre_check_task(stash: AccountId) -> bool;
    fn get_pre_task_by_authority_set(auth_list: Vec<AuthorityId>) -> Option<(AccountId, AuthorityId, BlockNumber)>;
    fn check_and_clean_obsolete_task(maximum_due: BlockNumber) -> Weight;
    fn take_data_for_pre_check(check_config: PreCheckTaskConfig) -> PreCheckList;
    fn save_pre_check_result(stash: AccountId, bn: BlockNumber, pre_check_list: PreCheckList, auth: AuthorityId) -> PreCheckStatus;
    fn get_pre_check_status(stash: AccountId) -> Option<(BlockNumber, PreCheckStatus)>;
    fn clean_pre_check_status(stash: AccountId);
    fn create_pre_check_task(stash: AccountId, auth: AuthorityId, bn: BlockNumber) -> bool;
}

impl<AC, AU, B> ICertusOraclePreCheck<AC, AU, B> for () {
    fn has_pre_check_task(_stash: AC) -> bool {
        false
    }
    fn get_pre_task_by_authority_set(_auth_list: Vec<AU>) -> Option<(AC, AU, B)> {
        None
    }
    fn check_and_clean_obsolete_task(_maximum_due: B) -> u64 {
        0
    }
    fn take_data_for_pre_check(_check_config: PreCheckTaskConfig) -> PreCheckList {
        Default::default()
    }
    fn save_pre_check_result(_stash: AC, _bn: B, _pre_check_list: PreCheckList, _auth: AU) -> PreCheckStatus { PreCheckStatus::Review }
    fn get_pre_check_status(_stash: AC) -> Option<(B, PreCheckStatus)> {
        None
    }
    fn clean_pre_check_status(_stash: AC) {}
    fn create_pre_check_task(_stash: AC, _auth: AU, _bn: B) -> bool {
        false
    }
}

pub trait SymbolInfo<BlockNumber> {
    fn data(symbol: &DataKey) -> Result<(JsonValue, BlockNumber), ()>;
}

pub trait ConvertData {
    fn try_to_data(self) -> Option<JsonValue>;
    fn convert_to_json_data_value(self) -> JsonDataValue;
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct JsonDataValue {
    pub data: JsonValue,
}

impl JsonDataValue {
    pub fn new(data: JsonValue) -> Self {
        Self { data }
    }
}

impl Default for JsonDataValue {
    fn default() -> Self {
        Self {
            data: JsonValue::Null,
        }
    }
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub enum DataStatus {
    Review,
    Prohibit,
    Pass,
}

impl Default for DataStatus {
    fn default() -> Self {
        Self::Prohibit
    }
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct PreCheckDataStruct {
    pub data_key: DataKey,
    pub data_value: JsonDataValue,
    pub timestamp: u64,
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct PreCheckDataStatus {
    pub status: DataStatus,
}

pub type PreCheckList = BoundedVec<PreCheckDataStruct, MaximumPreCheckListSize>;

pub trait IOracleAvgDataEvents<BlockNumber, DataKey> {
    fn avg_data_update(symbol: DataKey, bn: BlockNumber, data: JsonValue);
}

pub trait IStashAndAuthority<StashAcc, AuthroityAcc> {
    fn get_auth_id(stash: &StashAcc) -> Option<AuthroityAcc>;
    fn get_stash_id(auth: &AuthroityAcc) -> Option<StashAcc>;
    fn get_authority_list_of_local() -> Vec<AuthroityAcc>;
    fn get_list_of_storage() -> Vec<(StashAcc, AuthroityAcc)>;
    fn check_block_author_and_sotre_key_the_same(block_author: &AuthroityAcc) -> bool;
}

impl <StashAcc, AuthroityAcc> IStashAndAuthority <StashAcc, AuthroityAcc> for () {

    /// Get the `certus-authority` through `stash-id`
    fn get_auth_id(stash: &StashAcc) -> Option<AuthroityAcc> {
        None
    }

    /// Get the `stash-id` through `certus-authority`
    fn get_stash_id(auth: &AuthroityAcc) -> Option<StashAcc> {
        None
    }

    /// Get all `certus-authorities` users in keystore.
    fn get_authority_list_of_local() -> Vec<AuthroityAcc> {
        Vec::new()
    }

    fn get_list_of_storage() -> Vec<(StashAcc, AuthroityAcc)> {
        Vec::new()
    }

    /// Check whether the authority of the current block author has a private key on the local node.
    fn check_block_author_and_sotre_key_the_same(block_author: &AuthroityAcc) -> bool {
        false
    }
}