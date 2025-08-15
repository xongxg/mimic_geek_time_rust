use std::marker::PhantomData;
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_ID: AtomicU64 = AtomicU64::new(1);

pub struct Customer<T> {
    id: u64,
    name: String,
    _type: PhantomData<T>,
}

pub trait Free {
    fn feature1(&self);
    fn feature2(&self);
}

pub trait Personal: Free {
    fn advanced_feature(&self);
}

impl<T> Free for Customer<T> {
    fn feature1(&self) {
        println!("feature 1 for {}", self.name);
    }

    fn feature2(&self) {
        println!("feature 2 for {}", self.name);
    }
}

pub struct FreePlan;
pub struct PersonalPlan(f32);

impl Personal for Customer<PersonalPlan> {
    fn advanced_feature(&self) {
        println!(
            "Dear {}(as our valuable customer {}), enjoy this advanced feature!",
            self.name, self.id
        );
    }
}

impl<T> Customer<T> {
    pub fn new<A: AsRef<str>>(name: A) -> Self {
        Self {
            id: NEXT_ID.fetch_add(1, Ordering::Relaxed),
            name: name.as_ref().to_string(),
            _type: PhantomData::default(),
        }
    }
}

impl From<Customer<FreePlan>> for Customer<PersonalPlan> {
    fn from(customer: Customer<FreePlan>) -> Self {
        Self::new(customer.name)
    }
}

pub fn subscribe(customer: Customer<FreePlan>, payment: f32) -> Customer<PersonalPlan> {
    let _plan = PersonalPlan(payment);
    // 存储 plan 到 DB
    // ...
    customer.into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_customer() {
        let customer = Customer::<FreePlan>::new("Try");
        customer.feature1();
        customer.feature2();

        let customer = subscribe(customer, 64.3);
        customer.feature1();
        customer.feature2();

        customer.advanced_feature();
    }
}
