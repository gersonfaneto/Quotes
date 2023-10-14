```markdown
  ___              _
 / _ \ _   _  ___ | |_ ___  ___         # Basic CRUD API written in Rust.
| | | | | | |/ _ \| __/ _ \/ __|                                          
| |_| | |_| | (_) | ||  __/\__ \        # by @gersonfaneto
 \__\_\\__,_|\___/ \__\___||___/
```

<div align="center">

[![Activity](https://img.shields.io/github/last-commit/gersonfaneto/Quotes?color=blue&style=for-the-badge&logo=git)](https://github.com/gersonfaneto/Quotes/commit/main)
[![License](https://img.shields.io/github/license/gersonfaneto/Quotes?color=blue&style=for-the-badge)](https://github.com/gersonfaneto/Quotes/blob/main/LICENSE)
[![Stars](https://img.shields.io/github/stars/gersonfaneto/Quotes?color=blue&style=for-the-badge&logo=github)](https://github.com/gersonfaneto/Quotes)
![Language](https://img.shields.io/static/v1?label=LANGUAGE&message=Rust&color=informational&style=for-the-badge)

</div>

> A simple RESTful API for managing book quotes built in Rust using the Axum Web Framework, Tokyo
> asynchronous runtime,and SQLx for PostgreSQL database connectivity. This project was built for
> learning purposes and showcases containerization with Docker.

<h4 align="center">
  <a href="#about">About</a>
  ·
  <a href="#build">Build</a>
  ·
  <a href="#usage">Usage</a>
  ·
  <a href="#license">License</a>
</h4>

## About

This project is a basic CRUD (Create, Read, Update, Delete) API written in Rust. It provides
functionality for managing book quotes using a RESTful API approach. The application is built
using:

- The Axum web framework.
- Tokyo as the asynchronous runtime.
- SQLx for PostgreSQL database connectivity.

The primary purpose of this project is to serve as a learning tool, demonstrating how to create
a simple yet functional API in Rust. It showcases essential concepts of asynchronous programming,
web development, and database integration in the Rust programming language.

The core objective of this project is to act as an educational resource, showcasing the creation
of a simple yet powerful API in Rust. It particularly emphasizes understanding fundamental
concepts of asynchronous programming, web development, and database integration within the Rust
ecosystem. Moreover, a significant focus is placed on demonstrating effective containerization
practices, highlighting the deployment and management aspects through Docker

#### Key Features:

- Create, Read, Update, and Delete book quotes via HTTP requests.
- Asynchronous handling of requests for improved performance and scalability.
- PostgreSQL database integration using SQLx for persistent storage.
- Containerized deployment using Docker for easy setup and management

## Build

Firstly, make sure you have Docker and Docker Compose installed and available in your environment.

- Clone this project into your machine using `git`.

```console
git clone https://github.com/gersonfaneto/Quotes --depth 1
```

- Move into the project folder.

```console
cd Quotes
```

- Then run the following Docker command and everything will be taken care 😄.

```console
docker compose up --build --detach
```

> 📝 **Note**  
> This step may take some time on the first run as the cache hasn't been built.

If everything went well the containers are now running and you can start using the API!

## Usage

After following the previous steps, the API will be available under `localhost:8080`, with the
following endpoints:

- **GET** _/_

  - Serves as a "health" check for the API, if everything is right it should return a HTTP
    status code of 200 (OK) and a string of "Hello, World!".

- **GET** _/quotes_

  - Returns all the registered quotes.

- **POST** _/quotes_

  - Creates a new quote in the DB. Returns a HTTP status code of 201 (CREATED) if the quote was
    created and the respectful JSON object for it, or a 500 (INTERNAL_SERVER_ERROR) if it wasn't.

- **PUT** _/quotes/:id_

  - Updates s quote in the DB given it's ID. Returns a HTTP status code of 200 (OK) if the quotes
    was found and updated, of either a 404 (NOT_FOUND) or a 500 (INTERNAL_SERVER_ERROR) if it wasn't.

- **DELETE** _/quotes/:id_

  - Deletes a quote from the DB given it's ID. Returns a HTTP status code of 200 (OK) if the quote
    was found and deleted, or either a 404 (NOT_FOUND) or a 500 (INTERNAL_SERVER_ERROR) if it wasn't.

When creating or updating a quote using the API endpoints described above, you should provide the
following JSON structure:

```json
{
  "book": "Les Misérables",
  "quote": "It is nothing to die; it is dreadful not to live."
}
```

> 📝 **Note**  
> Both the `book` and `quote` fields are required.

## License

Released under [MIT](https://github.com/gersonfaneto/Quotes/blob/main/LICENSE) by
[@gersonfaneto](https://github.com/gersonfaneto).
