use sha2::{Digest, Sha256};

pub fn sha256_hash(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input);
    let result = hasher.finalize();
    format!("{:x}", result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sha256_hash() {
        assert_eq!(
            sha256_hash("salam"),
            "0582bd2c13fff71d7f40ef5586e3f4da05a3a61fe5ba9f0b4d06e99905ab83ea"
        );
        assert_eq!(
            sha256_hash("hello are you ok #!?"),
            "d5ef5c1a3a959f846ae09ebe1472a51a7ae784a3f726457d8939e833f8f1d7ce"
        );
        assert_eq!(
            sha256_hash("12345Aa@&$hello%^"),
            "60fcd6b50b3d0d0bbf8d13ed5ff7e4b1844a1239fec1a94b0fe189222670e832"
        );
    }
}
