//! Canonical role hierarchy for admin user-management endpoints.
//!
//! Consumers map their typed role enum to role-name strings (or read them
//! from the user record) and delegate the privilege rules here, instead of
//! each carrying a copy of the same `can_manage_user` logic.

use std::collections::BTreeMap;
use std::fmt;

/// Why a management/assignment operation was rejected.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RoleHierarchyError {
    /// Callers may not manage their own account (self-demotion/suspension).
    CannotManageSelf,
    /// Super-role accounts may only be managed by other super-role callers.
    CannotManageSuperRole,
    /// Callers may only manage users strictly below their own level.
    InsufficientPrivileges {
        caller_level: i32,
        target_level: i32,
    },
    /// Only super-role callers may promote/assign the super role.
    CannotPromoteToSuperRole,
}

impl RoleHierarchyError {
    /// Stable machine code suitable for API error envelopes.
    pub fn code(&self) -> &'static str {
        match self {
            Self::CannotManageSelf => "role_hierarchy.cannot_manage_self",
            Self::CannotManageSuperRole => "role_hierarchy.cannot_manage_super_role",
            Self::InsufficientPrivileges { .. } => "role_hierarchy.insufficient_privileges",
            Self::CannotPromoteToSuperRole => "role_hierarchy.cannot_promote_to_super_role",
        }
    }
}

impl fmt::Display for RoleHierarchyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CannotManageSelf => write!(f, "you cannot modify your own account"),
            Self::CannotManageSuperRole => {
                write!(
                    f,
                    "only super-role accounts can manage other super-role accounts"
                )
            }
            Self::InsufficientPrivileges { .. } => {
                write!(
                    f,
                    "you can only manage users with lower privileges than your own"
                )
            }
            Self::CannotPromoteToSuperRole => {
                write!(f, "only super-role accounts can assign the super role")
            }
        }
    }
}

impl std::error::Error for RoleHierarchyError {}

/// Role hierarchy with an explicit super role.
///
/// The standard Underlay hierarchy is:
/// `user < tester < editor < support < admin < superadmin`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoleHierarchy {
    levels: BTreeMap<String, i32>,
    super_role: String,
}

impl RoleHierarchy {
    /// The canonical Underlay hierarchy used by the reference consumers.
    pub fn standard() -> Self {
        let levels = [
            ("user", 0),
            ("tester", 1),
            ("editor", 2),
            ("support", 3),
            ("admin", 4),
            ("superadmin", 5),
        ]
        .into_iter()
        .map(|(name, level)| (name.to_string(), level))
        .collect();

        Self {
            levels,
            super_role: "superadmin".to_string(),
        }
    }

    /// Build a custom hierarchy. `levels` maps role name to rank (higher is
    /// more privileged); `super_role` names the top role with manage-anyone
    /// semantics. Unknown roles resolve to level 0.
    pub fn new(levels: &[(&str, i32)], super_role: &str) -> Self {
        Self {
            levels: levels
                .iter()
                .map(|(name, level)| (name.to_string(), *level))
                .collect(),
            super_role: super_role.to_string(),
        }
    }

    pub fn level(&self, role: &str) -> i32 {
        self.levels.get(role).copied().unwrap_or(0)
    }

    pub fn is_super_role(&self, role: &str) -> bool {
        role == self.super_role
    }

    /// Can a caller with `caller_roles` manage a target whose current role is
    /// `target_role`? `is_self` must be true when caller and target are the
    /// same account.
    pub fn can_manage(
        &self,
        caller_roles: &[&str],
        target_role: &str,
        is_self: bool,
    ) -> Result<(), RoleHierarchyError> {
        if is_self {
            return Err(RoleHierarchyError::CannotManageSelf);
        }

        let caller_level = self.max_level(caller_roles);
        let target_level = self.level(target_role);

        if self.has_super_role(caller_roles) {
            if self.is_super_role(target_role) {
                return Err(RoleHierarchyError::CannotManageSuperRole);
            }
            return Ok(());
        }

        if caller_level <= target_level {
            return Err(RoleHierarchyError::InsufficientPrivileges {
                caller_level,
                target_level,
            });
        }

        if self.is_super_role(target_role) {
            return Err(RoleHierarchyError::CannotPromoteToSuperRole);
        }

        Ok(())
    }

    /// Can a caller with `caller_roles` assign `role` (user creation or role
    /// assignment)? Super-role callers may assign anything; others only roles
    /// strictly below their own level.
    pub fn can_assign(&self, caller_roles: &[&str], role: &str) -> Result<(), RoleHierarchyError> {
        if self.has_super_role(caller_roles) {
            return Ok(());
        }

        let caller_level = self.max_level(caller_roles);
        let target_level = self.level(role);

        if caller_level <= target_level {
            if self.is_super_role(role) {
                return Err(RoleHierarchyError::CannotPromoteToSuperRole);
            }
            return Err(RoleHierarchyError::InsufficientPrivileges {
                caller_level,
                target_level,
            });
        }

        Ok(())
    }

    fn has_super_role(&self, caller_roles: &[&str]) -> bool {
        caller_roles.iter().any(|role| self.is_super_role(role))
    }

    fn max_level(&self, caller_roles: &[&str]) -> i32 {
        caller_roles
            .iter()
            .map(|role| self.level(role))
            .max()
            .unwrap_or(0)
    }
}

#[cfg(test)]
#[path = "tests/role_hierarchy_tests.rs"]
mod tests;
