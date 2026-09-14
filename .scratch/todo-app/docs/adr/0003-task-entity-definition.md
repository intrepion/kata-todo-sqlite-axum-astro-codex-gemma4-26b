# ADR 0003: Task Entity Definition

## Status
Accepted

## Context
A To-Do List application is only as useful as the information it can store about tasks. We need to define the schema for the core entity.

## Decision
A Task will be defined with the following properties:
- `id`: UUID (Primary Key)
- `title`: String (Required)
- `description`: String (Optional)
- `is_completed`: Boolean (Default: false)
- `priority`: Enum (Low, Medium, High)
- `due_date`: Timestamp (Optional)
- `created_at`: Timestamp (Default: current time)

## Consequences
- **Pros**:
    - Provides enough metadata for useful sorting, filtering, and organization.
    - Allows for future expansion (e.g., tags, subtasks) with minimal breaking changes.
- **Cons**:
    - Slightly more complex than a minimal title/status model.
