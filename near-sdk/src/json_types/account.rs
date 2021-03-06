use borsh::{BorshDeserialize, BorshSerialize};
use serde::{de, Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt;

use crate::env::is_valid_account_id;
use crate::AccountId;

/// Helper class to validate account ID during serialization and deserializiation.
/// This type wraps an [`AccountId`].
///
/// # Example
/// ```
/// use near_sdk::AccountId;
/// use near_sdk::json_types::ValidAccountId;
///
/// let id: AccountId = "bob.near".to_string();
/// let validated: ValidAccountId = id.parse().unwrap();
/// ```
#[derive(
    Debug, Clone, PartialEq, PartialOrd, Ord, Eq, BorshDeserialize, BorshSerialize, Serialize,
)]
pub struct ValidAccountId(AccountId);

impl ValidAccountId {
    pub fn to_string(&self) -> String {
        self.0.clone()
    }
}

impl fmt::Display for ValidAccountId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<AccountId> for ValidAccountId {
    fn as_ref(&self) -> &AccountId {
        &self.0
    }
}

impl<'de> Deserialize<'de> for ValidAccountId {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as de::Deserializer<'de>>::Error>
    where
        D: de::Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        Self::try_from(s).map_err(de::Error::custom)
    }
}

impl TryFrom<&str> for ValidAccountId {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

fn validate_account_id(id: &str) -> Result<(), ParseAccountIdError> {
    if is_valid_account_id(id.as_bytes()) {
        Ok(())
    } else {
        Err(ParseAccountIdError { kind: ParseAccountIdErrorKind::InvalidAccountId })
    }
}

impl TryFrom<String> for ValidAccountId {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        validate_account_id(value.as_str())?;
        Ok(Self(value))
    }
}

impl std::str::FromStr for ValidAccountId {
    type Err = Box<dyn std::error::Error>;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        validate_account_id(value)?;
        Ok(Self(value.to_string()))
    }
}

impl From<ValidAccountId> for AccountId {
    fn from(value: ValidAccountId) -> Self {
        value.0
    }
}

#[derive(Debug)]
pub struct ParseAccountIdError {
    kind: ParseAccountIdErrorKind,
}

#[derive(Debug)]
enum ParseAccountIdErrorKind {
    InvalidAccountId,
}

impl std::fmt::Display for ParseAccountIdError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.kind {
            ParseAccountIdErrorKind::InvalidAccountId => write!(f, "the account ID is invalid"),
        }
    }
}

impl std::error::Error for ParseAccountIdError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deser() {
        let key: ValidAccountId = serde_json::from_str("\"alice.near\"").unwrap();
        assert_eq!(key.0, "alice.near".to_string());

        let key: Result<ValidAccountId, _> = serde_json::from_str("Alice.near");
        assert!(key.is_err());
    }

    #[test]
    fn test_ser() {
        let key: ValidAccountId = "alice.near".parse().unwrap();
        let actual: String = serde_json::to_string(&key).unwrap();
        assert_eq!(actual, "\"alice.near\"");
    }

    #[test]
    fn test_from_str() {
        let key = ValidAccountId::try_from("alice.near").unwrap();
        assert_eq!(key.as_ref(), &"alice.near".to_string());
    }
}
