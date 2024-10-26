# Infraestructura

Este directorio contiene los scripts y configuraciones de infraestructura necesarios para ejecutar los servicios en contenedores y desplegarlos en Kubernetes y en la nube.

## Contenido

- **Dockerfiles**: Archivos para construir las imágenes de Docker para cada microservicio (API Gateway, Auth Service, y User Service).
- **Kubernetes**: Configuraciones de despliegue de Kubernetes para cada servicio.
- **Terraform**: Scripts de Terraform para crear la infraestructura en la nube, incluyendo redes, bases de datos y un clúster de Kubernetes.

## Uso

### Construcción de Imágenes de Docker

Para construir las imágenes de Docker, usa los siguientes comandos:

```bash
docker build -t api-gateway -f Dockerfile.api-gateway .
docker build -t auth-service -f Dockerfile.auth-service .
docker build -t user-service -f Dockerfile.user-service .

