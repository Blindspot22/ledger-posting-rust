# Ledger Postings (Rust)

This project is a high-performance, safe, and robust Rust library for double-entry bookkeeping. It is a rewrite of the core postings engine from the battle-tested Java-based [Ledgers](https://github.com/adorsys/ledgers) project, focusing exclusively on the accounting logic.

The library is designed to be embedded within other applications, providing a solid foundation for financial transaction processing without imposing a specific application framework or runtime.

## Features

### Core Accounting Functionality
*   **Journaling:** Classic journaling of transactions.
*   **Account Balances:** Real-time and historical balance inquiries.
*   **Statements:** Generation of account statements.

### Advanced & Innovative Features
*   **Immutability:** Journal entries are read-only once created, ensuring a tamper-proof audit trail.
*   **Data Integrity:** The integrity of postings is secured using hash chains, linking each transaction to the previous one.
*   **Temporal Flexibility:** The system decouples the processing time from the effective entry time, allowing for the storage of future-dated (post-dated) and past-dated (retroactive) transactions.
*   **High Throughput:** Asynchronous balance computation allows for parallel processing of journal entries, significantly increasing throughput.
*   **Designed for Scalability:**
    *   **Vertical Partitioning:** Allows for time-based partitioning of entries (e.g., off-loading closed accounting periods) to increase the performance of write operations.
    *   **Horizontal Partitioning:** Supports eventual consistency techniques for scaling across multiple nodes.

## Architecture

This library is designed as an embeddable core, not a standalone application. It follows a modular, database-agnostic architecture organized into a Cargo workspace to enforce separation of concerns.

The key crates in the workspace are:

*   `postings-api`: Defines the public API, including business object structs and service traits. It has minimal dependencies.
*   `postings-db`: Defines database-agnostic repository traits for persistence.
*   `postings-logic`: The implementation of the business logic, implementing the service traits from `postings-api` and using the repository traits from `postings-db`.
*   `postings-db-postgres`: A concrete implementation of the `postings-db` traits for PostgreSQL, using `sqlx`.
*   `postings-db-mariadb`: A concrete implementation of the `postings-db` traits for MariaDB, using `sqlx`.

This structure allows consumers to depend on the `postings-logic` and a database implementation of their choice.

## Getting Started

### Prerequisites
*   Rust toolchain (installed via [rustup](https://rustup.rs/))
*   A supported database (PostgreSQL or MariaDB) for running integration tests.

### Build

To build all crates in the workspace, run:
```bash
cargo build --workspace
```

### Test

To run all unit and integration tests, run:
```bash
cargo test --workspace
```
Integration tests require a running database instance. Refer to the test configurations for connection details.

## Usage

To use this library in your own project, add the required crates as dependencies in your `Cargo.toml` file. You will typically need `postings-logic` and one of the database implementation crates.

**Example `Cargo.toml`:**
```toml
[dependencies]
postings-logic = { path = "postings-logic" }
postings-db-postgres = { path = "postings-db-postgres" }
```

You can then instantiate the services and repositories to integrate the ledger functionality into your application.

## Contributing

Contributions are highly welcome! We use the [GitFlow](http://nvie.com/posts/a-successful-git-branching-model/) branching model for development.

Please ensure your code is formatted with `rustfmt` before submitting a pull request. For more details, please see the `CONTRIBUTING.md` file.

## Acknowledgements

This project is a Rust rewrite of the original Java-based [ledgers](https://github.com/adorsys/ledgers) project by [adorsys](https://adorsys.de/en/index.html).

We extend our gratitude to the original authors for their foundational work:
*   Francis Pouchata
*   Dmytro Mishchuk
*   Denys Golubiev
*   Dmytro Storozhyk
*   Petro Rudenko
*   Mariia Polikarpova
