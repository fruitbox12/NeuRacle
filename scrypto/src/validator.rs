use scrypto::prelude::*;
use crate::utilities::*;

#[derive(NonFungibleData)]
pub struct Staker {
    #[scrypto(mutable)]
    pub unstaking: Decimal,
    #[scrypto(mutable)]
    pub end: u64,
    #[scrypto(mutable)]
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
        datas: HashMap<String, String>,
        vote: bool,
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
                .updateable_non_fungible_data(rule!(require(controller_badge.resource_address())), LOCKED)
                .no_initial_supply();

            let rules = AccessRules::new()
                .method("change_fee", rule!(require(badge)))
                .method("withdraw_fee", rule!(require(badge)))
                .method("update_data", rule!(require(badge)))
                .method("vote", rule!(require(badge)))
                .method("reset_status", rule!(require(neura_controller_badge)))
                .method("get_datas", rule!(require(neura_controller_badge)))
                .method("mint", rule!(require(neura_controller_badge)))
                .method("burn", rule!(require(neura_controller_badge)))
                .default(rule!(allow_all));

            let component = Self {
                staker: HashMap::new(),
                name: name.clone(),
                fee: fee / dec!("100"),
                unstake_vault: Vault::new(neura),
                staked_vault: Vault::new(neura),
                fee_vault: Vault::new(neura),
                controller_badge: Vault::with_bucket(controller_badge),
                staker_badge: staker_badge,
                neura: neura,
                unstake_delay: unstake_delay,
                datas: HashMap::new(),
                vote: false,
                active: false
                }
                .instantiate()
                .add_access_check(rules)
                .globalize();
    
            info!("{} Validator Address: {}", name.clone(), component);
            info!("{} Staker Badge: {}", name, staker_badge);
            return component
        }

        pub fn stake(&mut self, bucket: Bucket) -> Bucket {

            assert_resource(bucket.resource_address(), self.neura, bucket.amount(), Decimal::zero());

            let user_id: NonFungibleId = NonFungibleId::random();

            let badge = self.controller_badge.authorize(|| {
                borrow_resource_manager!(self.staker_badge)
                .mint_non_fungible(&user_id, Staker{
                    unstaking: Decimal::zero(),
                    end: 0,
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

            info!("You have added stake {} NAR to {} validator", bucket.amount(), self.name);

            self.staked_vault.put(bucket);
            
            return identity

        }

        pub fn show_my_stake_amount(&mut self, identity: Bucket) -> Bucket {
            
            let id = identity.non_fungible::<Staker>().id();

            info!("You have staked {} NAR to {} validator", self.staker[&id],self.name);
            
            return identity

        }

        pub fn get_current_staked_value(&self) -> Decimal {
            self.staked_vault.amount()
        }

        pub fn unstake(&mut self, amount: Decimal, mut identity: Bucket) -> Bucket {

            assert_resource(identity.resource_address(), self.staker_badge, identity.amount(), dec!("1"));
            
            let mut data: Staker = identity.non_fungible().data();

            let id = identity.non_fungible::<Staker>().id();

            let current = Runtime::current_epoch();

            if current >= data.end {
 
                data.unstaked += data.unstaking;
                data.unstaking = Decimal::zero();
                data.end = current;
                self.controller_badge
                .authorize(|| identity.non_fungible().update_data(data));

            }

            let mut data: Staker = identity.non_fungible().data();

            assert!(
                data.unstaking == Decimal::zero(),
                "You must wait or stop unstaking before start unstake again"
            );

            assert!(
                amount <= self.staker[&id], 
                "Not enough amount for unstake."
            );

            if let Some(staked_amount) = self.staker.get_mut(&id) {
                *staked_amount -= amount;
            } 

            let end = current + self.unstake_delay;
            

            data.unstaking = amount;
            data.end = end;

            self.controller_badge
                .authorize(|| identity.non_fungible().update_data(data));

            info!("Unstaking {} NAR, estimated done in epoch {}", amount, end);

            self.unstake_vault.put(self.staked_vault.take(amount));

            return identity
            
        }

        pub fn show_unstake_record(&self, identity: Bucket) -> Bucket {
            let data: Staker = identity.non_fungible().data();
            info!("You are currently unstaking {}, estimated done in epoch {}", data.unstaking, data.end);
            return identity
        }


        pub fn stop_unstake(&mut self, mut identity: Bucket) -> Bucket {

            assert_resource(identity.resource_address(), self.staker_badge, identity.amount(), dec!("1"));

            let mut data: Staker = identity.non_fungible().data();

            let id = identity.non_fungible::<Staker>().id();

            let current = Runtime::current_epoch();

            if current >= data.end {
 
                data.unstaked += data.unstaking;
                data.unstaking = Decimal::zero();
                data.end = current;
                self.controller_badge
                .authorize(|| identity.non_fungible().update_data(data));

            }

            let mut data: Staker = identity.non_fungible().data();

            assert!(
                data.unstaking != Decimal::zero(),
                "You currently don't have token unstaking"
            );

            if let Some(staked_amount) = self.staker.get_mut(&id) {
                *staked_amount += data.unstaking;
            }

            self.staked_vault.put(self.unstake_vault.take(data.unstaking));

            data.unstaking = Decimal::zero();
            data.end = current;

            self.controller_badge
                .authorize(|| identity.non_fungible().update_data(data));

            info!("You have stop unstake all your current unstaking amount.");

            return identity
        }

        pub fn withdraw(&mut self, amount: Decimal, mut identity: Bucket) -> (Bucket, Bucket) {

            assert_resource(identity.resource_address(), self.staker_badge, identity.amount(), dec!("1"));

            let mut data: Staker = identity.non_fungible().data();

            let current = Runtime::current_epoch();

            if current >= data.end {
 
                data.unstaked += data.unstaking;
                data.unstaking = Decimal::zero();
                data.end = current;
                self.controller_badge
                .authorize(|| identity.non_fungible().update_data(data));

            }
            
            let data: Staker = identity.non_fungible().data();

            assert!(
                amount <= data.unstaked,
                "Not enough unstaked amount for withdrawal"
            );

            info!("You have withdrawed {} NAR token.", amount);

            return (self.unstake_vault.take(amount), identity)
            
        }

        pub fn update_data(&mut self, datas: HashMap<String, String>) {
            self.datas = datas
        }

        pub fn vote(&mut self, vote: bool) {

            self.active = true;
            
            self.vote = vote

        }

        pub fn reset_status(&mut self) {
            self.active = false
        }

        pub fn get_status(&self) -> bool {
            self.active
        }

        pub fn get_data(&self) -> HashMap<String, String> {
            self.datas.clone()
        }

        pub fn get_vote(&self) -> bool {
            self.vote
        }

        pub fn mint(&mut self, rate: Decimal) {

            let amount = self.staked_vault.amount();

            let fee = rate * self.fee;

            let staker_rate = rate * (dec!("1") - self.fee);

            let reward = amount * rate;

            let mut bucket = borrow_resource_manager!(self.neura)
                .mint(reward);

            self.fee_vault.put(bucket.take(fee));

            for val in self.staker.values_mut() {
                *val *= dec!("1") + staker_rate
            };

            self.staked_vault.put(bucket);

            info!("Your node is rewarded with {}%, keep up the good work!", rate*dec!("100"))

        }

        pub fn burn(&mut self, rate: Decimal) {

            let amount = self.staked_vault.amount();

            let punish = amount * rate;

            let bucket = self.staked_vault.take(punish);

            borrow_resource_manager!(self.neura).burn(bucket);
    
            for val in self.staker.values_mut() {
                *val *= dec!("1") - rate
            };

            info!("Your node is punished for {}%, don't slacking around!", rate*dec!("100"))

        }

        pub fn change_fee(&mut self, fee: Decimal) {
            assert_fee(fee);
            self.fee = fee
        }

        pub fn withdraw_fee(&mut self) -> Bucket {
            self.fee_vault.take_all()
        }
    }
}