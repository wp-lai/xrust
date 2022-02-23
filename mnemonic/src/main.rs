use ethers::{
    prelude::rand::thread_rng,
    signers::coins_bip39::{English, Mnemonic},
};

fn main() {
    let mut rng = thread_rng();
    let mnemonic: Mnemonic<English> = Mnemonic::new(&mut rng);
    println!("{}", mnemonic.to_phrase().unwrap());
}
