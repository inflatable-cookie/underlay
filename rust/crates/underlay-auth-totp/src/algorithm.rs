#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TotpAlgorithm {
    Sha1,
}

impl TotpAlgorithm {
    pub fn as_str(&self) -> &'static str {
        match self {
            TotpAlgorithm::Sha1 => "SHA1",
        }
    }
}
