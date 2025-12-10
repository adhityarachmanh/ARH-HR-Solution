import re
import os

# --- CONFIG ---
INPUT_FILE = './migrations/V20250204.3_insert_into_zipcodes(1).txt'
OUTPUT_FILE = './migrations/10_insert_data_zipcodes_pg.sql'
FAILED_LOG = './migrations/failed_statements.log'
LONG_VALUES_LOG = './migrations/long_values.log'  # LOG KHUSUS NILAI PANJANG

def read_file_safely(path):
    """Baca file UTF-16LE dengan BOM atau tanpa, fallback ke UTF-8"""
    with open(path, 'rb') as f:
        raw = f.read()
    
    # Cek BOM
    if raw.startswith(b'\xff\xfe'):
        encoding = 'utf-16le'
        raw = raw[2:]
    elif raw.startswith(b'\xfe\xff'):
        encoding = 'utf-16be'
        raw = raw[2:]
    else:
        try:
            text = raw.decode('utf-16le')
            encoding = 'utf-16le'
        except UnicodeDecodeError:
            try:
                text = raw.decode('utf-8')
                encoding = 'utf-8'
            except:
                text = raw.decode('latin-1', errors='replace')
                encoding = 'latin-1'
        else:
            return text, encoding
    
    text = raw.decode(encoding, errors='replace')
    return text, encoding

def find_balanced_cast(sql):
    casts = []
    i = 0
    while i < len(sql):
        match = re.search(r'\bCAST\s*\(', sql[i:], flags=re.IGNORECASE)
        if not match:
            break
        start = i + match.start()
        depth = 1
        j = start + len(match.group(0))
        while j < len(sql) and depth > 0:
            if sql[j] == '(':
                depth += 1
            elif sql[j] == ')':
                depth -= 1
            j += 1
        if depth == 0:
            cast_text = sql[start:j]
            inner = cast_text[5:-1].strip()
            as_pos = inner.upper().rfind(' AS ')
            value = inner[:as_pos].strip() if as_pos != -1 else inner
            value = re.sub(r"^N'", "", value, flags=re.IGNORECASE)
            casts.append((start, j, value))
        i = j
    return casts

def convert_to_postgres(sql, index):
    if not sql.strip():
        return None, "Empty"

    try:
        # 1. Hapus komentar
        sql = re.sub(r'--.*?$', '', sql, flags=re.MULTILINE)
        sql = re.sub(r'/\*.*?\*/', '', sql, flags=re.DOTALL)

        # 2. Ganti header
        header_match = re.search(r'INSERT\s+\[(?:dbo|public)?\]\.\[?(\w+)\]?\s*\(', sql, flags=re.IGNORECASE)
        if not header_match:
            return None, "Header tidak valid"
        table_name = header_match.group(1)
        sql = re.sub(
            r'INSERT\s+\[(?:dbo|public)?\]\.\[?\w+\]?\s*\(',
            f'INSERT INTO "{table_name}" (',
            sql,
            flags=re.IGNORECASE
        )

        # 3. Ganti [kolom]
        sql = re.sub(r'\[([^\]]+)\]', r'"\1"', sql)

        # 4. Ganti CAST
        casts = find_balanced_cast(sql)
        for start, end, value in reversed(casts):
            if value.startswith("'") and 'T' in value:
                cleaned = "'" + value.strip("'").replace('T', ' ') + "'"
            else:
                cleaned = re.sub(r'\.0{6,}', '.0', value.strip("'"))
            sql = sql[:start] + cleaned + sql[end:]

        # 5. Hapus N'
        sql = re.sub(r"\bN'", "'", sql, flags=re.IGNORECASE)

        # 6. Parse VALUES
        values_match = re.search(r'VALUES\s*[\s\n\r]*\(([\s\S]*?)\)\s*;?$', sql, flags=re.IGNORECASE)
        if not values_match:
            return None, "VALUES tidak ditemukan"

        raw_values = values_match.group(1)

        # 7. Split nilai
        parts = []
        current = ""
        in_string = False
        i = 0
        while i < len(raw_values):
            c = raw_values[i]
            if c == "'":
                if in_string and i+1 < len(raw_values) and raw_values[i+1] == "'":
                    current += "''"
                    i += 2
                    continue
                in_string = not in_string
            elif c == ',' and not in_string:
                parts.append(current.strip())
                current = ""
                i += 1
                continue
            current += c
            i += 1
        if current.strip():
            parts.append(current.strip())

        if not parts:
            return None, "Tidak ada nilai"

        # 8. CEK PANJANG >10 char → LOG
        long_values = []
        for idx, p in enumerate(parts):
            cleaned = p.strip("'")
            if len(cleaned) > 10 and not p.upper() == 'NULL' and not re.match(r'^-?\d+(\.\d+)?$', p):
                long_values.append((idx, p, len(cleaned)))
        
        if long_values:
            with open(LONG_VALUES_LOG, 'a', encoding='utf-8') as logf:
                logf.write(f"# BARIS {index} - ADA NILAI PANJANG >10 char\n")
                for col_idx, val, length in long_values:
                    logf.write(f"  Kolom {col_idx}: '{val}' ({length} char)\n")
                logf.write("\n")
            print(f"PERINGATAN: Baris {index} ada nilai panjang >10 char → lihat {LONG_VALUES_LOG}")

        # 9. Tambahkan kutip
        new_parts = []
        for p in parts:
            if p.upper() == 'NULL':
                new_parts.append('NULL')
            elif re.match(r'^-?\d+(\.\d+)?$', p):
                new_parts.append(p)
            else:
                p = p.replace("'", "''")
                new_parts.append(f"'{p}'")

        new_values = ', '.join(new_parts)
        sql = sql[:values_match.start(1)] + new_values + sql[values_match.end(1):]

        # 10. Bersihkan
        sql = re.sub(r'\)\s*;?\s*$', ');', sql)

        return sql + "\n", None

    except Exception as e:
        return None, f"Exception: {str(e)}"

def process_file():
    if not os.path.exists(INPUT_FILE):
        print(f"File tidak ditemukan: {INPUT_FILE}")
        return

    # Baca file
    content, encoding_used = read_file_safely(INPUT_FILE)
    print(f"Berhasil baca: {encoding_used}")

    # Gabung INSERT
    lines = content.splitlines()
    cleaned_lines = []
    current_insert = ""

    for line in lines:
        line = line.strip()
        if not line:
            continue
        if line.upper().startswith('GO'):
            if current_insert:
                cleaned_lines.append(current_insert)
                current_insert = ""
            continue
        if line.upper().startswith('INSERT'):
            if current_insert:
                cleaned_lines.append(current_insert)
            current_insert = line
        else:
            current_insert += " " + line

    if current_insert:
        cleaned_lines.append(current_insert)

    # Proses
    converted = []
    failed = []

    print(f"Proses {len(cleaned_lines):,} INSERT...")

    for i, stmt in enumerate(cleaned_lines):
        pg, error = convert_to_postgres(stmt, i)
        if pg and error is None:
            converted.append(pg)
        else:
            reason = error or "Unknown"
            failed.append((i, reason, stmt))
            print(f"Gagal [{i}]: {reason}")

    # === TULIS OUTPUT ===
    os.makedirs(os.path.dirname(OUTPUT_FILE), exist_ok=True)
    with open(OUTPUT_FILE, 'w', encoding='utf-8', newline='\n') as f:
        f.write("-- Migration: 10_insert_data_zipcodes_pg.sql\n")
        f.write("-- Generated for PostgreSQL + sqlx\n")
        f.write(f"-- Source: {os.path.basename(INPUT_FILE)}\n")
        f.write(f"-- Encoding: {encoding_used}\n")
        f.write(f"-- Total records: {len(converted)}\n")
        f.write("--\n\n")
        for stmt in converted:
            f.write(stmt)

    # Log gagal
    with open(FAILED_LOG, 'w', encoding='utf-8') as f:
        f.write(f"# TOTAL GAGAL: {len(failed)}\n\n")
        for idx, reason, stmt in failed:
            f.write(f"# [{idx}] {reason}\n")
            f.write(stmt.strip() + "\n\n")

    print(f"\nSELESAI!")
    print(f"   Output: {OUTPUT_FILE}")
    print(f"   Sukses: {len(converted):,}")
    print(f"   Gagal : {len(failed)}")
    if len(failed) == 0:
        print("   100% SIAP: sqlx migrate run")
    print(f"\n   Cek nilai panjang:")
    print(f"   cat {LONG_VALUES_LOG}")

if __name__ == "__main__":
    process_file()