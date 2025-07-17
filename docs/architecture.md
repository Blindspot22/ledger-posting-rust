# Ledger-Posting Technical Architecture

This document outlines the technical architecture of the `ledger-posting` application, a Rust-based system for managing double-entry accounting postings. The architecture is designed to be modular, testable, and maintainable by separating concerns into distinct layers and modules.

## 1. Module Structure

The application is organized into a Cargo workspace with several crates (modules), each with a specific responsibility.

### 1.1. `@postings-api` - The Public Interface

-   **Purpose:** This crate defines the public API and the core domain model of the application. It serves as the primary contract for any consumer of this library.
-   **Contents:**
    -   **`domain/`**: Contains the "Business Objects" (BOs). These are rich domain models that represent the core concepts of the accounting system, such as `Posting`, `Ledger`, `LedgerAccount`, and `AccountStmt`. They are completely independent of any database or implementation details.
    -   **`service/`**: Defines the service traits (e.g., `PostingService`, `LedgerService`). These traits act as interfaces, specifying the business operations that can be performed. They define *what* the application can do, not *how* it does it.
-   **Key Principle:** This crate has no knowledge of the underlying business logic implementation or the database technology used. It depends only on essential libraries like `serde` for serialization and `chrono` for date/time handling.

### 1.2. `@postings-db` - The Data Access Abstraction Layer

-   **Purpose:** This crate defines the interface for the data persistence layer. It abstracts away the specifics of any particular database.
-   **Contents:**
    -   **`models/`**: Contains the data models (e.g., `Posting`, `LedgerAccount`). These structs are designed to map directly to the database schema. They often include database-specific attributes (like `#[sqlx(FromRow)]`) but are not tied to a single database *vendor*. They represent the shape of the data as it is stored.
    -   **`repositories/`**: Defines the repository traits (e.g., `PostingRepository`, `LedgerRepository`). These traits specify the data access operations required by the business logic, such as `find_by_id` or `save`.
-   **Key Principle:** This crate provides the blueprint for data access without implementing it. It depends on `@postings-api` because repository methods may accept or return domain objects, although the primary focus is on the database models.

### 1.3. `@postings-logic` - The Business Logic Implementation

-   **Purpose:** This is the heart of the application. It contains the concrete implementation of the business logic defined in `@postings-api`.
-   **Contents:**
    -   **`services/`**: Implements the service traits from `@postings-api`. For example, `PostingServiceImpl` implements the `PostingService` trait. These services orchestrate the business rules, calling repositories to fetch and persist data.
    -   **`mappers/`**: This crucial component is responsible for translating between the domain objects (`@postings-api`) and the database models (`@postings-db`). For instance, `PostingMapper` converts a `postings_api::domain::posting::Posting` into a `postings_db::models::posting::Posting` and vice-versa. This mapping is essential for decoupling the public API from the database schema.
    -   **`caching/`**: Provides a caching layer for repositories to improve performance. It uses a **Decorator Pattern**, where a caching repository (e.g., `CachingChartOfAccountRepository`) wraps a concrete repository implementation. This allows adding caching functionality without modifying the original repository. It uses `moka` for a high-performance, in-memory cache.
-   **Key Principle:** This crate orchestrates the application's functionality. It depends on `@postings-api` (for the interfaces it implements and the domain objects it uses) and `@postings-db` (for the repository interfaces it uses to access data).

### 1.4. `@postings-db-mariadb` & `@postings-db-postgres` - Concrete Database Implementations

-   **Purpose:** These crates provide the concrete implementations of the repository traits defined in `@postings-db` for specific database vendors (MariaDB and PostgreSQL, respectively).
-   **Contents:**
    -   **`repositories/`**: Contains structs (e.g., `PostgresPostingRepository`) that implement the corresponding repository traits from `@postings-db`. They contain the actual SQL queries and logic for interacting with the database, using `sqlx`.
    -   **`migrations/`**: SQL files that define the database schema for that specific database vendor.
-   **Key Principle:** These crates are swappable. The application can be configured to use either PostgreSQL or MariaDB without changing any of the business logic in `@postings-logic`. This is achieved by depending on `@postings-db` and implementing its traits.

## 2. Layered Architecture & Data Flow

The architecture follows a classic layered (or "onion") approach, ensuring a clean separation of concerns and a unidirectional flow of dependencies.

```
+-----------------------------------------------------------------+
|                   Application Consumers                         |
| (e.g., a web server, a CLI, or another Rust application)        |
+-----------------------------------------------------------------+
      |
      v
+-----------------------------------------------------------------+
| @postings-api                                                   |
| - Domain Objects (Ledger, Posting)                              |
| - Service Traits (LedgerService, PostingService)                |
+-----------------------------------------------------------------+
      |
      v
+-----------------------------------------------------------------+
| @postings-logic                                                 |
| - Service Implementations (PostingServiceImpl)                  |
| - Mappers (Domain <-> DB Model)                                 |
| - Business Rules & Orchestration                              |
+-----------------------------------------------------------------+
      |
      v
+-----------------------------------------------------------------+
| @postings-db                                                    |
| - DB Models (structs mapping to tables)                         |
| - Repository Traits (find_by_id, save)                          |
+-----------------------------------------------------------------+
      |
      v
+-----------------------------------------------------------------+
| @postings-db-postgres | @postings-db-mariadb                    |
| - Concrete Repository Implementations (SQL queries)             |
+-----------------------------------------------------------------+
      |
      v
+-----------------------------------------------------------------+
|                   Database (PostgreSQL / MariaDB)               |
+-----------------------------------------------------------------+
```

### Data Flow Example: Creating a New Posting

1.  **Consumer:** An application consumer (e.g., a web API endpoint) creates a `postings_api::domain::posting::Posting` object and calls the `new_posting` method on a `PostingService` trait object.
2.  **Logic Layer:** The call is dispatched to `postings_logic::services::posting_service::PostingServiceImpl`.
3.  **Business Rules:** The service validates the posting (e.g., ensuring debits equal credits).
4.  **Mapping:** The `PostingMapper` is used to convert the `Posting` domain object into a `postings_db::models::posting::Posting` model suitable for database insertion.
5.  **Data Access:** The service calls the `save` method on the `PostingRepository` trait object, passing the database model.
6.  **Concrete Implementation:** The call is dispatched to the configured concrete repository (e.g., `postings_db_postgres::repositories::posting_repository::PostgresPostingRepository`).
7.  **Database:** The repository executes the `INSERT` SQL statement against the PostgreSQL database.

This strict separation ensures that the business logic is not polluted with SQL queries and that the public API is not tied to the database schema.

## 3. Caching Strategy

To optimize performance and reduce database load, the application implements a caching layer. The strategy is designed to be multi-level, although only the first level is currently implemented.

### 3.1. First-Level (L1) Cache: In-Memory Cache

-   **Implementation:** The L1 cache is an in-memory cache implemented within the `@postings-logic` crate using the `moka` library.
-   **Scope:** This cache lives within the application process. For example, the `CachingChartOfAccountRepository` maintains a cache of `ChartOfAccount` objects that have been recently accessed, either by ID or by name.
-   **Mechanism:** It follows the **cache-aside** pattern. When a service requests data (e.g., a Chart of Accounts):
    1.  The `CachingChartOfAccountRepository` first checks if the item is in its `moka` cache.
    2.  If found (a cache hit), it returns the data directly, avoiding a database call.
    3.  If not found (a cache miss), it calls the underlying (decorated) repository to fetch the data from the database.
    4.  The fetched data is then stored in the cache before being returned to the service.
-   **Invalidation:** The cache is automatically invalidated when data is changed. For example, calling `save` on the `CachingChartOfAccountRepository` will remove the corresponding entry from both the ID and name caches to prevent stale data.

### 3.2. Second-Level (L2) Cache: Shared/Distributed Cache (Potential Extension)

-   **Status:** Not currently implemented.
-   **Purpose:** A second-level cache would typically be a shared, out-of-process cache (like Redis or Memcached). This would be beneficial in a distributed environment where multiple instances of the application are running.
-   **Strategy:** An L2 cache would sit between the L1 cache and the database. If a cache miss occurs in L1, the application would then check L2 before finally hitting the database. This would provide a shared cache for all application instances, further reducing database load and improving consistency across a cluster.
