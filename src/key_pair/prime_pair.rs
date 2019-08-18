extern crate num_integer;
extern crate rand;

use primes::PrimeSet;
use rand::Rng;
use std::fmt;

fn gen_number(comp: u8) -> u8 {
    let mut rng = rand::thread_rng();
    let r = rng.gen();
    if r == comp {
        rng.gen()
    }
    r
}

fn gen_rand_pair() -> (u8, u8) {
    let first = gen_number(1);
    let second = gen_number(first);
    (first, second)
}

fn gen_prime(n: u64) -> u64 {
    let mut pset = PrimeSet::new();
    let (_, p) = pset.find(n);
    p
}

fn gen_prime_pair() -> PrimePair {
    let (r1, r2) = gen_rand_pair();
    let p = gen_prime(r1 as u64);
    let q = gen_prime(r2 as u64);
    let pair = PrimePair::new(p, q);
    pair
}

pub struct PrimePair {
    pub p: u64,
    pub q: u64,
}

impl PrimePair {
    fn new(p: u64, q: u64) -> PrimePair {
        PrimePair { p, q }
    }
    pub fn generate() -> PrimePair {
        gen_prime_pair()
    }
}

impl fmt::Display for PrimePair {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "First prime p {}, second prime q {}", &self.p, &self.q)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_pair() {
        let (r1, r2) = gen_rand_pair();
        assert_ne!(r1, r2);
    }

    #[test]
    fn test_single_prime() {
        assert_eq!(2, gen_prime(2 as u64));
        assert!(gen_prime(8 as u64) > 8);
    }

    #[test]
    fn test_prime_pair() {
        let pair = gen_prime_pair();
        assert_ne!(pair.p, pair.q);
        assert_eq!(
            num_integer::gcd(pair.p * pair.q, (pair.p - 1) * (pair.q - 1)),
            1
        );
    }
}
