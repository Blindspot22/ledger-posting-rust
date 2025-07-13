use sha2::{Sha256, Digest};
use serde::Serialize;

pub fn hash_serialize<T: Serialize>(item: &T) -> Result<String, serde_json::Error> {
    let mut hasher = Sha256::new();
    let json = serde_json::to_string(item)?;
    hasher.update(json.as_bytes());
    let result = hasher.finalize();
    Ok(format!("{result:x}"))
}
