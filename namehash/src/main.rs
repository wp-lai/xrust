use clap::{arg, Command};
use sha3::{Digest, Keccak256};

fn main() {
    let matches = Command::new("namehash")
        .about("Returns the ENS Namehash of the name")
        .arg(arg!(<name>))
        .get_matches();

    let name = matches.value_of("name").unwrap();

    let hash = hex::encode(namehash(name));
    println!("0x{}", hash);
}

fn namehash(name: &str) -> Vec<u8> {
    let mut node = vec![0u8; 32];

    if name.is_empty() {
        return node;
    }

    let labels = name.rsplit(".");

    for label in labels {
        let labelhash = Keccak256::digest(label);
        node.append(&mut labelhash.to_vec());
        node = Keccak256::digest(node).to_vec();
    }

    return node;
}

#[cfg(test)]
mod test {
    use crate::namehash;

    #[test]
    fn test_namehash() {
        let test_cases = vec![
            (
                "",
                "0x0000000000000000000000000000000000000000000000000000000000000000",
            ),
            (
                "eth",
                "0x93cdeb708b7545dc668eb9280176169d1c33cfd8ed6f04690a0bcc88a93fc4ae",
            ),
            (
                "ricmoo.firefly.eth",
                "0x0bcad17ecf260d6506c6b97768bdc2acfb6694445d27ffd3f9c1cfbee4a9bd6d",
            ),
            (
                "ricmoo.xyz",
                "0x7d56aa46358ba2f8b77d8e05bcabdd2358370dcf34e87810f8cea77ecb3fc57d",
            ),
        ];

        for (name, expected_hash) in test_cases {
            let hash = format!("0x{}", hex::encode(namehash(name)));
            assert_eq!(expected_hash, hash);
        }
    }
}
