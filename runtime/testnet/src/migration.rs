use crate::*;
use frame_support::{traits::OnRuntimeUpgrade, weights::Weight};
use pallet_tx_pause::{PalletCallNameOf, PalletNameOf, WeightInfo};
use sp_std::vec;

/// Migration to set up the TxPause pallet.
pub struct TxPauseRuntimeMigration;
impl OnRuntimeUpgrade for TxPauseRuntimeMigration {
	fn on_runtime_upgrade() -> Weight {
		log::info!("Running TxPause runtime migration...");

		let pallet_name: PalletNameOf<Runtime> = match b"CollatorStaking".to_vec().try_into() {
			Ok(name) => name,
			Err(_) => {
				log::error!("Failed to convert pallet name");
				return Weight::zero();
			},
		};

		let call_names = vec![
			b"set_invulnerables".to_vec(),
			b"set_desired_candidates".to_vec(),
			b"set_min_candidacy_bond".to_vec(),
			b"register_as_candidate".to_vec(),
			b"leave_intent".to_vec(),
			b"add_invulnerable".to_vec(),
			b"remove_invulnerable".to_vec(),
			b"stake".to_vec(),
			b"unstake_from".to_vec(),
			b"unstake_all".to_vec(),
			b"release".to_vec(),
			b"set_autocompound".to_vec(),
			b"set_collator_reward_percentage".to_vec(),
			b"set_extra_reward".to_vec(),
			b"set_minimum_stake".to_vec(),
			b"stop_extra_reward".to_vec(),
			b"top_up_extra_rewards".to_vec(),
			b"lock".to_vec(),
			b"unlock".to_vec(),
			b"update_candidacy_bond".to_vec(),
			b"claim_rewards".to_vec(),
			// only claim_rewards_other is allowed
		];
		let total_calls = call_names.len() as u64;

		for raw_call_name in call_names {
			let call: PalletCallNameOf<Runtime> = match raw_call_name.clone().try_into() {
				Ok(name) => name,
				Err(_) => {
					log::error!("Failed to convert call name '{:?}'", raw_call_name);
					return Weight::zero()
				},
			};
			if let Err(e) = pallet_tx_pause::Pallet::<Runtime>::pause(RuntimeOrigin::root(), (pallet_name.clone(), call)) {
				log::error!("Failed to pause call '{:?}': {:?}", raw_call_name, e);
			}
		}

		log::info!("Completed TxPause runtime migration.");
		weights::pallet_tx_pause::WeightInfo::<Runtime>::pause().saturating_mul(total_calls)
	}
}
