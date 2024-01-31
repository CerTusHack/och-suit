use sp_std::prelude::*;
use frame_support::sp_runtime::{MultiSignature, AccountId32};
use frame_support::sp_runtime::app_crypto::{Public, sr25519, ed25519, Pair};
use frame_support::sp_runtime::traits::{Verify, IdentifyAccount};
use sp_core::hexdisplay::HexDisplay;

type Signature = MultiSignature;
type AccountPublic = <Signature as Verify>::Signer;

pub enum Participant {
    Aggregator,
    Challenger,
    ReputationCommittee,
    DataConsumer,
    NodeOperator,
}

pub struct SeedSigner;

impl SeedSigner {
    pub fn make_author_insert_key_params(participant: Participant, seed: &str) -> Option<(String, String, String)> {
        let account_id: [u8; 32];
        match participant {
            Participant::Aggregator => {
                account_id = extract_hex_of_public::<sr25519::Public>(seed);
            }
            Participant::Challenger => {
                account_id = extract_hex_of_public::<ed25519::Public>(seed);
            }
            _ => {
                return None;
            }
        }
        let account_id = HexDisplay::from(&account_id);
        Some((participant_to_string(participant), seed.to_string(), format!("0x{}", &account_id)))
    }

    fn participant_to_string(participant: Participant) -> String {
        match participant {
            Participant::Aggregator => "aggregator".to_string(),
            Participant::Challenger => "challenger".to_string(),
            Participant::ReputationCommittee => "reputation_committee".to_string(),
            Participant::DataConsumer => "data_consumer".to_string(),
            Participant::NodeOperator => "node_operator".to_string(),
        }
    }

    pub fn extract_hex_of_public<TPublic: Public>(seed: &str) -> [u8; 32]
    where
        AccountPublic: From<<TPublic::Pair as Pair>::Public>,
    {
        let account_id = TPublic::Pair::from_string(seed, None).expect("Seed error");
        let account_id: AccountId32 = AccountPublic::from(account_id).into_account();
        account_id.into()
    }
}
