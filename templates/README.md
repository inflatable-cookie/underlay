# Underlay Project Templates

This directory contains template files for setting up new Underlay-based projects.

## Placeholders

All templates use mustache-style placeholders that should be replaced before use:

| Placeholder | Description | Example |
|-------------|-------------|---------|
| `{{PROJECT_NAME}}` | Human-readable project name | `Acme`, `MyApp` |
| `{{PROJECT_SLUG}}` | Lowercase project identifier | `acme`, `myapp` |
| `{{DB_NAME}}` | PostgreSQL database name | `acme`, `myapp_dev` |
| `{{DB_USER}}` | PostgreSQL username | `root`, `postgres` |
| `{{DB_PASSWORD}}` | PostgreSQL password | `postgres` |
| `{{API_PORT}}` | API server port | `40011`, `3000` |
| `{{ADMIN_PORT}}` | Admin frontend port | `40012`, `3001` |
| `{{FRONT_PORT}}` | Public frontend port | `40013`, `3002` |

## Templates

### docker/docker-compose.dev.yml

Docker Compose configuration for local development services:

- **PostgreSQL 16**: Database on port 5432
- **MinIO**: S3-compatible blob storage (API: 9000, Console: 9001)
- **MailHog**: Email testing (SMTP: 1025, Web UI: 8025)

Usage:
```bash
# Copy and customize
cp templates/docker/docker-compose.dev.yml ./docker-compose.yml
sed -i 's/{{PROJECT_NAME}}/myapp/g' docker-compose.yml
sed -i 's/{{DB_NAME}}/myapp/g' docker-compose.yml
sed -i 's/{{DB_USER}}/root/g' docker-compose.yml
sed -i 's/{{DB_PASSWORD}}/postgres/g' docker-compose.yml

# Start services
docker compose up -d
```

### scripts/setup.sh

One-time setup script for new developers:

1. Checks prerequisites (Docker, Rust, Bun)
2. Starts Docker services
3. Creates `.env` files from examples
4. Runs database migrations
5. Generates JWT keys
6. Installs frontend dependencies

Usage:
```bash
# Copy and customize
cp templates/scripts/setup.sh ./scripts/setup.sh
chmod +x ./scripts/setup.sh

# Edit placeholders
# Then run:
./scripts/setup.sh
```

### scripts/reset-db.sh

Database reset script for development:

1. Drops existing database
2. Creates fresh database
3. Runs migrations
4. Optionally runs seed data

Usage:
```bash
./scripts/reset-db.sh         # Reset and migrate
./scripts/reset-db.sh --seed  # Also run seeds
```

## Quick Start

For a new project named "MyApp":

```bash
# 1. Create project structure
mkdir myapp && cd myapp
mkdir -p scripts

# 2. Copy templates
cp /path/to/underlay/templates/docker/docker-compose.dev.yml ./docker-compose.yml
cp /path/to/underlay/templates/scripts/setup.sh ./scripts/
cp /path/to/underlay/templates/scripts/reset-db.sh ./scripts/
chmod +x scripts/*.sh

# 3. Replace placeholders (macOS)
sed -i '' 's/{{PROJECT_NAME}}/MyApp/g' docker-compose.yml scripts/*.sh
sed -i '' 's/{{PROJECT_SLUG}}/myapp/g' docker-compose.yml scripts/*.sh
sed -i '' 's/{{DB_NAME}}/myapp/g' docker-compose.yml scripts/*.sh
sed -i '' 's/{{DB_USER}}/root/g' docker-compose.yml scripts/*.sh
sed -i '' 's/{{DB_PASSWORD}}/postgres/g' docker-compose.yml scripts/*.sh
sed -i '' 's/{{API_PORT}}/3000/g' scripts/*.sh
sed -i '' 's/{{ADMIN_PORT}}/3001/g' scripts/*.sh
sed -i '' 's/{{FRONT_PORT}}/3002/g' scripts/*.sh

# 4. Run setup
./scripts/setup.sh
```

## Customization

These templates are starting points. You should customize them for your project:

- Add or remove Docker services as needed
- Adjust port mappings to avoid conflicts
- Add project-specific setup steps
- Include seed data creation
- Add health checks and monitoring

## See Also

- [Local Development Guide](../docs/guides/140-local-development.md)
- [Configuration Guide](../docs/guides/120-configuration.md)
- [Database Guide](../docs/guides/050-database.md)
