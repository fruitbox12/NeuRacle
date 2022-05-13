use scrypto::prelude::*;
use crate::validator::Validator;
use crate::utilities::*;

blueprint! {
    
    struct DataApi {
        datas: HashMap<String, Decimal>,
        non_validated_datas: HashMap<String, Decimal>,
        leader: String,
        api: String,
        reward_rate: Decimal,
        punishment: Decimal
    }

    impl DataApi {

        pub fn new(api: String, controller_badge: ResourceAddress, initial_query: String, reward_rate: Decimal, punishment: Decimal) -> ComponentAddress {

            let rules = AccessRules::new()
                .default(rule!(require(controller_badge)));

            let mut map = HashMap::new();

            map.insert(initial_query, Decimal::zero());

            let component = Self {
                datas: map,
                non_validated_datas: HashMap::new(),
                leader: String::default(),
                api: api,
                reward_rate: reward_rate,
                punishment: punishment
            }
                .instantiate()
                .add_access_check(rules)
                .globalize();

            return component
        }

        pub fn new_query(&mut self, query: String) {
            self.datas.insert(query, Decimal::zero());
        }

        pub fn get_data(&self, query: String) -> Decimal {
            return self.datas[&query]
        }

        pub fn new_round(&mut self, validators: HashMap<ComponentAddress, Decimal>) -> HashMap<ComponentAddress, Decimal> {

            let random_leader_validator: (ComponentAddress, Decimal) = validators.clone().drain().next().unwrap();
            
            let validator: Validator = random_leader_validator.0.into();

            self.non_validated_datas = validator.get_data(self.api.clone());

            self.leader = random_leader_validator.0.to_string();

            info!("Begin voting round on {} api datas, format (query, data): {:?}", self.api, self.non_validated_datas);

            return validators

        }

        pub fn end_round(&mut self, validators: HashMap<ComponentAddress, Decimal>) {

            let result = vote_for_api(self.api.clone(), validators.clone());

            match result {
                Some(true) => {
                    self.datas = self.non_validated_datas.clone();
                    for (&address, _weight) in &validators {
    
                        let validator: Validator = address.into();
                        if validator.get_status() {
                            if validator.get_vote(self.api.clone()) {validator.mint(self.reward_rate)}
                            else {validator.burn(self.reward_rate * self.punishment)}
                        }
                    }
                }
                None => {}
                Some(false) => {
                    let address = ComponentAddress::from_str(&self.leader).unwrap();
                    let validator: Validator = address.into();
                    validator.burn(self.reward_rate * self.punishment * 5); //malicious validator will be punished 5 times untruthful validator
                    for (address, _weight) in validators {

                    let validator: Validator = address.into();
                        if validator.get_status() {
                            if validator.get_vote(self.api.clone()) {validator.burn(self.reward_rate * self.punishment)}
                            else {validator.mint(self.reward_rate)}
                        }
                    }
                }
            }
        }

    }
}


    

