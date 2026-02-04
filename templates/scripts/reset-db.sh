#!/usr/bin/env bash
#
# {{PROJECT_NAME}} - Database Reset Script
#
# Drops and recreates the database, then runs migrations.
# Use this when you need a completely fresh database.
#
# Placeholders (replace before use):
#   {{PROJECT_NAME}}   - Your project name
#   {{PROJECT_SLUG}}   - Lowercase project slug
#   {{DB_NAME}}        - Database name
#   {{DB_USER}}        - Database user
#
# Usage:
#   ./scripts/reset-db.sh         # Reset and migrate
#   ./scripts/reset-db.sh --seed  # Reset, migrate, and seed
#
# WARNING: This will DELETE ALL DATA in the database!

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
# Confirmation
# =============================================================================

confirm_reset() {
    echo ""
    warn "This will DELETE ALL DATA in the database!"
    echo ""
    read -p "Are you sure you want to reset the database? (y/N) " -n 1 -r
    echo ""
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        info "Database reset cancelled."
        exit 0
    fi
}

# =============================================================================
# Database Operations
# =============================================================================

drop_database() {
    info "Dropping database {{DB_NAME}}..."

    # Using docker exec to run psql
    docker compose exec -T postgres psql -U {{DB_USER}} -c "DROP DATABASE IF EXISTS {{DB_NAME}};" postgres
    success "Database dropped"
}

create_database() {
    info "Creating database {{DB_NAME}}..."

    docker compose exec -T postgres psql -U {{DB_USER}} -c "CREATE DATABASE {{DB_NAME}};" postgres
    success "Database created"
}

run_migrations() {
    info "Running database migrations..."
    cd "$PROJECT_ROOT/{{PROJECT_SLUG}}-api"

    # Option 1: Using sqlx-cli
    if command -v sqlx &> /dev/null; then
        sqlx migrate run
        success "Migrations complete"
        return
    fi

    # Option 2: Using a migration binary
    # cargo run -p {{PROJECT_SLUG}}-db --bin migrate

    warn "sqlx-cli not found. Run migrations manually."
}

run_seeds() {
    info "Running seed data..."
    cd "$PROJECT_ROOT/{{PROJECT_SLUG}}-api"

    # Option 1: Using a seed binary
    # cargo run -p {{PROJECT_SLUG}}-db --bin seed

    # Option 2: Using SQL files
    if [ -d "$PROJECT_ROOT/{{PROJECT_SLUG}}-api/migrations_dev" ]; then
        for f in "$PROJECT_ROOT/{{PROJECT_SLUG}}-api/migrations_dev"/*.sql; do
            if [ -f "$f" ]; then
                info "Running $(basename "$f")..."
                docker compose exec -T postgres psql -U {{DB_USER}} -d {{DB_NAME}} -f - < "$f"
            fi
        done
        success "Seeds complete"
    else
        warn "No migrations_dev directory found. Skipping seeds."
    fi
}

# =============================================================================
# Main
# =============================================================================

main() {
    echo ""
    echo "=========================================="
    echo " {{PROJECT_NAME}} Database Reset"
    echo "=========================================="
    echo ""

    cd "$PROJECT_ROOT"

    # Check if Docker is running
    if ! docker compose ps postgres 2>/dev/null | grep -q "running"; then
        error "PostgreSQL container is not running. Start it with: docker compose up -d postgres"
    fi

    confirm_reset
    drop_database
    create_database
    run_migrations

    if [ "$1" == "--seed" ]; then
        run_seeds
    fi

    echo ""
    echo "=========================================="
    success "Database reset complete!"
    echo "=========================================="
    echo ""
}

main "$@"
