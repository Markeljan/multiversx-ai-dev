#![no_std]

multiversx_sc::imports!();

#[multiversx_sc::contract]
pub trait CounterContract {
    #[view(getCounter)]
    #[storage_mapper("counter")]
    fn counter(&self) -> SingleValueMapper<BigUint>;

    #[init]
    fn init(&self) {
        self.counter().set(&BigUint::zero());
    }

    #[endpoint]
    fn increment(&self) {
        self.counter().update(|counter| *counter += 1u32);
    }

    #[endpoint]
    fn decrement(&self) {
        self.counter().update(|counter| {
            if *counter > BigUint::zero() {
                *counter -= 1u32;
            }
        });
    }

    #[endpoint]
    fn reset(&self) {
        self.counter().set(&BigUint::zero());
    }
}