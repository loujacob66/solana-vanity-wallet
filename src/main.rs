use bip39::{Language, Mnemonic};
use clap::Parser;
use rand::rngs::OsRng;
use rand::RngCore;
use rayon::prelude::*;
use serde_json::json;
use solana_sdk::signature::{Keypair, SeedDerivable, Signer};
use std::fs;
use std::io::Write;
use std::path::Path;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

#[derive(Parser)]
struct Args {
    /// Desired prefix for the wallet
    prefix: String,
    /// Output format (json or text)
    #[arg(long, short, default_value = "text")]
    format: String,
    /// Test first character distribution
    #[arg(long)]
    test_chars: bool,
}

fn calculate_expected_iterations(prefix: &str) -> u64 {
    // Base58 alphabet has 58 characters
    // Expected iterations = 58^(prefix_length) / 2 (on average)
    let base: u64 = 58;
    let length = prefix.len() as u32;
    base.pow(length) / 2
}

fn format_duration(seconds: f64) -> String {
    if seconds < 60.0 {
        format!("{:.1}s", seconds)
    } else if seconds < 3600.0 {
        format!("{:.1}m", seconds / 60.0)
    } else if seconds < 86400.0 {
        format!("{:.1}h", seconds / 3600.0)
    } else {
        format!("{:.1}d", seconds / 86400.0)
    }
}

fn format_number(num: u64) -> String {
    if num < 1_000 {
        format!("{}", num)
    } else if num < 1_000_000 {
        format!("{:.1}K", num as f64 / 1_000.0)
    } else if num < 1_000_000_000 {
        format!("{:.1}M", num as f64 / 1_000_000.0)
    } else if num < 1_000_000_000_000 {
        format!("{:.1}B", num as f64 / 1_000_000_000.0)
    } else {
        format!("{:.1}T", num as f64 / 1_000_000_000_000.0)
    }
}

fn format_json_compact_array(value: &serde_json::Value) -> String {
    let pretty = serde_json::to_string_pretty(value).unwrap();

    // Use regex to replace the keypair_json array formatting
    let re = regex::Regex::new(r#""keypair_json": \[\s*([0-9,\s\n]+)\s*\]"#).unwrap();

    re.replace_all(&pretty, |caps: &regex::Captures| {
        let array_content = &caps[1];
        // Extract just the numbers and commas, remove whitespace and newlines
        let numbers: Vec<&str> = array_content
            .split(',')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect();

        format!("\"keypair_json\": [{}]", numbers.join(", "))
    })
    .to_string()
}

fn is_valid_base58_prefix(prefix: &str) -> bool {
    // Base58 alphabet: 123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz
    // Notable exclusions: 0, O, l (to avoid confusion)
    const BASE58_ALPHABET: &str = "123456789ABCDEFGHIJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

    if prefix.is_empty() {
        return false;
    }

    prefix.chars().all(|c| BASE58_ALPHABET.contains(c))
}

fn main() {
    let args = Args::parse();

    // Validate the prefix contains only valid Base58 characters
    if !is_valid_base58_prefix(&args.prefix) {
        eprintln!("❌ Error: Invalid prefix '{}'", args.prefix);
        eprintln!();
        eprintln!("Valid Base58 characters are:");
        eprintln!("  Numbers: 1-9 (excludes 0)");
        eprintln!("  Uppercase: A-Z (excludes O)");
        eprintln!("  Lowercase: a-z (excludes l)");
        eprintln!();
        eprintln!("Examples of valid prefixes: ABC, Sol, 123, MyWallet, IJKL");
        eprintln!("Examples of invalid prefixes: 0, O, l, _, +, =, /");
        std::process::exit(1);
    }

    let found = Arc::new(AtomicBool::new(false));
    let total_iterations = Arc::new(AtomicU64::new(0));
    let cpu_count = num_cpus::get();

    let expected_iterations = calculate_expected_iterations(&args.prefix);

    println!("🚀 Solana Vanity Wallet Generator");
    println!("==================================");
    println!("Prefix: {}", args.prefix);
    println!("Threads: {}", cpu_count);
    println!(
        "Expected iterations: {}",
        format_number(expected_iterations)
    );
    println!(
        "Estimated difficulty: 1 in {}",
        format_number(expected_iterations * 2)
    );
    println!();

    let start_time = Instant::now();
    let stats_counter = Arc::clone(&total_iterations);
    let stats_found = Arc::clone(&found);

    // Statistics thread
    let stats_thread = thread::spawn(move || {
        let mut last_count = 0;
        let mut last_time = Instant::now();

        while !stats_found.load(Ordering::Relaxed) {
            thread::sleep(Duration::from_secs(1));

            let current_count = stats_counter.load(Ordering::Relaxed);
            let current_time = Instant::now();
            let elapsed = current_time.duration_since(last_time).as_secs_f64();

            if elapsed >= 1.0 {
                let iterations_per_second = ((current_count - last_count) as f64) / elapsed;
                let total_elapsed = current_time.duration_since(start_time).as_secs_f64();
                let overall_rate = current_count as f64 / total_elapsed;

                let progress = (current_count as f64 / expected_iterations as f64) * 100.0;
                let eta_seconds = if overall_rate > 0.0 {
                    (expected_iterations as f64 - current_count as f64) / overall_rate
                } else {
                    0.0
                };

                print!(
                    "\r🔍 Iterations: {} | Rate: {}/s | Progress: {:.2}% | ETA: {} | Elapsed: {}",
                    format_number(current_count),
                    format_number(iterations_per_second as u64),
                    progress.min(100.0),
                    format_duration(eta_seconds),
                    format_duration(total_elapsed)
                );

                use std::io::{self, Write};
                io::stdout().flush().unwrap();

                last_count = current_count;
                last_time = current_time;
            }
        }
    });

    // Result storage
    let result_data = Arc::new(parking_lot::Mutex::new(
        None::<(String, String, String, Vec<u8>, u64, f64)>,
    ));

    // Worker threads
    (0..cpu_count).into_par_iter().for_each(|_| {
        let local_found = Arc::clone(&found);
        let local_counter = Arc::clone(&total_iterations);
        let local_result = Arc::clone(&result_data);
        let mut rng = OsRng;
        let mut local_iterations = 0u64;

        while !local_found.load(Ordering::Relaxed) {
            // Generate 16 bytes of entropy for 12-word mnemonic
            let mut entropy = [0u8; 16];
            rng.fill_bytes(&mut entropy);

            let mnemonic = Mnemonic::from_entropy_in(Language::English, &entropy).unwrap();
            
            // Generate keypair from the mnemonic seed to ensure they match
            let seed = mnemonic.to_seed("");
            // Create ed25519 keypair from seed
            let keypair = Keypair::from_seed(&seed[..32]).unwrap();
            let pubkey = bs58::encode(keypair.pubkey().to_bytes()).into_string();

            local_iterations += 1;

            // Update global counter every 1000 iterations to reduce contention
            if local_iterations % 1000 == 0 {
                local_counter.fetch_add(1000, Ordering::Relaxed);
            }

            if pubkey.starts_with(&args.prefix) {
                local_found.store(true, Ordering::Relaxed);
                local_counter.fetch_add(local_iterations % 1000, Ordering::Relaxed);

                let secret_key = bs58::encode(keypair.to_bytes()).into_string();
                let keypair_bytes = keypair.to_bytes().to_vec();
                let final_iterations = local_counter.load(Ordering::Relaxed);
                let elapsed_time = start_time.elapsed().as_secs_f64();

                *local_result.lock() = Some((
                    mnemonic.to_string(),
                    pubkey,
                    secret_key,
                    keypair_bytes,
                    final_iterations,
                    elapsed_time,
                ));
                break;
            }
        }
    });

    // Wait for statistics thread to finish
    stats_thread.join().unwrap();

    // Print final results
    let result = result_data.lock().take();

    if let Some((mnemonic, pubkey, secret_key, keypair_bytes, final_iterations, elapsed_time)) =
        result
    {
        println!("\n");
        println!("🎉 SUCCESS! Vanity wallet generated!");
        println!("====================================");
        println!("Total iterations: {}", format_number(final_iterations));
        println!("Time elapsed: {}", format_duration(elapsed_time));
        println!(
            "Average rate: {}/s",
            format_number((final_iterations as f64 / elapsed_time) as u64)
        );
        println!(
            "Luck factor: {:.2}x {} than expected",
            expected_iterations as f64 / final_iterations as f64,
            if final_iterations < expected_iterations {
                "better"
            } else {
                "worse"
            }
        );
        println!();

        // Prepare output data
        let output_json = json!({
            "mnemonic": mnemonic,
            "public_key": pubkey,
            "secret_key": secret_key,
            "keypair_json": keypair_bytes,
            "statistics": {
                "iterations": final_iterations,
                "elapsed_seconds": elapsed_time,
                "iterations_per_second": final_iterations as f64 / elapsed_time,
                "expected_iterations": expected_iterations,
                "luck_factor": expected_iterations as f64 / final_iterations as f64
            }
        });

        // Determine log file path
        let output_dir = Path::new("output");
        if !output_dir.exists() {
            fs::create_dir(output_dir).expect("Unable to create output directory");
        }
        let wallet_prefix = &pubkey[..10.min(pubkey.len())];
        
        if args.format == "json" {
            // JSON format: print and save as JSON
            let output_string = format_json_compact_array(&output_json);
            println!("{}", output_string);
            
            let file_name = format!("{}_output.json", wallet_prefix);
            let file_path = output_dir.join(file_name);
            let mut file = fs::File::create(file_path).expect("Unable to create log file");
            file.write_all(output_string.as_bytes())
                .expect("Unable to write data");
        } else {
            // Text format: print formatted text, save as text file
            let console_output = format!(
                "Mnemonic: {}\nPublic Key: {}\nSecret Key: {}\nKeypair JSON: [{}]",
                mnemonic,
                pubkey,
                secret_key,
                keypair_bytes.iter().map(|b| b.to_string()).collect::<Vec<_>>().join(", ")
            );
            println!("{}", console_output);
            
            let file_output = format!(
                "Solana Vanity Wallet Generated\n\
                ==============================\n\
                Mnemonic: {}\n\
                Public Key: {}\n\
                Secret Key: {}\n\
                Keypair JSON: [{}]\n\
                \n\
                Statistics:\n\
                -----------\n\
                Total iterations: {}\n\
                Time elapsed: {}\n\
                Average rate: {}/s\n\
                Expected iterations: {}\n\
                Luck factor: {:.2}x {} than expected\n",
                mnemonic,
                pubkey,
                secret_key,
                keypair_bytes.iter().map(|b| b.to_string()).collect::<Vec<_>>().join(", "),
                format_number(final_iterations),
                format_duration(elapsed_time),
                format_number((final_iterations as f64 / elapsed_time) as u64),
                format_number(expected_iterations),
                expected_iterations as f64 / final_iterations as f64,
                if final_iterations < expected_iterations { "better" } else { "worse" }
            );
            
            let file_name = format!("{}_output.txt", wallet_prefix);
            let file_path = output_dir.join(file_name);
            let mut file = fs::File::create(file_path).expect("Unable to create log file");
            file.write_all(file_output.as_bytes())
                .expect("Unable to write data");
        }
    }
}
