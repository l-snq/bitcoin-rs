use uint::construct_uint;

construct_uint! {
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
