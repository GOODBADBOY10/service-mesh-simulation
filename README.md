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
# 🧑‍💻 User Service

A protected microservice for managing user profiles, built with Rust and Axum. Part of a service mesh simulation project demonstrating inter-service communication and token-based authentication.

## 📋 Table of Contents

- [Overview](#overview)
- [Features](#features)
- [Tech Stack](#tech-stack)
- [Architecture](#architecture)
- [Project Structure](#project-structure)
- [Getting Started](#getting-started)
- [API Endpoints](#api-endpoints)
- [Authentication](#authentication)
- [Usage Examples](#usage-examples)
- [Error Handling](#error-handling)
- [Inter-Service Communication](#inter-service-communication)
- [Configuration](#configuration)

## Overview

The User Service is responsible for managing user profile data. Unlike the Auth Service which handles credentials and tokens, this service focuses on storing and retrieving user information like names, bios, and other profile details.

**Key Responsibility:** All endpoints are protected and require a valid JWT token from the Auth Service.

### How It Works

```
Client Request                 User Service                 Auth Service
     │                              │                            │
     │  GET /users/123              │                            │
     │  Authorization: Bearer xxx   │                            │
     │ ────────────────────────►    │                            │
     │                              │  POST /validate            │
     │                              │  {token: "xxx"}            │
     │                              │ ──────────────────────►    │
     │                              │                            │
     │                              │  {valid: true, user_id}    │
     │                              │ ◄──────────────────────    │
     │                              │                            │
     │  {user profile data}         │                            │
     │ ◄────────────────────────    │                            │
```

## Features

- ✅ CRUD operations for user profiles
- ✅ JWT token validation via Auth Service
- ✅ Authorization header parsing
- ✅ Inter-service communication with Reqwest
- ✅ Ownership validation (users can only modify their own data)
- ✅ Partial updates support
- ✅ Comprehensive error handling
- ✅ Health check endpoint

## Tech Stack

| Technology | Purpose |
|------------|---------|
| **Rust** | Programming language |
| **Axum** | Web framework |
| **Tokio** | Async runtime |
| **Reqwest** | HTTP client for inter-service calls |
| **Serde** | Serialization/deserialization |
| **chrono** | Timestamp generation |
| **thiserror** | Error type definitions |
| **anyhow** | Error handling in main |

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                      USER SERVICE                            │
│                    (localhost:3001)                          │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  ┌──────────────────────────────────────────────────────┐   │
│  │                    Handlers                           │   │
│  │  ┌─────────┐ ┌─────────┐ ┌────────┐ ┌────────────┐   │   │
│  │  │GET user │ │GET users│ │ CREATE │ │UPDATE/DELETE│   │   │
│  │  └────┬────┘ └────┬────┘ └───┬────┘ └─────┬──────┘   │   │
│  └───────┼───────────┼──────────┼────────────┼──────────┘   │
│          │           │          │            │               │
│          ▼           ▼          ▼            ▼               │
│  ┌──────────────────────────────────────────────────────┐   │
│  │                  Auth Module                          │   │
│  │         (Token extraction & validation)               │   │
│  └───────────────────────┬──────────────────────────────┘   │
│                          │                                   │
│                          │ HTTP Request (Reqwest)            │
│                          ▼                                   │
│  ┌──────────────────────────────────────────────────────┐   │
│  │              Auth Service (localhost:3000)            │   │
│  │                    POST /validate                     │   │
│  └──────────────────────────────────────────────────────┘   │
│                                                              │
│  ┌──────────────────────────────────────────────────────┐   │
│  │                    AppState                           │   │
│  │    profiles: Mutex<HashMap<String, UserProfile>>      │   │
│  │    http_client: reqwest::Client                       │   │
│  │    auth_service_url: String                           │   │
│  └──────────────────────────────────────────────────────┘   │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

## Project Structure

```
user-service/
├── src/
│   ├── main.rs           # Application entry point
│   ├── route.rs          # Route definitions
│   ├── state.rs          # Shared application state
│   ├── error.rs          # Custom error types
│   ├── models.rs         # Data structures
│   ├── auth.rs           # Token validation logic
│   ├── health_check.rs   # Health check handler
│   └── handlers/
│       ├── mod.rs        # Handler module exports
│       ├── get_user.rs   # GET /users/:id
│       ├── get_users.rs  # GET /users
│       ├── create_user.rs# POST /users
│       ├── update_user.rs# PUT /users/:id
│       └── delete_user.rs# DELETE /users/:id
├── Cargo.toml
└── README.md
```

## Getting Started

### Prerequisites

- Rust (1.70 or higher)
- Cargo
- Auth Service running on port 3000

### Installation

1. Clone the repository:
```bash
git clone <repository-url>
cd user-service
```

2. Build the project:
```bash
cargo build
```

3. **Start the Auth Service first** (required):
```bash
cd ../auth-service
cargo run
# Running on http://localhost:3000
```

4. Start the User Service:
```bash
cd ../user-service
cargo run
# Running on http://localhost:3001
```

### Dependencies

```toml
[dependencies]
axum = "0.7"
tokio = { version = "1", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
reqwest = { version = "0.11", features = ["json"] }
thiserror = "1.0"
anyhow = "1.0"
chrono = "0.4"
```

## API Endpoints

| Method | Endpoint | Description | Auth Required |
|--------|----------|-------------|---------------|
| `GET` | `/` | Health check | ❌ |
| `GET` | `/users` | Get all user profiles | ✅ |
| `POST` | `/users` | Create a new profile | ✅ |
| `GET` | `/users/:id` | Get profile by ID | ✅ |
| `PUT` | `/users/:id` | Update profile | ✅ (own only) |
| `DELETE` | `/users/:id` | Delete profile | ✅ (own only) |

## Authentication

All protected endpoints require a valid JWT token in the Authorization header:

```
Authorization: Bearer <your-jwt-token>
```

### Getting a Token

1. Register with Auth Service:
```bash
curl -X POST http://localhost:3000/register \
  -H "Content-Type: application/json" \
  -d '{"username": "alice", "email": "alice@test.com", "password": "secret123"}'
```

2. Login to get token:
```bash
curl -X POST http://localhost:3000/login \
  -H "Content-Type: application/json" \
  -d '{"username": "alice", "password": "secret123"}'
```

3. Use the returned token in User Service requests.

### Token Validation Flow

1. User Service extracts token from `Authorization` header
2. Sends token to Auth Service's `/validate` endpoint
3. Auth Service verifies signature and expiration
4. Returns user info if valid
5. User Service proceeds with the request or returns 401

## Usage Examples

### Health Check

```bash
curl http://localhost:3001/
```

**Response:**
```json
{
  "status": "ok",
  "service": "user-service",
  "message": "User service is running"
}
```

### Create User Profile

```bash
curl -X POST http://localhost:3001/users \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer <your-token>" \
  -d '{
    "user_id": "550e8400-e29b-41d4-a716-446655440000",
    "username": "alice",
    "email": "alice@example.com",
    "full_name": "Alice Smith",
    "bio": "Software developer passionate about Rust"
  }'
```

**Success Response (200 OK):**
```json
{
  "user_id": "550e8400-e29b-41d4-a716-446655440000",
  "message": "Profile created successfully"
}
```

### Get User Profile

```bash
curl http://localhost:3001/users/550e8400-e29b-41d4-a716-446655440000 \
  -H "Authorization: Bearer <your-token>"
```

**Success Response (200 OK):**
```json
{
  "user_id": "550e8400-e29b-41d4-a716-446655440000",
  "username": "alice",
  "email": "alice@example.com",
  "full_name": "Alice Smith",
  "bio": "Software developer passionate about Rust",
  "created_at": "2024-01-15T10:30:00Z"
}
```

### Get All Users

```bash
curl http://localhost:3001/users \
  -H "Authorization: Bearer <your-token>"
```

**Success Response (200 OK):**
```json
[
  {
    "user_id": "550e8400-e29b-41d4-a716-446655440000",
    "username": "alice",
    "email": "alice@example.com",
    "full_name": "Alice Smith",
    "bio": "Software developer",
    "created_at": "2024-01-15T10:30:00Z"
  },
  {
    "user_id": "660e8400-e29b-41d4-a716-446655440001",
    "username": "bob",
    "email": "bob@example.com",
    "full_name": "Bob Johnson",
    "bio": "DevOps engineer",
    "created_at": "2024-01-16T14:20:00Z"
  }
]
```

### Update User Profile (Partial Update)

```bash
curl -X PUT http://localhost:3001/users/550e8400-e29b-41d4-a716-446655440000 \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer <your-token>" \
  -d '{
    "bio": "Senior software developer specializing in microservices"
  }'
```

**Success Response (200 OK):**
```json
{
  "user_id": "550e8400-e29b-41d4-a716-446655440000",
  "message": "Profile updated successfully"
}
```

### Delete User Profile

```bash
curl -X DELETE http://localhost:3001/users/550e8400-e29b-41d4-a716-446655440000 \
  -H "Authorization: Bearer <your-token>"
```

**Success Response (200 OK):**
```json
{
  "user_id": "550e8400-e29b-41d4-a716-446655440000",
  "message": "Profile deleted successfully"
}
```

## Error Handling

### Error Types

| Error | Status Code | Description |
|-------|-------------|-------------|
| `MissingAuthHeader` | 401 | No Authorization header provided |
| `InvalidAuthHeader` | 401 | Header format is wrong (not "Bearer xxx") |
| `InvalidToken` | 401 | Token is invalid or expired |
| `UserNotFound` | 404 | Requested profile doesn't exist |
| `UserAlreadyExists` | 409 | Profile with this ID already exists |
| `Forbidden` | 403 | User trying to modify another user's data |
| `AuthServiceUnavailable` | 503 | Cannot reach Auth Service |
| `InternalError` | 500 | Unexpected server error |

### Error Response Format

All errors return a consistent JSON structure:
```json
{
  "status": "401",
  "message": "Invalid or expired token"
}
```

## Inter-Service Communication

### How User Service Calls Auth Service

```rust
// 1. Extract token from header
let token = extract_token(&headers)?;

// 2. Send validation request to Auth Service
let response = http_client
    .post("http://localhost:3000/validate")
    .json(&ValidateTokenRequest { token })
    .send()
    .await?;

// 3. Parse response
let validation: ValidateTokenResponse = response.json().await?;

// 4. Check if valid
if !validation.valid {
    return Err(AppError::InvalidToken);
}
```

### Service Dependencies

```
┌─────────────────┐       ┌─────────────────┐
│  User Service   │ ────► │  Auth Service   │
│  (port 3001)    │       │  (port 3000)    │
└─────────────────┘       └─────────────────┘
        │
        │ Requires Auth Service to be running
        │ for token validation
        ▼
```

## Configuration

### Environment Variables (Recommended for Production)

| Variable | Description | Default |
|----------|-------------|---------|
| `PORT` | User Service port | 3001 |
| `AUTH_SERVICE_URL` | Auth Service base URL | http://localhost:3000 |

### Changing Auth Service URL

In `main.rs`:
```rust
let auth_service_url = "http://localhost:3000".to_string();
// Or use environment variable:
// let auth_service_url = std::env::var("AUTH_SERVICE_URL")
//     .unwrap_or_else(|_| "http://localhost:3000".to_string());
```

## Data Models

### UserProfile
```rust
struct UserProfile {
    user_id: String,      // Unique identifier
    username: String,     // Username
    email: String,        // Email address
    full_name: String,    // Display name
    bio: String,          // User biography
    created_at: String,   // ISO 8601 timestamp
}
```

### CreateProfileRequest
```rust
struct CreateProfileRequest {
    user_id: String,
    username: String,
    email: String,
    full_name: String,
    bio: String,
}
```

### UpdateProfileRequest
```rust
struct UpdateProfileRequest {
    full_name: Option<String>,  // Optional - only update if provided
    bio: Option<String>,        // Optional - only update if provided
}
```

## Security Considerations

### Current Implementation
- Token validation on every request
- Ownership checks (users can only modify their own profiles)
- No sensitive data stored (passwords are in Auth Service)

### Production Recommendations
- Use HTTPS for all communications
- Store Auth Service URL in environment variables
- Add rate limiting
- Implement request logging
- Add circuit breaker pattern for Auth Service calls
- Consider caching validated tokens briefly


# 🌐 Gateway Service

An API Gateway microservice built with Rust and Axum. Acts as the single entry point for all client requests, routing them to appropriate internal services. Part of a service mesh simulation project.

## 📋 Table of Contents

- [Overview](#overview)
- [Features](#features)
- [Tech Stack](#tech-stack)
- [Architecture](#architecture)
- [Project Structure](#project-structure)
- [Getting Started](#getting-started)
- [API Routes](#api-routes)
- [Usage Examples](#usage-examples)
- [Error Handling](#error-handling)
- [Request Flow](#request-flow)
- [Configuration](#configuration)

## Overview

The Gateway Service is the **single entry point** for all external clients. Instead of clients communicating directly with internal services, they send all requests to the gateway, which then routes them to the appropriate service.

### Why Use a Gateway?

```
Without Gateway:                    With Gateway:
                                    
Client ──► Auth Service             Client ──► Gateway ──► Auth Service
Client ──► User Service                              └──► User Service
                                    
❌ Multiple endpoints               ✅ Single endpoint
❌ Services exposed                 ✅ Services hidden
❌ No central control               ✅ Central logging/auth
```

## Features

- ✅ Single entry point for all API requests
- ✅ Request routing to internal services
- ✅ Header forwarding (Authorization tokens)
- ✅ Response proxying with status code preservation
- ✅ Service health monitoring
- ✅ Centralized error handling
- ✅ Clean API namespace (`/api/*`)

## Tech Stack

| Technology | Purpose |
|------------|---------|
| **Rust** | Programming language |
| **Axum** | Web framework |
| **Tokio** | Async runtime |
| **Reqwest** | HTTP client for service communication |
| **Serde** | JSON handling |
| **thiserror** | Error definitions |
| **anyhow** | Error handling |

## Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                       GATEWAY SERVICE                            │
│                       (localhost:3002)                           │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│   External Requests                                              │
│         │                                                        │
│         ▼                                                        │
│   ┌──────────────────────────────────────────────────────────┐  │
│   │                      Router                               │  │
│   │                                                           │  │
│   │   /api/register ──┐      ┌── /api/users                  │  │
│   │   /api/login ─────┼──────┼── /api/users/:id              │  │
│   │   /api/validate ──┘      └── (CRUD operations)           │  │
│   └───────────┬────────────────────────┬─────────────────────┘  │
│               │                        │                         │
│               ▼                        ▼                         │
│   ┌───────────────────┐    ┌───────────────────┐                │
│   │   Auth Handlers   │    │   User Handlers   │                │
│   └─────────┬─────────┘    └─────────┬─────────┘                │
│             │                        │                           │
│             │    Reqwest (HTTP)      │                           │
│             ▼                        ▼                           │
│   ┌───────────────────┐    ┌───────────────────┐                │
│   │   Auth Service    │    │   User Service    │                │
│   │  localhost:3000   │    │  localhost:3001   │                │
│   └───────────────────┘    └───────────────────┘                │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

## Project Structure

```
gateway-service/
├── src/
│   ├── main.rs           # Application entry point
│   ├── route.rs          # Route definitions
│   ├── state.rs          # Shared application state
│   ├── error.rs          # Custom error types
│   ├── health_check.rs   # Health check handler
│   └── handlers/
│       ├── mod.rs        # Handler module exports
│       ├── auth.rs       # Auth service forwarding
│       └── users.rs      # User service forwarding
├── Cargo.toml
└── README.md
```

## Getting Started

### Prerequisites

- Rust (1.70 or higher)
- Cargo
- Auth Service running on port 3000
- User Service running on port 3001

### Installation

1. Clone the repository:
```bash
git clone <repository-url>
cd gateway-service
```

2. Build the project:
```bash
cargo build
```

3. **Start internal services first:**
```bash
# Terminal 1 - Auth Service
cd auth-service
cargo run
# Running on http://localhost:3000

# Terminal 2 - User Service
cd user-service
cargo run
# Running on http://localhost:3001
```

4. Start the Gateway Service:
```bash
# Terminal 3 - Gateway Service
cd gateway-service
cargo run
# Running on http://localhost:3002
```

### Dependencies

```toml
[dependencies]
axum = "0.7"
tokio = { version = "1", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
reqwest = { version = "0.11", features = ["json"] }
thiserror = "1.0"
anyhow = "1.0"
```

## API Routes

### Health Check

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/` | Gateway health check |

### Auth Routes (→ Auth Service)

| Method | Gateway Endpoint | Internal Route | Description |
|--------|------------------|----------------|-------------|
| `POST` | `/api/register` | `/register` | Register new user |
| `POST` | `/api/login` | `/login` | Login and get token |
| `POST` | `/api/validate` | `/validate` | Validate JWT token |

### User Routes (→ User Service)

| Method | Gateway Endpoint | Internal Route | Description |
|--------|------------------|----------------|-------------|
| `GET` | `/api/users` | `/users` | Get all users |
| `POST` | `/api/users` | `/users` | Create user profile |
| `GET` | `/api/users/:id` | `/users/:id` | Get user by ID |
| `PUT` | `/api/users/:id` | `/users/:id` | Update user profile |
| `DELETE` | `/api/users/:id` | `/users/:id` | Delete user profile |

## Usage Examples

### Health Check

```bash
curl http://localhost:3002/
```

**Response:**
```json
{
  "status": "ok",
  "service": "gateway-service",
  "message": "Gateway service is running",
  "routes": {
    "auth": {
      "register": "POST /api/register",
      "login": "POST /api/login",
      "validate": "POST /api/validate"
    },
    "users": {
      "get_all": "GET /api/users",
      "create": "POST /api/users",
      "get_one": "GET /api/users/:id",
      "update": "PUT /api/users/:id",
      "delete": "DELETE /api/users/:id"
    }
  }
}
```

### Register User

```bash
curl -X POST http://localhost:3002/api/register \
  -H "Content-Type: application/json" \
  -d '{
    "username": "alice",
    "email": "alice@example.com",
    "password": "securePassword123"
  }'
```

**Response:**
```json
{
  "user_id": "550e8400-e29b-41d4-a716-446655440000",
  "username": "alice",
  "message": "User registered successfully"
}
```

### Login

```bash
curl -X POST http://localhost:3002/api/login \
  -H "Content-Type: application/json" \
  -d '{
    "username": "alice",
    "password": "securePassword123"
  }'
```

**Response:**
```json
{
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "token_type": "Bearer",
  "expires_in": 86400
}
```

### Create User Profile

```bash
curl -X POST http://localhost:3002/api/users \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer <your-token>" \
  -d '{
    "user_id": "550e8400-e29b-41d4-a716-446655440000",
    "username": "alice",
    "email": "alice@example.com",
    "full_name": "Alice Smith",
    "bio": "Software developer"
  }'
```

### Get User Profile

```bash
curl http://localhost:3002/api/users/550e8400-e29b-41d4-a716-446655440000 \
  -H "Authorization: Bearer <your-token>"
```

### Update User Profile

```bash
curl -X PUT http://localhost:3002/api/users/550e8400-e29b-41d4-a716-446655440000 \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer <your-token>" \
  -d '{
    "bio": "Senior software developer"
  }'
```

### Delete User Profile

```bash
curl -X DELETE http://localhost:3002/api/users/550e8400-e29b-41d4-a716-446655440000 \
  -H "Authorization: Bearer <your-token>"
```

## Error Handling

### Gateway-Level Errors

| Error | Status Code | Description |
|-------|-------------|-------------|
| `AuthServiceUnavailable` | 503 | Cannot reach Auth Service |
| `UserServiceUnavailable` | 503 | Cannot reach User Service |
| `BadRequest` | 400 | Invalid request format |
| `InternalError` | 500 | Unexpected gateway error |

### Forwarded Errors

Errors from internal services are forwarded with their original status codes:

| Source | Status Code | Description |
|--------|-------------|-------------|
| Auth Service | 401 | Invalid credentials |
| Auth Service | 409 | User already exists |
| User Service | 401 | Invalid token |
| User Service | 403 | Forbidden (not owner) |
| User Service | 404 | User not found |

### Error Response Format

```json
{
  "status": "503",
  "message": "Auth service unavailable"
}
```

## Request Flow

### Complete Request Lifecycle

```
┌────────┐      ┌─────────┐      ┌──────────────┐      ┌──────────────┐
│ Client │ ──►  │ Gateway │ ──►  │ User Service │ ──►  │ Auth Service │
└────────┘      └─────────┘      └──────────────┘      └──────────────┘
    │               │                   │                     │
    │ GET /api/     │                   │                     │
    │ users/123     │                   │                     │
    │ + Auth Header │                   │                     │
    │ ─────────────►│                   │                     │
    │               │                   │                     │
    │               │ GET /users/123    │                     │
    │               │ + Auth Header     │                     │
    │               │ ─────────────────►│                     │
    │               │                   │                     │
    │               │                   │ POST /validate      │
    │               │                   │ {token}             │
    │               │                   │ ───────────────────►│
    │               │                   │                     │
    │               │                   │ {valid: true}       │
    │               │                   │ ◄───────────────────│
    │               │                   │                     │
    │               │ {user profile}    │                     │
    │               │ ◄─────────────────│                     │
    │               │                   │                     │
    │ {user profile}│                   │                     │
    │ ◄─────────────│                   │                     │
```

### Header Forwarding

The gateway automatically forwards these headers to internal services:

| Header | Purpose |
|--------|---------|
| `Authorization` | JWT token for authentication |
| `Content-Type` | Request body format |

## Configuration

### Environment Variables (Recommended for Production)

| Variable | Description | Default |
|----------|-------------|---------|
| `PORT` | Gateway port | 3002 |
| `AUTH_SERVICE_URL` | Auth Service URL | http://localhost:3000 |
| `USER_SERVICE_URL` | User Service URL | http://localhost:3001 |

### Changing Service URLs

In `main.rs`:
```rust
let auth_service_url = "http://localhost:3000".to_string();
let user_service_url = "http://localhost:3001".to_string();

// Or use environment variables:
// let auth_service_url = std::env::var("AUTH_SERVICE_URL")
//     .unwrap_or_else(|_| "http://localhost:3000".to_string());
```

## Service Mesh Overview

```
┌─────────────────────────────────────────────────────────────┐
│                     SERVICE MESH                             │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│   ┌─────────────────┐                                       │
│   │     Client      │                                       │
│   │ (Browser/Mobile)│                                       │
│   └────────┬────────┘                                       │
│            │                                                 │
│            ▼                                                 │
│   ┌─────────────────┐                                       │
│   │ Gateway Service │  ◄── Single Entry Point               │
│   │   (port 3002)   │                                       │
│   └────────┬────────┘                                       │
│            │                                                 │
│      ┌─────┴─────┐                                          │
│      ▼           ▼                                          │
│ ┌──────────┐ ┌──────────┐                                   │
│ │  Auth    │ │  User    │  ◄── Internal Services            │
│ │ Service  │ │ Service  │      (Not exposed externally)     │
│ │ (3000)   │ │ (3001)   │                                   │
│ └──────────┘ └────┬─────┘                                   │
│                   │                                          │
│                   ▼                                          │
│              ┌──────────┐                                    │
│              │  Auth    │  ◄── Token Validation              │
│              │ Service  │                                    │
│              └──────────┘                                    │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

### Port Assignments

| Service | Port | Exposed |
|---------|------|---------|
| Auth Service | 3000 | Internal only |
| User Service | 3001 | Internal only |
| Gateway Service | 3002 | **Public** |

## Production Considerations

### Security
- Use HTTPS for all communications
- Implement rate limiting
- Add request logging
- Consider API key authentication for gateway

### Reliability
- Add health checks for internal services
- Implement circuit breaker pattern
- Add retry logic for failed requests
- Consider request timeouts

### Monitoring
- Add request/response logging
- Track latency metrics
- Monitor service availability
- Set up alerting


## License

MIT License

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request