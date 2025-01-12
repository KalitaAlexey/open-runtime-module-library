#![cfg(test)]

use super::*;
use cumulus_primitives_core::ParaId;
use frame_support::{assert_noop, assert_ok, traits::Currency};
use mock::*;
use orml_traits::MultiCurrency;
// use polkadot_parachain::primitives::{AccountIdConversion, Sibling};
use polkadot_parachain::primitives::AccountIdConversion;
use sp_runtime::AccountId32;
use xcm::v0::{Junction, NetworkId};
use xcm_simulator::TestExt;

fn para_a_account() -> AccountId32 {
	ParaId::from(1).into_account()
}

// fn para_b_account() -> AccountId32 {
// 	ParaId::from(2).into_account()
// }

// fn sibling_a_account() -> AccountId32 {
// 	use sp_runtime::traits::AccountIdConversion;
// 	Sibling::from(1).into_account()
// }

// fn sibling_b_account() -> AccountId32 {
// 	use sp_runtime::traits::AccountIdConversion;
// 	Sibling::from(2).into_account()
// }

// fn sibling_c_account() -> AccountId32 {
// 	use sp_runtime::traits::AccountIdConversion;
// 	Sibling::from(3).into_account()
// }

#[test]
fn send_relay_chain_asset_to_relay_chain() {
	TestNet::reset();

	Relay::execute_with(|| {
		let _ = RelayBalances::deposit_creating(&para_a_account(), 1_000);
	});

	ParaA::execute_with(|| {
		assert_ok!(ParaXTokens::transfer(
			Some(ALICE).into(),
			CurrencyId::R,
			500,
			(
				Parent,
				Junction::AccountId32 {
					network: NetworkId::Kusama,
					id: BOB.into(),
				},
			)
				.into(),
			30,
		));
		assert_eq!(ParaTokens::free_balance(CurrencyId::R, &ALICE), 500);
	});

	Relay::execute_with(|| {
		assert_eq!(RelayBalances::free_balance(&para_a_account()), 500);
		assert_eq!(RelayBalances::free_balance(&BOB), 470);
	});
}

#[test]
fn cannot_lost_fund_on_send_failed() {
	TestNet::reset();

	ParaA::execute_with(|| {
		assert_ok!(ParaXTokens::transfer(
			Some(ALICE).into(),
			CurrencyId::R,
			500,
			(
				Parent,
				Parachain(3),
				Junction::AccountId32 {
					network: NetworkId::Kusama,
					id: BOB.into(),
				},
			)
				.into(),
			30,
		));
		para::System::events().iter().any(|r| {
			if let para::Event::XTokens(Event::<para::Runtime>::TransferFailed(_, _, _, _, _)) = r.event {
				true
			} else {
				false
			}
		});

		assert_eq!(ParaTokens::free_balance(CurrencyId::R, &ALICE), 1_000);
	});
}

// #[test]
// fn send_relay_chain_asset_to_sibling() {
// 	TestNet::reset();

// 	Relay::execute_with(|| {
// 		let _ = RelayBalances::deposit_creating(&para_a_account(), 100);
// 	});

// 	ParaA::execute_with(|| {
// 		assert_ok!(ParaXTokens::transfer(
// 			Some(ALICE).into(),
// 			CurrencyId::R,
// 			30,
// 			(
// 				Parent,
// 				Parachain { id: 2 },
// 				Junction::AccountId32 {
// 					network: NetworkId::Any,
// 					id: BOB.into(),
// 				},
// 			)
// 				.into(),
// 		));
// 		assert_eq!(ParaTokens::free_balance(CurrencyId::R, &ALICE), 70);
// 	});

// 	Relay::execute_with(|| {
// 		assert_eq!(RelayBalances::free_balance(&para_a_account()), 70);
// 		assert_eq!(RelayBalances::free_balance(&para_b_account()), 30);
// 	});

// 	ParaB::execute_with(|| {
// 		assert_eq!(ParaTokens::free_balance(CurrencyId::R, &BOB), 30);
// 	});
// }

// #[test]
// fn send_sibling_asset_to_reserve_sibling() {
// 	TestNet::reset();

// 	ParaA::execute_with(|| {
// 		assert_ok!(ParaTokens::deposit(CurrencyId::B, &ALICE, 100));
// 	});

// 	ParaB::execute_with(|| {
// 		assert_ok!(ParaTokens::deposit(CurrencyId::B, &sibling_a_account(), 100));
// 	});

// 	ParaA::execute_with(|| {
// 		assert_ok!(ParaXTokens::transfer(
// 			Some(ALICE).into(),
// 			CurrencyId::B,
// 			30,
// 			(
// 				Parent,
// 				Parachain { id: 2 },
// 				Junction::AccountId32 {
// 					network: NetworkId::Any,
// 					id: BOB.into(),
// 				},
// 			)
// 				.into(),
// 		));

// 		assert_eq!(ParaTokens::free_balance(CurrencyId::B, &ALICE), 70);
// 	});

// 	ParaB::execute_with(|| {
// 		assert_eq!(ParaTokens::free_balance(CurrencyId::B, &sibling_a_account()),
// 70); 		assert_eq!(ParaTokens::free_balance(CurrencyId::B, &BOB), 30);
// 	});
// }

// #[test]
// fn send_sibling_asset_to_non_reserve_sibling() {
// 	TestNet::reset();

// 	ParaA::execute_with(|| {
// 		assert_ok!(ParaTokens::deposit(CurrencyId::B, &ALICE, 100));
// 	});

// 	ParaB::execute_with(|| {
// 		assert_ok!(ParaTokens::deposit(CurrencyId::B, &sibling_a_account(), 100));
// 	});

// 	ParaA::execute_with(|| {
// 		assert_ok!(ParaXTokens::transfer(
// 			Some(ALICE).into(),
// 			CurrencyId::B,
// 			30,
// 			(
// 				Parent,
// 				Parachain { id: 3 },
// 				Junction::AccountId32 {
// 					network: NetworkId::Any,
// 					id: BOB.into(),
// 				},
// 			)
// 				.into(),
// 		));
// 		assert_eq!(ParaTokens::free_balance(CurrencyId::B, &ALICE), 70);
// 	});

// 	// check reserve accounts
// 	ParaB::execute_with(|| {
// 		assert_eq!(ParaTokens::free_balance(CurrencyId::B, &sibling_a_account()),
// 70); 		assert_eq!(ParaTokens::free_balance(CurrencyId::B,
// &sibling_c_account()), 30); 	});

// 	ParaC::execute_with(|| {
// 		assert_eq!(ParaTokens::free_balance(CurrencyId::B, &BOB), 30);
// 	});
// }

// #[test]
// fn send_self_parachain_asset_to_sibling() {
// 	TestNet::reset();

// 	ParaA::execute_with(|| {
// 		assert_ok!(ParaTokens::deposit(CurrencyId::A, &ALICE, 100));

// 		assert_ok!(ParaXTokens::transfer(
// 			Some(ALICE).into(),
// 			CurrencyId::A,
// 			30,
// 			(
// 				Parent,
// 				Parachain { id: 2 },
// 				Junction::AccountId32 {
// 					network: NetworkId::Any,
// 					id: BOB.into(),
// 				},
// 			)
// 				.into(),
// 		));

// 		assert_eq!(ParaTokens::free_balance(CurrencyId::A, &ALICE), 70);
// 		assert_eq!(ParaTokens::free_balance(CurrencyId::A, &sibling_b_account()),
// 30); 	});

// 	ParaB::execute_with(|| {
// 		para_b::System::events().iter().for_each(|r| {
// 			println!(">>> {:?}", r.event);
// 		});
// 		assert_eq!(ParaTokens::free_balance(CurrencyId::A, &BOB), 30);
// 	});
// }

#[test]
fn transfer_no_reserve_assets_fails() {
	TestNet::reset();

	ParaA::execute_with(|| {
		assert_noop!(
			ParaXTokens::transfer_multiasset(
				Some(ALICE).into(),
				MultiAsset::ConcreteFungible {
					id: GeneralKey("B".into()).into(),
					amount: 100
				},
				(
					Parent,
					Parachain(2),
					Junction::AccountId32 {
						network: NetworkId::Any,
						id: BOB.into()
					}
				)
					.into(),
				50,
			),
			Error::<para::Runtime>::AssetHasNoReserve
		);
	});
}

#[test]
fn transfer_to_self_chain_fails() {
	TestNet::reset();

	ParaA::execute_with(|| {
		assert_noop!(
			ParaXTokens::transfer_multiasset(
				Some(ALICE).into(),
				MultiAsset::ConcreteFungible {
					id: (Parent, Parachain(1), GeneralKey("A".into())).into(),
					amount: 100
				},
				(
					Parent,
					Parachain(1),
					Junction::AccountId32 {
						network: NetworkId::Any,
						id: BOB.into()
					}
				)
					.into(),
				50,
			),
			Error::<para::Runtime>::NotCrossChainTransfer
		);
	});
}

#[test]
fn transfer_to_invalid_dest_fails() {
	TestNet::reset();

	ParaA::execute_with(|| {
		assert_noop!(
			ParaXTokens::transfer_multiasset(
				Some(ALICE).into(),
				MultiAsset::ConcreteFungible {
					id: (Parent, Parachain(1), GeneralKey("A".into())).into(),
					amount: 100,
				},
				(Junction::AccountId32 {
					network: NetworkId::Any,
					id: BOB.into()
				})
				.into(),
				50,
			),
			Error::<para::Runtime>::InvalidDest
		);
	});
}
