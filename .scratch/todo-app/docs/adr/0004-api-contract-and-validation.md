# ADR 0004: API Contract and Validation

## Status
Accepted

## Context
To ensure robust communication between the Rust backend and the TypeScript frontend, we need to establish how data is validated and how the API contract is maintained.

## Decision
We will use a type-safe approach for both sides of the communication:
- **Backend**: [Serde](https://serde.rs/) will be used in Rust for JSON serialization and deserialization.
- **Frontend**: [Zod](https://zod.dev/) will be used in TypeScript for runtime validation of API responses and form inputs.
- **Contract**: The API will be documented as a RESTful service. While we will start with manual documentation, we will structure the code to easily support [OpenAPI](https://swagger.io/specification/) generation in the future.

## Consequences
- **Pros**:
    - Prevents malformed data from entering the system.
    - Provides better developer experience with improved type safety.
    - Reduces bugs related to unexpected API response formats.
- **Cons**:
    - Requires maintaining validation logic on both the client and server.
