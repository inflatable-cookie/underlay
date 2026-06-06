use super::*;

// This is a compile-time test to ensure the trait is object-safe
fn _assert_object_safe(_: &dyn MediaRepository) {}

fn _assert_usage_repository_object_safe(_: &dyn MediaUsageRepository) {}
