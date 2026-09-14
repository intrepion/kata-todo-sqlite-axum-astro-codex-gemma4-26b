# ADR 0002: Astro Islands with Preact

## Status
Accepted

## Context
We need to decide how to handle client-side interactivity in an Astro-based frontend. We want a snappy, "app-like" experience for the To-Do list (adding/toggling items without full page reloads) while maintaining the performance benefits of Astro.

## Decision
We will use Astro's "Islands Architecture" and hydrate the main To-Do list component using [Preact](https://preactjs.com/).

## Consequences
- **Pros**:
    - Provides a high-performance, reactive UI for the core task list.
    - Minimizes the JavaScript payload compared to a full Single Page Application (SPA).
    - Leverages Astro's excellent developer experience and fast build times.
- **Cons**:
    - Requires managing component hydration and state between the server-rendered HTML and the client-side Preact components.
