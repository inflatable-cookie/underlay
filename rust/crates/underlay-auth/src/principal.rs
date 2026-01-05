use std::collections::BTreeSet;

use underlay_core::Uuid;

/// Generic authenticated principal for Underlay-based apps.
///
/// Applications can either use this directly or wrap it in their own domain-specific
/// principal type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Principal {
    pub user_id: Uuid,
    pub roles: RoleSet,
}

impl Principal {
    pub fn has_role(&self, role: &str) -> bool {
        self.roles.contains(role)
    }
}

/// A stable, order-independent set of roles.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RoleSet(BTreeSet<String>);

impl RoleSet {
    pub fn new<I, S>(roles: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let mut set = BTreeSet::new();
        for role in roles {
            set.insert(role.into());
        }
        RoleSet(set)
    }

    pub fn contains(&self, role: &str) -> bool {
        self.0.contains(role)
    }

    pub fn iter(&self) -> impl Iterator<Item = &String> {
        self.0.iter()
    }
}
