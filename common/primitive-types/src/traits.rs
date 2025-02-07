use serde::{Deserialize, Serialize};

use crate::errors::GeneralError::ParseError;
use crate::errors::Result;

/// A generic type that can be converted to a hexadecimal string.
pub trait ToHex: Sized {
    /// Hexadecimal representation of this type.
    fn to_hex(&self) -> String;

    /// Tries to parse the type from the hexadecimal representation.
    fn from_hex(str: &str) -> Result<Self>;
}

/// A type that can be serialized and deserialized to a binary form.
///
/// Implementing this trait automatically implements ToHex trait
/// which then uses the serialize method.
pub trait BinarySerializable: Sized {
    /// Minimum size of this type in bytes.
    const SIZE: usize;

    /// Deserializes the type from a binary blob.
    fn from_bytes(data: &[u8]) -> Result<Self>;

    /// Serializes the type into a fixed size binary blob.
    fn to_bytes(&self) -> Box<[u8]>;
}

/// Type implementing this trait has automatic binary serialization/deserialization capability
/// using the default binary format, which is currently `bincode`.
pub trait AutoBinarySerializable: Serialize + for<'a> Deserialize<'a> {
    /// Minimum size of an automatically serialized type in bytes is 1.
    const SIZE: usize = 1;
}

impl<T> BinarySerializable for T
where
    T: AutoBinarySerializable,
{
    const SIZE: usize = Self::SIZE;

    /// Deserializes the type from a binary blob.
    fn from_bytes(data: &[u8]) -> Result<Self> {
        bincode::deserialize(data).map_err(|_| ParseError)
    }

    /// Serializes the type into a fixed size binary blob.
    fn to_bytes(&self) -> Box<[u8]> {
        bincode::serialize(&self).unwrap().into_boxed_slice()
    }
}

impl<T> ToHex for T
where
    T: BinarySerializable,
{
    fn to_hex(&self) -> String {
        format!("0x{}", hex::encode(&self.to_bytes()))
    }

    fn from_hex(str: &str) -> Result<Self> {
        if !str.is_empty() && str.len() % 2 == 0 {
            let data = if &str[..2] == "0x" || &str[..2] == "0X" {
                &str[2..]
            } else {
                str
            };

            hex::decode(data)
                .map_err(|_| ParseError)
                .and_then(|bytes| T::from_bytes(&bytes))
        } else {
            Err(ParseError)
        }
    }
}

/// Allows type to be multiplied and divided by a float in range [0.0, 1.0].
pub trait UnitaryFloatOps: Sized {
    /// Multiply with float in the interval [0.0, 1.0]
    fn mul_f64(&self, rhs: f64) -> Result<Self>;
    /// Divide by float in the interval (0.0, 1.0]
    fn div_f64(&self, rhs: f64) -> Result<Self>;
}

/// Extension trait for fixed size numbers to allow conversion to/from endian representations.
pub trait IntoEndian<const N: usize> {
    /// Create instance from Big Endian bytes. Should panic if size is more than `N`.
    fn from_be_bytes<T: AsRef<[u8]>>(bytes: T) -> Self;
    /// Create instance from Little Endian bytes. Should panic if size is more than `N`.
    fn from_le_bytes<T: AsRef<[u8]>>(bytes: T) -> Self;
    /// Convert instance to Little Endian bytes.
    fn to_le_bytes(self) -> [u8; N];
    /// Convert instance to Big Endian bytes.
    fn to_be_bytes(self) -> [u8; N];
}

/// A trait that adds extension method to represent a time object as `Duration` since Unix epoch.
pub trait AsUnixTimestamp {
    /// Represents self as `Duration` since Unix epoch.
    fn as_unix_timestamp(&self) -> std::time::Duration;
}

impl AsUnixTimestamp for std::time::SystemTime {
    fn as_unix_timestamp(&self) -> std::time::Duration {
        self.saturating_sub(std::time::SystemTime::UNIX_EPOCH)
    }
}

/// A trait that adds extension method to perform saturated substractions on `SystemTime` instances.
pub trait SaturatingSub {
    /// Performs saturated substraction on `SystemTime` instances.
    fn saturating_sub(&self, earlier: std::time::SystemTime) -> std::time::Duration;
}

impl SaturatingSub for std::time::SystemTime {
    fn saturating_sub(&self, earlier: std::time::SystemTime) -> std::time::Duration {
        self.duration_since(earlier).unwrap_or(std::time::Duration::ZERO)
    }
}
