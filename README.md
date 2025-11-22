# 🔐 Auth Service

A secure authentication microservice built with Rust and Axum. Part of a service mesh simulation project demonstrating inter-service communication, token-based authentication, and proper error handling.

## 📋 Table of Contents

- [Overview](#overview)
- [Features](#features)
- [Tech Stack](#tech-stack)
- [Project Structure](#project-structure)
- [Getting Started](#getting-started)
- [API Endpoints](#api-endpoints)
- [Usage Examples](#usage-examples)
- [Error Handling](#error-handling)
- [Security](#security)
- [Configuration](#configuration)

## Overview

The Auth Service is responsible for:
- User registration and credential storage
- User authentication and JWT token generation
- Token validation for other microservices

This service acts as the central authentication authority in a microservices architecture, allowing other services to verify user identity by validating JWT tokens.

## Features

- ✅ User registration with email validation
- ✅ Secure password hashing with bcrypt
- ✅ JWT token generation and validation
- ✅ RESTful API design
- ✅ Comprehensive error handling with custom error types
- ✅ Thread-safe in-memory storage
- ✅ Health check endpoint

## Tech Stack

| Technology | Purpose |
|------------|---------|
| **Rust** | Programming language |
| **Axum** | Web framework |
| **Tokio** | Async runtime |
| **Serde** | Serialization/deserialization |
| **bcrypt** | Password hashing |
| **jsonwebtoken** | JWT token handling |
| **uuid** | Unique ID generation |
| **chrono** | Date/time handling |
| **thiserror** | Error type definitions |
| **anyhow** | Error handling in main |

## Project Structure

```
auth-service/
├── src/
│   ├── main.rs          # Application entry point
│   ├── route.rs         # Route definitions and state initialization
│   ├── state.rs         # Shared application state
│   ├── error.rs         # Custom error types and responses
│   ├── register.rs      # User registration handler
│   ├── login.rs         # Login and JWT generation handler
│   ├── validate.rs      # Token validation handler
│   └── health_check.rs  # Health check handler
├── Cargo.toml           # Dependencies
└── README.md
```

## Getting Started

### Prerequisites

- Rust (1.70 or higher)
- Cargo

### Installation

1. Clone the repository:
```bash
git clone <repository-url>
cd auth-service
```

2. Build the project:
```bash
cargo build
```

3. Run the service:
```bash
cargo run
```

The service will start on `http://localhost:3000`.

### Dependencies

Add these to your `Cargo.toml`:

```toml
[dependencies]
axum = "0.7"
tokio = { version = "1", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
bcrypt = "0.15"
jsonwebtoken = "9"
uuid = { version = "1.0", features = ["v4", "serde"] }
chrono = "0.4"
thiserror = "1.0"
anyhow = "1.0"
```

## API Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/` | Health check |
| `POST` | `/register` | Register a new user |
| `POST` | `/login` | Authenticate and get JWT token |
| `POST` | `/validate` | Validate a JWT token |

## Usage Examples

### Health Check

```bash
curl http://localhost:3000/
```

**Response:**
```json
{
  "status": "ok",
  "message": "API is running",
  "endpoints": {
    "health": "/",
    "register": "/register",
    "login": "/login",
    "validate": "/validate"
  }
}
```

### Register a New User

```bash
curl -X POST http://localhost:3000/register \
  -H "Content-Type: application/json" \
  -d '{
    "username": "john_doe",
    "email": "john@example.com",
    "password": "securePassword123"
  }'
```

**Success Response (200 OK):**
```json
{
  "user_id": "550e8400-e29b-41d4-a716-446655440000",
  "username": "john_doe",
  "message": "User registered successfully"
}
```

**Error Response (409 Conflict):**
```json
{
  "status": "409",
  "message": "Username already exists"
}
```

### Login

```bash
curl -X POST http://localhost:3000/login \
  -H "Content-Type: application/json" \
  -d '{
    "username": "john_doe",
    "password": "securePassword123"
  }'
```

**Success Response (200 OK):**
```json
{
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "token_type": "Bearer",
  "expires_in": 86400
}
```

**Error Response (401 Unauthorized):**
```json
{
  "status": "401",
  "message": "Invalid username or password"
}
```

### Validate Token

```bash
curl -X POST http://localhost:3000/validate \
  -H "Content-Type: application/json" \
  -d '{
    "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
  }'
```

**Success Response (200 OK):**
```json
{
  "valid": true,
  "user_id": "550e8400-e29b-41d4-a716-446655440000",
  "username": "john_doe"
}
```

**Error Response (401 Unauthorized):**
```json
{
  "status": "401",
  "message": "Invalid or malformed token"
}
```

## Error Handling

The service uses a custom `AppError` enum for consistent error responses:

| Error | Status Code | Message |
|-------|-------------|---------|
| `UserAlreadyExists` | 409 | Username already exists |
| `EmailAlreadyExists` | 409 | Email already registered |
| `InvalidCredentials` | 401 | Invalid username or password |
| `InvalidToken` | 401 | Invalid or malformed token |
| `TokenExpired` | 401 | Token has expired |
| `PasswordHashError` | 500 | Failed to hash password |
| `TokenGenerationError` | 500 | Failed to generate token |
| `InternalError` | 500 | Internal server error |

All errors return a consistent JSON structure:
```json
{
  "status": "<HTTP_STATUS_CODE>",
  "message": "<ERROR_MESSAGE>"
}
```

## Security

### Password Storage
- Passwords are **never** stored in plain text
- bcrypt hashing with default cost factor
- Each password has a unique salt

### JWT Tokens
- Tokens are signed using HS256 algorithm
- Default expiration: 24 hours
- Contains user ID and username in claims

### Security Recommendations for Production
- Store `JWT_SECRET` in environment variables
- Use HTTPS in production
- Implement rate limiting
- Add request logging and monitoring
- Consider token refresh mechanism
- Implement token blacklisting for logout

## Configuration

### Environment Variables (Recommended for Production)

| Variable | Description | Default |
|----------|-------------|---------|
| `PORT` | Server port | 3000 |
| `JWT_SECRET` | Secret key for signing tokens | Hardcoded (change in production!) |
| `TOKEN_EXPIRY` | Token validity in hours | 24 |

### Changing the JWT Secret

In `login.rs`, update the constant:
```rust
pub const JWT_SECRET: &[u8] = b"your-super-secret-key-here";
```

⚠️ **Warning:** In production, always use environment variables for secrets!

## Architecture

```
┌─────────────────────────────────────────────────────────┐
│                     AUTH SERVICE                         │
├─────────────────────────────────────────────────────────┤
│                                                          │
│   Client Request                                         │
│         │                                                │
│         ▼                                                │
│   ┌──────────┐     ┌──────────┐     ┌──────────┐       │
│   │ Register │     │  Login   │     │ Validate │       │
│   └────┬─────┘     └────┬─────┘     └────┬─────┘       │
│        │                │                 │             │
│        ▼                ▼                 ▼             │
│   ┌─────────────────────────────────────────────┐      │
│   │              AppState                        │      │
│   │     users: Mutex<HashMap<String, User>>      │      │
│   └─────────────────────────────────────────────┘      │
│                                                          │
└─────────────────────────────────────────────────────────┘
```

## Related Services

This auth-service is part of a larger service mesh simulation:

- **auth-service** (this service) - Authentication and token management
- **user-service** - User data management
- **gateway-service** - API gateway and request routing

## License

MIT License

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request
