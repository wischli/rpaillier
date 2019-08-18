extern crate num_integer;

mod prime_pair;
mod utils;

use rand::Rng;
use std::fmt;
use utils::{calc_l, mod_exp, mod_inverse};

pub struct KeyPair {
    pub public_key: PublicKey,
    private_key: PrivateKey,
}

impl KeyPair {
    pub fn generate() -> KeyPair {
        let prime_pair = prime_pair::PrimePair::generate();
        gen_key_pair_ne(prime_pair.p, prime_pair.q)
    }
    fn new(n: u64, g: u64, lambda: u64, my: u64) -> KeyPair {
        KeyPair {
            public_key: PublicKey::new(n, g),
            private_key: PrivateKey::new(lambda, my),
        }
    }
    pub fn decrypt(&self, c: u64) -> u64 {
        let n = self.public_key.n;
        let n_2 = self.public_key.n_2;
        let lambda = self.private_key.lambda;
        let my = self.private_key.my;
        let l_value = calc_l(mod_exp(c, lambda, n_2), n);

        (l_value * my) % n
    }
}

impl fmt::Display for KeyPair {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "PublicKey (n: {}, g: {}) | PrivateKey (lambda: {}, my: {})",
            &self.public_key.n, &self.public_key.g, &self.private_key.lambda, &self.private_key.my
        )
    }
}

// first method for equally long primes p and q
// fn gen_key_pair_eq(p: u64, q: u64) -> KeyPair {
//     let n = p * q;
//     let g = n + 1;
//     let lambda = (p - 1) * (q - 1);
//     let my = mod_inverse(lambda, n);
//     KeyPair::new(n, g, lambda, my)
// }

// second method for any primes and q
fn gen_key_pair_ne(p: u64, q: u64) -> KeyPair {
    let n = p * q;
    let n_2 = n * n;
    let lambda = num_integer::lcm(p - 1, q - 1);
    let g = n + 1;
    let mod_expo = mod_exp(g, lambda, n_2);
    let l_value = calc_l(mod_expo, n);
    let my = mod_inverse(l_value, n);
    KeyPair::new(n, g, lambda, my)
}

pub struct PublicKey {
    pub n: u64,
    pub g: u64,
    pub n_2: u64,
}

impl PublicKey {
    pub fn encrypt(&self, m: u64) -> u64 {
        let mut rng = rand::thread_rng();
        let r = rng.gen_range(&self.n, &self.n_2);
        let mut result = mod_exp(self.g, m, self.n_2);
        let x = mod_exp(r, self.n, self.n_2);
        result = result * x;
        result = result % &self.n_2;
        result
    }
    pub fn new(n: u64, g: u64) -> PublicKey {
        PublicKey { n, g, n_2: n * n }
    }
}

struct PrivateKey {
    lambda: u64,
    my: u64,
}

impl PrivateKey {
    pub fn new(lambda: u64, my: u64) -> PrivateKey {
        PrivateKey { lambda, my }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encryption_decryption() {
        let msg = 1337;
        let pair = KeyPair::generate();
        let encryption = pair.public_key.encrypt(msg);
        let decryption = pair.decrypt(encryption);
        assert_eq!(msg, decryption);
    }
}
