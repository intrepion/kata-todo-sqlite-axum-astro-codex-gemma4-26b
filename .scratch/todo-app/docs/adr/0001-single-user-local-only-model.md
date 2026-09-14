# ADR 0001: Single-User, Local-Only Model

## Status
Accepted

## Context
The initial goal is to build a lightweight, fast To-Do List application. We need to decide whether the system should support multiple users with authentication or if it should be a single-user tool.

## Decision
We will implement a single-user, local-only model. The application will not require user registration or authentication. The SQLite database will be managed locally on the host machine.

## Consequences
- **Pros**:
    - Significantly reduced complexity in both the backend (no JWT, session management, or user tables) and frontend.
    - Faster initial development and deployment.
    - Ideal for personal use and local productivity.
- **Cons**:
    - The application cannot be used by multiple people simultaneously with private data.
    - Adding multi-tenancy in the future will require a significant refactor of the database schema and API.
