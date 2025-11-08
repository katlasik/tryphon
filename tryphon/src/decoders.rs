//! Built-in implementations of [`ConfigValueDecoder`] for common types.
//!
//! This module provides decoder implementations for:
//! - `String` - passes through the raw value unchanged
//! - **Primitive types**: `bool`, `char`
//! - **Integers**: `u8`, `u16`, `u32`, `u64`, `u128`, `usize`, `i8`, `i16`, `i32`, `i64`, `i128`, `isize`
//! - **Floats**: `f32`, `f64`
//! - **Non-zero integers**: `NonZeroU8`, `NonZeroU16`, `NonZeroU32`, `NonZeroU64`, `NonZeroU128`, `NonZeroUsize`, `NonZeroI8`, `NonZeroI16`, `NonZeroI32`, `NonZeroI64`, `NonZeroI128`, `NonZeroIsize`
//! - **Network types**: `IpAddr`, `Ipv4Addr`, `Ipv6Addr`, `SocketAddr`, `SocketAddrV4`, `SocketAddrV6`
//! - **Path types**: `PathBuf`
//! - **Wrappers**: `Option<T>`, `Secret<T>`
//!
//! # Examples
//!
//! ## Basic Types
//!
//! ```rust
//! use tryphon::{Config, Secret};
//!
//! #[derive(Config)]
//! struct AppConfig {
//!     // String decoder - passes through raw value
//!     #[env("APP_NAME")]
//!     name: String,
//!
//!     // Numeric decoder - parses from string
//!     #[env("PORT")]
//!     port: u16,
//!
//!     // Boolean decoder - parses true/false
//!     #[env("DEBUG")]
//!     debug: bool,
//!
//!     // Character decoder
//!     #[env("SEPARATOR")]
//!     #[default(',')]
//!     separator: char,
//!
//!     // Option decoder - makes field optional
//!     #[env("LOG_LEVEL")]
//!     log_level: Option<String>,
//!
//!     // Secret decoder - masks value in output
//!     #[env("API_KEY")]
//!     api_key: Secret<String>,
//! }
//! ```
//!
//! ## Network and Path Types
//!
//! ```rust
//! use tryphon::Config;
//! use std::net::{IpAddr, SocketAddr};
//! use std::path::PathBuf;
//!
//! #[derive(Config)]
//! struct ServerConfig {
//!     // IP address decoder - parses from "192.168.1.1" or "::1"
//!     #[env("BIND_IP")]
//!     bind_ip: IpAddr,
//!
//!     // Socket address decoder - parses from "127.0.0.1:8080"
//!     #[env("LISTEN_ADDR")]
//!     listen_addr: SocketAddr,
//!
//!     // Path decoder - parses from "/var/lib/myapp"
//!     #[env("DATA_DIR")]
//!     data_dir: PathBuf,
//! }
//! ```
//!
//! ## Non-Zero Types
//!
//! ```rust
//! use tryphon::Config;
//! use std::num::NonZeroU32;
//!
//! #[derive(Config)]
//! struct PoolConfig {
//!     // Non-zero ensures value cannot be 0
//!     // Will fail to parse if value is "0"
//!     #[env("POOL_SIZE")]
//!     pool_size: NonZeroU32,
//!
//!     #[env("MAX_CONNECTIONS")]
//!     max_connections: NonZeroU32,
//! }
//! ```

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};
use std::num::{
    NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI128, NonZeroIsize, NonZeroU8,
    NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU128, NonZeroUsize,
};
use std::path::PathBuf;

use crate::config_value_decoder::ConfigValueDecoder;
use crate::secret::Secret;

impl ConfigValueDecoder for String {
    fn decode(raw: String) -> Result<String, String> {
        Ok(raw)
    }
}

/// Internal macro to generate `ConfigValueDecoder` implementations for types
/// that implement `FromStr`.
///
/// Used to implement decoders for primitive types like numbers and booleans.
/// If parsing fails, the `FromStr` error is wrapped in a `ConfigFieldError::ParsingError`.
macro_rules! make_config_value_decoder {
    ($ty: tt) => {
        impl ConfigValueDecoder for $ty {
            fn decode(raw: String) -> Result<$ty, String> {
                raw.parse::<$ty>().map_err(|e| e.to_string())
            }
        }
    };
}

/// Internal macro to generate `ConfigValueDecoder` implementations for wrapper
/// types that contain a decodable type.
///
/// Used to implement decoders for `Option<T>` and `Secret<T>`, which wrap
/// an inner type `T` that itself implements `ConfigValueDecoder`.
macro_rules! make_nested_config_value_decoder {
    ($ty: tt, $constr: expr) => {
        impl<T: ConfigValueDecoder> ConfigValueDecoder for $ty<T> {
            fn decode(raw: String) -> Result<$ty<T>, String> {
                T::decode(raw).map($constr)
            }
        }
    };
}

// Wrapper types
make_nested_config_value_decoder!(Secret, Secret);
make_nested_config_value_decoder!(Option, Some);

// Primitive types
make_config_value_decoder!(bool);
make_config_value_decoder!(char);

// Unsigned integers
make_config_value_decoder!(u8);
make_config_value_decoder!(u16);
make_config_value_decoder!(u32);
make_config_value_decoder!(u64);
make_config_value_decoder!(u128);
make_config_value_decoder!(usize);

// Signed integers
make_config_value_decoder!(i8);
make_config_value_decoder!(i16);
make_config_value_decoder!(i32);
make_config_value_decoder!(i64);
make_config_value_decoder!(i128);
make_config_value_decoder!(isize);

// Floating point
make_config_value_decoder!(f32);
make_config_value_decoder!(f64);

// Non-zero unsigned integers
make_config_value_decoder!(NonZeroU8);
make_config_value_decoder!(NonZeroU16);
make_config_value_decoder!(NonZeroU32);
make_config_value_decoder!(NonZeroU64);
make_config_value_decoder!(NonZeroU128);
make_config_value_decoder!(NonZeroUsize);

// Non-zero signed integers
make_config_value_decoder!(NonZeroI8);
make_config_value_decoder!(NonZeroI16);
make_config_value_decoder!(NonZeroI32);
make_config_value_decoder!(NonZeroI64);
make_config_value_decoder!(NonZeroI128);
make_config_value_decoder!(NonZeroIsize);

// Network types
make_config_value_decoder!(IpAddr);
make_config_value_decoder!(Ipv4Addr);
make_config_value_decoder!(Ipv6Addr);
make_config_value_decoder!(SocketAddr);
make_config_value_decoder!(SocketAddrV4);
make_config_value_decoder!(SocketAddrV6);

// Path types
make_config_value_decoder!(PathBuf);

#[cfg(test)]
mod tests {
    use crate::*;
    use std::net::*;
    use std::num::*;

    #[test]
    fn test_bool_decoder() {
        assert_eq!(bool::decode("true".to_string()).unwrap(), true);
        assert_eq!(bool::decode("false".to_string()).unwrap(), false);
        assert!(bool::decode("invalid".to_string()).is_err());
    }

    #[test]
    fn test_char_decoder() {
        assert_eq!(char::decode("a".to_string()).unwrap(), 'a');
        assert_eq!(char::decode("€".to_string()).unwrap(), '€');
        assert!(char::decode("ab".to_string()).is_err());
        assert!(char::decode("".to_string()).is_err());
    }

    #[test]
    fn test_unsigned_integer_decoders() {
        // Test each unsigned integer type with valid input
        assert_eq!(u8::decode("255".to_string()).unwrap(), 255u8);
        assert_eq!(u16::decode("65535".to_string()).unwrap(), 65535u16);
        assert_eq!(
            u32::decode("4294967295".to_string()).unwrap(),
            4294967295u32
        );
        assert_eq!(
            u64::decode("18446744073709551615".to_string()).unwrap(),
            18446744073709551615u64
        );
        assert_eq!(
            u128::decode("340282366920938463463374607431768211455".to_string()).unwrap(),
            340282366920938463463374607431768211455u128
        );
        assert_eq!(usize::decode("1000".to_string()).unwrap(), 1000usize);

        // Test overflow
        assert!(u8::decode("256".to_string()).is_err());
        assert!(u16::decode("65536".to_string()).is_err());

        // Test invalid input
        assert!(u32::decode("not_a_number".to_string()).is_err());
        assert!(u64::decode("-1".to_string()).is_err());
    }

    #[test]
    fn test_signed_integer_decoders() {
        // Test each signed integer type with valid input
        assert_eq!(i8::decode("-128".to_string()).unwrap(), -128i8);
        assert_eq!(i16::decode("-32768".to_string()).unwrap(), -32768i16);
        assert_eq!(
            i32::decode("-2147483648".to_string()).unwrap(),
            -2147483648i32
        );
        assert_eq!(
            i64::decode("-9223372036854775808".to_string()).unwrap(),
            -9223372036854775808i64
        );
        assert_eq!(
            i128::decode("-170141183460469231731687303715884105728".to_string()).unwrap(),
            -170141183460469231731687303715884105728i128
        );
        assert_eq!(isize::decode("-1000".to_string()).unwrap(), -1000isize);

        // Test positive values
        assert_eq!(i8::decode("127".to_string()).unwrap(), 127i8);
        assert_eq!(i32::decode("42".to_string()).unwrap(), 42i32);

        // Test overflow
        assert!(i8::decode("128".to_string()).is_err());
        assert!(i16::decode("32768".to_string()).is_err());

        // Test invalid input
        assert!(i32::decode("not_a_number".to_string()).is_err());
    }

    #[test]
    fn test_float_decoders() {
        assert_eq!(f32::decode("3.14".to_string()).unwrap(), 3.14f32);
        assert_eq!(f64::decode("2.71828".to_string()).unwrap(), 2.71828f64);
        assert_eq!(f32::decode("-1.5".to_string()).unwrap(), -1.5f32);
        assert_eq!(f64::decode("0.0".to_string()).unwrap(), 0.0f64);
        assert_eq!(f32::decode("1e10".to_string()).unwrap(), 1e10f32);
        assert_eq!(f64::decode("1.5e-5".to_string()).unwrap(), 1.5e-5f64);

        // Test invalid input
        assert!(f32::decode("not_a_float".to_string()).is_err());
    }

    #[test]
    fn test_nonzero_unsigned_decoders() {
        assert_eq!(NonZeroU8::decode("1".to_string()).unwrap().get(), 1);
        assert_eq!(NonZeroU16::decode("100".to_string()).unwrap().get(), 100);
        assert_eq!(NonZeroU32::decode("1000".to_string()).unwrap().get(), 1000);
        assert_eq!(
            NonZeroU64::decode("10000".to_string()).unwrap().get(),
            10000
        );
        assert_eq!(
            NonZeroU128::decode("100000".to_string()).unwrap().get(),
            100000
        );
        assert_eq!(NonZeroUsize::decode("42".to_string()).unwrap().get(), 42);

        assert!(NonZeroU8::decode("0".to_string()).is_err());
        assert!(NonZeroU32::decode("0".to_string()).is_err());
        assert_eq!(
            NonZeroU16::decode("not_a_number".to_string()).unwrap_err(),
            "invalid digit found in string"
        );
    }

    #[test]
    fn test_nonzero_signed_decoders() {
        assert_eq!(NonZeroI8::decode("1".to_string()).unwrap().get(), 1);
        assert_eq!(NonZeroI16::decode("-100".to_string()).unwrap().get(), -100);
        assert_eq!(NonZeroI32::decode("1000".to_string()).unwrap().get(), 1000);
        assert_eq!(
            NonZeroI64::decode("-10000".to_string()).unwrap().get(),
            -10000
        );
        assert_eq!(
            NonZeroI128::decode("100000".to_string()).unwrap().get(),
            100000
        );
        assert_eq!(NonZeroIsize::decode("-42".to_string()).unwrap().get(), -42);

        assert!(NonZeroI8::decode("0".to_string()).is_err());
        assert!(NonZeroI32::decode("0".to_string()).is_err());
        assert_eq!(
            NonZeroI16::decode("not_a_number".to_string()).unwrap_err(),
            "invalid digit found in string"
        );
    }

    #[test]
    fn test_ip_address_decoders() {
        assert_eq!(
            IpAddr::decode("192.168.1.1".to_string()).unwrap(),
            "192.168.1.1".parse::<IpAddr>().unwrap()
        );
        assert_eq!(
            IpAddr::decode("::1".to_string()).unwrap(),
            "::1".parse::<IpAddr>().unwrap()
        );

        assert_eq!(
            Ipv4Addr::decode("127.0.0.1".to_string()).unwrap(),
            Ipv4Addr::new(127, 0, 0, 1)
        );
        assert_eq!(
            Ipv4Addr::decode("192.168.0.1".to_string()).unwrap(),
            Ipv4Addr::new(192, 168, 0, 1)
        );

        // Test Ipv6Addr
        assert_eq!(
            Ipv6Addr::decode("::1".to_string()).unwrap(),
            Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1)
        );
        assert_eq!(
            Ipv6Addr::decode("2001:db8::1".to_string()).unwrap(),
            "2001:db8::1".parse::<Ipv6Addr>().unwrap()
        );

        // Test invalid input
        assert_eq!(
            Ipv4Addr::decode("256.1.1.1".to_string()).unwrap_err(),
            "invalid IPv4 address syntax"
        );
        assert_eq!(
            Ipv6Addr::decode("not_an_ip".to_string()).unwrap_err(),
            "invalid IPv6 address syntax"
        );
    }

    #[test]
    fn test_socket_address_decoders() {
        assert_eq!(
            SocketAddr::decode("127.0.0.1:8080".to_string()).unwrap(),
            "127.0.0.1:8080".parse::<SocketAddr>().unwrap()
        );
        assert_eq!(
            SocketAddr::decode("[::1]:8080".to_string()).unwrap(),
            "[::1]:8080".parse::<SocketAddr>().unwrap()
        );

        // Test SocketAddrV4
        assert_eq!(
            SocketAddrV4::decode("192.168.1.1:3000".to_string()).unwrap(),
            "192.168.1.1:3000".parse::<SocketAddrV4>().unwrap()
        );

        // Test SocketAddrV6
        assert_eq!(
            SocketAddrV6::decode("[2001:db8::1]:8080".to_string()).unwrap(),
            "[2001:db8::1]:8080".parse::<SocketAddrV6>().unwrap()
        );

        // Test invalid input
        assert_eq!(
            SocketAddr::decode("127.0.0.1".to_string()).unwrap_err(),
            "invalid socket address syntax"
        );
        assert_eq!(
            SocketAddr::decode("not_a_socket".to_string()).unwrap_err(),
            "invalid socket address syntax"
        );
    }

    #[test]
    fn test_option_decoder() {
        // Option wraps another decoder
        assert_eq!(
            Option::<String>::decode("hello".to_string()).unwrap(),
            Some("hello".to_string())
        );
        assert_eq!(Option::<i32>::decode("42".to_string()).unwrap(), Some(42));
        assert_eq!(
            Option::<bool>::decode("true".to_string()).unwrap(),
            Some(true)
        );

        assert!(Option::<i32>::decode("not_a_number".to_string()).is_err());
    }

    #[test]
    fn test_secret_decoder() {
        // Secret wraps another decoder
        let secret = Secret::<String>::decode("my_secret".to_string()).unwrap();
        assert_eq!(*secret, "my_secret");

        let secret = Secret::<i32>::decode("42".to_string()).unwrap();
        assert_eq!(*secret, 42);

        assert!(Secret::<i32>::decode("not_a_number".to_string()).is_err());
    }
}
