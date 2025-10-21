-- migrations/20251018000000_create_initial_schema.down.sql

DROP TABLE IF EXISTS "AttendanceLogs";
DROP TABLE IF EXISTS "Attendances";
DROP TABLE IF EXISTS "Archives";
DROP TABLE IF EXISTS "EmergencyContacts";
DROP TABLE IF EXISTS "FormalEducations";
DROP TABLE IF EXISTS "InformalEducations";
DROP TABLE IF EXISTS "Families";
DROP TABLE IF EXISTS "EmployeeFiles";
DROP TABLE IF EXISTS "TimeOffs";
DROP TABLE IF EXISTS "WorkExperiences";
DROP TABLE IF EXISTS "Downloads";
DROP TABLE IF EXISTS "OvertimeApprovalRequests";
DROP TABLE IF EXISTS "TimeOffApprovalRequests";
DROP TABLE IF EXISTS "Perangkat";
DROP TABLE IF EXISTS "RoleEndpoint";
DROP TABLE IF EXISTS "RoleMenu";

ALTER TABLE "Employees" DROP CONSTRAINT IF EXISTS "Employees_ManagerId_fkey";

DROP TABLE IF EXISTS "Employees";
DROP TABLE IF EXISTS "Branches";
DROP TABLE IF EXISTS "Organizations";
DROP TABLE IF EXISTS "JobLevels";
DROP TABLE IF EXISTS "JobPositions";
DROP TABLE IF EXISTS "EmploymentStatus";
DROP TABLE IF EXISTS "Classes";
DROP TABLE IF EXISTS "Grades";
DROP TABLE IF EXISTS "PtkpTypes";
DROP TABLE IF EXISTS "OvertimeSettings";
DROP TABLE IF EXISTS "TimeOffPolicies";
DROP TABLE IF EXISTS "AttendanceTypes";
DROP TABLE IF EXISTS "ZipCodes";
DROP TABLE IF EXISTS "Shifts";
DROP TABLE IF EXISTS "StandardReferences";
DROP TABLE IF EXISTS "Menus";
DROP TABLE IF EXISTS "Endpoints";
DROP TABLE IF EXISTS "TimeZones";
DROP TABLE IF EXISTS "Company";

DROP TABLE IF EXISTS user_roles;
DROP TABLE IF EXISTS role_permissions;
DROP TABLE IF EXISTS users;
DROP TABLE IF EXISTS roles;
DROP TABLE IF EXISTS permissions;