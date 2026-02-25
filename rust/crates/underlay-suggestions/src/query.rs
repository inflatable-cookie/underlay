//! Suggestion query builder.

use serde::{Deserialize, Serialize};

/// Ordering strategy for suggestion results.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum SuggestionOrder {
    /// Show hint items first (in hint order), then fill with recent items.
    /// This is the most common strategy - prioritize user's recent selections,
    /// then show recently created/modified items.
    #[default]
    HintsThenRecent,

    /// Show hint items first, then fill alphabetically.
    HintsThenAlphabetical,

    /// Show hint items first, then fill by popularity (requires tracking).
    HintsThenPopular,

    /// Show only server-decided items (ignore hints).
    ServerOnly,
}

/// Builder for suggestion queries.
///
/// This struct helps construct the parameters needed to build an efficient
/// suggestion query. It doesn't generate SQL directly (since that's database
/// and schema-specific), but provides the structured data needed.
///
/// # Example
///
/// ```rust
/// use underlay_suggestions::{SuggestionQuery, SuggestionOrder};
///
/// let query = SuggestionQuery::new()
///     .with_hints(vec!["id1", "id2", "id3"])
///     .with_limit(15)
///     .with_order(SuggestionOrder::HintsThenRecent);
///
/// // Use in your repository:
/// // - query.hint_ids() for the WHERE IN clause
/// // - query.limit() for LIMIT clause
/// // - query.fill_limit() for how many non-hint items to fetch
/// ```
#[derive(Debug, Clone)]
pub struct SuggestionQuery {
    hints: Vec<String>,
    limit: usize,
    order: SuggestionOrder,
}

impl Default for SuggestionQuery {
    fn default() -> Self {
        Self {
            hints: Vec::new(),
            limit: 15,
            order: SuggestionOrder::default(),
        }
    }
}

impl SuggestionQuery {
    /// Create a new suggestion query with default settings.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the hint IDs (recently selected by user).
    pub fn with_hints<I, S>(mut self, hints: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.hints = hints.into_iter().map(Into::into).collect();
        self
    }

    /// Set the maximum number of suggestions to return.
    pub fn with_limit(mut self, limit: usize) -> Self {
        self.limit = limit;
        self
    }

    /// Set the ordering strategy.
    pub fn with_order(mut self, order: SuggestionOrder) -> Self {
        self.order = order;
        self
    }

    /// Get the hint IDs.
    pub fn hint_ids(&self) -> &[String] {
        &self.hints
    }

    /// Get the total limit.
    pub fn limit(&self) -> usize {
        self.limit
    }

    /// Get the ordering strategy.
    pub fn order(&self) -> SuggestionOrder {
        self.order
    }

    /// Check if there are any hints.
    pub fn has_hints(&self) -> bool {
        !self.hints.is_empty()
    }

    /// Get the number of hints.
    pub fn hint_count(&self) -> usize {
        self.hints.len()
    }

    /// Calculate how many "fill" items to fetch (non-hint items).
    ///
    /// This accounts for the fact that some hints might not exist or might
    /// be filtered out, so we may need extra items to fill the limit.
    ///
    /// # Arguments
    ///
    /// * `valid_hint_count` - How many hints were actually found/valid
    ///
    /// # Example
    ///
    /// ```rust
    /// use underlay_suggestions::SuggestionQuery;
    ///
    /// let query = SuggestionQuery::new()
    ///     .with_hints(vec!["id1", "id2", "id3"])
    ///     .with_limit(10);
    ///
    /// // If all 3 hints are valid, we need 7 more items
    /// assert_eq!(query.fill_limit(3), 7);
    ///
    /// // If only 1 hint is valid, we need 9 more items
    /// assert_eq!(query.fill_limit(1), 9);
    /// ```
    pub fn fill_limit(&self, valid_hint_count: usize) -> usize {
        self.limit.saturating_sub(valid_hint_count)
    }

    /// Generate a SQL fragment for ordering hint items by their position in the hints list.
    ///
    /// This returns a CASE expression that can be used in ORDER BY to preserve
    /// the client's hint order.
    ///
    /// # Arguments
    ///
    /// * `id_column` - The name of the ID column (e.g., "level_id")
    ///
    /// # Example
    ///
    /// ```rust
    /// use underlay_suggestions::SuggestionQuery;
    ///
    /// let query = SuggestionQuery::new()
    ///     .with_hints(vec!["id1", "id2", "id3"]);
    ///
    /// let order_sql = query.hint_order_sql("level_id");
    /// // Returns: "CASE level_id WHEN 'id1' THEN 0 WHEN 'id2' THEN 1 WHEN 'id3' THEN 2 ELSE 999999 END"
    /// ```
    pub fn hint_order_sql(&self, id_column: &str) -> String {
        if self.hints.is_empty() {
            return "0".to_string();
        }

        let cases: Vec<String> = self
            .hints
            .iter()
            .enumerate()
            .map(|(i, id)| format!("WHEN '{}' THEN {}", id.replace('\'', "''"), i))
            .collect();

        format!("CASE {} {} ELSE 999999 END", id_column, cases.join(" "))
    }

    /// Generate a SQL array literal for the hint IDs.
    ///
    /// Useful for `WHERE id = ANY(...)` clauses in PostgreSQL.
    ///
    /// # Example
    ///
    /// ```rust
    /// use underlay_suggestions::SuggestionQuery;
    ///
    /// let query = SuggestionQuery::new()
    ///     .with_hints(vec!["id1", "id2"]);
    ///
    /// let array_sql = query.hint_array_sql();
    /// // Returns: "ARRAY['id1','id2']"
    /// ```
    pub fn hint_array_sql(&self) -> String {
        if self.hints.is_empty() {
            return "ARRAY[]::text[]".to_string();
        }

        let escaped: Vec<String> = self
            .hints
            .iter()
            .map(|id| format!("'{}'", id.replace('\'', "''")))
            .collect();

        format!("ARRAY[{}]", escaped.join(","))
    }
}

#[cfg(test)]
#[path = "tests/query_tests.rs"]
mod tests;
