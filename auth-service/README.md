# Auth Service

Este módulo proporciona servicios de autenticación, incluyendo inicio de sesión, generación de tokens JWT, y soporte para OAuth2/OpenID Connect.

## Configuración

Crea un archivo `.env` con los siguientes valores:

```env
SERVER_ADDR=127.0.0.1:8081
JWT_SECRET=super_secret_key
OAUTH_CLIENT_ID=your_oauth_client_id
OAUTH_CLIENT_SECRET=your_oauth_client_secret
OAUTH_REDIRECT_URL=https://yourapp.com/oauth/callback

