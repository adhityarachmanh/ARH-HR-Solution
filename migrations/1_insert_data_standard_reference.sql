-- INSERT data ke StandardReferences (Master List)
INSERT INTO "StandardReferences" ("StandardReferenceId", "StandardReferenceName", "IsActive", "CreatedDate", "CreatedBy", "UpdatedDate", "UpdatedBy") 
VALUES
('BloodType', 'Blood Type', TRUE, NOW(), 'System', NOW(), 'System'),
('GenderType', 'Gender Type', TRUE, NOW(), 'System', NOW(), 'System'),
('ContactRelationType', 'Contact Relation Type', TRUE, NOW(), 'System', NOW(), 'System'),
('EducationLevelType', 'Education Level Type', TRUE, NOW(), 'System', NOW(), 'System');

-- INSERT data ke StandardReferenceItems (Detail Items)

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
('EducationLevelType', 'DOKSP', 'Dokter Spesialis', TRUE, NOW(), 'System', NOW(), 'System');