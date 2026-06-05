/// SameSite cookie policy.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum SameSite {
    /// Cookie is sent with same-site and cross-site top-level navigations.
    #[default]
    Lax,
    /// Cookie is only sent with same-site requests.
    Strict,
    /// Cookie is sent with all requests (requires Secure flag).
    None,
}

impl SameSite {
    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            SameSite::Lax => "Lax",
            SameSite::Strict => "Strict",
            SameSite::None => "None",
        }
    }
}
