-- migrations/20251018000000_create_initial_schema.down.sql

-- Hapus tabel relasi Many-to-Many terlebih dahulu
DROP TABLE IF EXISTS user_roles;
DROP TABLE IF EXISTS role_permissions;

-- Hapus tabel utama
DROP TABLE IF EXISTS users;
DROP TABLE IF EXISTS roles;
DROP TABLE IF EXISTS permissions;