/// A wrapper type that masks sensitive values in `Debug` and `Display` output.
use std::collections::hash_map::DefaultHasher;
use std::fmt::{Debug, Display};
use std::hash::{Hash, Hasher};
use std::ops::Deref;

/// Use `Secret<T>` to wrap sensitive configuration values like passwords, API keys,
/// and tokens. When printed or logged, the value will appear as `***` instead of
/// the actual value, preventing accidental exposure of secrets in logs or error messages.
///
/// # Examples
///
/// ## Basic Usage
///
/// ```rust
/// use tryphon::{Config, Secret};
///
/// #[derive(Debug, Config)]
/// struct AppConfig {
///     #[env("DATABASE_PASSWORD")]
///     password: Secret<String>,
///
///     #[env("API_KEY")]
///     api_key: Secret<String>,
/// }
///
/// # unsafe { std::env::set_var("DATABASE_PASSWORD", "super-secret"); }
/// # unsafe { std::env::set_var("API_KEY", "key-12345"); }
/// let config = AppConfig::load().unwrap();
///
/// // Safe to print - secrets are masked with their hash
/// println!("{:?}", config);  // Shows: AppConfig { password: Secret(a3f8d2e1c9b4f6a7), api_key: Secret(b8e9c3d4a1f5b2e6) }
///
/// // Access the actual value when needed (via Deref)
/// let password: &String = &config.password;
/// assert_eq!(password, "super-secret");
/// ```
///
/// ## Accessing the Value
///
/// `Secret<T>` implements `Deref<Target = T>`, so you can access the wrapped value
/// transparently:
///
/// ```rust
/// use tryphon::Secret;
///
/// let secret = Secret("my-api-key".to_string());
///
/// // Access via deref
/// assert_eq!(secret.len(), 10);  // String methods work directly
/// assert_eq!(&*secret, "my-api-key");  // Explicit deref
/// ```
///
/// # Security Note
///
/// While `Secret<T>` prevents *accidental* logging of sensitive values, it does not
/// provide cryptographic protection. The actual value is still stored in memory in
/// plaintext and can be accessed intentionally via dereferencing.
#[derive(Clone)]
pub struct Secret<T>(pub T);

impl<T: Hash> Secret<T> {
    /// Computes a hash of the secret value for logging or comparison purposes.
    ///
    /// Uses Rust's standard library [`DefaultHasher`] to compute a hash of the wrapped
    /// value and returns it as a lowercase hexadecimal string. This hash is also used
    /// automatically in `Debug` and `Display` implementations.
    ///
    /// # Use Cases
    ///
    /// - **Logging**: Identify which secret was used without exposing the value
    /// - **Comparison**: Check if two secrets have the same value without revealing it
    /// - **Debugging**: Track secret changes across application runs
    ///
    /// # Security Note
    ///
    /// This hash is **not cryptographically secure** and should not be used for:
    /// - Password storage (use proper password hashing like bcrypt/argon2)
    /// - Authentication tokens
    /// - Cryptographic operations
    ///
    /// The hash algorithm ([`DefaultHasher`]) may change between Rust versions and
    /// is designed for hash tables, not security.
    ///
    /// # Returns
    ///
    /// A lowercase hexadecimal string representing the hash (e.g., `"a3f8d2e1c9b4f6a7"`).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tryphon::Secret;
    ///
    /// let secret1 = Secret("my-api-key".to_string());
    /// let secret2 = Secret("my-api-key".to_string());
    /// let secret3 = Secret("different-key".to_string());
    ///
    /// // Same values produce same hashes
    /// assert_eq!(secret1.hashed(), secret2.hashed());
    ///
    /// // Different values produce different hashes
    /// assert_ne!(secret1.hashed(), secret3.hashed());
    ///
    /// // Hash doesn't reveal the original value
    /// let hash = secret1.hashed();
    /// assert!(!hash.contains("my-api-key"));
    /// ```
    ///
    /// [`DefaultHasher`]: std::collections::hash_map::DefaultHasher
    pub fn hashed(&self) -> String {
        let mut hasher = DefaultHasher::new();
        self.0.hash(&mut hasher);
        format!("{:x}", hasher.finish())
    }
}

impl<T> Deref for Secret<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Hash> Debug for Secret<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(format!("Secret({})", self.hashed()).as_str())
    }
}

impl<T: Hash> Display for Secret<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(format!("Secret({})", self.hashed()).as_str())
    }
}

#[cfg(test)]
mod tests {

    use super::Secret;

    #[test]
    fn test_secret_debug() {
        let secret = Secret("test_value".to_string());

        let str = format!("{:?}", secret);

        assert!(!str.contains("test_value"))
    }

    #[test]
    fn test_secret_display() {
        let secret = Secret("test_value".to_string());

        let str = format!("{}", secret);

        assert!(!str.contains("test_value"))
    }
}
