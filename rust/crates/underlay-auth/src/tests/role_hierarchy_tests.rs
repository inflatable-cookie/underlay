use super::*;

#[test]
fn can_assign_rejects_roles_outside_the_hierarchy() {
    let hierarchy = RoleHierarchy::standard();
    for caller in ["admin", "superadmin"] {
        assert_eq!(
            hierarchy.can_assign(&[caller], "unknown"),
            Err(RoleHierarchyError::UnknownRole)
        );
    }

    let custom = RoleHierarchy::new(&[("guest", -1), ("member", 0)], "owner");
    assert_eq!(
        custom.can_assign(&["unknown"], "guest"),
        Err(RoleHierarchyError::UnknownRole)
    );
    assert!(custom.can_assign(&[], "guest").is_err());
    assert!(custom.can_assign(&["member"], "guest").is_ok());
    assert!(custom.can_assign(&["owner"], "member").is_ok());
}

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
}

#[test]
fn super_role_is_protected_independent_of_numeric_level() {
    let h = RoleHierarchy::new(&[("owner", 10), ("admin", 15), ("support", 5)], "owner");
    for role in ["owner", "admin", "support"] {
        assert_eq!(
            h.can_manage(&[role], "owner", false),
            Err(RoleHierarchyError::CannotManageSuperRole)
        );
    }
}

#[test]
fn only_super_role_can_assign_super_role_regardless_of_numeric_level() {
    let h = RoleHierarchy::new(&[("owner", 10), ("admin", 15), ("support", 5)], "owner");
    assert_eq!(
        h.can_assign(&["support", "admin"], "owner"),
        Err(RoleHierarchyError::CannotPromoteToSuperRole)
    );
    assert!(h.can_assign(&["owner"], "owner").is_ok());
    assert!(h.can_assign(&["admin"], "support").is_ok());
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
fn unknown_roles_resolve_to_level_zero_for_rank_lookups() {
    let h = standard();
    assert_eq!(h.level("nonexistent"), 0);
}

#[test]
fn can_manage_fails_closed_on_unknown_and_empty_roles() {
    let h = standard();
    for caller in ["admin", "superadmin"] {
        assert_eq!(
            h.can_manage(&[caller], "nonexistent", false),
            Err(RoleHierarchyError::UnknownRole)
        );
    }
    assert_eq!(
        h.can_manage(&["nonexistent"], "user", false),
        Err(RoleHierarchyError::UnknownRole)
    );
    assert_eq!(
        h.can_manage(&["admin", "nonexistent"], "user", false),
        Err(RoleHierarchyError::UnknownRole)
    );
    assert!(matches!(
        h.can_manage(&[], "user", false),
        Err(RoleHierarchyError::InsufficientPrivileges { .. })
    ));

    let custom = RoleHierarchy::new(&[("guest", -1), ("member", 0)], "owner");
    assert_eq!(
        custom.can_manage(&["unknown"], "guest", false),
        Err(RoleHierarchyError::UnknownRole)
    );
    assert_eq!(
        custom.can_manage(&["owner"], "unknown", false),
        Err(RoleHierarchyError::UnknownRole)
    );
    assert!(matches!(
        custom.can_manage(&[], "guest", false),
        Err(RoleHierarchyError::InsufficientPrivileges { .. })
    ));
    assert!(custom.can_manage(&["member"], "guest", false).is_ok());
    assert!(matches!(
        custom.can_manage(&["guest"], "member", false),
        Err(RoleHierarchyError::InsufficientPrivileges {
            caller_level: -1,
            target_level: 0
        })
    ));
    assert!(custom.can_manage(&["owner"], "member", false).is_ok());
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
