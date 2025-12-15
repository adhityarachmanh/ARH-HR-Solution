-- PostgreSQL INSERT - Generated from user data

-- =========================================================================
-- INSERT data ke StandardReferences (MASTER LISTS)
-- =========================================================================

INSERT INTO "StandardReferences" ("StandardReferenceId", "StandardReferenceName", "IsActive", "CreatedDate", "CreatedBy", "UpdatedDate", "UpdatedBy") 
VALUES
('BloodType', 'Blood Type', TRUE, NOW(), 'System', NOW(), 'System'),
('GenderType', 'Gender Type', TRUE, NOW(), 'System', NOW(), 'System'),
('ContactRelationType', 'Contact Relation Type', TRUE, NOW(), 'System', NOW(), 'System'),
('EducationLevelType', 'Education Level Type', TRUE, NOW(), 'System', NOW(), 'System'),
('MaritalStatusType', 'Status Perkawinan', TRUE, NOW(), 'System', NOW(), 'System'),
('ReligionType', 'Agama', TRUE, NOW(), 'System', NOW(), 'System'),
('BankAccountType', 'Bank Account', TRUE, NOW(), 'System', NOW(), 'System'),
('DurationType', 'Tipe Durasi', TRUE, NOW(), 'System', NOW(), 'System'),
('AnnouncementCategoryType', 'Announcement Category', TRUE, NOW(), 'System', NOW(), 'System');

-- =========================================================================
-- INSERT data ke StandardReferenceItems (DETAIL ITEMS)
-- =========================================================================

-- BloodType
INSERT INTO "StandardReferenceItems" ("StandardReferenceId", "ItemId", "ItemName", "IsActive", "CreatedDate", "CreatedBy", "UpdatedDate", "UpdatedBy") 
VALUES
('BloodType', 'A', 'A', TRUE, NOW(), 'System', NOW(), 'System'),
('BloodType', 'B', 'B', TRUE, NOW(), 'System', NOW(), 'System'),
('BloodType', 'O', 'O', TRUE, NOW(), 'System', NOW(), 'System'),
('BloodType', 'AB', 'AB', TRUE, NOW(), 'System', NOW(), 'System');

-- GenderType
INSERT INTO "StandardReferenceItems" ("StandardReferenceId", "ItemId", "ItemName", "IsActive", "CreatedDate", "CreatedBy", "UpdatedDate", "UpdatedBy") 
VALUES
('GenderType', 'L', 'Laki-Laki', TRUE, NOW(), 'System', NOW(), 'System'),
('GenderType', 'P', 'Perempuan', TRUE, NOW(), 'System', NOW(), 'System');

-- ContactRelationType
INSERT INTO "StandardReferenceItems" ("StandardReferenceId", "ItemId", "ItemName", "IsActive", "CreatedDate", "CreatedBy", "UpdatedDate", "UpdatedBy") 
VALUES 
('ContactRelationType', 'Ayah', 'Ayah', TRUE, NOW(), 'System', NOW(), 'System'),
('ContactRelationType', 'Ibu', 'Ibu', TRUE, NOW(), 'System', NOW(), 'System'),
('ContactRelationType', 'SaudaraKandung', 'Saudara Kandung', TRUE, NOW(), 'System', NOW(), 'System'),
('ContactRelationType', 'Pasangan', 'Pasangan', TRUE, NOW(), 'System', NOW(), 'System'),
('ContactRelationType', 'Anak', 'Anak', TRUE, NOW(), 'System', NOW(), 'System'),
('ContactRelationType', 'Sepupu', 'Sepupu', TRUE, NOW(), 'System', NOW(), 'System'),
('ContactRelationType', 'Keponakan', 'Keponakan', TRUE, NOW(), 'System', NOW(), 'System'),
('ContactRelationType', 'Mertua', 'Mertua', TRUE, NOW(), 'System', NOW(), 'System'),
('ContactRelationType', 'IparL', 'Ipar Laki-Laki', TRUE, NOW(), 'System', NOW(), 'System'),
('ContactRelationType', 'IparP', 'Ipar Perempuan', TRUE, NOW(), 'System', NOW(), 'System'),
('ContactRelationType', 'Paman', 'Paman', TRUE, NOW(), 'System', NOW(), 'System'),
('ContactRelationType', 'Bibi', 'Bibi', TRUE, NOW(), 'System', NOW(), 'System'),
('ContactRelationType', 'Kakek', 'Kakek', TRUE, NOW(), 'System', NOW(), 'System'),
('ContactRelationType', 'Nenek', 'Nenek', TRUE, NOW(), 'System', NOW(), 'System'),
('ContactRelationType', 'Teman', 'Teman', TRUE, NOW(), 'System', NOW(), 'System'),
('ContactRelationType', 'RekanKerja', 'Rekan Kerja', TRUE, NOW(), 'System', NOW(), 'System'),
('ContactRelationType', 'Lain-Lain', 'Lain-Lain', TRUE, NOW(), 'System', NOW(), 'System');

-- EducationLevelType
INSERT INTO "StandardReferenceItems" ("StandardReferenceId", "ItemId", "ItemName", "IsActive", "CreatedDate", "CreatedBy", "UpdatedDate", "UpdatedBy") 
VALUES 
('EducationLevelType', 'TK', 'Kindergarten', TRUE, NOW(), 'System', NOW(), 'System'),
('EducationLevelType', 'SD', 'Elementary School', TRUE, NOW(), 'System', NOW(), 'System'),
('EducationLevelType', 'SMP', 'Junior High School', TRUE, NOW(), 'System', NOW(), 'System'),
('EducationLevelType', 'SMA', 'Senior High School', TRUE, NOW(), 'System', NOW(), 'System'),
('EducationLevelType', 'SMK', 'Vocational High School', TRUE, NOW(), 'System', NOW(), 'System'),
('EducationLevelType', 'SMF', 'Vocational High School of Farmasi', TRUE, NOW(), 'System', NOW(), 'System'),
('EducationLevelType', 'VOKASI', 'Vocational High School', TRUE, NOW(), 'System', NOW(), 'System'),
('EducationLevelType', 'D1', 'Associate Degree 1 (D1)', TRUE, NOW(), 'System', NOW(), 'System'),
('EducationLevelType', 'D2', 'Associate Degree 2 (D2)', TRUE, NOW(), 'System', NOW(), 'System'),
('EducationLevelType', 'D3', 'Associate Degree 3 (D3)', TRUE, NOW(), 'System', NOW(), 'System'),
('EducationLevelType', 'D4', 'Associate Degree 4 (D4)', TRUE, NOW(), 'System', NOW(), 'System'),
('EducationLevelType', 'S1', 'Bachelor Degree (S1)', TRUE, NOW(), 'System', NOW(), 'System'),
('EducationLevelType', 'SFarmApt', 'Bachelor Degree (S1) + Profesi Apoteker', TRUE, NOW(), 'System', NOW(), 'System'),
('EducationLevelType', 'SKepNers', 'Bachelor Degree (S1) + Profesi Keperawatan (Ners)', TRUE, NOW(), 'System', NOW(), 'System'),
('EducationLevelType', 'SPsi', 'Bachelor Degree (S1) + Profesi Psikologi', TRUE, NOW(), 'System', NOW(), 'System'),
('EducationLevelType', 'SKed', 'Bachelor Degree (S1) + Profesi Dokter', TRUE, NOW(), 'System', NOW(), 'System'),
('EducationLevelType', 'S2', 'Master Degree (S2)', TRUE, NOW(), 'System', NOW(), 'System'),
('EducationLevelType', 'S3', 'Doctoral Degree (S3)', TRUE, NOW(), 'System', NOW(), 'System'),
('EducationLevelType', 'ANTI', 'Ahli Nautika Tingkat I', TRUE, NOW(), 'System', NOW(), 'System'),
('EducationLevelType', 'ANTII', 'Ahli Nautika Tingkat II', TRUE, NOW(), 'System', NOW(), 'System'),
('EducationLevelType', 'ANTIII', 'Ahli Nautika Tingkat III', TRUE, NOW(), 'System', NOW(), 'System'),
('EducationLevelType', 'ANTIV', 'Ahli Nautika Tingkat IV', TRUE, NOW(), 'System', NOW(), 'System'),
('EducationLevelType', 'ATTI', 'Ahli Teknika Tingkat I', TRUE, NOW(), 'System', NOW(), 'System'),
('EducationLevelType', 'ATTII', 'Ahli Teknika Tingkat II', TRUE, NOW(), 'System', NOW(), 'System'),
('EducationLevelType', 'ATTIII', 'Ahli Teknika Tingkat III', TRUE, NOW(), 'System', NOW(), 'System'),
('EducationLevelType', 'ATTIV', 'Ahli Teknika Tingkat IV', TRUE, NOW(), 'System', NOW(), 'System'),
('EducationLevelType', 'EDD', 'Professional Education', TRUE, NOW(), 'System', NOW(), 'System'),
('EducationLevelType', 'MILEDU', 'Military Education', TRUE, NOW(), 'System', NOW(), 'System'),
('EducationLevelType', 'DOKSP', 'Dokter Spesialis', TRUE, NOW(), 'System', NOW(), 'System'),
('EducationLevelType', 'D3NERS', 'Associate Degree 3 (D3) Ners', TRUE, NOW(), 'System', NOW(), 'System'),
('EducationLevelType', 'D3TTK', 'Associate Degree 3 (D3) TTK', TRUE, NOW(), 'System', NOW(), 'System'),
('EducationLevelType', 'SNutrition', 'Bachelor Degree (S1) Nutrition', TRUE, NOW(), 'System', NOW(), 'System'),
('EducationLevelType', 'D3RADIOLOGI', 'Associate Degree 3 (D3) Radiologi', TRUE, NOW(), 'System', NOW(), 'System'),
('EducationLevelType', 'SKeb', 'Bachelor Degree (S1) Kebidanan', TRUE, NOW(), 'System', NOW(), 'System'),
('EducationLevelType', 'D3Keb', 'Associate Degree 3 (D3) Kebidanan', TRUE, NOW(), 'System', NOW(), 'System'),
('EducationLevelType', 'SFarm', 'Bachelor Degree (S1) Farmakolog', TRUE, NOW(), 'System', NOW(), 'System'),
('EducationLevelType', 'D3Kes', 'Associate Degree 3 (D3) Kesehatan', TRUE, NOW(), 'System', NOW(), 'System'),
('EducationLevelType', 'D3RMIK', 'Associate Degree 3 (D3) Rekam Medis dan Informasi Kesehatan', TRUE, NOW(), 'System', NOW(), 'System'),
('EducationLevelType', 'D1Keb', 'Associate Degree 1 (D1) Kebidanan', TRUE, NOW(), 'System', NOW(), 'System'),
('EducationLevelType', 'SKes', 'Bachelor Degree (S1) Kesehatan', TRUE, NOW(), 'System', NOW(), 'System');

-- MaritalStatusType
INSERT INTO "StandardReferenceItems" ("StandardReferenceId", "ItemId", "ItemName", "IsActive", "CreatedDate", "CreatedBy", "UpdatedDate", "UpdatedBy") 
VALUES
('MaritalStatusType', 'Lajang', 'Lajang', TRUE, NOW(), 'System', NOW(), 'System'),
('MaritalStatusType', 'Menikah', 'Menikah', TRUE, NOW(), 'System', NOW(), 'System'),
('MaritalStatusType', 'Bercerai', 'Bercerai', TRUE, NOW(), 'System', NOW(), 'System'),
('MaritalStatusType', 'Janda', 'Janda', TRUE, NOW(), 'System', NOW(), 'System'),
('MaritalStatusType', 'Duda', 'Duda', TRUE, NOW(), 'System', NOW(), 'System');

-- ReligionType
INSERT INTO "StandardReferenceItems" ("StandardReferenceId", "ItemId", "ItemName", "IsActive", "CreatedDate", "CreatedBy", "UpdatedDate", "UpdatedBy") 
VALUES
('ReligionType', 'Islam', 'Islam', TRUE, NOW(), 'System', NOW(), 'System'),
('ReligionType', 'Kristen Protestan', 'Kristen Protestan', TRUE, NOW(), 'System', NOW(), 'System'),
('ReligionType', 'Kristen Katolik', 'Kristen Katolik', TRUE, NOW(), 'System', NOW(), 'System'),
('ReligionType', 'Hindu', 'Hindu', TRUE, NOW(), 'System', NOW(), 'System'),
('ReligionType', 'Buddha', 'Buddha', TRUE, NOW(), 'System', NOW(), 'System'),
('ReligionType', 'Khonghucu', 'Khonghucu', TRUE, NOW(), 'System', NOW(), 'System');

-- BankAccountType
INSERT INTO "StandardReferenceItems" ("StandardReferenceId", "ItemId", "ItemName", "IsActive", "CreatedDate", "CreatedBy", "UpdatedDate", "UpdatedBy") 
VALUES
('BankAccountType', 'BCA', 'Bank Central Asia', TRUE, NOW(), 'System', NOW(), 'System'),
('BankAccountType', 'BRI', 'Bank Rakyat Indonesia', TRUE, NOW(), 'System', NOW(), 'System'),
('BankAccountType', 'BNI', 'Bank Negara Indonesia', TRUE, NOW(), 'System', NOW(), 'System'),
('BankAccountType', 'BMRI', 'Bank Mandiri', TRUE, NOW(), 'System', NOW(), 'System'),
('BankAccountType', 'BDMN', 'Bank Danamon', TRUE, NOW(), 'System', NOW(), 'System'),
('BankAccountType', 'BTN', 'Bank Tabungan Negara', TRUE, NOW(), 'System', NOW(), 'System'),
('BankAccountType', 'CIMB', 'CIMB Niaga', TRUE, NOW(), 'System', NOW(), 'System'),
('BankAccountType', 'Maybank', 'Maybank Indonesia', TRUE, NOW(), 'System', NOW(), 'System'),
('BankAccountType', 'Permata', 'Bank Permata', TRUE, NOW(), 'System', NOW(), 'System'),
('BankAccountType', 'BTPN', 'Bank BTPN', TRUE, NOW(), 'System', NOW(), 'System'),
('BankAccountType', 'Panin', 'Bank Panin', TRUE, NOW(), 'System', NOW(), 'System'),
('BankAccountType', 'Mega', 'Bank Mega', TRUE, NOW(), 'System', NOW(), 'System'),
('BankAccountType', 'OCBC', 'OCBC NISP', TRUE, NOW(), 'System', NOW(), 'System'),
('BankAccountType', 'BBKP', 'KB Bank (Bukopin)', TRUE, NOW(), 'System', NOW(), 'System'),
('BankAccountType', 'BRIS', 'Bank Syariah Indonesia', TRUE, NOW(), 'System', NOW(), 'System'),
('BankAccountType', 'Jago', 'Bank Jago', TRUE, NOW(), 'System', NOW(), 'System'),
('BankAccountType', 'BJB', 'Bank Jawa Barat dan Banten (BJB)', TRUE, NOW(), 'System', NOW(), 'System'),
('BankAccountType', 'BJTM', 'Bank Jawa Timur', TRUE, NOW(), 'System', NOW(), 'System'),
('BankAccountType', 'Sumut', 'Bank Sumut', TRUE, NOW(), 'System', NOW(), 'System'),
('BankAccountType', 'DKI', 'Bank DKI', TRUE, NOW(), 'System', NOW(), 'System'),
('BankAccountType', 'NTB', 'Bank NTB Syariah', TRUE, NOW(), 'System', NOW(), 'System');

-- DurationType
INSERT INTO "StandardReferenceItems" ("StandardReferenceId", "ItemId", "ItemName", "IsActive", "CreatedDate", "CreatedBy", "UpdatedDate", "UpdatedBy") 
VALUES
('DurationType', 'Hari', 'Hari', TRUE, NOW(), 'System', NOW(), 'System'),
('DurationType', 'Jam', 'Jam', TRUE, NOW(), 'System', NOW(), 'System');

-- AnnouncementCategoryType
INSERT INTO "StandardReferenceItems" ("StandardReferenceId", "ItemId", "ItemName", "IsActive", "CreatedDate", "CreatedBy", "UpdatedDate", "UpdatedBy") 
VALUES
('AnnouncementCategoryType', 'Ann.Uncategorized', 'Uncategorized', TRUE, NOW(), 'System', NOW(), 'System'),
('AnnouncementCategoryType', 'Ann.Pengumuman', 'Pengumuman', TRUE, NOW(), 'System', NOW(), 'System'),
('AnnouncementCategoryType', 'Ann.Pemberitahuan', 'Pemberitahuan', TRUE, NOW(), 'System', NOW(), 'System'),
('AnnouncementCategoryType', 'Ann.Informasi', 'Informasi', TRUE, NOW(), 'System', NOW(), 'System');