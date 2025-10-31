import re
import os

# --- CONFIG ---
INPUT_FILE = './migrations/V20250204.3_insert_into_zipcodes(1).txt'
OUTPUT_FILE = './migrations/10_insert_data_zipcodes_pg.sql'
ENCODINGS_TO_TRY = ['utf-8', 'utf-16le', 'utf-16', 'latin-1', 'cp1252']

def convert_to_postgres(sql):
    if not sql.strip():
        return ""

    # Hapus GO dan trim
    sql = re.sub(r'\bGO\b.*$', '', sql, flags=re.IGNORECASE).strip()

    # 1. Ganti header INSERT
    sql = re.sub(
        r'INSERT\s+\[(?:dbo|public)?\]\.\[?(\w+)\]?\s*\(',
        lambda m: f'INSERT INTO "{m.group(1)}" (',
        sql,
        flags=re.IGNORECASE
    )

    # 2. Ganti semua [brackets] jadi "quotes"
    sql = re.sub(r'\[([^\]]+)\]', r'"\1"', sql)

    # 3. Hapus N' 
    sql = re.sub(r"\bN'", "'", sql, flags=re.IGNORECASE)

    # 4. Proses CAST satu per satu dengan loop untuk hindari overlap
    while True:
        cast_match = re.search(
            r'CAST\s*\(\s*(.*?)\s+AS\s+([\w()]+)\s*\)',
            sql,
            flags=re.IGNORECASE | re.DOTALL
        )
        if not cast_match:
            break
        inner_value = cast_match.group(1).strip()
        type_part = cast_match.group(2).strip()
        
        # Hapus N' dari inner jika ada
        inner_value = re.sub(r"^N'", "'", inner_value, flags=re.IGNORECASE)
        
        # Jika string (kutip), ganti T jadi spasi
        if inner_value.startswith("'") and inner_value.endswith("'"):
            cleaned = f"'{inner_value[1:-1].replace('T', ' ')}'"
        else:
            # Numerik: hapus kurung dan biarkan angka
            cleaned = re.sub(r'[()]', '', inner_value)
        
        # Ganti CAST keseluruhan dengan cleaned
        sql = sql.replace(cast_match.group(0), cleaned)

    # 5. Bersihkan sisa kurung di numerik
    sql = re.sub(r'\(\s*([\d\.\-]+)\s*\)', r'\1', sql)

    # 6. Pastikan VALUES ada dan akhir ;
    if 'VALUES' not in sql.upper():
        return ""  # invalid
    sql = re.sub(r';?\s*$', ';', sql)

    return sql

def process_file():
    if not os.path.exists(INPUT_FILE):
        print(f"❌ File tidak ditemukan: {INPUT_FILE}")
        return

    content = None
    encoding_used = None
    for enc in ENCODINGS_TO_TRY:
        try:
            with open(INPUT_FILE, 'r', encoding=enc) as f:
                content = f.read()
            encoding_used = enc
            print(f"✔ Berhasil baca dengan encoding: {enc}")
            break
        except Exception as e:
            continue

    if not content:
        print("❌ Gagal baca file.")
        return

    # Split per INSERT statement
    inserts = re.split(r'(?=^INSERT\b)', content, flags=re.MULTILINE | re.IGNORECASE)
    converted = []
    count = 0

    for stmt in inserts:
        stmt = stmt.strip()
        if not stmt or not stmt.upper().startswith('INSERT'):
            continue
        try:
            pg_stmt = convert_to_postgres(stmt)
            if pg_stmt and 'AS' not in pg_stmt.upper():
                converted.append(pg_stmt)
                count += 1
            else:
                print(f"⚠️ Masih ada 'AS' atau gagal: {stmt[:100]}...")
        except Exception as e:
            print(f"❌ Error konversi: {e}")

    # Tulis file output
    os.makedirs(os.path.dirname(OUTPUT_FILE), exist_ok=True)
    with open(OUTPUT_FILE, 'w', encoding='utf-8') as f:
        f.write(f"-- PostgreSQL INSERT - Generated\n-- Source: {INPUT_FILE}\n-- Encoding: {encoding_used}\n\n")
        f.write('\n'.join(converted))

    print(f"\n✅ SELESAI!")
    print(f"   Output: {OUTPUT_FILE}")
    print(f"   INSERT sukses: {count}")
    print("\n💡 Cek manual: grep -i 'AS\\|CAST' {OUTPUT_FILE}  --> harus kosong!")

# Jalankan
if __name__ == "__main__":
    process_file()