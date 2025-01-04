# Rust_AI

## Sources

- [Primary Main source](https://github.com/bitswired/rustgpt)
- [Present fork](https://github.com/StripyRaven/rustgpt)

## Materials

### RUST

- [A large cheat sheet on Rust 1/2.](https://habr.com/ru/companies/timeweb/articles/785096/)
- [A large cheat sheet on Rust 2/2.](https://habr.com/ru/companies/timeweb/articles/787924/)
- [Try the RustGPT hosted demo example](https://rustgpt.bitswired.com)
- [Read the useful blog article](https://www.bitswired.com/en/blog/post/rustgpt-journey-rust-htmx-web-dev)
- [Some useful](https://habr.com/ru/articles/714980/)
- [By example](https://doc.rust-lang.ru/stable/rust-by-example/index.html)

#### Errors handling

##### Errors Code

- [RUST erroe code index](https://doc.rust-lang.org/error_codes/error-index.html)

```sh:example
rustc --explain E0277
```

- [Tracing subscriber](https://www.shuttle.dev/blog/2024/01/09/getting-started-tracing-rust)

### Sqlx

- [SQLite docs](https://www.sqlite.org/docs.html)
- [Sqlx-cli](https://github.com/launchbadge/sqlx/blob/main/sqlx-cli/README.md)
- [Sqlx quary_as!](https://docs.rs/sqlx/latest/sqlx/macro.query_as.html)
- [Sqlite datatypes](https://docs.rs/sqlx/latest/sqlx/sqlite/types/index.html)
- [Useful habr](https://habr.com/ru/companies/otus/articles/771288/)

### Axum

- [Axum doc](https://crates.io/crates/axum)
- - [last](https://crates.io/crates/axum/0.7.9)
- [Router](https://docs.rs/axum/latest/axum/struct.Router.html)
- [Eamples](https://github.com/tokio-rs/axum/tree/main/examples)
- [Showcases](https://github.com/tokio-rs/axum/blob/main/ECOSYSTEM.md#project-showcase)
- [Tutorials](https://github.com/tokio-rs/axum/blob/main/ECOSYSTEM.md#tutorials)
- [Community projects](https://github.com/tokio-rs/axum/blob/main/ECOSYSTEM.md)

****

- [more example](https://codevoweb.com/rust-crud-api-example-with-axum-and-postgresql/)

### Acc & Mutex

- [Arc and Mutex in Rust](https://itsallaboutthebit.com/arc-mutex/)

### HTTP

- [status code registry](https://www.iana.org/assignments/http-status-codes/http-status-codes.xhtml)

### tailwindcss

- [Configuration](https://tailwindcss.com/docs/configuration)
- [Tailwindcss play](https://play.tailwindcss.com/)

## Introduction

Rust-AI is latest experiment in cloning the abilities of OpenAI's Chat-AI.
In this repository, a Rust-based server leveraging the Axum framework combined with HTMX, providing a Rusty web development experience. From database operations to streaming responses, this project covers a broad spectrum of backend functionalities and real-time web interactions.
So, for Rust enthusiasts and web developers alike, dive in to explore a world where web development is redefined with the power of Rust!

## Features

- **Rust with Axum Framework**: A fast and reliable server that's all about performance and simplicity.
- **SQLite**: A lightweight yet powerful database for all your data persistence needs.
- **Server Sent Events (SSE)**: Real-time streaming made easy to bring life to the ChatGPT interactions.
- **HTMX**: No hefty JavaScript frameworks needed—HTMX keeps interactions snappy with simple HTML attributes.
  - [HTMX common](https://habr.com/ru/companies/hexlet/articles/592961/)

## Tech Stack

- [`sqlx`](https://github.com/launchbadge/sqlx): Direct and type-safe SQL queries and migrations.
- [`tera`](https://github.com/Keats/tera): A templating engine inspired by Jinja2, for rendering the HTML views.
- [`axum`](https://github.com/tokio-rs/axum): A web application framework that's easy to use and incredibly fast.

For those eyeing some client-side WASM magic, you might also want to check out [Yew](https://github.com/yewstack/yew) or [Leptos](https://github.com/LeptosProject/leptos) for more complex applications.

## Quickstart

1. Clone the repository.
2. Create a .env

### .env

```env
MIGRATIONS_PATH=db/migrations
TEMPLATES_PATH=templates
DATABASE_URL=sqlite:db/db.db
DATABASE_PATH=db/db.db
OPENAI_API_KEY=<api-key> (only necessary for tests, users will add their own keys)
```

1. Install TailwindCSS Standalone in [this repository:](https://tailwindcss.com/blog/standalone-cli)
  1.1 [Brew](https://formulae.brew.sh/formula/tailwindcss)
2. `cargo install just` : install [Just](https://github.com/casey/just)
3. `just init`          : install additional tools and migrate the db
4. `just dev`           : concurrently run tailwind and cargo run in watch mode
5. Open your browser and enjoy chatting with your Rust-powered ChatGPT clone (port 3000 by default)

### Install tailwindcss

1. execute

```sh
title:terminal
# Example for macOS arm64
curl -sLO https://github.com/tailwindlabs/tailwindcss/releases/download/v3.4.17/tailwindcss-macos-x64
chmod +x tailwindcss-macos-x64
mv tailwindcss-macos-x64 tailwindcss
```

- rename executable to `tailwindcss`

## Contributin

Contributions are what make the open-source community an incredible place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

If you have a suggestion that would make RustGPT better, please fork the repo and create a pull request. You can also simply open an issue. Don't forget to give the project a star! Thank you again!

## Acknowledgments

Hats off to the wonderful crates and libraries that made Rust_AI possible!

****
