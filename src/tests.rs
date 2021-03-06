// Tests to be written here

use crate::mock::*;
use crate::nft::UniqueAssets;
use crate::*;
use frame_support::{assert_err, assert_ok, Hashable};
use frame_system::RawOrigin as Origin;
use sp_core::H256;

#[test]
fn mint() {
    new_test_ext().execute_with(|| {
        assert_eq!(SUT::total(), 0);
        assert_eq!(SUT::total_for_account(&1), 0);
        assert_eq!(<SUT as UniqueAssets<_>>::total(), 0);
        assert_eq!(<SUT as UniqueAssets<_>>::total_for_account(&1), 0);
        assert_eq!(
            SUT::account_for_commodity::<H256>(Vec::<u8>::default().blake2_256().into()),
            None
        );

        assert_ok!(SUT::mint(Origin::Root.into(), 1, Vec::<u8>::default()));

        assert_eq!(SUT::total(), 1);
        assert_eq!(<SUT as UniqueAssets<_>>::total(), 1);
        assert_eq!(SUT::burned(), 0);
        assert_eq!(<SUT as UniqueAssets<_>>::burned(), 0);
        assert_eq!(SUT::total_for_account(&1), 1);
        assert_eq!(<SUT as UniqueAssets<_>>::total_for_account(&1), 1);
        let commodities_for_account = SUT::commodities_for_account::<u64>(1);
        assert_eq!(commodities_for_account.len(), 1);
        assert_eq!(
            commodities_for_account[0].0,
            Vec::<u8>::default().blake2_256().into()
        );
        assert_eq!(commodities_for_account[0].1, Vec::<u8>::default());
        assert_eq!(
            SUT::account_for_commodity::<H256>(Vec::<u8>::default().blake2_256().into()),
            Some(1)
        );
    });
}

#[test]
fn mint_err_non_admin() {
    new_test_ext().execute_with(|| {
        assert_err!(
            SUT::mint(Origin::Signed(1).into(), 1, Vec::<u8>::default()),
            sp_runtime::DispatchError::BadOrigin
        );
    });
}

#[test]
fn mint_err_dupe() {
    new_test_ext().execute_with(|| {
        assert_ok!(SUT::mint(Origin::Root.into(), 1, Vec::<u8>::default()));

        assert_err!(
            SUT::mint(Origin::Root.into(), 2, Vec::<u8>::default()),
            Error::<Test, DefaultInstance>::CommodityExists
        );
    });
}

#[test]
fn mint_err_max_user() {
    new_test_ext().execute_with(|| {
        assert_ok!(SUT::mint(Origin::Root.into(), 1, vec![]));
        assert_ok!(SUT::mint(Origin::Root.into(), 1, vec![0]));

        assert_err!(
            SUT::mint(Origin::Root.into(), 1, vec![1]),
            Error::<Test, DefaultInstance>::TooManyCommoditiesForAccount
        );
    });
}

#[test]
fn mint_err_max() {
    new_test_ext().execute_with(|| {
        assert_ok!(SUT::mint(Origin::Root.into(), 1, vec![]));
        assert_ok!(SUT::mint(Origin::Root.into(), 2, vec![0]));
        assert_ok!(SUT::mint(Origin::Root.into(), 3, vec![1]));
        assert_ok!(SUT::mint(Origin::Root.into(), 4, vec![2]));
        assert_ok!(SUT::mint(Origin::Root.into(), 5, vec![3]));

        assert_err!(
            SUT::mint(Origin::Root.into(), 6, vec![4]),
            Error::<Test, DefaultInstance>::TooManyCommodities
        );
    });
}

#[test]
fn burn() {
    new_test_ext().execute_with(|| {
        assert_ok!(SUT::mint(Origin::Root.into(), 1, Vec::<u8>::from("test")));
        assert_eq!(SUT::total_for_account(1), 1);

        let assets = SUT::assets_for_account(&1);

        assert_ok!(SUT::burn(Origin::Signed(1).into(), assets[0].0));

        assert_eq!(SUT::total(), 0);
        assert_eq!(SUT::burned(), 1);
        assert_eq!(SUT::total_for_account(1), 0);
        assert_eq!(SUT::commodities_for_account::<u64>(1), vec![]);
        assert_eq!(
            SUT::account_for_commodity::<H256>(Vec::<u8>::default().blake2_256().into()),
            None
        );
    });
}

#[test]
fn burn_err_not_owner() {
    new_test_ext().execute_with(|| {
        assert_ok!(SUT::mint(Origin::Root.into(), 1, Vec::<u8>::default()));

        assert_err!(
            SUT::burn(
                Origin::Signed(2).into(),
                Vec::<u8>::default().blake2_256().into()
            ),
            Error::<Test, DefaultInstance>::NotCommodityOwner
        );
    });
}

#[test]
fn burn_err_not_exist() {
    new_test_ext().execute_with(|| {
        assert_err!(
            SUT::burn(
                Origin::Signed(1).into(),
                Vec::<u8>::default().blake2_256().into()
            ),
            Error::<Test, DefaultInstance>::NotCommodityOwner
        );
    });
}

#[test]
fn transfer() {
    new_test_ext().execute_with(|| {
        assert_ok!(SUT::mint(Origin::Root.into(), 1, "test".into()));

        let assets = SUT::assets_for_account(&1);

        assert_ok!(SUT::transfer(Origin::Signed(1).into(), 2, assets[0].0));

        assert_eq!(SUT::total(), 1);
        assert_eq!(SUT::burned(), 0);
        assert_eq!(SUT::total_for_account(1), 0);
        assert_eq!(SUT::total_for_account(2), 1);
        assert_eq!(SUT::commodities_for_account::<u64>(1), vec![]);
        let commodities_for_account = SUT::commodities_for_account::<u64>(2);
        assert_eq!(commodities_for_account.len(), 1);
        assert_eq!(commodities_for_account[0].0, assets[0].0);
        assert_eq!(commodities_for_account[0].1, Vec::<u8>::from("test"));
        assert_eq!(
            SUT::account_for_commodity::<H256>(commodities_for_account[0].0),
            Some(2)
        );
    });
}

#[test]
fn transfer_err_not_owner() {
    new_test_ext().execute_with(|| {
        assert_ok!(SUT::mint(Origin::Root.into(), 1, Vec::<u8>::default()));

        assert_err!(
            SUT::transfer(
                Origin::Signed(0).into(),
                2,
                Vec::<u8>::default().blake2_256().into()
            ),
            Error::<Test, DefaultInstance>::NotCommodityOwner
        );
    });
}

#[test]
fn transfer_err_not_exist() {
    new_test_ext().execute_with(|| {
        assert_err!(
            SUT::transfer(
                Origin::Signed(1).into(),
                2,
                Vec::<u8>::default().blake2_256().into()
            ),
            Error::<Test, DefaultInstance>::NotCommodityOwner
        );
    });
}

#[test]
fn transfer_err_max_user() {
    new_test_ext().execute_with(|| {
        assert_ok!(SUT::mint(Origin::Root.into(), 1, vec![0]));
        assert_ok!(SUT::mint(Origin::Root.into(), 1, vec![1]));
        assert_ok!(SUT::mint(Origin::Root.into(), 2, Vec::<u8>::default()));
        assert_eq!(
            SUT::account_for_commodity::<H256>(Vec::<u8>::default().blake2_256().into()),
            Some(2)
        );

        assert_err!(
            SUT::transfer(
                Origin::Signed(2).into(),
                1,
                Vec::<u8>::default().blake2_256().into()
            ),
            Error::<Test, DefaultInstance>::TooManyCommoditiesForAccount
        );
    });
}
