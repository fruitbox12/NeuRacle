use scrypto::prelude::*;
use crate::validator::Validator;

pub fn assert_resource(resource1: ResourceAddress, resource2: ResourceAddress, amount: Decimal, require: Decimal) {

    assert!(
        resource1 == resource2,
        "Wrong resource address."
    );

    assert!(
        amount >= require,
        "Not enough resource."
    )

}

pub fn assert_fee(fee: Decimal) {
    assert!(
        (fee >= Decimal::zero()) && (fee <= dec!("100")),
        "Fee must be in the range of 0 to 100"
    );
}

pub fn vote_for_api(api: String, validators: HashMap<ComponentAddress, Decimal>) -> Option<bool> {

    let mut total_weight: Decimal = Decimal::zero();

            let mut result: Decimal = Decimal::zero();

            for (address, weight) in validators {

                let validator: Validator = address.into();
                if validator.get_status() {
                    if validator.get_vote(api.clone()) {result += weight}
                    total_weight += weight
                }
            }

            if result * dec!("3") >= total_weight * dec!("2") {
                return Some(true)
            }
            else if result * dec!("3") >= total_weight {
                return None
            }
            else {
               return Some(false)
            }
}
