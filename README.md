# Rust Axum Clean Architecture - REST API Boilerplate

A production-ready REST API starter template built with **Rust**, **Axum**, **SQLx**, and **PostgreSQL** following **Clean Architecture** (Layered & Hexagonal Architecture) principles. Designed for developers looking for a scalable, maintainable, and testable Rust backend structure with session authentication, dependency inversion, and async best practices.

## Key Features

- **Clean Architecture in Rust**: Strict separation of concerns across Domain, Application, Infrastructure, and Interface layers.
- **Dependency Inversion Principle (DIP)**: Business logic depends on abstract repository traits rather than concrete database implementations.
- **Async PostgreSQL with SQLx**: Connection pooling, raw SQL file embedding at compile time, and automated database migrations.
- **Session-Based Authentication**: Built-in user login, logout, and protected routes using Tower Sessions and task-local request context.
- **CPU-Bound Concurrency Control**: Password hashing and verification using Bcrypt offloaded to Tokio blocking threads and bounded with a Semaphore to prevent async runtime starvation.
- **Structured Observability**: Request duration middleware and structured logging powered by Tracing.

## Architecture Overview

This project organizes code into four distinct layers where dependencies point strictly inward:

```text
src/
├── domain/           # Enterprise business rules (Entities & Repository Traits)
├── application/      # Application business rules (Services, Use Cases & DTOs)
├── infrastructure/   # External frameworks & drivers (PostgreSQL, SQLx & Migrations)
└── interface/        # Delivery mechanisms (Axum Routers, HTTP Handlers & Middleware)
```

1. **Domain Layer**: Contains core entities (User, TollRoads) and repository trait contracts. Completely independent of web frameworks and database drivers.
2. **Application Layer**: Orchestrates domain objects to perform use cases (AuthService, TollRoadService) and transforms data via Data Transfer Objects (DTOs).
3. **Infrastructure Layer**: Implements domain repository traits using SQLx and PostgreSQL, manages database connection pools, and runs SQL migrations.
4. **Interface Layer**: Handles HTTP requests and responses with Axum, manages application state, separates public and private routes, and applies middleware layers.

## Tech Stack

- **Language**: Rust (Edition 2024)
- **Web Framework**: Axum 0.8
- **Async Runtime**: Tokio
- **Database & Migrations**: PostgreSQL & SQLx 0.9
- **Authentication & Security**: Tower Sessions & Bcrypt
- **Serialization**: Serde
- **Error Handling**: Thiserror
- **Logging & Diagnostics**: Tracing & Tracing Subscriber

## Getting Started

### Prerequisites

- Rust and Cargo installed
- A running PostgreSQL instance

### Installation & Setup

1. Clone the repository:

```bash
git clone https://github.com/your-username/rust-axum-clean-architecture.git
cd rust-axum-clean-architecture
```

2. Create your environment file from the example template:

```bash
cp .env.example .env
```

3. Configure your PostgreSQL database URL and server address inside the .env file:

```env
DATABASE_URL=postgres://postgres:postgres@localhost:5432/test_db
SERVER_ADDR=localhost:9090
```

4. Build and run the server (database migrations run automatically on startup):

```bash
cargo run
```

## API Endpoints

### Public Routes
- **GET /test** - Health check endpoint to verify server status
- **POST /auth/login** - Authenticate user with email and password and establish a session

### Protected Routes (Requires Active Session)
- **GET /auth/me** - Retrieve profile information of the currently authenticated user
- **POST /auth/logout** - Invalidate and delete the active user session
- **GET /toll-roads** - Retrieve the list of toll roads

## Highlights & Rust Backend Patterns

- **Non-Blocking Password Verification**: Hashing algorithms like Bcrypt are intentionally CPU-intensive. To avoid blocking Tokio worker threads during high traffic, password verification is executed inside Tokio spawn_blocking and throttled with a Tokio Semaphore.
- **Externalized SQL Queries**: SQL statements are stored in dedicated .sql files and loaded at compile time via the include_str macro, keeping Rust repository code clean and easy to maintain.
- **Zero-Leakage Error Handling**: Domain and application errors are defined independently and mapped to HTTP status codes and JSON payloads exclusively inside the interface layer.

## License

This project is open-source and available under the MIT License.
