/* src/main.rs */

mod api;
mod ip;
mod lookup;

use ip::get_public_ip_addr;
use api::{lookup, LookupProvider};

fn main() {
    match get_public_ip_addr() {
        Some(ip) => println!("{}", ip),
        None => {
            eprintln!("- Failed to fetch public IP");
            return;
        }
    }

    let mut any_success = false;

    for &provider in LookupProvider::all() {
        println!("{:?}", provider);
        match lookup(provider) {
            Some(result) => {
                any_success = true;
                println!("{}", serde_json::to_string_pretty(&result).unwrap());
            }
            None => println!("- Lookup failed for {:?}", provider),
        }
    }

    if !any_success {
        println!("- All providers failed");
    }
}
