
# Chiave

**Chiave** is an open-source identity and access management (IAM) solution written in Rust. Inspired by Keycloak, it offers robust authentication and authorization, user management, roles, and permissions, with support for OAuth2, OpenID Connect, and JWT.

## Key Features

- **Authentication**:
  - Support for **JWT**, **OAuth2**, and **OpenID Connect**.
  - Session management and token renewal.
  - Multi-factor Authentication (MFA).

- **Authorization**:
  - **Role-based access control (RBAC)**: Flexible management of roles and permissions.
  - **Attribute-based access control (ABAC)**: Authorization based on user attributes.
  - Permission and resource access auditing.

- **User Management**:
  - Create, edit, and delete users.
  - Password recovery and reset.
  - Email verification and profile management.

- **Admin Portal**:
  - GUI for managing users, roles, and access policies.
  - Log and audit visualization.
  - Security policy configuration.

- **User Portal**:
  - User registration and login.
  - Password recovery.
  - Personal profile management.

## Project Structure

Chiave follows a microservices architecture, with each component implemented as a standalone service communicating through the **API Gateway**.

```
/chiave/
├── api-gateway/             # Routing requests to microservices
├── auth-service/            # Authentication service
├── user-service/            # User management service
├── role-service/            # Role and permissions management service
├── config-service/          # Global configuration service
├── database-migrations/     # SQL scripts for database migration
├── infra/                   # Infrastructure files (Docker, Kubernetes, Terraform)
├── tests/                   # Unit and integration tests
├── web/                     # Web portals (Admin and User Portals)
└── docs/                    # Technical documentation
```

### Key Services

- **API Gateway**: Central point exposing routes for authentication, users, and roles.
- **Auth Service**: Responsible for authentication, token generation, and session management.
- **User Service**: Service for user CRUD.
- **Role Service**: Management of roles and permissions.
- **Config Service**: Global configuration for the application.

## System Requirements

- **Rust** (minimum version: 1.60)
- **PostgreSQL** or **MySQL** for database
- **Redis** or **Memcached** for caching
- **Docker** and **Kubernetes** (for deployment and orchestration)
- **Node.js** and **npm/yarn** (for frontend)

## Installation and Setup

### 1. Clone the Repository

```bash
git clone https://github.com/your-username/chiave.git
cd chiave
```

### 2. Compile and Run Microservices

Chiave uses **Cargo** to compile each service. Make sure Rust is installed and follow the steps for each microservice:

```bash
# Compile and run API Gateway
cd api-gateway
cargo run

# Compile and run Auth Service
cd auth-service
cargo run

# Compile and run User Service
cd user-service
cargo run

# Repeat for other services...
```

### 3. Database Setup

Chiave supports **PostgreSQL** and **MySQL**. Run the database migrations:

```bash
# Ensure your database is running
psql -U postgres -d chiave_db < database-migrations/001_create_users_table.sql
```

### 4. Run with Docker

You can also use Docker to run Chiave services:

```bash
# Build Docker image
docker-compose up --build
```

### 5. Frontend Setup

```bash
# Navigate to admin or user portal
cd web/admin-portal
npm install
npm run start
```

### 6. Deploy with Kubernetes

If you want to deploy Chiave on Kubernetes, use the deployment files in `infra/kubernetes/`.

```bash
kubectl apply -f infra/kubernetes/api-gateway-deployment.yaml
kubectl apply -f infra/kubernetes/auth-service-deployment.yaml
```

## Contributing

Contributions are welcome. If you'd like to contribute to **Chiave**, please follow the guidelines in [CONTRIBUTING.md](docs/CONTRIBUTING.md).

1. Fork the project.
2. Create a new branch (`git checkout -b feature/new-feature`).
3. Make necessary changes.
4. Submit a Pull Request.

## License

Chiave is open-source and distributed under the [MIT License](LICENSE).

## Contact

If you have questions or need support, please open an issue or reach out via [your-email@domain.com](mailto:your-email@domain.com).
