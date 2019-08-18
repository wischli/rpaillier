pub fn calc_l(x: u64, n: u64) -> u64 {
    (x - 1) / n
}

pub fn mod_inverse(lhs: u64, m: u64) -> u64 {
    let mut x = std::cmp::max(m / lhs, 1);
    loop {
        x = x + 1;
        if (lhs * x) % m == 1 {
            break;
        }
    }
    x
}

pub fn mod_exp(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    if modulus == 1 { return 0 }
    let mut result = 1;
    base = base % modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            result = result * base % modulus;
        }
        exp = exp >> 1;
        base = base * base % modulus
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_l() {
        assert_eq!(calc_l(1, 1), 0);
        assert_eq!(calc_l(10, 1), 9);
    }

    #[test]
    fn test_mod_inverse() {
        assert_eq!(mod_inverse(10, 17), 12);
        assert_eq!(mod_inverse(3, 11), 4);
    }

    #[test]
    fn test_mod_exp() {
        assert_eq!(mod_exp(5, 117, 19), 1);
        assert_eq!(mod_exp(23, 20, 29), 24);
        assert_eq!(mod_exp(23, 391, 55), 12);
        assert_eq!(mod_exp(31, 397, 55), 26);
    }
}
