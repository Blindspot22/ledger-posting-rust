use multihash_codetable::{Code, MultihashDigest};
use serde::Serialize;

pub fn hash_serialize<T: Serialize>(item: &T) -> Result<[u8; 34], serde_json::Error> {
    let json = serde_json::to_string(item)?;
    let hash = Code::Sha2_256.digest(json.as_bytes());
    let bytes = hash.to_bytes();
    let mut result = [0u8; 34];
    result.copy_from_slice(&bytes[..34]);
    Ok(result)
}
