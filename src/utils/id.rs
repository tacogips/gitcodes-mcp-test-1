use std::time::{SystemTime, UNIX_EPOCH};
use rand::{thread_rng, Rng};

/// Generate a unique ID based on timestamp and random characters
///
/// # Returns
///
/// A unique ID string
pub fn generate_id() -> String {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis();
    
    let random_suffix = thread_rng().gen::<u32>();
    
    format!("{}-{:08x}", timestamp, random_suffix)
}

/// Generate a UUID v4
///
/// # Returns
///
/// A UUID v4 string
pub fn generate_uuid() -> String {
    let mut rng = thread_rng();
    
    // Generate 16 random bytes
    let mut bytes = [0u8; 16];
    rng.fill(&mut bytes);
    
    // Set the version (4) and variant (RFC4122)
    bytes[6] = (bytes[6] & 0x0F) | 0x40; // version 4
    bytes[8] = (bytes[8] & 0x3F) | 0x80; // RFC4122 variant
    
    format!(
        "{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
        bytes[0], bytes[1], bytes[2], bytes[3],
        bytes[4], bytes[5],
        bytes[6], bytes[7],
        bytes[8], bytes[9],
        bytes[10], bytes[11], bytes[12], bytes[13], bytes[14], bytes[15]
    )
}

/// Generate a short ID (6 characters) for display purposes
///
/// # Returns
///
/// A short ID string
pub fn generate_short_id() -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    const ID_LEN: usize = 6;
    
    let mut rng = thread_rng();
    
    let short_id: String = (0..ID_LEN)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();
    
    short_id
}

/// Generate a prefixed ID for a specific resource type
///
/// # Arguments
///
/// * `prefix` - The prefix to use (e.g., "usr" for users, "doc" for documents)
///
/// # Returns
///
/// A prefixed ID string
pub fn generate_prefixed_id(prefix: &str) -> String {
    let short_id = generate_short_id();
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs() % 10000; // Keep only the last 4 digits
    
    format!("{}-{}-{:04}", prefix, short_id, timestamp)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_generate_id_uniqueness() {
        let id1 = generate_id();
        let id2 = generate_id();
        assert_ne!(id1, id2);
    }
    
    #[test]
    fn test_generate_uuid_format() {
        let uuid = generate_uuid();
        
        // Check format: 8-4-4-4-12 (32 chars + 4 hyphens = 36)
        assert_eq!(uuid.len(), 36);
        assert_eq!(uuid.chars().filter(|&c| c == '-').count(), 4);
        
        // Check UUID parts
        let parts: Vec<&str> = uuid.split('-').collect();
        assert_eq!(parts.len(), 5);
        assert_eq!(parts[0].len(), 8);
        assert_eq!(parts[1].len(), 4);
        assert_eq!(parts[2].len(), 4);
        assert_eq!(parts[3].len(), 4);
        assert_eq!(parts[4].len(), 12);
    }
    
    #[test]
    fn test_generate_short_id_length() {
        let id = generate_short_id();
        assert_eq!(id.len(), 6);
    }
    
    #[test]
    fn test_generate_prefixed_id_format() {
        let id = generate_prefixed_id("usr");
        
        // Check format: prefix-shortid-timestamp
        assert!(id.starts_with("usr-"));
        
        let parts: Vec<&str> = id.split('-').collect();
        assert_eq!(parts.len(), 3);
        assert_eq!(parts[0], "usr");
        assert_eq!(parts[1].len(), 6);
        assert_eq!(parts[2].len(), 4);
    }
}