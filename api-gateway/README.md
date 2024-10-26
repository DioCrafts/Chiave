# API Gateway

Este módulo actúa como punto central para gestionar la autenticación, autorización y enrutamiento hacia otros microservicios.

## Configuración

Crea un archivo `.env` con los siguientes valores:

```env
SERVER_ADDR=127.0.0.1:8080
AUTH_SERVICE_URL=http://auth-service:8081
USER_SERVICE_URL=http://user-service:8082
ROLE_SERVICE_URL=http://role-service:8083

