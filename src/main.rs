mod key_pair;

fn main() {
    let msg = 1337;
    let pair = key_pair::KeyPair::generate();
    let encryption = pair.public_key.encrypt(msg);
    let decryption = pair.decrypt(encryption);
    println!("Encrypting {} to {}", msg, encryption);
    println!("Decrypting back to to {}", decryption);
}
