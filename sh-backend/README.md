# Smart Home Backend

A smart home themed backend project,
made with the intent to learn about the tech stack.

## Setup

### Prerequisites

#### sqlx-cli

Install sqlx-cli with the following command:

```bash
cargo install sqlx-cli
```

### Database

#### Init

Having sqlite installed on your machine,
create the database with the following command:

```bash
sqlite3 development.db < migrations/20250531214811_initial_schema.sql
```

#### Seed

TODO

#### Migrate

Migrations are run automatically in development mode when starting the server,  
but can be run manually with the following command:

```bash
sqlx migrate run
```

### Environment Variables

Copy the `sample.env` file and rename it to `.env`.

## Running

### Simple

If you just want to run the backend, and don't plan on changing anything,  
run the backend with the following command:

```bash
cargo run
```

### Watch

You can run in watch mode with cargo-watch.  
To install cargo-watch, run the following command:

```bash
cargo install cargo-watch
```

Run the backend in watch mode with the following command:

```bash
cargo watch -x run
```
