# To-Do List Application

## Overview
A modern, full-stack To-Do List application designed for simplicity and speed. The application features a high-performance Rust backend and a lightweight, interactive TypeScript frontend.

## Architecture
The project follows a decoupled client-server architecture:

- **Frontend (Web)**: Built with [Astro](https://astro.build/), leveraging "Islands Architecture" for optimized interactivity. The client-side UI is managed by [Preact](https://preactjs.com/) to provide a snappy, app-like experience.
- **Backend (Server)**: A high-performance REST API built with [Axum](https://github.com/tokio-rs/axum) in Rust.
- **Storage**: Persistent data is stored in a [SQLite](https://sqlite.org/) database.

## Tech Stack
- **Language (Backend)**: Rust
- **Web Framework (Backend)**: Axum
- **Language (Frontend)**: TypeScript
- **Web Framework (Frontend)**: Astro + Preact
- **Database**: SQLite
- **Communication**: REST (JSON over HTTP)

## Project Structure
- `server/`: Contains the Rust Axum backend and SQLite management.
- `web/`: Contains the Astro frontend and TypeScript client.
- `.scratch/`: Stores design documentation, ADRs, and issue tracking.
