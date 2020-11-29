# api-adresse-rs
A library written in Rust for consume api https://api.gouv.fr/les-api/base-adresse-nationale from french gouvernement

# Example

```rust
use api_adresse_rs::get_address_info;

fn main() {
    let result =  get_address_info("140 Av. des Champs-Élysées").unwrap();
    println!("{:?}", result.features[0]);
}

```

