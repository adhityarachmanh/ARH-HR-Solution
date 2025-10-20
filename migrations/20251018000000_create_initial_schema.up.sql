-- migrations/20251020_create_full_hris_schema.up.sql

-- ============================================================
-- 1. USER MANAGEMENT (RBAC CUSTOM)
-- ============================================================

CREATE TABLE users (
    id BIGSERIAL PRIMARY KEY,
    username VARCHAR(100) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    email VARCHAR(255) UNIQUE, 
    is_active BOOLEAN NOT NULL DEFAULT TRUE, 
    last_login TIMESTAMPTZ,                   
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE roles (
    id SERIAL PRIMARY KEY,
    name VARCHAR(50) UNIQUE NOT NULL, 
    display_name VARCHAR(100) NOT NULL 
);

CREATE TABLE permissions (
    id SERIAL PRIMARY KEY,
    name VARCHAR(50) UNIQUE NOT NULL,
    description VARCHAR(255) 
);

CREATE TABLE user_roles (
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    role_id INT NOT NULL REFERENCES roles(id) ON DELETE CASCADE,
    PRIMARY KEY (user_id, role_id)
);

CREATE TABLE role_permissions (
    role_id INT NOT NULL REFERENCES roles(id) ON DELETE CASCADE,
    permission_id INT NOT NULL REFERENCES permissions(id) ON DELETE CASCADE,
    PRIMARY KEY (role_id, permission_id)
);

-- ============================================================
-- 2. HRIS CORE TABLES (Dikonversi dari T-SQL ke PostgreSQL)
--    Semua tabel dari dump Anda, dihubungkan melalui Foreign Keys.
-- ============================================================

CREATE TABLE "TimeZones" (
  "TimeZoneId" INT PRIMARY KEY,
  "TimeZoneName" VARCHAR(100) NOT NULL
);

CREATE TABLE "Company" (
  "CompId" INT PRIMARY KEY,
  "CompName" VARCHAR(150),
  "CompanyPrevMonthDayPayroll" INT NOT NULL DEFAULT 21,
  "CompanyCurrentMonthDayPayroll" INT NOT NULL DEFAULT 20
);

CREATE TABLE "Branches" (
  "BranchId" INT PRIMARY KEY,
  "CompId" INT REFERENCES "Company" ("CompId"),
  "TimeZoneId" INT REFERENCES "TimeZones" ("TimeZoneId"),
  "BranchName" VARCHAR(100) NOT NULL
);

CREATE TABLE "Organizations" (
  "OrganizationId" INT PRIMARY KEY,
  "OrganizationName" VARCHAR(200) NOT NULL,
  "ManagerId" INT 
);

CREATE TABLE "JobLevels" (
  "JobLevelId" INT PRIMARY KEY,
  "JobLevelName" VARCHAR(100) NOT NULL
);

CREATE TABLE "JobPositions" (
  "JobPositionId" INT PRIMARY KEY,
  "JobPositionName" VARCHAR(100)
);

CREATE TABLE "EmploymentStatus" (
  "EmploymentStatusId" INT PRIMARY KEY,
  "EmploymentStatusName" VARCHAR(100) NOT NULL
);

CREATE TABLE "Classes" (
  "ClassId" INT PRIMARY KEY,
  "ClassCode" VARCHAR(50)
);

CREATE TABLE "Grades" (
  "GradeId" INT PRIMARY KEY,
  "GradeCode" VARCHAR(50) NOT NULL
);

CREATE TABLE "PtkpTypes" (
  "PtkpTypeId" INT PRIMARY KEY,
  "PtkpCode" VARCHAR(20)
);

CREATE TABLE "OvertimeSettings" (
  "OvertimeSettingId" INT PRIMARY KEY,
  "OvertimeName" VARCHAR(200)
);

CREATE TABLE "ZipCodes" (
  "ZipCodeId" VARCHAR(10) PRIMARY KEY,
  "District" VARCHAR(50) NOT NULL,
  "County" VARCHAR(50) NOT NULL,
  "City" VARCHAR(50) NOT NULL
);

CREATE TABLE "Shifts" (
  "ShiftId" INT PRIMARY KEY,
  "ShiftCode" VARCHAR(50),
  "ShiftName" VARCHAR(100)
  -- Kolom tambahan lainnya dari T-SQL diabaikan untuk fokus relasi
);

CREATE TABLE "TimeOffPolicies" (
  "TimeOffPolicyId" INT PRIMARY KEY,
  "TimeOffPolicyCode" VARCHAR(20) NOT NULL,
  "TimeOffPolicyName" VARCHAR(255) NOT NULL
);

CREATE TABLE "AttendanceTypes" (
  "AttendanceTypeId" INT PRIMARY KEY,
  "AttendanceTypeCode" VARCHAR(20)
);

CREATE TABLE "StandardReferences" (
  "StandardReferenceId" VARCHAR(100) PRIMARY KEY,
  "StandardReferenceName" VARCHAR(255) NOT NULL
);

-- ============================================================
-- 3. EMPLOYEES (BRIDGING HRIS TO RBAC)
-- ============================================================

CREATE TABLE "Employees" (
  "EmployeeId" INT PRIMARY KEY,
  "BranchId" INT REFERENCES "Branches" ("BranchId"),
  "OrganizationId" INT REFERENCES "Organizations" ("OrganizationId"),
  "JobLevelId" INT REFERENCES "JobLevels" ("JobLevelId"),
  "JobPositionId" INT REFERENCES "JobPositions" ("JobPositionId"),
  "EmploymentStatusId" INT REFERENCES "EmploymentStatus" ("EmploymentStatusId"),
  "ManagerId" INT, -- Self-reference: REFERENCES "Employees" ("EmployeeId")
  "ClassId" INT REFERENCES "Classes" ("ClassId"),
  "UserId" BIGINT REFERENCES users ("id"), -- FK KE USERS CUSTOM
  "GradeId" INT REFERENCES "Grades" ("GradeId"),
  "EmployeeNumber" VARCHAR(20) NOT NULL,
  "Email" VARCHAR(54),
  "BirthDate" TIMESTAMP WITHOUT TIME ZONE,
  "IsActive" BOOLEAN,
  "GajiPokok" FLOAT
  -- Kolom lainnya diabaikan
);

ALTER TABLE "Employees" ADD FOREIGN KEY ("ManagerId") REFERENCES "Employees" ("EmployeeId");

-- ============================================================
-- 4. RELASI TRANSAKSI DAN LOG KE EMPLOYEES
-- ============================================================

CREATE TABLE "AttendanceLogs" (
  "AttendanceLogId" INT PRIMARY KEY,
  "EmployeeId" INT REFERENCES "Employees" ("EmployeeId"),
  "AttendanceTypeId" INT REFERENCES "AttendanceTypes" ("AttendanceTypeId"),
  "ShiftId" INT REFERENCES "Shifts" ("ShiftId"),
  "TimeZoneId" INT REFERENCES "TimeZones" ("TimeZoneId"),
  "Clock" TIMESTAMP WITHOUT TIME ZONE, -- datetime disesuaikan
  "Latitude" DECIMAL(11,8),
  "Longitude" DECIMAL(11,8)
  -- Kolom lainnya diabaikan
);

CREATE TABLE "Attendances" (
  "AttendanceId" INT PRIMARY KEY,
  "EmployeeId" INT REFERENCES "Employees" ("EmployeeId"),
  "AttendanceDate" TIMESTAMP WITHOUT TIME ZONE
  -- Kolom lainnya diabaikan
);

-- Tabel yang mereferensi EmployeeId (Daftar singkat)
CREATE TABLE "Archives" ( "ArchiveId" INT PRIMARY KEY, "EmployeeId" INT REFERENCES "Employees" ("EmployeeId"), "Category" VARCHAR(50) );
CREATE TABLE "EmergencyContacts" ( "EmergencyContactId" INT PRIMARY KEY, "EmployeeId" INT REFERENCES "Employees" ("EmployeeId"), "ContactName" VARCHAR(150) );
CREATE TABLE "FormalEducations" ( "FormalEducationId" INT PRIMARY KEY, "EmployeeId" INT REFERENCES "Employees" ("EmployeeId"), "EducationLevelType" VARCHAR(100) );
CREATE TABLE "InformalEducations" ( "InformalEducationId" INT PRIMARY KEY, "EmployeeId" INT REFERENCES "Employees" ("EmployeeId"), "EducationName" VARCHAR(255) );
CREATE TABLE "Families" ( "FamilyId" INT PRIMARY KEY, "EmployeeId" INT REFERENCES "Employees" ("EmployeeId"), "FamilyName" VARCHAR(150) );
CREATE TABLE "EmployeeFiles" ( "EmployeeFileId" INT PRIMARY KEY, "EmployeeId" INT REFERENCES "Employees" ("EmployeeId"), "Category" VARCHAR(50) );
CREATE TABLE "TimeOffs" ( "TimeOffId" INT PRIMARY KEY, "EmployeeId" INT REFERENCES "Employees" ("EmployeeId"), "TimeOffPolicyId" INT REFERENCES "TimeOffPolicies" ("TimeOffPolicyId"), "StartDate" TIMESTAMP WITHOUT TIME ZONE );
CREATE TABLE "WorkExperiences" ( "WorkExperienceId" INT PRIMARY KEY, "EmployeeId" INT REFERENCES "Employees" ("EmployeeId"), "CompanyName" VARCHAR(255) );
CREATE TABLE "Downloads" ( "DownloadId" INT PRIMARY KEY, "DownloadEmployeeId" INT REFERENCES "Employees" ("EmployeeId") );
CREATE TABLE "OvertimeApprovalRequests" ( "OvertimeRequestId" INT PRIMARY KEY, "RequestEmployeeId" INT REFERENCES "Employees" ("EmployeeId") );
CREATE TABLE "TimeOffApprovalRequests" ( "TimeOffRequestId" INT PRIMARY KEY, "RequestEmployeeId" INT REFERENCES "Employees" ("EmployeeId") );


-- ============================================================
-- 5. SEEDING DATA
-- ============================================================

INSERT INTO roles (name, display_name) VALUES 
    ('super_admin', 'Super Administrator'), 
    ('employee', 'Employee'); 

INSERT INTO permissions (name, description) VALUES
    ('manage_users', 'Allows full CRUD management over user accounts.'),
    ('manage_roles', 'Allows creating, editing, and deleting user roles and permissions.'),
    ('manage_content', 'Allows creating and editing general site content.'),
    ('view_own_data', 'Allows user to view their own profile/basic data.');

INSERT INTO role_permissions (role_id, permission_id)
SELECT r.id, p.id
FROM roles r, permissions p
WHERE r.name = 'super_admin';

INSERT INTO role_permissions (role_id, permission_id)
SELECT r.id, p.id
FROM roles r, permissions p
WHERE r.name = 'employee' AND p.name = 'view_own_data';