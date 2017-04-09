//! Keystore files pseudo-random functions

use std::{error, fmt};
use std::str::FromStr;

/// `HMAC_SHA256` pseudo-random function name
pub const HMAC_SHA256_PRF_NAME: &'static str = "hmac-sha256";

/// Pseudo-Random Functions (PRFs)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Prf {
    /// HMAC-SHA-256 (specified in (RFC 4868)[https://tools.ietf.org/html/rfc4868])
    HmacSha256,
}

impl Default for Prf {
    fn default() -> Prf {
        Prf::HmacSha256
    }
}

impl FromStr for Prf {
    type Err = PrfParserError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            _ if s == HMAC_SHA256_PRF_NAME => Ok(Prf::HmacSha256),
            _ => Err(PrfParserError::UnsupportedPrf(s.to_string())),
        }
    }
}

impl fmt::Display for Prf {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Prf::HmacSha256 => f.write_str(HMAC_SHA256_PRF_NAME),
        }
    }
}

/// `Prf` enum parser errors
#[derive(Debug)]
pub enum PrfParserError {
    /// An unsupported pseudo-random function
    UnsupportedPrf(String),
}

impl fmt::Display for PrfParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PrfParserError::UnsupportedPrf(ref str) => {
                write!(f, "Unsupported pseudo-random function: {}", str)
            }
        }
    }
}

impl error::Error for PrfParserError {
    fn description(&self) -> &str {
        "Pseudo-random function parser error"
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            _ => None,
        }
    }
}