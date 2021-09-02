//! Caesar Cipher
//! Based on cipher_crypt::caesar
//!
//! # Algorithm
//!
//! Rotate each ascii character by shift. The most basic example is ROT 13, which rotates 'a' to
//! 'n'. This implementation does not rotate unicode characters.

/// Encrypts a given [`&str`] using Caesar cipher.
///
/// See [Caesar cipher](https://en.wikipedia.org/wiki/Caesar_cipher) for more information.
///
/// # Arguments
///
/// `cipher` - String to transform.
///
/// `shift` - Amount to right-shift.
///
/// # Returns
///
/// An owned [`String`]
///
/// # Panic
///
/// This function won't panic
///
/// # Examples
/// ```
/// # use rust_algorithms::ciphers::caesar;
/// let encoded = caesar("one sheep two sheep", 3);
/// ```
pub fn caesar(cipher: &str, shift: u8) -> String {
    cipher
        .chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let first = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                // modulo the distance to keep character range
                (first + (c as u8 + shift - first) % 26) as char
            } else {
                c
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(caesar("", 13), "");
    }

    #[test]
    fn caesar_rot_13() {
        assert_eq!(caesar("rust", 13), "ehfg");
    }

    #[test]
    fn caesar_unicode() {
        assert_eq!(caesar("attack at dawn 攻", 5), "fyyfhp fy ifbs 攻");
    }
}
