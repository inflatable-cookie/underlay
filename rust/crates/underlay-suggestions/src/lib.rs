//! Server-side suggestion query building utilities.
//!
//! This crate provides utilities for building efficient suggestion queries
//! for RelationSelector-style components. It handles:
//!
//! - Parsing client-provided hints (recently selected IDs)
//! - Building queries that prioritize hints while filling with server-decided items
//! - Limiting and ordering results appropriately
//!
//! # Overview
//!
//! When a RelationSelector opens, it requests suggestions from the server.
//! The client may provide "hints" - IDs of recently selected items that should
//! be prioritized in the results. The server uses these hints along with its
//! own criteria (recency, popularity, etc.) to build a curated suggestion list.
//!
//! # Example
//!
//! ```rust
//! use underlay_suggestions::{SuggestionParams, SuggestionQuery, SuggestionOrder};
//!
//! // Parse params from request query string
//! let params = SuggestionParams::from_query_string("suggestions=true&recentHints=id1,id2,id3");
//!
//! if params.wants_suggestions() {
//!     // Build a suggestion query
//!     let query = SuggestionQuery::new()
//!         .with_hints(params.recent_hints())
//!         .with_limit(15)
//!         .with_order(SuggestionOrder::HintsThenRecent);
//!
//!     // Use query.hint_ids(), query.limit(), etc. in your SQL
//! }
//! ```

mod params;
mod query;

pub use params::SuggestionParams;
pub use query::{SuggestionOrder, SuggestionQuery};
