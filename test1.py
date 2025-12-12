import os
import sys

# --- CONFIG ---
# Target file path (berdasarkan referensi Anda)
INPUT_FILE = './migrations/10_insert_data_zipcodes_pg.sql'
OUTPUT_FILE = './migrations/10_insert_data_zipcodes_pg_cleaned.sql' 
# Output file baru untuk keamanan, agar tidak menimpa file asli secara langsung

def clean_quotes_in_file(input_filename, output_filename):
    """
    Membaca file SQL, mengganti semua kemunculan ''' dengan ', dan menulis ke file baru.
    """
    if not os.path.exists(input_filename):
        print(f"❌ File tidak ditemukan: {input_filename}")
        return

    try:
        # 1. Baca konten file asli
        with open(input_filename, 'r', encoding='utf-8') as f:
            content = f.read()
            
        print(f"✔ Berhasil membaca konten dari: {input_filename}")
        
        # 2. Lakukan penggantian: ''' menjadi '
        # Catatan: Penggantian ini mungkin bermasalah jika ''' digunakan sebagai bagian dari data 
        # yang valid, tetapi ini adalah asumsi untuk memperbaiki error umum.
        
        original_count = content.count("'''")
        cleaned_content = content.replace("'''", "'")
        new_count = cleaned_content.count("'''")

        # 3. Tulis konten yang sudah dibersihkan ke file output
        with open(output_filename, 'w', encoding='utf-8') as f:
            f.write(cleaned_content)

        print(f"\n✅ SELESAI!")
        print(f"   Total kemunculan ' tripled quotes (''') yang diganti: {original_count}")
        print(f"   Output disimpan ke: {output_filename}")

    except Exception as e:
        print(f"❌ Terjadi kesalahan saat memproses data: {e}")
        # Jika Anda ingin melihat detail error
        # import traceback; traceback.print_exc()

# Jalankan fungsi utama
if __name__ == "__main__":
    clean_quotes_in_file(INPUT_FILE, OUTPUT_FILE)