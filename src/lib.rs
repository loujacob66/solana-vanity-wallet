use solana_sdk::signature::{Keypair, Signer};

pub fn is_valid_base58_prefix(prefix: &str) -> bool {
    // Base58 alphabet: 123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz
    // Notable exclusions: 0, O, I, l (to avoid confusion)
    const BASE58_ALPHABET: &str = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

    if prefix.is_empty() {
        return false;
    }

    prefix.chars().all(|c| BASE58_ALPHABET.contains(c))
}

pub fn calculate_expected_iterations(prefix: &str) -> u64 {
    // Base58 alphabet has 58 characters
    // Expected iterations = 58^(prefix_length) / 2 (on average)
    let base: u64 = 58;
    let length = prefix.len() as u32;
    base.pow(length) / 2
}

pub fn generate_solana_keypair() -> (String, Vec<u8>) {
    let keypair = Keypair::new();
    let pubkey = bs58::encode(keypair.pubkey().to_bytes()).into_string();
    let keypair_bytes = keypair.to_bytes().to_vec();
    (pubkey, keypair_bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_base58_prefixes() {
        assert!(is_valid_base58_prefix("A"));
        assert!(is_valid_base58_prefix("1"));
        assert!(is_valid_base58_prefix("ABC"));
        assert!(is_valid_base58_prefix("Sov")); // 'l' is not valid in base58
        assert!(is_valid_base58_prefix("123"));
        assert!(is_valid_base58_prefix("JAZZ"));
        assert!(is_valid_base58_prefix("MyWavvet"));
    }

    #[test]
    fn test_invalid_base58_prefixes() {
        assert!(!is_valid_base58_prefix(""));
        assert!(!is_valid_base58_prefix("0"));
        assert!(!is_valid_base58_prefix("O"));
        assert!(!is_valid_base58_prefix("I"));
        assert!(!is_valid_base58_prefix("l"));
        assert!(!is_valid_base58_prefix("_"));
        assert!(!is_valid_base58_prefix("Test0"));
        assert!(!is_valid_base58_prefix("ABC_"));
        assert!(!is_valid_base58_prefix("Sol+"));
    }

    #[test]
    fn test_expected_iterations_calculation() {
        assert_eq!(calculate_expected_iterations("A"), 29); // 58/2
        assert_eq!(calculate_expected_iterations("AB"), 1682); // 58^2/2
        assert_eq!(calculate_expected_iterations("ABC"), 97556); // 58^3/2
    }

    #[test]
    fn test_solana_keypair_generation() {
        let (pubkey, keypair_bytes) = generate_solana_keypair();

        // Test that public key is valid Base58
        assert!(bs58::decode(&pubkey).into_vec().is_ok());

        // Test that keypair bytes are correct length (64 bytes)
        assert_eq!(keypair_bytes.len(), 64);

        // Test that public key is not empty
        assert!(!pubkey.is_empty());

        // Test that public key is reasonable length (typically 32-44 characters)
        assert!(pubkey.len() >= 32 && pubkey.len() <= 44);
    }

    #[test]
    fn test_keypair_consistency() {
        // Generate multiple keypairs and ensure they're different
        let (pubkey1, _) = generate_solana_keypair();
        let (pubkey2, _) = generate_solana_keypair();

        assert_ne!(pubkey1, pubkey2);
    }

    #[test]
    fn test_base58_alphabet_completeness() {
        // Test that all expected characters are considered valid
        let valid_chars = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

        for c in valid_chars.chars() {
            assert!(
                is_valid_base58_prefix(&c.to_string()),
                "Character '{}' should be valid",
                c
            );
        }
    }

    #[test]
    fn test_edge_cases() {
        // Test edge cases for prefix validation
        assert!(!is_valid_base58_prefix(" "));
        assert!(!is_valid_base58_prefix("\n"));
        assert!(!is_valid_base58_prefix("\t"));
        assert!(!is_valid_base58_prefix("A B"));
        assert!(!is_valid_base58_prefix("A\n"));
    }
}
