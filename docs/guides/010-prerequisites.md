# 010 - Prerequisites

Before starting a new project with Underlay, ensure your development environment meets the requirements below.

## Required Tools

### 1. Rust Toolchain

**Version:** 1.75 or later

```bash
# Verify installation
rustc --version
cargo --version

# If not installed or outdated, install via rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup update stable
```

**Required Components:**
- `rustc` - Rust compiler
- `cargo` - Rust package manager
- `rustfmt` - Code formatter (usually included)
- `clippy` - Linter (usually included)

### 2. Node.js

**Version:** 20 or later (LTS recommended)

```bash
# Verify installation
node --version
npm --version

# If not installed, install via nvm
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash
nvm install 20
nvm use 20
```

### 3. pnpm

**Version:** 9 or later

```bash
# Verify installation
pnpm --version

# If not installed, install via npm
npm install -g pnpm
```

### 4. PostgreSQL

**Version:** 14 or later

```bash
# Verify installation
psql --version

# macOS (via Homebrew)
brew install postgresql@15
brew services start postgresql@15

# Ubuntu/Debian
sudo apt-get install postgresql postgresql-contrib

# Docker (alternative)
docker run -d --name postgres \
  -e POSTGRES_USER=user \
  -e POSTGRES_PASSWORD=pass \
  -e POSTGRES_DB=myapp \
  -p 5432:5432 \
  postgres:15
```

### 5. sqlx-cli

**Version:** Latest (matches sqlx crate version)

```bash
# Install with PostgreSQL support only
cargo install sqlx-cli --no-default-features --features postgres

# Verify installation
sqlx --version
```

## Verification Checklist

Run the following commands to verify your environment:

```bash
echo "=== Rust ==="
rustc --version
cargo --version

echo ""
echo "=== Node.js ==="
node --version
npm --version

echo ""
echo "=== pnpm ==="
pnpm --version

echo ""
echo "=== PostgreSQL ==="
psql --version

echo ""
echo "=== sqlx-cli ==="
sqlx --version
```

**Expected Output:**
```
=== Rust ===
rustc 1.75.0 (stable)
cargo 1.75.0

=== Node.js ===
v20.10.0
10.2.4

=== pnpm ===
9.0.0

=== PostgreSQL ===
psql (15.4)

=== sqlx-cli ===
sqlx 0.8.6
```

## Optional Tools

### GitHub CLI (Recommended for PR workflows)

```bash
# macOS
brew install gh

# Verify
gh --version
```

### Docker (Recommended for containerized services)

```bash
# macOS
brew install --cask docker

# Verify
docker --version
```

### Postman or HTTP Client (For API testing)

```bash
# Or use HTTPie (command-line alternative)
pip install httpie
```

## Database Setup

### Create Local Database

```bash
# Connect to PostgreSQL as superuser
sudo -u postgres psql

# Create user and database
CREATE USER myapp_user WITH PASSWORD 'secure_password';
CREATE DATABASE myapp_db OWNER myapp_user;
GRANT ALL PRIVILEGES ON DATABASE myapp_db TO myapp_user;

# Exit
\q
```

### Verify Connection

```bash
# Test connection
psql -h localhost -U myapp_user -d myapp_db
```

### Set Environment Variable

Add to your shell profile (`~/.zshrc` or `~/.bashrc`):

```bash
export DATABASE_URL="postgres://myapp_user:secure_password@localhost:5432/myapp_db"
```

## IDE Configuration

### VS Code (Recommended)

**Extensions:**
- `rust-analyzer` - Rust language support
- `svelte for VS Code` - Svelte syntax highlighting
- `ESLint` - JavaScript/TypeScript linting
- `Prettier` - Code formatting

**Settings (`~/.config/Code/User/settings.json`):**

```json
{
  "editor.formatOnSave": true,
  "editor.defaultFormatter": "esbenp.prettier-vscode",
  "rust-analyzer.check.command": "clippy",
  "files.associations": {
    "*.sql": "sql"
  }
}
```

### Zed (Alternative)

- Install Rust extension
- Install Svelte extension
- Configure `rust-analyzer` for clippy

## Account Setup

### GitHub

1. Create a GitHub organization for your project
2. Generate a personal access token (Classic) with `repo` scope
3. Configure git credential helper:

```bash
git config --global credential.helper store
```

### Container Registry (Optional)

If using Docker:

```bash
# Docker Hub (or GitHub Container Registry, GCR, etc.)
docker login
```

## Troubleshooting

### Issue: `sqlx` command not found

```bash
# Ensure cargo bin is in PATH
echo $PATH | grep ~/.cargo/bin

# If not, add to shell profile
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.zshrc
source ~/.zshrc

# Reinstall sqlx-cli
cargo install sqlx-cli --no-default-features --features postgres
```

### Issue: `psql: command not found`

```bash
# macOS - find PostgreSQL installation
brew --prefix postgresql@15

# Add to PATH
echo 'export PATH="/usr/local/opt/postgresql@15/bin:$PATH"' >> ~/.zshrc
source ~/.zshrc
```

### Issue: Node.js version mismatch

```bash
# Check required version in package.json engines field
cat package.json | grep '"engines"'

# Use nvm to switch versions
nvm install 20
nvm use 20
```

### Issue: pnpm permission errors

```bash
# Fix pnpm store permissions
pnpm store prune
pnpm config set store-dir ~/.pnpm-store
```

## Next Steps

Once all prerequisites are verified, proceed to [020-project-structure](./020-project-structure.md) to create the project directory layout.
