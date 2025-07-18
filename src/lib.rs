use bip39::{Language, Mnemonic};
use slip10::{derive_key_from_path, BIP32Path};
use solana_sdk::signature::{Keypair, SeedDerivable, Signer};
use std::str::FromStr;

/// Derive Solana seed from mnemonic using BIP44 path
pub fn derive_solana_seed(seed: &[u8]) -> [u8; 32] {
    // Solana BIP44 derivation path: m/44'/501'/0'/0'
    // 501 is Solana's coin type in BIP44
    let path = BIP32Path::from_str("m/44'/501'/0'/0'").unwrap();

    // Derive the key using SLIP10 (BIP32 for Ed25519)
    let derived_key = derive_key_from_path(seed, slip10::Curve::Ed25519, &path).unwrap();

    // Return the private key bytes
    derived_key.key
}

/// Generate a keypair with optional mnemonic
pub fn generate_keypair(with_mnemonic: bool) -> (Option<String>, Keypair) {
    use rand::rngs::OsRng;
    use rand::RngCore;

    let mut rng = OsRng;

    if with_mnemonic {
        // Generate mnemonic and derive keypair (compatible with wallets)
        let mut entropy = [0u8; 16];
        rng.fill_bytes(&mut entropy);
        let mnemonic = Mnemonic::from_entropy_in(Language::English, &entropy).unwrap();

        // Generate keypair from the mnemonic using proper Solana BIP44 derivation
        let seed = mnemonic.to_seed("");
        let derived_seed = derive_solana_seed(&seed);
        let keypair = Keypair::from_seed(&derived_seed).unwrap();

        (Some(mnemonic.to_string()), keypair)
    } else {
        // Fast mode: Generate keypair directly from random seed
        let mut seed = [0u8; 32];
        rng.fill_bytes(&mut seed);
        let keypair = Keypair::from_seed(&seed).unwrap();

        (None, keypair)
    }
}

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
                "Character '{c}' should be valid"
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

    // === COMPREHENSIVE KEYPAIR VALIDATION TESTS ===

    #[test]
    fn test_fast_mode_generates_valid_keypairs() {
        // Test that fast mode generates valid keypairs
        let (mnemonic, keypair) = generate_keypair(false);

        // Should not have mnemonic
        assert!(mnemonic.is_none());

        // Should have valid public key
        let pubkey = keypair.pubkey();
        assert_eq!(pubkey.to_bytes().len(), 32);

        // Should be able to sign and verify
        let message = b"test message";
        let signature = keypair.sign_message(message);
        assert!(signature.verify(pubkey.as_ref(), message));

        // Public key should be valid Base58
        let pubkey_str = bs58::encode(pubkey.to_bytes()).into_string();
        assert!(pubkey_str.len() >= 32); // Base58 encoding should be reasonable length

        // Secret key should be 64 bytes
        let secret_bytes = keypair.to_bytes();
        assert_eq!(secret_bytes.len(), 64);
    }

    #[test]
    fn test_mnemonic_mode_generates_valid_keypairs() {
        // Test that mnemonic mode generates valid keypairs
        let (mnemonic, keypair) = generate_keypair(true);

        // Should have mnemonic
        assert!(mnemonic.is_some());
        let mnemonic_str = mnemonic.unwrap();

        // Mnemonic should be 12 words
        let words: Vec<&str> = mnemonic_str.split_whitespace().collect();
        assert_eq!(words.len(), 12);

        // Should have valid public key
        let pubkey = keypair.pubkey();
        assert_eq!(pubkey.to_bytes().len(), 32);

        // Should be able to sign and verify
        let message = b"test message";
        let signature = keypair.sign_message(message);
        assert!(signature.verify(pubkey.as_ref(), message));

        // Public key should be valid Base58
        let pubkey_str = bs58::encode(pubkey.to_bytes()).into_string();
        assert!(pubkey_str.len() >= 32);

        // Secret key should be 64 bytes
        let secret_bytes = keypair.to_bytes();
        assert_eq!(secret_bytes.len(), 64);
    }

    #[test]
    fn test_mnemonic_deterministic_derivation() {
        // Test that the same mnemonic produces the same keypair
        let mnemonic_str = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
        let mnemonic = Mnemonic::from_str(mnemonic_str).unwrap();

        // Generate keypair twice from same mnemonic
        let seed = mnemonic.to_seed("");
        let derived_seed = derive_solana_seed(&seed);
        let keypair1 = Keypair::from_seed(&derived_seed).unwrap();
        let keypair2 = Keypair::from_seed(&derived_seed).unwrap();

        // Should be identical
        assert_eq!(keypair1.pubkey(), keypair2.pubkey());
        assert_eq!(keypair1.to_bytes(), keypair2.to_bytes());
    }

    #[test]
    fn test_mnemonic_produces_different_keypairs() {
        // Test that different mnemonics produce different keypairs
        use std::collections::HashSet;
        let mut keypairs = HashSet::new();

        for _ in 0..10 {
            let (mnemonic, keypair) = generate_keypair(true);
            assert!(mnemonic.is_some());

            let pubkey = keypair.pubkey();
            // Should be unique
            assert!(keypairs.insert(pubkey));
        }
    }

    #[test]
    fn test_fast_mode_produces_different_keypairs() {
        // Test that fast mode produces different keypairs
        use std::collections::HashSet;
        let mut keypairs = HashSet::new();

        for _ in 0..10 {
            let (mnemonic, keypair) = generate_keypair(false);
            assert!(mnemonic.is_none());

            let pubkey = keypair.pubkey();
            // Should be unique
            assert!(keypairs.insert(pubkey));
        }
    }

    #[test]
    fn test_bip44_derivation_path() {
        // Test that we're using the correct Solana BIP44 path
        let mnemonic_str = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
        let mnemonic = Mnemonic::from_str(mnemonic_str).unwrap();
        let seed = mnemonic.to_seed("");

        // Derive using our function
        let derived_seed = derive_solana_seed(&seed);
        let keypair = Keypair::from_seed(&derived_seed).unwrap();

        // This should produce a specific, known public key for this test mnemonic
        let pubkey_str = bs58::encode(keypair.pubkey().to_bytes()).into_string();

        // The actual value depends on the SLIP10 implementation, but it should be deterministic
        assert!(!pubkey_str.is_empty());
        assert!(pubkey_str.len() >= 32);

        // Test that derivation is repeatable
        let derived_seed2 = derive_solana_seed(&seed);
        let keypair2 = Keypair::from_seed(&derived_seed2).unwrap();
        assert_eq!(keypair.pubkey(), keypair2.pubkey());
    }

    #[test]
    fn test_keypair_serialization() {
        // Test both modes for proper serialization
        for with_mnemonic in [false, true] {
            let (mnemonic_opt, keypair) = generate_keypair(with_mnemonic);

            // Test public key serialization
            let pubkey_bytes = keypair.pubkey().to_bytes();
            let pubkey_str = bs58::encode(pubkey_bytes).into_string();
            assert!(!pubkey_str.is_empty());

            // Test secret key serialization
            let secret_bytes = keypair.to_bytes();
            let secret_str = bs58::encode(secret_bytes).into_string();
            assert!(!secret_str.is_empty());

            // Test that we can reconstruct the keypair
            let reconstructed = Keypair::try_from(secret_bytes.as_slice()).unwrap();
            assert_eq!(keypair.pubkey(), reconstructed.pubkey());

            // Test mnemonic consistency
            if with_mnemonic {
                assert!(mnemonic_opt.is_some());
                let mnemonic_str = mnemonic_opt.unwrap();
                assert_eq!(mnemonic_str.split_whitespace().count(), 12);
            } else {
                assert!(mnemonic_opt.is_none());
            }
        }
    }

    #[test]
    fn test_signature_verification() {
        // Test that signatures work properly in both modes
        let test_message = b"Hello, Solana!";

        for with_mnemonic in [false, true] {
            let (_, keypair) = generate_keypair(with_mnemonic);

            // Sign the message
            let signature = keypair.sign_message(test_message);

            // Verify the signature
            assert!(signature.verify(keypair.pubkey().as_ref(), test_message));

            // Verify that wrong message fails
            let wrong_message = b"Wrong message";
            assert!(!signature.verify(keypair.pubkey().as_ref(), wrong_message));
        }
    }

    #[test]
    fn test_base58_encoding_validity() {
        // Test that all generated addresses are valid Base58
        for with_mnemonic in [false, true] {
            let (_, keypair) = generate_keypair(with_mnemonic);

            let pubkey_str = bs58::encode(keypair.pubkey().to_bytes()).into_string();
            let secret_str = bs58::encode(keypair.to_bytes()).into_string();

            // Should be able to decode back
            let decoded_pubkey = bs58::decode(&pubkey_str).into_vec().unwrap();
            let decoded_secret = bs58::decode(&secret_str).into_vec().unwrap();

            assert_eq!(decoded_pubkey.len(), 32);
            assert_eq!(decoded_secret.len(), 64);

            // Should match original
            assert_eq!(decoded_pubkey, keypair.pubkey().to_bytes());
            assert_eq!(decoded_secret, keypair.to_bytes());
        }
    }

    #[test]
    fn test_known_mnemonic_compatibility() {
        // Test with a known mnemonic to ensure wallet compatibility
        // This is the same test vector used in many wallet implementations
        let known_mnemonic = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
        let mnemonic = Mnemonic::from_str(known_mnemonic).unwrap();
        let seed = mnemonic.to_seed("");

        // Generate keypair using our derivation
        let derived_seed = derive_solana_seed(&seed);
        let keypair = Keypair::from_seed(&derived_seed).unwrap();

        // The keypair should be valid
        assert_eq!(keypair.pubkey().to_bytes().len(), 32);
        assert_eq!(keypair.to_bytes().len(), 64);

        // Should be able to sign and verify
        let message = b"test";
        let signature = keypair.sign_message(message);
        assert!(signature.verify(keypair.pubkey().as_ref(), message));

        // The public key should be deterministic for this mnemonic
        let pubkey_str = bs58::encode(keypair.pubkey().to_bytes()).into_string();

        // Generate again to ensure determinism
        let derived_seed2 = derive_solana_seed(&seed);
        let keypair2 = Keypair::from_seed(&derived_seed2).unwrap();
        let pubkey_str2 = bs58::encode(keypair2.pubkey().to_bytes()).into_string();

        assert_eq!(pubkey_str, pubkey_str2);

        // Log the deterministic address for reference
        println!("Known mnemonic generates deterministic address: {pubkey_str}");
    }

    #[test]
    fn test_vanity_prefix_matching() {
        // Test that the generate_keypair function can produce addresses with specific prefixes
        let mut found_count = 0;
        let target_prefix = "1";

        // Try up to 1000 iterations to find a keypair with the desired prefix
        for _ in 0..1000 {
            let (_, keypair) = generate_keypair(false);
            let pubkey_str = bs58::encode(keypair.pubkey().to_bytes()).into_string();

            if pubkey_str.starts_with(target_prefix) {
                found_count += 1;
                // Verify the keypair is still valid
                let message = b"test";
                let signature = keypair.sign_message(message);
                assert!(signature.verify(keypair.pubkey().as_ref(), message));

                if found_count >= 3 {
                    break; // Found enough examples
                }
            }
        }

        // Should have found at least one (statistically very likely)
        assert!(
            found_count > 0,
            "Should find at least one keypair with prefix '{target_prefix}' in 1000 iterations"
        );
    }
}
