use std::fmt;

#[derive(PartialEq, Eq)]
pub struct ConfigError {
    pub name: String,
    pub message: String,
    pub reason: ConfigErrorType,
}

#[derive(PartialEq, Eq)]
pub enum ConfigErrorType {
    NotPresent,
    Invalid,
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Config is invalid: {} is {}: {}", self.name, self.reason, self.message)
    }
}

impl fmt::Debug for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Config is invalid: {} is {}: {}", self.name, self.reason, self.message)
    }
}

impl fmt::Display for ConfigErrorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigErrorType::NotPresent => write!(f, "not present"),
            ConfigErrorType::Invalid => write!(f, "invalid"),
        }
    }
}

impl fmt::Debug for ConfigErrorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigErrorType::NotPresent => write!(f, "not present"),
            ConfigErrorType::Invalid => write!(f, "invalid"),
        }
    }
}