#!/usr/bin/env bash
#
# {{PROJECT_NAME}} - Development Setup Script
#
# Template for local development setup. Customize for your project.
#
# Placeholders (replace before use):
#   {{PROJECT_NAME}}   - Your project name (e.g., "Acme", "MyApp")
#   {{PROJECT_SLUG}}   - Lowercase project slug (e.g., "acme", "myapp")
#   {{API_PORT}}       - API server port (e.g., 40011)
#   {{ADMIN_PORT}}     - Admin frontend port (e.g., 40012)
#   {{FRONT_PORT}}     - Public frontend port (e.g., 40013)
#   {{DB_NAME}}        - Database name
#   {{DB_USER}}        - Database user
#
# This script sets up everything needed for local development:
#   1. Starts Docker services (PostgreSQL, MinIO, MailHog)
#   2. Creates environment files from templates
#   3. Runs database migrations
#   4. Generates JWT keys
#   5. Installs frontend dependencies
#
# Usage:
#   ./scripts/setup.sh           # Full setup
#   ./scripts/setup.sh --reset   # Reset database and start fresh
#
# Requirements:
#   - Docker and Docker Compose
#   - Rust toolchain (cargo)
#   - Bun (for TypeScript projects)
#   - Underlay symlink in place (if using local development)

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

info() { echo -e "${BLUE}[INFO]${NC} $1"; }
success() { echo -e "${GREEN}[OK]${NC} $1"; }
warn() { echo -e "${YELLOW}[WARN]${NC} $1"; }
error() { echo -e "${RED}[ERROR]${NC} $1"; exit 1; }

# =============================================================================
# Check Prerequisites
# =============================================================================

check_prerequisites() {
    info "Checking prerequisites..."

    # Check Docker
    if ! command -v docker &> /dev/null; then
        error "Docker is not installed. Please install Docker Desktop."
    fi

    # Check Docker Compose
    if ! docker compose version &> /dev/null; then
        error "Docker Compose is not available. Please update Docker Desktop."
    fi

    # Check Rust
    if ! command -v cargo &> /dev/null; then
        error "Rust is not installed. Please install via rustup: https://rustup.rs"
    fi

    # Check Bun
    if ! command -v bun &> /dev/null; then
        error "Bun is not installed. Please install: https://bun.sh"
    fi

    # Check Underlay symlink (optional - remove if using published package)
    if [ -L "$PROJECT_ROOT/underlay" ]; then
        success "Underlay symlink found"
    else
        warn "Underlay symlink not found. Using published package."
    fi

    success "All prerequisites met"
}

# =============================================================================
# Docker Services
# =============================================================================

start_docker_services() {
    info "Starting Docker services..."
    cd "$PROJECT_ROOT"

    if [ "$1" == "--reset" ]; then
        warn "Resetting Docker volumes..."
        docker compose down -v 2>/dev/null || true
    fi

    docker compose up -d postgres minio minio-init mailhog

    # Wait for PostgreSQL to be ready
    info "Waiting for PostgreSQL..."
    for i in {1..30}; do
        if docker compose exec -T postgres pg_isready -U {{DB_USER}} -d {{DB_NAME}} &>/dev/null; then
            success "PostgreSQL is ready"
            return
        fi
        sleep 1
    done
    error "PostgreSQL failed to start"
}

# =============================================================================
# Environment Files
# =============================================================================

setup_env_files() {
    info "Setting up environment files..."

    # API
    if [ ! -f "$PROJECT_ROOT/{{PROJECT_SLUG}}-api/.env" ]; then
        if [ -f "$PROJECT_ROOT/{{PROJECT_SLUG}}-api/.env.example" ]; then
            cp "$PROJECT_ROOT/{{PROJECT_SLUG}}-api/.env.example" "$PROJECT_ROOT/{{PROJECT_SLUG}}-api/.env"
            success "Created {{PROJECT_SLUG}}-api/.env"
        else
            warn "No .env.example found for API"
        fi
    else
        warn "{{PROJECT_SLUG}}-api/.env already exists, skipping"
    fi

    # Admin (if exists)
    if [ -d "$PROJECT_ROOT/{{PROJECT_SLUG}}-admin" ]; then
        if [ ! -f "$PROJECT_ROOT/{{PROJECT_SLUG}}-admin/.env" ]; then
            if [ -f "$PROJECT_ROOT/{{PROJECT_SLUG}}-admin/.env.example" ]; then
                cp "$PROJECT_ROOT/{{PROJECT_SLUG}}-admin/.env.example" "$PROJECT_ROOT/{{PROJECT_SLUG}}-admin/.env"
                success "Created {{PROJECT_SLUG}}-admin/.env"
            fi
        else
            warn "{{PROJECT_SLUG}}-admin/.env already exists, skipping"
        fi
    fi

    # Front (if exists)
    if [ -d "$PROJECT_ROOT/{{PROJECT_SLUG}}-front" ]; then
        if [ ! -f "$PROJECT_ROOT/{{PROJECT_SLUG}}-front/.env" ]; then
            if [ -f "$PROJECT_ROOT/{{PROJECT_SLUG}}-front/.env.example" ]; then
                cp "$PROJECT_ROOT/{{PROJECT_SLUG}}-front/.env.example" "$PROJECT_ROOT/{{PROJECT_SLUG}}-front/.env"
                success "Created {{PROJECT_SLUG}}-front/.env"
            fi
        else
            warn "{{PROJECT_SLUG}}-front/.env already exists, skipping"
        fi
    fi
}

# =============================================================================
# Database Setup
# =============================================================================

run_migrations() {
    info "Running database migrations..."
    cd "$PROJECT_ROOT/{{PROJECT_SLUG}}-api"

    # Option 1: Using a migration binary
    # if ! cargo run -p {{PROJECT_SLUG}}-db --bin migrate 2>&1; then
    #     error "Migration failed. Check the error above."
    # fi

    # Option 2: Using sqlx-cli
    if command -v sqlx &> /dev/null; then
        sqlx migrate run
        success "Database migrations complete"
    else
        warn "sqlx-cli not found. Install with: cargo install sqlx-cli"
        warn "Then run: sqlx migrate run"
    fi
}

# =============================================================================
# Auth Keys
# =============================================================================

generate_jwt_keys() {
    info "Checking JWT keys..."
    cd "$PROJECT_ROOT/{{PROJECT_SLUG}}-api"

    # Check if keys already exist in .env
    if grep -q "AUTH_JWT_PRIVATE_KEY=.\+" .env 2>/dev/null; then
        warn "JWT keys already configured, skipping"
        return
    fi

    info "Generating JWT keys..."

    # Generate ES256 key pair
    PRIVATE_KEY=$(openssl ecparam -genkey -name prime256v1 -noout 2>/dev/null | base64 -w 0)
    PUBLIC_KEY=$(echo "$PRIVATE_KEY" | base64 -d | openssl ec -pubout 2>/dev/null | base64 -w 0)

    if [ -n "$PRIVATE_KEY" ] && [ -n "$PUBLIC_KEY" ]; then
        echo "" >> .env
        echo "# JWT Keys (ES256)" >> .env
        echo "AUTH_JWT_PRIVATE_KEY=$PRIVATE_KEY" >> .env
        echo "AUTH_JWT_PUBLIC_KEY=$PUBLIC_KEY" >> .env
        success "JWT keys generated and added to .env"
    else
        warn "Failed to generate JWT keys. Generate manually or use your auth binary."
    fi
}

# =============================================================================
# Frontend Dependencies
# =============================================================================

install_frontend_deps() {
    info "Installing frontend dependencies..."

    # Client
    if [ -d "$PROJECT_ROOT/{{PROJECT_SLUG}}-client" ]; then
        info "Installing {{PROJECT_SLUG}}-client dependencies..."
        cd "$PROJECT_ROOT/{{PROJECT_SLUG}}-client"
        bun install
    fi

    # Admin
    if [ -d "$PROJECT_ROOT/{{PROJECT_SLUG}}-admin" ]; then
        info "Installing {{PROJECT_SLUG}}-admin dependencies..."
        cd "$PROJECT_ROOT/{{PROJECT_SLUG}}-admin"
        bun install
    fi

    # Front
    if [ -d "$PROJECT_ROOT/{{PROJECT_SLUG}}-front" ]; then
        info "Installing {{PROJECT_SLUG}}-front dependencies..."
        cd "$PROJECT_ROOT/{{PROJECT_SLUG}}-front"
        bun install
    fi

    success "Frontend dependencies installed"
}

# =============================================================================
# Main
# =============================================================================

main() {
    echo ""
    echo "=========================================="
    echo " {{PROJECT_NAME}} Development Setup"
    echo "=========================================="
    echo ""

    cd "$PROJECT_ROOT"

    check_prerequisites
    start_docker_services "$1"
    setup_env_files
    run_migrations
    generate_jwt_keys
    install_frontend_deps

    echo ""
    echo "=========================================="
    success "Setup complete!"
    echo "=========================================="
    echo ""
    echo "To start development:"
    echo ""
    echo "  # Terminal 1: API server"
    echo "  cd {{PROJECT_SLUG}}-api && cargo run"
    echo ""
    echo "  # Terminal 2: Admin frontend"
    echo "  cd {{PROJECT_SLUG}}-admin && bun dev"
    echo ""
    echo "  # Terminal 3: Public frontend (if applicable)"
    echo "  cd {{PROJECT_SLUG}}-front && bun dev"
    echo ""
    echo "Services:"
    echo "  - API:      http://localhost:{{API_PORT}}"
    echo "  - Admin:    http://localhost:{{ADMIN_PORT}}"
    echo "  - Front:    http://localhost:{{FRONT_PORT}}"
    echo "  - MailHog:  http://localhost:8025"
    echo "  - MinIO:    http://localhost:9001"
    echo ""
}

main "$@"
