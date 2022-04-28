// Creating mock runtime here
use crate as pallet_nft;

use crate::{Config, Module, Pallet};
use frame_support::traits::Contains;
use frame_support::{parameter_types, weights::Weight};
use frame_system as system;
use frame_system::{ConsumerLimits, SetCode};
use sp_core::H256;
use sp_runtime::{
    testing::Header,
    traits::{BlakeTwo256, IdentityLookup},
};
use sp_std::convert::{TryFrom, TryInto};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

// impl_outer_origin! {
//     pub enum Origin for Test where system = frame_system {}
// }

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
    pub enum Test where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
        Commodity: pallet_nft::{Pallet, Call, Storage, Event<T>},
    }
);

parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub const SS58Prefix: u8 = 42;

}

pub struct TestBaseCallFilter;
impl Contains<Call> for TestBaseCallFilter {
    fn contains(c: &Call) -> bool {
        match *c {
            _ => true,
        }
    }
}

impl system::Config for Test {
    type BaseCallFilter = TestBaseCallFilter;
    type Origin = Origin;
    type Call = Call;
    type Index = u64;
    type BlockNumber = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = u64;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type Event = ();
    type BlockHashCount = BlockHashCount;
    type BlockWeights = ();
    type BlockLength = ();
    type DbWeight = ();
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = ();
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = SS58Prefix;
    type OnSetCode = ();
    type MaxConsumers = frame_support::traits::ConstU32<16>;
}

parameter_types! {
    pub const MaxCommodities: u128 = 5;
    pub const MaxCommoditiesPerUser: u64 = 2;
}

// For testing the pallet, we construct most of a mock runtime. This means
// first constructing a configuration type (`Test`) which `impl`s each of the
// configuration traits of pallets we want to use.
// #[derive(Clone, Eq, PartialEq)]
// pub struct Test;

impl Config for Test {
    type Event = ();
    type CommodityAdmin = frame_system::EnsureRoot<Self::AccountId>;
    type CommodityInfo = Vec<u8>;
    type CommodityLimit = MaxCommodities;
    type UserCommodityLimit = MaxCommoditiesPerUser;
}

// system under test
pub type SUT = Module<Test>;

// This function basically just builds a genesis storage key/value store according to
// our desired mockup.
pub fn new_test_ext() -> sp_io::TestExternalities {
    system::GenesisConfig::default()
        .build_storage::<Test>()
        .unwrap()
        .into()
}
