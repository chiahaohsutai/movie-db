# Movie-DB

This project is a full-stack application for managing movie data. The backend, built with Rust and Actix-Web, provides a CRUD API and a search endpoint using a fuzzy matching engine. Data is stored in an in-memory SQLite database populated with movie data from Kaggle. The frontend, built with SolidJS, provides an interface for interacting with the API, supporting Create, Read, Update, Delete, and search operations.

# Installation

### Prerequisites

Ensure the following are installed on your system:
- **Rust**: Install Rust using [rustup](https://rustup.rs/).
- **Node.js**: Download from [nodejs.org](https://nodejs.org/) and install either the LTS or the latest version.
- **bunyan** (Optional): For viewing test logs in a readable format, you can install `bunyan` using Cargo after setting up Rust.

###  Backend Setup
1. **Clone the repository**:
```bash
git clone <repository-url>
cd movie-db
```

2. **Navigate to the backend directory**:
```bash
cd service
```

3. **Install Rust dependencies**:
```bash
cargo build
```

4. **Set up the database**:
- Use SQLite for in-memory storage (no additional setup required).
- If using a persistent database like PostgreSQL, configure the connection string in the `.env` file.

5. **Run the backend server**:
```bash
cargo run
```
The server will be available at `http://localhost:8000`.

### Frontend Setup
1. **Navigate to the frontend directory**:
```bash
cd client
```

2. **Install Node.js dependencies**:
```bash
yarn install
```

3. **Start the development server**:
```bash
yarn dev
```
The frontend will be accessible at `http://localhost:3000`.

# Tests

1. Navigate to the `service` directory:
```bash
cd service
```
2. Run the tests:
```bash
cargo test
```
3. To run the tests with logs enabled:
- Install `bunyan` for better log readability:
```bash
cargo install bunyan
```
- Run the tests with logs:
```bash
TEST_LOG=true cargo test | bunyan
```