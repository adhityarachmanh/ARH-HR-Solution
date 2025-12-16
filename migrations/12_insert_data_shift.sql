INSERT INTO "Shifts" (
    "ShiftCode", 
    "ShiftName", 
    "IsShowInRequest", 
    "IsWorkShift", 
    "ShiftColorHex", 
    "CreatedDate", 
    "CreatedBy", 
    "UpdatedDate", 
    "UpdatedBy"
)
VALUES (
    'DAYOFF', 
    'Day Off', 
    TRUE, 
    FALSE, 
    '#ED2939', 
    NOW(), 
    'System', 
    NOW(), 
    'System'
);