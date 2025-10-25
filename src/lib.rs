use num_bigint::{BigUint, RandBigInt};
use num_integer::Integer;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Primality {
    Prime(BigUint),
    Composite(BigUint),
}

fn generate_viable_witness(x: &BigUint) -> BigUint {
    let mut rng = rand::thread_rng();
    let one = BigUint::from(1_u8);
    loop {
        let a = &rng.gen_biguint_range(&2_u8.into(), x);
        if x.gcd(a) == one {
            return a.clone();
        }
    }
}

pub fn fermat_primality_test<T>(x: T, iterations: u8) -> Primality
where
    T: Into<BigUint>,
{
    let x = x.into();
    if x <= 1_u8.into() {
        return Primality::Composite(x);
    }
    if x == 2_u8.into() || x == 3_u8.into() {
        return Primality::Prime(x);
    }
    for _ in 0..iterations {
        let a = generate_viable_witness(&x);
        if a.modpow(&(x.clone() - BigUint::from(1_u8)), &x) != 1_u8.into() {
            return Primality::Composite(x);
        }
    }
    Primality::Prime(x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fermat_primality_test_test() {
        let prime_numbers: [u8; 5] = [2, 3, 5, 7, 13];
        prime_numbers.into_iter().for_each(|p| {
            assert_eq!(fermat_primality_test(p, 10), Primality::Prime(p.into()));
        });
        let not_prime_numbers: [u8; 5] = [4, 6, 8, 9, 10];
        not_prime_numbers.into_iter().for_each(|x| {
            assert_eq!(fermat_primality_test(x, 10), Primality::Composite(x.into()));
        });
    }
}
