# LoanSasa

LoanSasa is a Rust-based web application that facilitates peer-to-peer lending, enabling users to offer and receive loans seamlessly. The application leverages modern Rust frameworks and libraries like Actix-web, Diesel, and Redis to provide a secure, high-performance platform for managing loans.

## Project Structure

The repository is organized into the following directories:

- **`contracts/`**: Contains smart contracts written in Cairo for use with StarkNet.
- **`web/`**: The web application component, including frontend and backend code.

## Getting Started

### Prerequisites

Ensure you have the following installed:
- Rust and Cargo
- Diesel CLI
- Redis
- PostgreSQL (or MySQL, depending on your configuration)

### Setup and Installation

1. **Clone the Repository**

    ```sh
    git clone <repository-url>
    cd LoanSasa
    ```

2. **Set Up the Database**

    Configure your database settings in the `.env` file or your preferred configuration file. Run the database migrations:

    ```sh
    cd web
    diesel migration run
    ```

3. **Run the Application**

    To start the web server:

    ```sh
    cd web
    cargo run
    ```


4. **Interacting with the Application**

    Open your browser and navigate to `http://localhost:8080` to access the web interface.

## Testing

To run the tests for the web application, use:

```sh
cd web
cargo test
```

## Contributing
Contributions are welcome! Please fork the repository and submit a pull request with your changes.