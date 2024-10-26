-- 003_add_policies_column.sql
ALTER TABLE roles
ADD COLUMN policies JSONB DEFAULT '{}'::JSONB;

-- Añadir un índice para búsquedas eficientes de políticas
CREATE INDEX idx_roles_policies ON roles USING GIN (policies);

