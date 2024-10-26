# Database Migrations

Este directorio contiene los scripts de migración para la base de datos. Cada archivo SQL define una estructura o cambio específico en la base de datos. A continuación se describe cada migración y su propósito:

## Migraciones

- `001_create_users_table.sql`: Crea la tabla `users`, que almacena los datos de usuario, incluyendo nombre de usuario, correo electrónico y contraseña.
- `002_create_roles_table.sql`: Crea la tabla `roles` y la tabla de relación `user_roles`, permitiendo asignar múltiples roles a los usuarios.
- `003_add_policies_column.sql`: Añade una columna `policies` a la tabla `roles`, que permite definir permisos específicos para cada rol.

## Cómo Ejecutar las Migraciones

1. Asegúrate de que la base de datos esté configurada y accesible.
2. Usa una herramienta de migración (por ejemplo, `diesel` en Rust) o ejecuta manualmente cada archivo SQL en la base de datos en el orden correcto.
3. Verifica que las tablas y columnas se hayan creado o modificado correctamente.

## Consideraciones

- Las migraciones deben ejecutarse en el orden indicado para evitar errores de dependencia.
- Asegúrate de hacer un backup de la base de datos antes de realizar cambios significativos.

