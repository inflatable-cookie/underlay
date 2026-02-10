# 067 – Authorization (Role-Based Access Control)

This guide shows how to implement role-based access control (RBAC) in your Underlay-based application, building on the authentication foundation from [060-authentication](./060-authentication.md).

## Overview

**Authentication** answers: "Who are you?"  
**Authorization** answers: "What are you allowed to do?"

This guide covers:
- Defining user roles
- Extracting roles from auth tokens
- Protecting backend API handlers by role
- Protecting frontend routes by role
- Permission checking patterns

## Prerequisites

- [060-authentication](./060-authentication.md) - JWT authentication setup
- [065-session-management](./065-session-management.md) - Session flows

---

## Backend: Role-Based Authorization

### Step 1: Define User Roles

Create `apps/api/crates/auth/src/principal.rs`:

```rust
use underlay_core::Uuid;
use serde::{Deserialize, Serialize};

/// Strongly-typed user identifier.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct UserId(pub Uuid);

/// Application roles.
///
/// Keep roles coarse-grained. Fine-grained permissions should be
/// checked in business logic, not encoded as roles.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = \"lowercase\")]
pub enum UserRole {
    /// Platform superuser (internal only).
    ///
    /// Implies all other roles for access control purposes.
    Superadmin,
    
    /// Regular end users.
    User,
    
    /// Content editors (can create/edit content).
    Editor,
    
    /// Full administrators (can manage users, settings, etc.).
    Admin,
}

/// The authenticated user's identity and permissions.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserPrincipal {
    pub user_id: UserId,
    pub roles: Vec\u003cUserRole\u003e,
    pub email: Option\u003cString\u003e,
    pub display_name: Option\u003cString\u003e,
}

impl UserPrincipal {
    /// Check if the user has a specific role.
    ///
    /// Superadmin is treated as \"has every role\".
    pub fn has_role(\u0026self, role: UserRole) -\u003e bool {
        self.roles.contains(\u0026UserRole::Superadmin) || self.roles.contains(\u0026role)
    }

    /// Check if the user is a staff member (Editor or Admin).
    pub fn is_staff(\u0026self) -\u003e bool {
        if self.roles.contains(\u0026UserRole::Superadmin) {
            return true;
        }

        self.roles.iter().any(|role| {
            matches!(role, UserRole::Editor | UserRole::Admin)
        })
    }

    /// Check if the user is a superadmin.
    ///
    /// For internal tooling and emergency access only, not routine checks.
    pub fn is_superadmin(\u0026self) -\u003e bool {
        self.roles.contains(\u0026UserRole::Superadmin)
    }

    /// Check if the user has ANY of the specified roles.
    pub fn has_any_role(\u0026self, roles: \u0026[UserRole]) -\u003e bool {
        if self.roles.contains(\u0026UserRole::Superadmin) {
            return true;
        }

        roles.iter().any(|role| self.roles.contains(role))
    }
}
```

### Step 2: Extract Roles from JWT

Update your auth provider to include roles in the JWT claims.

In `apps/api/crates/auth/src/jwt.rs`:

```rust
use underlay_auth_jwt::JwtService;

impl SessionManager {
    pub fn issue_session(
        \u0026self,
        user_id: Uuid,
        roles: Vec\u003cString\u003e,  // Role strings from database
    ) -\u003e AuthResult\u003cSessionTokens\u003e {
        // Convert role strings to your domain UserRole enum
        let role_strings: Vec\u003cString\u003e = roles.iter()
            .map(|r| r.to_lowercase())
            .collect();

        let access_token = self.jwt.issue_access_token(user_id, role_strings.clone())?;
        let refresh_token = self.jwt.issue_refresh_token(user_id, role_strings)?;

        Ok(SessionTokens {
            access_token,
            refresh_token,
            expires_in: 900,
        })
    }
}
```

### Step 3: Convert Underlay Principal to Domain Principal

Create `apps/api/crates/auth/src/underlay.rs`:

```rust
use crate::{UserId, UserPrincipal, UserRole};
use underlay_auth::Principal;

/// Convert Underlay's generic Principal to your app's UserPrincipal.
pub fn user_principal_from_underlay(principal: Principal) -\u003e UserPrincipal {
    let roles: Vec\u003cUserRole\u003e = principal
        .roles
        .iter()
        .filter_map(|r| match r.to_ascii_lowercase().as_str() {
            \"superadmin\" =\u003e Some(UserRole::Superadmin),
            \"user\" =\u003e Some(UserRole::User),
            \"editor\" =\u003e Some(UserRole::Editor),
            \"admin\" =\u003e Some(UserRole::Admin),
            _ =\u003e None,  // Ignore unknown roles
        })
        .collect();

    UserPrincipal {
        user_id: UserId(principal.user_id),
        roles,
        email: principal.email,
        display_name: principal.display_name,
    }
}
```

### Step 4: Custom Extractor for Protected Routes

Create an extractor that wraps Underlay's `Authenticated` and converts to your domain type.

In `apps/api/crates/api/src/extractors.rs`:

```rust
use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use underlay_auth::Authenticated;
use underlay_core::ErrorEnvelope;

use myapp_auth::{UserPrincipal, user_principal_from_underlay};

/// Extractor for authenticated users.
///
/// Converts Underlay's Principal to your app's UserPrincipal.
pub struct AuthenticatedUser(pub UserPrincipal);

#[async_trait]
impl\u003cS\u003e FromRequestParts\u003cS\u003e for AuthenticatedUser
where
    S: Send + Sync + underlay_auth::HasAuthProvider,
{
    type Rejection = Response;

    async fn from_request_parts(parts: \u0026mut Parts, state: \u0026S) -\u003e Result\u003cSelf, Self::Rejection\u003e {
        let Authenticated(principal) = Authenticated::from_request_parts(parts, state)
            .await
            .map_err(|e| e.into_response())?;

        Ok(AuthenticatedUser(user_principal_from_underlay(principal)))
    }
}
```

### Step 5: Role-Based Extractor

Create an extractor that requires a specific role:

```rust
use myapp_auth::UserRole;

/// Extractor that requires a specific role.
///
/// Usage: RequireRole\u003c{ UserRole::Admin }\u003e
pub struct RequireRole\u003cconst R: UserRole\u003e(pub UserPrincipal);

#[async_trait]
impl\u003cS, const R: UserRole\u003e FromRequestParts\u003cS\u003e for RequireRole\u003cR\u003e
where
    S: Send + Sync + underlay_auth::HasAuthProvider,
{
    type Rejection = Response;

    async fn from_request_parts(parts: \u0026mut Parts, state: \u0026S) -\u003e Result\u003cSelf, Self::Rejection\u003e {
        let AuthenticatedUser(principal) = AuthenticatedUser::from_request_parts(parts, state).await?;

        if !principal.has_role(R) {
            return Err((
                StatusCode::FORBIDDEN,
                Json(ErrorEnvelope {
                    error: underlay_core::AppError {
                        code: \"forbidden\".into(),
                        message: \"Insufficient permissions\".into(),
                        details: None,
                    },
                }),
            ).into_response());
        }

        Ok(RequireRole(principal))
    }
}
```

**Note**: Const generics with enums are experimental. The recommended alternative is a dedicated extractor per role:

```rust
use serde_json::json;

/// Admin-only extractor.
///
/// Validates that the user is both authenticated AND has the Admin role.
/// Returns 403 Forbidden if the user lacks admin privileges.
pub struct AdminUser(pub UserPrincipal);

#[async_trait]
impl<S> FromRequestParts<S> for AdminUser
where
    S: Send + Sync + underlay_auth::HasAuthProvider,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let AuthenticatedUser(user) = AuthenticatedUser::from_request_parts(parts, state).await?;

        if !user.has_role(UserRole::Admin) {
            return Err((
                StatusCode::FORBIDDEN,
                Json(json!({
                    "ok": false,
                    "error": {
                        "code": "auth.forbidden",
                        "message": "Admin access required."
                    }
                })),
            ).into_response());
        }

        Ok(AdminUser(user))
    }
}
```

### Why Use Role Extractors (Recommended Pattern)

Role extractors like `AdminUser` provide significant benefits over manual role checks in handlers:

| Approach | Lines per handler | Total (60 handlers) |
|----------|-------------------|---------------------|
| Manual `has_role()` check | ~7 lines | ~420 lines |
| `AdminUser` extractor | 0 lines | 0 lines (+ 30-line extractor) |

**Benefits:**
- **Type-safe authorization**: If you have an `AdminUser`, you're guaranteed to be an admin
- **Eliminates duplicate code**: No repeated role-checking boilerplate
- **Prevents accidental omissions**: Can't forget the role check if the type requires it
- **Single point of change**: Update authorization logic in one place
- **Cleaner handlers**: Focus on business logic, not authorization

**Real-world example**: In Farmyard (Acowtancy), introducing `AdminUser` eliminated 68 manual role checks across 4 files, removing ~540 lines of boilerplate code.

### Step 6: Use in Handlers

#### Any Authenticated User

```rust
use axum::{Json, extract::State};
use underlay_core::SingleResponse;

use crate::extractors::AuthenticatedUser;
use crate::state::AppState;

pub async fn list_modules(
    State(state): State\u003cAppState\u003e,
    AuthenticatedUser(principal): AuthenticatedUser,
) -\u003e Json\u003cSingleResponse\u003cVec\u003cModuleDto\u003e\u003e\u003e {
    tracing::info!(user_id = %principal.user_id.0, \"Listing modules\");

    let modules = state.module_repo.list().await?;

    Json(SingleResponse { data: modules })
}
```

#### Admin-Only Handler

```rust
use crate::extractors::AdminUser;

pub async fn delete_user(
    State(state): State<AppState>,
    AdminUser(user): AdminUser,
    Path(user_id): Path<Uuid>,
) -> Json<SingleResponse<()>> {
    tracing::warn!(
        admin_user_id = %user.user_id.0,
        target_user_id = %user_id,
        "Admin deleting user"
    );

    state.user_repo.delete(user_id).await?;

    Json(SingleResponse { data: () })
}
```

#### Manual Permission Check

```rust
pub async fn update_content(
    State(state): State\u003cAppState\u003e,
    AuthenticatedUser(principal): AuthenticatedUser,
    Path(content_id): Path\u003cUuid\u003e,
    Json(req): Json\u003cUpdateContentRequest\u003e,
) -\u003e Json\u003cSingleResponse\u003cContentDto\u003e\u003e {
    // Check if user can edit this content
    if !principal.has_any_role(\u0026[UserRole::Editor, UserRole::Admin]) {
        return Err(AuthError::Forbidden(\"Editor role required\".into()));
    }

    let content = state.content_repo.update(content_id, req).await?;

    Json(SingleResponse { data: content })
}
```

---

## Frontend: Route Protection

### Server-Side Route Guards

Use SvelteKit load functions to enforce authorization.

In `apps/web/src/routes/admin/+layout.server.ts`:

```typescript
import type { LayoutServerLoad } from \"./$types\";
import { error } from \"@sveltejs/kit\";

export const load: LayoutServerLoad = async ({ locals, parent }) =\u003e {
  // Ensure user is authenticated
  if (!locals.isAuthenticated || !locals.user) {
    throw error(403, \"Authentication required\");
  }

  // Check if user has admin role
  const user = locals.user;
  const hasAdminRole = user.roles.includes(\"admin\") || user.roles.includes(\"superadmin\");

  if (!hasAdminRole) {
    throw error(403, \"Admin access required\");
  }

  return {
    user,
  };
};
```

### Update Hooks to Include User Data

Modify `apps/web/src/hooks.server.ts` to decode user from token:

```typescript
import type { Handle } from \"@sveltejs/kit\";

function decodeJwtPayload(token: string): any {
  try {
    const [, payloadPart] = token.split(\".\");
    if (!payloadPart) return null;

    const base64 = payloadPart
      .replace(/-/g, \"+\")
      .replace(/_/g, \"/\")
      .padEnd(Math.ceil(payloadPart.length / 4) * 4, \"=\");

    return JSON.parse(atob(base64));
  } catch {
    return null;
  }
}

export const handle: Handle = async ({ event, resolve }) =\u003e {
  let accessToken = event.cookies.get(ACCESS_COOKIE) ?? null;
  // ... session refresh logic ...

  // Decode user from token
  let user = null;
  if (accessToken) {
    const payload = decodeJwtPayload(accessToken);
    if (payload) {
      user = {
        id: payload.sub,
        roles: payload.roles || [],
        email: payload.email,
        displayName: payload.display_name,
      };
    }
  }

  event.locals.authToken = accessToken;
  event.locals.isAuthenticated = accessToken != null;
  event.locals.user = user;

  return resolve(event);
};
```

Update `apps/web/src/app.d.ts`:

```typescript
declare global {
  namespace App {
    interface Locals {
      authToken: string | null;
      isAuthenticated: boolean;
      user: {
        id: string;
        roles: string[];
        email?: string;
        displayName?: string;
      } | null;
    }
  }
}

export {};
```

### Client-Side Role Checks

In components, use the user data from page props:

```svelte
\u003cscript lang=\"ts\"\u003e
  import type { PageData } from \"./$types\";

  export let data: PageData;

  $: isAdmin = data.user?.roles.includes(\"admin\") || data.user?.roles.includes(\"superadmin\");
  $: isEditor = data.user?.roles.includes(\"editor\") || isAdmin;
\u003c/script\u003e

{#if isAdmin}
  \u003ca href=\"/admin/users\"\u003eManage Users\u003c/a\u003e
{/if}

{#if isEditor}
  \u003cbutton on:click={editContent}\u003eEdit Content\u003c/button\u003e
{/if}
```

---

## Database: Storing Roles

### User Table with Role

```sql
CREATE TYPE user_role AS ENUM ('superadmin', 'user', 'editor', 'admin');

CREATE TABLE auth.users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email VARCHAR(255) NOT NULL UNIQUE,
    role user_role NOT NULL DEFAULT 'user',
    status user_status NOT NULL DEFAULT 'active',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_users_role ON auth.users(role);
```

### Loading Roles in Login

```rust
async fn verify_login(\u0026self, email: \u0026str, password: \u0026str) -\u003e AuthResult\u003c(User, String)\u003e {
    let row = sqlx::query(\n        r#\"\n        SELECT u.id, u.email, u.role, u.status,\n               c.secret_encrypted\n        FROM auth.users u\n        JOIN auth.credentials c ON c.user_id = u.id AND c.type = 'password'\n        WHERE u.email = $1\n        \"#,\n    )\n    .bind(email)\n    .fetch_one(\u0026self.pool)\n    .await?;\n\n    let password_hash: String = row.get(\"secret_encrypted\");\n    self.password_hasher.verify(password.as_bytes(), \u0026password_hash)?;\n\n    let role: String = row.get(\"role\");\n    let user = User {\n        id: row.get(\"id\"),\n        email: row.get(\"email\"),\n        // ...\n    };\n\n    Ok((user, role))
}
```

---

## Permission Patterns

### 1. Route-Level (Coarse)

Use extractors like `AdminUser` for entire handlers.

**When to use**: Admin panels, superuser tools, role-specific features.

### 2. Resource-Level (Fine)

Check ownership or permissions on individual resources.

```rust
pub async fn update_article(
    State(state): State\u003cAppState\u003e,
    AuthenticatedUser(principal): AuthenticatedUser,
    Path(article_id): Path\u003cUuid\u003e,
    Json(req): Json\u003cUpdateArticleRequest\u003e,
) -\u003e Json\u003cSingleResponse\u003cArticleDto\u003e\u003e {
    let article = state.article_repo.find(article_id).await?
        .ok_or(AppError::not_found(\"Article not found\"))?;

    // Check ownership or editor role
    let can_edit = article.author_id == principal.user_id.0
        || principal.has_role(UserRole::Editor);

    if !can_edit {
        return Err(AuthError::Forbidden(\"Cannot edit this article\".into()));
    }

    let updated = state.article_repo.update(article_id, req).await?;

    Json(SingleResponse { data: updated })
}
```

**When to use**: User-generated content, private resources, collaborative editing.

### 3. Field-Level (Granular)

Conditionally include sensitive fields based on permissions.

```rust
#[derive(Serialize)]
pub struct UserDto {
    pub id: Uuid,
    pub email: String,
    pub display_name: String,
    
    #[serde(skip_serializing_if = \"Option::is_none\")]
    pub internal_notes: Option\u003cString\u003e,  // Only for staff
}

pub fn to_dto(user: User, viewer: \u0026UserPrincipal) -\u003e UserDto {
    UserDto {
        id: user.id,
        email: user.email,
        display_name: user.display_name,
        internal_notes: if viewer.is_staff() {
            Some(user.internal_notes)
        } else {
            None
        },
    }
}
```

**When to use**: Sensitive data, audit logs, admin-only fields.

---

## Security Best Practices

### 1. Fail Secure

Default to **deny** if role checking logic fails or is ambiguous.

```rust
// ❌ WRONG - defaults to allow\nlet is_admin = principal.roles.iter().find(|r| r == \"admin\").is_some();

// ✅ CORRECT - explicit check
let is_admin = principal.has_role(UserRole::Admin);
```

### 2. Server-Side Enforcement

**Always** enforce authorization on the backend. Frontend checks are for UX only.

```typescript
// Frontend: Hide UI elements (convenience)
{#if isAdmin}
  \u003cbutton\u003eDelete\u003c/button\u003e
{/if}

// Backend: Enforce permission (security)
AdminUser(user): AdminUser
```

### 3. Audit Logging

Log authorization failures for security monitoring:

```rust
if !principal.has_role(UserRole::Admin) {
    tracing::warn!(
        user_id = %principal.user_id.0,
        required_role = \"admin\",
        \"Authorization denied\"
    );
    return Err(AuthError::Forbidden);
}
```

### 4. Principle of Least Privilege

Grant the minimum role needed for each task.

```rust
// ❌ WRONG - requires admin for content editing
AdminUser(_): AdminUser

// ✅ CORRECT - allows editor or admin
if !user.has_any_role(&[UserRole::Editor, UserRole::Admin]) {
    return Err(AuthError::Forbidden);
}
```

---

## Testing

### Test Role Extraction

```rust
#[tokio::test]
async fn test_jwt_includes_roles() {
    let jwt = JwtService::new(config).unwrap();
    let token = jwt.issue_access_token(user_id, vec![\"admin\".into(), \"user\".into()]).unwrap();
    
    let claims = jwt.verify_access_token(\u0026token).unwrap();
    assert_eq!(claims.roles, Some(vec![\"admin\".to_string(), \"user\".to_string()]));
}
```

### Test Permission Check

```rust
#[test]
fn test_has_role() {
    let principal = UserPrincipal {
        user_id: UserId(Uuid::new_v7()),
        roles: vec![UserRole::Editor],
        email: None,
        display_name: None,
    };

    assert!(principal.has_role(UserRole::Editor));
    assert!(!principal.has_role(UserRole::Admin));
}

#[test]
fn test_superadmin_has_all_roles() {
    let principal = UserPrincipal {
        user_id: UserId(Uuid::new_v7()),
        roles: vec![UserRole::Superadmin],
        email: None,
        display_name: None,
    };

    assert!(principal.has_role(UserRole::Admin));
    assert!(principal.has_role(UserRole::Editor));
    assert!(principal.has_role(UserRole::User));
}
```

---

## Next Steps

- [075-validation](./075-validation.md) - Request validation
- [070-api-handlers](./070-api-handlers.md) - Complete API patterns
- [176-ai-runtime-routing](./176-ai-runtime-routing.md) - AI routing admin role expectations

---

## Next Steps

- **[070-api-handlers.md](./070-api-handlers.md)** - Implement protected API endpoints
- **[075-validation.md](./075-validation.md)** - Add validation to your handlers
- **[100-frontend-web.md](./100-frontend-web.md)** - Build frontend with role-based UI

## Reference Implementation

See your project for complete working examples:
- Backend roles: `apps/api/crates/auth/src/principal.rs`
- Role conversion: `apps/api/crates/auth/src/underlay.rs`
- Protected handlers: `apps/api/crates/api/src/main.rs`
- Frontend guards: `admin/src/routes/admin/+layout.server.ts`
