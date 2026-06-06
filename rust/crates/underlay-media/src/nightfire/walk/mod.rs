mod anchor;
mod field_matcher;
mod nested;
mod pointer;
mod registry;

pub(in crate::nightfire) use anchor::BlockAnchor;
pub(in crate::nightfire) use pointer::normalize_relative_pointer;
