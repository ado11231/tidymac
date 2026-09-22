use std::fs::Metadata;
use std::os::unix::fs::MetadataExt;

#[must_use]
pub fn allocated_size(metadata: &Metadata) -> u64 {
    metadata.blocks().saturating_mul(512)
}
