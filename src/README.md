# rPaillier

This rust implementation of the Paillier cryptosystem was used as a learning project for rustlang. Therefore, I would really appreciate feedback concerning the code, if you stumble upon this randomly.

## How to use

1: Create a key pair:
```
    let key_pair = KeyPair::generate();
```

2. Encryption:
```
    let encryption = public_key.encrypt(m);
```

3. Decrypt the ciphertext:
```
    let m = key_pair.decrypt(encryption);
```

## ToDos
- [ ] Add user input
- [ ] Add support for strings
- [ ] Improve code

## References
 * [Public-Key Cryptosystems Based on Composite
Degree Residuosity Classes](http://www.cs.tau.ac.il/~fiat/crypt07/papers/Pai99pai.pdf)
 * [Wikipedia: Paillier cryptosystem](https://en.wikipedia.org/wiki/Paillier_cryptosystem)
 * [Wikipedia: Modular exponentiation](https://en.wikipedia.org/wiki/Modular_exponentiation)
 * [Wikipedia: Modular multiplicative inverse](https://en.wikipedia.org/wiki/Modular_multiplicative_inverse)