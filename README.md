# IP Lookup

A unified IP geolocation query library using multiple free public providers.

[![Documentation](https://docs.rs/ip-lookup/badge.svg)](https://docs.rs/ip-lookup)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Overview

The `ip-lookup` crate provides a simple and unified interface to query IP geolocation information from multiple free public providers. It supports various providers like IpApi, IpInfo, IpSb, and more, allowing you to retrieve details such as country, city, coordinates, and network information for a given IP address.

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
ip-lookup = "0.1.0"
```

## Usage

The crate provides functions to get the public IP address and query geolocation data using different providers. Below is an example of how to use the library:

```rust
use ip_lookup::{get_public_ip_addr, lookup, LookupProvider, LookupResult};

fn main() {
    // Get the public IP address
    if let Some(ip) = get_public_ip_addr() {
        println!("Public IP: {}", ip);
    }

    // Query geolocation data using a specific provider
    if let Some(result) = lookup(LookupProvider::IpApi) {
        println!("Geolocation Data: {:?}", result);
    }

    // Iterate over all available providers
    for provider in LookupProvider::all() {
        if let Some(result) = lookup(*provider) {
            println!("Provider: {:?}, Data: {:?}", provider, result);
        }
    }
}
```

### Example Output

The `lookup` function returns an `Option<LookupResult>`, where `LookupResult` contains detailed geolocation information. An example response might look like this:

```json
{
  "country": {
    "city": "Singapore",
    "code": "SG",
    "zip": "535225",
    "timezone": "Asia/Singapore"
  },
  "location": {
    "latitude": 1.3371,
    "longitude": 103.8946
  },
  "connection": {
    "is_proxy": false,
    "is_tor": false,
    "is_crawler": false,
    "is_datacenter": true,
    "is_vpn": false
  },
  "network": {
    "ip": "91.243.81.209",
    "isp": "G-Core Labs S.A.",
    "org": "G-Core Labs S.A.",
    "asn": "AS199524 G-Core Labs S.A."
  }
}
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
