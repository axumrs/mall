use md5::{Digest, Md5};
use sha2::Sha256;

use crate::{Error, Result};

pub fn md5(data: &str) -> Result<String> {
    let mut hasher = Md5::new();
    hasher.update(data.as_bytes());

    let hash = hasher.finalize();
    let mut buf = [0u8; 32];
    let hash = base16ct::lower::encode_str(hash.as_slice(), &mut buf).map_err(Error::from)?;
    Ok(hash.to_string())
}

pub fn sha256(data: &str) -> Result<String> {
    let mut hasher = Sha256::new();
    hasher.update(data.as_bytes());

    let hash = hasher.finalize();
    let mut buf = [0u8; 64];
    let hash = base16ct::lower::encode_str(hash.as_slice(), &mut buf).map_err(Error::from)?;
    Ok(hash.to_string())
}

#[cfg(test)]
mod test {
    const TEST_DATA: &str = "AXUM中文网";
    #[test]
    fn test_md5_hash() {
        let r = super::md5(TEST_DATA);
        assert!(r.is_ok());

        let r = r.unwrap();
        assert!(r.len() == 32);
        assert!(r == "b68aa4a1f19341cb4b99f4bec9d7b501");
    }
    #[test]
    fn test_sha256_hash() {
        let r = super::sha256(TEST_DATA);
        assert!(r.is_ok());

        let r = r.unwrap();
        assert!(r.len() == 64);
        assert!(r == "5dd6bf713227f6ee120dd85280981e2683d285f75d6155fc1b8807ffae9a979b");
    }
}
