// structs2.rs
// Address all the TODOs to make the tests pass!
// Execute `rustlings hint structs2` or use the `hint` watch subcommand for a hint.



struct Package {
    weight: f64,
    international: bool,
}

impl Package {
    fn is_international(&self) -> bool {
        self.international
    }

    fn get_fees(&self, cents_per_gram: u32) -> u32 {
        (self.weight * 1000.0 * cents_per_gram as f64) as u32
    }
}
