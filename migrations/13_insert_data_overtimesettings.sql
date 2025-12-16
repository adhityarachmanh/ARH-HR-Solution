-- File ini mengasumsikan tabel OvertimeMultiplierSettings sudah ada di database

-- =========================================================================
-- SETTING 1: Overtime For 5 Days Working (ID 1)
-- =========================================================================

INSERT INTO "OvertimeSettings" ("IsRounding", "OvertimeName", "IsDefaultCompensation", "CompensationDivideNumber", "OverrideRupiahPerHour")
VALUES(FALSE, 'Overtime For 5 Days Working', TRUE, 173, NULL);

-- Working Days (IsWorkingDay = TRUE)
INSERT INTO "OvertimeMultiplierSettings" ("OvertimeSettingId", "FromHour", "ToHour", "MultipleBy", "IsWorkingDay")
VALUES(1, 1, 1, 1.5, TRUE);
INSERT INTO "OvertimeMultiplierSettings" ("OvertimeSettingId", "FromHour", "ToHour", "MultipleBy", "IsWorkingDay")
VALUES(1, 2, 99, 2, TRUE);

-- Off Days (IsWorkingDay = FALSE)
INSERT INTO "OvertimeMultiplierSettings" ("OvertimeSettingId", "FromHour", "ToHour", "MultipleBy", "IsWorkingDay")
VALUES(1, 1, 8, 2, FALSE);
INSERT INTO "OvertimeMultiplierSettings" ("OvertimeSettingId", "FromHour", "ToHour", "MultipleBy", "IsWorkingDay")
VALUES(1, 9, 9, 3, FALSE);
INSERT INTO "OvertimeMultiplierSettings" ("OvertimeSettingId", "FromHour", "ToHour", "MultipleBy", "IsWorkingDay")
VALUES(1, 10, 99, 4, FALSE);


-- =========================================================================
-- SETTING 2: Overtime For 6 Days Working (ID 2)
-- =========================================================================

INSERT INTO "OvertimeSettings" ("IsRounding", "OvertimeName", "IsDefaultCompensation", "CompensationDivideNumber", "OverrideRupiahPerHour")
VALUES(FALSE, 'Overtime For 6 Days Working', TRUE, 173, NULL);

-- Working Days (IsWorkingDay = TRUE)
INSERT INTO "OvertimeMultiplierSettings" ("OvertimeSettingId", "FromHour", "ToHour", "MultipleBy", "IsWorkingDay")
VALUES(2, 1, 1, 2.5, TRUE);
INSERT INTO "OvertimeMultiplierSettings" ("OvertimeSettingId", "FromHour", "ToHour", "MultipleBy", "IsWorkingDay")
VALUES(2, 2, 8, 4, TRUE);

-- Off Days (IsWorkingDay = FALSE)
INSERT INTO "OvertimeMultiplierSettings" ("OvertimeSettingId", "FromHour", "ToHour", "MultipleBy", "IsWorkingDay")
VALUES(2, 1, 7, 3, FALSE);