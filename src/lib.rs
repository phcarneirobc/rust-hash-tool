use clap::ValueEnum;
use sha2::{Digest, Sha256, Sha512};

#[derive(Debug, Clone, Copy, ValueEnum)]
#[value(rename_all = "lowercase")]
pub enum Algorithm {
    Sha256,
    Sha512,
}

impl Algorithm {
    pub fn hash(&self, input: &str) -> String {
        match self {
            Algorithm::Sha256 => {
                let mut hasher = Sha256::new();
                hasher.update(input.as_bytes());
                format!("{:x}", hasher.finalize())
            }
            Algorithm::Sha512 => {
                let mut hasher = Sha512::new();
                hasher.update(input.as_bytes());
                format!("{:x}", hasher.finalize())
            }
        }
    }

    pub fn display_name(&self) -> &str {
        match self {
            Algorithm::Sha256 => "SHA256",
            Algorithm::Sha512 => "SHA512",
        }
    }
}
