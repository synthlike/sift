use tiny_keccak::{Hasher, Keccak};

pub fn compute_selector(signature: &str) -> [u8; 4] {
    let mut hasher = Keccak::v256();
    let mut output = [0u8; 32];

    hasher.update(signature.as_bytes());
    hasher.finalize(&mut output);

    let mut selector = [0u8; 4];
    selector.copy_from_slice(&output[0..4]);
    selector
}

pub fn format_selector(selector: &[u8; 4]) -> String {
    format!("0x{}", hex::encode(selector))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transfer() {
        let sig = "transfer(address,uint256)";
        let selector = compute_selector(sig);
        assert_eq!(format_selector(&selector), "0xa9059cbb");
    }

    #[test]
    fn balance_of() {
        let sig = "balanceOf(address)";
        let selector = compute_selector(sig);
        assert_eq!(format_selector(&selector), "0x70a08231");
    }

    #[test]
    fn approve() {
        let sig = "approve(address,uint256)";
        let selector = compute_selector(sig);
        assert_eq!(format_selector(&selector), "0x095ea7b3");
    }
}
