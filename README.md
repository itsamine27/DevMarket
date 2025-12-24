DevMarket is a Rust‑based backend project designed to explore building a marketplace‑style application with modern Rust tooling.
The repository currently includes foundational modules for handling products, users, error management, and authorization logic. 
It’s still in its early stages, but the goal is to evolve it into a scalable and maintainable backend service.

Features:
Products module: basic structure for managing product data.

User module: initial setup for handling user accounts and authentication.

Error handling: centralized error management via error.rs.

Authorization: groundwork for request validation and access control.

Main entry point: main.rs sets up the application and integrates modules.

Project Structure:
Code 
```
src/
 ├── products/        # Product-related logic
 ├── user/            # User management and authentication
 ├── error.rs         # Error handling utilities
 ├── ext.rs           # Authorization and extensions
 ├── main.rs          # Application entry point
 └── test.rs          # Initial test setup
 ```
Getting Started: 
Clone the repository

bash
```
git clone https://github.com/itsamine27/DevMarket.git
cd DevMarket
 ```
Run the project
```
bash
cargo run
