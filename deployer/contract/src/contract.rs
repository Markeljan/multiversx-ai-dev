#![no_std]

multiversx_sc::imports!();

#[multiversx_sc::contract]
pub trait Contract {
    #[view(getSum)]
    #[storage_mapper("sum")]
    fn sum(&self) -> SingleValueMapper<BigUint>;

    #[init]
    fn init(&self) {
        self.sum().set(&BigUint::zero());
    }

    #[endpoint]
    fn add(&self, value: BigUint) {
        self.sum().update(|sum| *sum += value);
    }

    #[endpoint]
    fn subtract(&self, value: BigUint) {
        self.sum().update(|sum| {
            if *sum > value {
                *sum -= value;
            } else {
                *sum = BigUint::zero();
            }
        });
    }

    #[endpoint]
    fn multiply_by_2(&self) {
        self.sum().update(|sum| *sum *= 2u32);
    }

    #[endpoint]
    fn add_random(&self) {
        let mut rand_source = RandomnessSource::new();
        let random_value = BigUint::from(rand_source.next_u16() as u32);
        self.sum().update(|sum| *sum += random_value);
    }
}