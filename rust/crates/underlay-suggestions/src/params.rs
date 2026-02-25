//! Request parameter parsing for suggestion queries.

use serde::Deserialize;

/// Parameters for suggestion requests, parsed from query string.
///
/// # Query String Format
///
/// - `suggestions=true` - Indicates this is a suggestion request (vs. full search)
/// - `recentHints=id1,id2,id3` - Comma-separated list of recently selected IDs
///
/// # Example
///
/// ```rust
/// use underlay_suggestions::SuggestionParams;
///
/// // Parse from a query string
/// let params = SuggestionParams::from_query_string(
///     "suggestions=true&recentHints=abc-123,def-456"
/// );
///
/// assert!(params.wants_suggestions());
/// assert_eq!(params.recent_hints(), vec!["abc-123", "def-456"]);
/// ```
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct SuggestionParams {
    /// Whether this is a suggestion request
    #[serde(default)]
    suggestions: bool,

    /// Comma-separated list of recently selected IDs (hints from client)
    #[serde(default, rename = "recentHints")]
    recent_hints_raw: String,
}

impl SuggestionParams {
    /// Create empty params (no suggestions requested).
    pub fn new() -> Self {
        Self::default()
    }

    /// Parse suggestion params from a query string.
    ///
    /// # Example
    ///
    /// ```rust
    /// use underlay_suggestions::SuggestionParams;
    ///
    /// let params = SuggestionParams::from_query_string(
    ///     "suggestions=true&recentHints=id1,id2"
    /// );
    /// ```
    pub fn from_query_string(query: &str) -> Self {
        serde_urlencoded::from_str(query).unwrap_or_default()
    }

    /// Create params indicating suggestions are wanted.
    pub fn suggestions() -> Self {
        Self {
            suggestions: true,
            recent_hints_raw: String::new(),
        }
    }

    /// Create params with hint IDs.
    pub fn with_hints<I, S>(hints: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        Self {
            suggestions: true,
            recent_hints_raw: hints
                .into_iter()
                .map(|s| s.as_ref().to_string())
                .collect::<Vec<_>>()
                .join(","),
        }
    }

    /// Whether this request wants suggestions (vs. a full search/list).
    pub fn wants_suggestions(&self) -> bool {
        self.suggestions
    }

    /// Get the list of hint IDs provided by the client.
    ///
    /// These are IDs of items the user has recently selected, and should
    /// be prioritized in the suggestion results (if they still exist/are valid).
    pub fn recent_hints(&self) -> Vec<&str> {
        if self.recent_hints_raw.is_empty() {
            return Vec::new();
        }

        self.recent_hints_raw
            .split(',')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect()
    }

    /// Get hint IDs as owned strings.
    pub fn recent_hints_owned(&self) -> Vec<String> {
        self.recent_hints().into_iter().map(String::from).collect()
    }

    /// Check if any hints were provided.
    pub fn has_hints(&self) -> bool {
        !self.recent_hints_raw.is_empty()
    }

    /// Get the number of hints provided.
    pub fn hint_count(&self) -> usize {
        self.recent_hints().len()
    }
}

#[cfg(test)]
#[path = "tests/params_tests.rs"]
mod tests;
