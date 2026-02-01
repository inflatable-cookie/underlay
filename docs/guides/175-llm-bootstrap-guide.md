# LLM Bootstrap Guide: From Zero to Working App

This guide provides step-by-step instructions for bootstrapping a new Underlay-based application using the reference implementation.

## Prerequisites

Before starting, ensure you have:
- Rust toolchain installed (`rustup`)
- Bun package manager (`bun`)
- PostgreSQL database running
- Underlay repository cloned as a sibling directory (or symlinked)

## Overview

You will:
1. Copy the `acme-*` reference projects
2. Rename everything to your project name
3. Set up the database
4. Configure environment variables
5. Run and verify

---

## Step 1: Create Project Directory

```bash
mkdir your-project
cd your-project
```

## Step 2: Copy Reference Implementation

Copy all four sub-projects from the reference:

```bash
# Assuming underlay is at ../underlay
cp -r ../underlay/reference/acme-api ./api
cp -r ../underlay/reference/acme-client ./api-client
cp -r ../underlay/reference/acme-admin ./admin
cp -r ../underlay/reference/acme-front ./front
```

## Step 3: Link Underlay

Create a symlink to the Underlay repository:

```bash
ln -s /path/to/underlay ./underlay
```

## Step 4: Rename Project

Systematically replace `acme` with your project name. For this example, we use `myapp`.

### 4.1 Rust Crate Names

**Files to modify:**
- `api/Cargo.toml` - workspace member names
- `api/crates/*/Cargo.toml` - package names and dependencies

**Substitutions:**
```
acme-api → myapp-api
acme-core → myapp-core
acme-infra → myapp-infra
acme-db → myapp-db
acme-auth → myapp-auth
acme-domain → myapp-domain
acme-jobs → myapp-jobs

acme_api → myapp_api
acme_core → myapp_core
acme_infra → myapp_infra
acme_db → myapp_db
acme_auth → myapp_auth
acme_domain → myapp_domain
acme_jobs → myapp_jobs
```

### 4.2 TypeScript Package Names

**Files to modify:**
- `api-client/package.json` - package name
- `admin/package.json` - package name and dependencies
- `front/package.json` - package name and dependencies

**Substitutions:**
```
@acme/client → @myapp/client
acme-admin → myapp-admin
acme-front → myapp-front
```

### 4.3 Service and Type Names

**Files to modify:**
- `api/crates/auth/src/lib.rs`
- `api/crates/auth/src/local.rs`
- `api-client/src/utils/client-factory.ts`
- `api-client/src/index.ts`

**Substitutions:**
```
AcmeLocalAuthService → MyAppLocalAuthService
AcmeLocalAuthProvider → MyAppLocalAuthProvider
configureAcmeClient → configureMyAppClient
AcmeClientConfig → MyAppClientConfig
```

### 4.4 Cookie and Token Names

**Files to modify:**
- `admin/src/lib/utils/auth-tokens.ts`
- Any files referencing token names

**Substitutions:**
```
acme_access_token → myapp_access_token
acme_refresh_token → myapp_refresh_token
```

### 4.5 Database Schema Names

**Files to modify:**
- `api/migrations/*` - SQL files
- `api/crates/db/src/lib.rs` - DEV_RESET_SCHEMAS

**Substitutions:**
```sql
-- In migrations
CREATE SCHEMA acme → CREATE SCHEMA myapp
acme.table_name → myapp.table_name
```

### 4.6 Environment Variables

**Files to modify:**
- `api/.env.example`
- Any code referencing ACME_* variables

**Substitutions:**
```
ACME_* → MYAPP_*
```

### 4.7 App Display Names

**Files to modify:**
- `api/crates/infra/src/config.rs`
- `api/crates/auth/src/local.rs`
- `admin/src/routes/` - page titles
- `front/src/routes/` - page titles

**Substitutions:**
```
"Acme" → "My App"
acme.example.com → myapp.com
```

### Automated Renaming Script

```bash
#!/bin/bash
# rename-project.sh - Run from project root

OLD_NAME="acme"
NEW_NAME="myapp"
OLD_DISPLAY="Acme"
NEW_DISPLAY="MyApp"

# Rust files
find api -type f \( -name "*.rs" -o -name "*.toml" -o -name "*.sql" \) \
  -exec sed -i '' \
    -e "s/${OLD_NAME}-/${NEW_NAME}-/g" \
    -e "s/${OLD_NAME}_/${NEW_NAME}_/g" \
    -e "s/${OLD_DISPLAY}LocalAuthService/${NEW_DISPLAY}LocalAuthService/g" \
    -e "s/${OLD_DISPLAY}LocalAuthProvider/${NEW_DISPLAY}LocalAuthProvider/g" \
    -e "s/\"${OLD_DISPLAY}\"/\"${NEW_DISPLAY}\"/g" \
    -e "s/${OLD_NAME}\.example\.com/${NEW_NAME}.example.com/g" \
    {} \;

# TypeScript files
find api-client admin front -type f \( -name "*.ts" -o -name "*.svelte" -o -name "*.json" \) \
  -exec sed -i '' \
    -e "s/@${OLD_NAME}/@${NEW_NAME}/g" \
    -e "s/${OLD_NAME}-/${NEW_NAME}-/g" \
    -e "s/${OLD_NAME}_/${NEW_NAME}_/g" \
    -e "s/configure${OLD_DISPLAY}Client/configure${NEW_DISPLAY}Client/g" \
    -e "s/${OLD_DISPLAY}ClientConfig/${NEW_DISPLAY}ClientConfig/g" \
    -e "s/${OLD_DISPLAY}/${NEW_DISPLAY}/g" \
    {} \;
```

## Step 5: Set Up Database

### 5.1 Create Database

```bash
createdb myapp
```

### 5.2 Configure Connection

```bash
cp api/.env.example api/.env
```

Edit `api/.env`:
```bash
DATABASE_URL=postgres://your_user@localhost:5432/myapp
```

### 5.3 Run Migrations

```bash
cd api
cargo run -p myapp-db --bin migrate_dev_db
```

**Expected output:**
```
Dev database migrations + seeds complete.
```

## Step 6: Generate Auth Keys

```bash
cd api
cargo run -p myapp-auth --bin generate-jwt-env >> .env
```

This appends `AUTH_JWT_PRIVATE_KEY` and `AUTH_JWT_PUBLIC_KEY` to your `.env` file.

## Step 7: Install TypeScript Dependencies

```bash
cd api-client && bun install
cd ../admin && bun install
cd ../front && bun install
```

## Step 8: Verify Builds

### 8.1 Rust API

```bash
cd api
cargo build
```

**Expected:** Compiles without errors.

### 8.2 TypeScript Client

```bash
cd api-client
bun run build
```

**Expected:** Builds to `dist/` without errors.

### 8.3 Admin Frontend

```bash
cd admin
bun check
```

**Expected:** No type errors.

### 8.4 Public Frontend

```bash
cd front
bun check
```

**Expected:** No type errors.

## Step 9: Run the Application

### Terminal 1: API Server

```bash
cd api
cargo run
```

**Expected output:**
```
listening on 127.0.0.1:3000
```

### Terminal 2: Admin Frontend

```bash
cd admin
bun dev
```

**Expected output:**
```
  VITE v5.x.x  ready in xxx ms
  ➜  Local:   http://localhost:4174/
```

### Terminal 3: Public Frontend

```bash
cd front
bun dev
```

**Expected output:**
```
  VITE v5.x.x  ready in xxx ms
  ➜  Local:   http://localhost:4173/
```

## Step 10: Verify Everything Works

1. **Health check:** `curl http://localhost:3000/api/health`
   - Expected: `{"status":"ok"}`

2. **Admin login page:** Open `http://localhost:4174/login`
   - Expected: Login form renders

3. **Front landing page:** Open `http://localhost:4173/`
   - Expected: Landing page renders

---

## Customization Checklist

After bootstrapping, customize your project:

- [ ] Update `api/.env` with production values
- [ ] Replace email templates in `api/templates/`
- [ ] Add your domain entities in `api/crates/domain/`
- [ ] Add database tables in `api/migrations/`
- [ ] Add API routes in `api/crates/api/src/routes/`
- [ ] Add client commands in `api-client/src/commands/`
- [ ] Build admin UI in `admin/src/routes/`
- [ ] Build public UI in `front/src/routes/`
- [ ] Update branding (logos, colors, titles)
- [ ] Configure CORS origins in `.env`
- [ ] Set up CI/CD pipeline

---

## Troubleshooting

### Cargo build fails with "can't find crate"

Ensure the Underlay symlink is correct:
```bash
ls -la underlay/rust/crates/
```

### "relation does not exist" errors

Run migrations:
```bash
cd api
cargo run -p myapp-db --bin migrate_dev_db
```

### "invalid signature" JWT errors

Regenerate keys:
```bash
cd api
cargo run -p myapp-auth --bin generate-jwt-env >> .env
```

### TypeScript import errors

Rebuild the client:
```bash
cd api-client
bun run build
```

Then reinstall in admin/front:
```bash
cd admin && bun install
cd front && bun install
```
