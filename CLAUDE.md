# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a high-performance Rust library for double-entry bookkeeping, organized as a Cargo workspace. It's designed to be embedded within other applications, providing accounting functionality without imposing a specific application framework.

## Key Architecture

The project follows a layered ("onion") architecture with clear separation of concerns and unidirectional dependencies:

### Core Crates
- **`postings-api`**: Public interface defining domain objects (BOs) and service traits. Contains no business logic implementation - only defines *what* the system can do, not *how*. Depends only on essential libraries (serde, chrono).
- **`postings-db`**: Data access abstraction with database models and repository traits. Models map to database schema with attributes like `#[sqlx(FromRow)]` but are vendor-agnostic.
- **`postings-logic`**: Heart of the application implementing service traits. Contains service implementations, mappers for domain↔database translation, and caching layer using decorator pattern.
- **`postings-db-postgres`/`postings-db-mariadb`**: Swappable concrete repository implementations with actual SQL queries using sqlx.

### Critical Components
- **Mappers**: Located in `postings-logic/src/mappers/`, these translate between domain objects (`postings_api::domain::*`) and database models (`postings_db::models::*`). Essential for decoupling API from database schema.
- **Caching**: Uses decorator pattern where `CachingChartOfAccountRepository` wraps concrete repositories. Implements cache-aside pattern with `moka` for L1 in-memory cache.

### Data Flow Example
1. Consumer creates domain object, calls service trait method
2. `postings-logic` service validates business rules
3. Mapper converts domain object to database model
4. Service calls repository trait method
5. Concrete repository executes SQL against database

## Development Commands

```bash
# Build all crates
cargo build --workspace

# Run all tests (requires database for integration tests)
cargo test --workspace

# Format code (required before PRs)
rustfmt src/**/*.rs

# Run specific crate tests
cargo test -p postings-logic
```

## Data Integrity Features

The system implements:
- **Hash chains**: Each transaction links to the previous one for tamper-proof audit trail
- **Immutability**: Journal entries are read-only once created
- **Temporal flexibility**: Supports future-dated and retroactive transactions
- **Caching layer**: Uses moka cache with decorator pattern for performance

## Database Support

Integration tests require either PostgreSQL or MariaDB. Database schemas are in `migrations/` directories within each database crate. The system is designed to be database-agnostic through the repository trait abstraction.

## Testing Structure

Integration tests are in `postings-logic/tests/` and use YAML test data files. Tests cover the full stack including caching behavior and cross-database compatibility.