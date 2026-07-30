use super::*;

fn standard() -> RoleHierarchy {
    RoleHierarchy::standard()
}

#[test]
fn self_management_always_rejected() {
    let h = standard();
    assert_eq!(
        h.can_manage(&["superadmin"], "user", true),
        Err(RoleHierarchyError::CannotManageSelf)
    );
    assert_eq!(
        h.can_manage(&["admin"], "admin", true),
        Err(RoleHierarchyError::CannotManageSelf)
    );
}

#[test]
fn superadmin_manages_anyone_except_superadmins() {
    let h = standard();
    assert!(h.can_manage(&["superadmin"], "admin", false).is_ok());
    assert!(h.can_manage(&["superadmin"], "user", false).is_ok());
    assert_eq!(
        h.can_manage(&["superadmin"], "superadmin", false),
        Err(RoleHierarchyError::CannotManageSuperRole)
    );
}

#[test]
fn admin_manages_only_lower_levels() {
    let h = standard();
    assert!(h.can_manage(&["admin"], "support", false).is_ok());
    assert!(h.can_manage(&["admin"], "user", false).is_ok());
    assert!(matches!(
        h.can_manage(&["admin"], "admin", false),
        Err(RoleHierarchyError::InsufficientPrivileges { .. })
    ));
    assert!(matches!(
        h.can_manage(&["admin"], "superadmin", false),
        Err(RoleHierarchyError::InsufficientPrivileges { .. })
    ));
}

#[test]
fn support_cannot_manage_admins() {
    let h = standard();
    assert!(matches!(
        h.can_manage(&["support"], "admin", false),
        Err(RoleHierarchyError::InsufficientPrivileges {
            caller_level: 3,
            target_level: 4
        })
    ));
}

#[test]
fn unknown_roles_resolve_to_level_zero() {
    let h = standard();
    assert_eq!(h.level("nonexistent"), 0);
    assert!(h.can_manage(&["admin"], "nonexistent", false).is_ok());
}

#[test]
fn assignment_rules() {
    let h = standard();
    assert!(h.can_assign(&["superadmin"], "superadmin").is_ok());
    assert!(h.can_assign(&["superadmin"], "admin").is_ok());
    assert!(h.can_assign(&["admin"], "support").is_ok());
    assert_eq!(
        h.can_assign(&["admin"], "superadmin"),
        Err(RoleHierarchyError::CannotPromoteToSuperRole)
    );
    assert!(matches!(
        h.can_assign(&["admin"], "admin"),
        Err(RoleHierarchyError::InsufficientPrivileges { .. })
    ));
    assert!(matches!(
        h.can_assign(&["support"], "admin"),
        Err(RoleHierarchyError::InsufficientPrivileges { .. })
    ));
}

#[test]
fn caller_max_level_wins_across_multiple_roles() {
    let h = standard();
    assert!(h.can_manage(&["user", "admin"], "editor", false).is_ok());
    assert!(matches!(
        h.can_manage(&["user", "tester"], "admin", false),
        Err(RoleHierarchyError::InsufficientPrivileges { .. })
    ));
}

#[test]
fn error_codes_are_stable() {
    assert_eq!(
        RoleHierarchyError::CannotManageSelf.code(),
        "role_hierarchy.cannot_manage_self"
    );
    assert_eq!(
        RoleHierarchyError::CannotPromoteToSuperRole.code(),
        "role_hierarchy.cannot_promote_to_super_role"
    );
}

#[test]
fn custom_hierarchy() {
    let h = RoleHierarchy::new(&[("member", 0), ("moderator", 1), ("owner", 2)], "owner");
    assert!(h.can_manage(&["owner"], "moderator", false).is_ok());
    assert_eq!(
        h.can_manage(&["owner"], "owner", false),
        Err(RoleHierarchyError::CannotManageSuperRole)
    );
    assert!(h.can_assign(&["moderator"], "member").is_ok());
    assert!(h.can_assign(&["moderator"], "owner").is_err());
}
