use uint::construct_uint;
use serde::{Deserialize, Serialize};

construct_uint! {
    // construct an unsignedc 256 bit integer. Consisting of 4 x 64 bit words
    #[derive(Serialize, Deserialize)]
    pub struct U256(4);
}

pub mod sha256;
pub mod types;
pub mod util;
pub mod crypto;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
