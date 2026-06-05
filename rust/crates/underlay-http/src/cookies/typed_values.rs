use super::{validation, AuthCookieError};
use std::fmt;
use std::str::FromStr;

/// Validated cookie name.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CookieName(String);

impl CookieName {
    pub fn parse(value: impl AsRef<str>) -> Result<Self, AuthCookieError> {
        let value = value.as_ref();
        validation::validate_cookie_name(value)?;
        Ok(Self(value.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for CookieName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for CookieName {
    type Err = AuthCookieError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::parse(value)
    }
}

/// Validated cookie path.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CookiePath(String);

impl CookiePath {
    pub fn parse(value: impl AsRef<str>) -> Result<Self, AuthCookieError> {
        let value = value.as_ref();
        validation::validate_cookie_path(value)?;
        Ok(Self(value.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for CookiePath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for CookiePath {
    type Err = AuthCookieError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::parse(value)
    }
}

/// Validated cookie domain.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CookieDomain(String);

impl CookieDomain {
    pub fn parse(value: impl AsRef<str>) -> Result<Self, AuthCookieError> {
        let value = value.as_ref();
        validation::validate_cookie_domain(value)?;
        Ok(Self(value.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for CookieDomain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for CookieDomain {
    type Err = AuthCookieError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::parse(value)
    }
}
