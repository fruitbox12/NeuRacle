use scrypto::prelude::*;
use crate::utilities::{assert_fee, assert_resource};

#[derive(NonFungibleData)]
pub struct Staker {
    pub unstake_record: (u64, Decimal),
    pub unstaked: Decimal
}

blueprint! {

    struct Validator {

        staker: HashMap<NonFungibleId, Decimal>,
        name: String,
        fee: Decimal,
        unstake_vault: Vault,
        staked_vault: Vault,
        fee_vault: Vault,
        controller_badge: Vault,
        staker_badge: ResourceAddress,
        neura: ResourceAddress,
        unstake_delay: u64,
        datas: LazyMap<String, HashMap<String, Decimal>>,
        vote: LazyMap<String, bool>,
        active: bool

    }

    impl Validator {

        pub fn new(neura: ResourceAddress, badge: ResourceAddress, neura_controller_badge: ResourceAddress, name: String, fee: Decimal, unstake_delay: u64) -> ComponentAddress {

            assert_fee(fee);

            let controller_badge = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", "Validator Controller Badge")
                .initial_supply(dec!("1"));

            let staker_badge = ResourceBuilder::new_non_fungible()
                .metadata("name", "staker Badge")
                .mintable(rule!(require(controller_badge.resource_address())), LOCKED)
                .burnable(rule!(require(controller_badge.resource_address())), LOCKED)
                .no_initial_supply();

            let rules = AccessRules::new()
                .method("change_fee", rule!(require(badge)))
                .method("withdraw_fee", rule!(require(badge)))
                .method("validate", rule!(require(badge)))
                .method("reset_status", rule!(require(neura_controller_badge)))
                .method("get_status", rule!(require(neura_controller_badge)))
                .method("get_datas", rule!(require(neura_controller_badge)))
                .method("get_vote", rule!(require(neura_controller_badge)))
                .method("mint", rule!(require(neura_controller_badge)))
                .method("burn", rule!(require(neura_controller_badge)))
                .default(rule!(allow_all));

            let component = Self {
                staker: HashMap::new(),
                name: name,
                fee: fee,
                unstake_vault: Vault::new(neura),
                staked_vault: Vault::new(neura),
                fee_vault: Vault::new(neura),
                controller_badge: Vault::with_bucket(controller_badge),
                staker_badge: staker_badge,
                neura: neura,
                unstake_delay: unstake_delay,
                datas: LazyMap::new(),
                vote: LazyMap::new(),
                active: false
                }
                .instantiate()
                .add_access_check(rules)
                .globalize();
    
            info!(
                "Validator Address: {}", component
            );
            return component
        }

        pub fn stake(&mut self, bucket: Bucket) -> Bucket {

            assert_resource(bucket.resource_address(), self.neura, bucket.amount(), Decimal::zero());

            let user_id: NonFungibleId = NonFungibleId::random();

            let badge = self.controller_badge.authorize(|| {
                borrow_resource_manager!(self.staker_badge)
                .mint_non_fungible(&user_id, Staker{
                    unstake_record: (0, Decimal::zero()),
                    unstaked: Decimal::zero()
                })
            });

            info!("You have staked {} NAR to {} validator", bucket.amount(), self.name);

            self.staker.insert(user_id, bucket.amount());

            self.staked_vault.put(bucket);

            return badge

        }
    
        pub fn add_stake(&mut self, bucket: Bucket, identity: Bucket) -> Bucket {

            assert_resource(identity.resource_address(), self.staker_badge, identity.amount(), dec!("1"));
            assert_resource(bucket.resource_address(), self.neura, bucket.amount(), dec!("0"));

            let id = identity.non_fungible::<Staker>().id();

            if let Some(staked_amount) = self.staker.get_mut(&id) {
                *staked_amount += bucket.amount();
            }

            info!("You have staked {} NAR to {} validator", self.staker[&id], self.name);

            self.staked_vault.put(bucket);
            
            return identity

        }

        pub fn get_my_stake_amount(&mut self, identity: Bucket) -> (Decimal, Bucket) {
            
            let id = identity.non_fungible::<Staker>().id();

            return (self.staker[&id], identity)

        }

        pub fn get_current_staked_value(&self) -> Decimal {
            self.staked_vault.amount()
        }

        pub fn unstake(&mut self, amount: Decimal, identity: Bucket) -> Bucket {

            assert_resource(identity.resource_address(), self.staker_badge, identity.amount(), dec!("1"));
            
            let mut data = identity.non_fungible::<Staker>().data();

            let id = identity.non_fungible::<Staker>().id();

            let current = Runtime::current_epoch();

            if current >= data.unstake_record.0 {
                if let Some(staked_amount) = self.staker.get_mut(&id) {
                    *staked_amount -= data.unstake_record.1;
                } 
                data.unstaked += data.unstake_record.1;
                data.unstake_record = (current, Decimal::zero())
            }

            assert!(
                data.unstake_record.1 == Decimal::zero(),
                "You must wait or stop unstaking before start unstake again"
            );

            assert!(
                amount <= self.staker[&id], 
                "Not enough amount for unstake."
            );

            let end = current + self.unstake_delay;

            data.unstake_record = (end, amount);

            self.unstake_vault.put(self.staked_vault.take(amount));

            return identity
            
        }

        pub fn stop_unstake(&mut self, identity: Bucket) -> Bucket {

            assert_resource(identity.resource_address(), self.staker_badge, identity.amount(), dec!("1"));

            let mut data = identity.non_fungible::<Staker>().data();

            let id = identity.non_fungible::<Staker>().id();

            let current = Runtime::current_epoch();

            if current >= data.unstake_record.0 {
                if let Some(staked_amount) = self.staker.get_mut(&id) {
                    *staked_amount -= data.unstake_record.1;
                } 
                data.unstaked += data.unstake_record.1;
                data.unstake_record = (current, Decimal::zero())
            }

            assert!(
                data.unstake_record.1 != Decimal::zero(),
                "You currently don't have token unstaking"
            );

            self.staked_vault.put(self.unstake_vault.take(data.unstake_record.1));

            data.unstake_record = (current, Decimal::zero());

            return identity
        }

        pub fn withdraw(&mut self, amount: Decimal, identity: Bucket) -> Bucket {

            assert_resource(identity.resource_address(), self.staker_badge, identity.amount(), dec!("1"));

            let mut data = identity.non_fungible::<Staker>().data();

            let id = identity.non_fungible::<Staker>().id();

            let current = Runtime::current_epoch();

            if current >= data.unstake_record.0 {
                if let Some(staked_amount) = self.staker.get_mut(&id) {
                    *staked_amount -= data.unstake_record.1;
                } 
                data.unstaked += data.unstake_record.1;
                data.unstake_record = (current, Decimal::zero())
            }
            
            assert!(
                amount <= data.unstaked,
                "Not enough unstaked amount for withdrawal"
            );

            return self.unstake_vault.take(amount)
            
        }

        pub fn update_data(&mut self, datas: LazyMap<String, HashMap<String, Decimal>>) {
            self.datas = datas
        }

        pub fn vote(&mut self, api: String, vote: bool) {

            self.active = true;
            
            self.vote.insert(api, vote)

        }

        pub fn reset_status(&mut self) {
            self.active = false
        }

        pub fn get_status(&self) -> bool {
            self.active
        }

        pub fn get_data(&self, api: String) -> HashMap<String, Decimal> {
            self.datas.get(&api).unwrap()
        }

        pub fn get_vote(&self, api: String) -> bool {
            self.vote.get(&api).unwrap()
        }

        pub fn mint(&mut self, rate: Decimal) {

            let amount = self.staked_vault.amount();

            let fee = rate * self.fee;

            let staker_rate = dec!("1") - self.fee;

            let reward = amount * rate;

            let mut bucket = borrow_resource_manager!(self.neura)
                .mint(reward);

            self.fee_vault.put(bucket.take(fee));

            for val in self.staker.values_mut() {
                *val *= dec!("1") + staker_rate
            };

            self.staked_vault.put(bucket)

        }

        pub fn burn(&mut self, rate: Decimal) {

            let amount = self.staked_vault.amount();

            let punish = amount * rate;

            let bucket = self.staked_vault.take(punish);

            borrow_resource_manager!(self.neura).burn(bucket);
    
            for val in self.staker.values_mut() {
                *val *= dec!("1") - rate
            };

        }

        pub fn change_fee(&mut self, fee: Decimal) {
            assert!(
                (fee >= Decimal::zero()) && (fee <= Decimal::zero()),
                "Fee must be in the range of 0 to 100"
            );
            self.fee = fee
        }

        pub fn withdraw_fee(&mut self) -> Bucket {
            self.fee_vault.take_all()
        }
    }
}