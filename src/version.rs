pub const SUPPORTED_VERSIONS: &[u32] = &[1, 2, 3];

pub fn is_supported(v: u32) -> bool {
    SUPPORTED_VERSIONS.contains(&v)
}

/// Width in bytes of the length prefix for `metadata_kv_count`, `tensor_count`,
/// strings, and arrays. v1 used u32; v2 promoted them to u64. v3 kept v2's layout.
pub fn count_prefix_bytes(version: u32) -> u64 {
    if version == 1 { 4 } else { 8 }
}
