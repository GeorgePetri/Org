use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DriveItem {
    pub file: DriveItemFile,
}

#[derive(Debug, Deserialize)]
pub struct DriveItemFile {
    pub hashes: DriveItemHashes,
}

#[derive(Debug, Deserialize)]
pub struct DriveItemHashes {
    #[serde(rename = "sha256Hash")]
    pub sha256_hash: String,
}
