#!/bin/bash

# --- BASH SCRIPT INTERAKTIF UNTUK BUILD FLUTTER (DENGAN FASTLANE) ---
# Script ini mengotomatiskan proses build Flutter interaktif dan memicu Fastlane
# untuk deployment ke Google Play Console atau App Store Connect.

# Keluar jika ada perintah gagal
set -e

# --- FUNGSI BANTU DAN UTILITY ---

# Fungsi untuk mendeteksi OS dan menyiapkan argumen sed yang benar
prepare_sed() {
    # Check if it's macOS (Darwin) atau Linux/others
    if [[ "$OSTYPE" == "darwin"* ]]; then
        # macOS (BSD sed) requires the backup extension, empty string means no backup file
        SED_INPLACE_EXT="''"
    else
        # Linux (GNU sed) atau Git Bash di Windows
        SED_INPLACE_EXT=""
    fi
}

get_current_version() {
    # Membaca versi saat ini dari app/pubspec.yaml
    if [ ! -f "app/pubspec.yaml" ]; then
        echo "Kesalahan: app/pubspec.yaml tidak ditemukan. Pastikan Anda menjalankan skrip di root proyek dan folder 'app' ada."
        exit 1
    fi
    # Gunakan grep dan cut untuk mengambil nilai 'version: X.Y.Z+B'
    VERSION_LINE=$(grep '^version:' app/pubspec.yaml)
    CURRENT_VERSION=$(echo "$VERSION_LINE" | awk '{print $2}')
    if [ -z "$CURRENT_VERSION" ]; then
        echo "1.0.0+1"
    else
        echo "$CURRENT_VERSION"
    fi
}

update_pubspec_version() {
    local new_version=$1
    echo -e "\n[INFO] Memperbarui versi di app/pubspec.yaml menjadi: ${new_version}"
    # Gunakan sed untuk in-place replacement
    sed -i $SED_INPLACE_EXT "s/^version: .*/version: $new_version/" app/pubspec.yaml
    echo "[INFO] app/pubspec.yaml berhasil diperbarui."
}

# Fungsi untuk mengatur signingConfig di build.gradle.kts berdasarkan BUILD_MODE
update_android_signing_config() {
    local gradle_file="app/android/app/build.gradle.kts"
    
    # Hanya jalankan jika platform adalah Android
    if [ "$PLATFORM" != "android" ]; then
        return
    fi

    echo -e "\n[INFO] Menyiapkan signingConfig di $gradle_file untuk mode: ${BUILD_MODE}..."
    
    if [ "$BUILD_MODE" == "release" ]; then
        # Jika Mode RELEASE, aktifkan 'release' signing config:
        sed -i $SED_INPLACE_EXT 's/^[ \t]*signingConfig = signingConfigs.getByName("debug").*$/\/\/ signingConfig = signingConfigs.getByName("debug")/' "$gradle_file"
        sed -i $SED_INPLACE_EXT 's/^[ \t]*\/\/ signingConfig = signingConfigs.getByName("release")/signingConfig = signingConfigs.getByName("release")/' "$gradle_file"
        echo "[INFO] Menggunakan 'release' signing config."
    else
        # Jika Mode DEBUG atau PROFILE, aktifkan 'debug' signing config:
        sed -i $SED_INPLACE_EXT 's/^[ \t]*\/\/ signingConfig = signingConfigs.getByName("debug")/signingConfig = signingConfigs.getByName("debug")/' "$gradle_file"
        sed -i $SED_INPLACE_EXT 's/^[ \t]*signingConfig = signingConfigs.getByName("release").*$/\/\/ signingConfig = signingConfigs.getByName("release")/' "$gradle_file"
        echo "[INFO] Menggunakan 'debug' signing config."
    fi
}


# --- 1. Pengaturan Versi ---
step_1_get_new_version() {
    CURRENT_VERSION=$(get_current_version)
    echo "--- 1. Pengaturan Versi ---"
    
    while true; do
        read -r -p "Versi saat ini: (${CURRENT_VERSION})${NEWLINE}Masukkan Versi Baru (ex: 2.1.0+45) atau tekan ENTER untuk default: " NEW_VERSION
        
        if [ -z "$NEW_VERSION" ]; then
            NEW_VERSION="$CURRENT_VERSION"
            echo "[PILIHAN] Menggunakan versi saat ini: ${CURRENT_VERSION}"
            IS_VERSION_UPDATED="false"
            break
        fi

        # Validasi format versi (dasar: angka.angka.angka+angka)
        if [[ "$NEW_VERSION" =~ ^[0-9]+\.[0-9]+\.[0-9]+\+[0-9]+$ ]]; then
            read -r -p "Apakah Anda ingin memperbarui versi di app/pubspec.yaml menjadi ${NEW_VERSION}? (y/N): " UPDATE_CHOICE
            UPDATE_CHOICE=$(echo "$UPDATE_CHOICE" | tr '[:upper:]' '[:lower:]')
            
            if [[ "$UPDATE_CHOICE" == "y" ]]; then
                IS_VERSION_UPDATED="true"
            else
                echo "[PERINGATAN] Versi build disetel ke ${NEW_VERSION}, tetapi app/pubspec.yaml TIDAK diperbarui."
                IS_VERSION_UPDATED="false"
            
            fi
            break
        else
            echo "[KESALAHAN] Format versi tidak valid. Gunakan format 'MAJOR.MINOR.PATCH+BUILD_NUMBER'."
        fi
    done
}

# --- 2. Pengaturan Environment (Env) ---
step_2_select_env() {
    echo -e "\n--- 2. Pengaturan Environment (Env) ---"
    
    # 1. Kumpulkan Environment Bernama (packages/env/.env.*)
    local NAMED_ENV_OPTIONS=()
    
    local ENV_FILE_PATHS=($(ls -1 packages/env/.env.* 2>/dev/null | sort))

    for path in "${ENV_FILE_PATHS[@]}"; do
        filename=$(basename "$path")
        
        if [[ "$filename" =~ ^\.env\. ]]; then
            ENV_NAME_EXTRACTED=$(echo "$filename" | sed 's/^\.env\.//')
            NAMED_ENV_OPTIONS+=("$ENV_NAME_EXTRACTED")
        fi
    done
    
    # 2. Cek dan ekstrak NAME dari .env (di root - prioritas default utama)
    ROOT_ENV_NAME_VALUE=""
    
    if [ -f ".env" ]; then
        ROOT_ENV_NAME_VALUE=$(grep '^NAME=' .env 2>/dev/null | head -n 1 | cut -d '=' -f 2- | tr -d "'\"")
        
        # --- Tampilkan konten .env default (ROOT) ---
        echo -e "\n--------------------------------------------------"
        echo "DEFAULT ENVIRONMENT (root/.env):"
        cat .env
        echo "--------------------------------------------------"
    fi
    
    # 3. Cek dan ekstrak NAME dari packages/env/.env (file yang Anda maksud)
    PACKAGE_ENV_PATH="packages/env/.env"
    PKG_ENV_NAME_VALUE=""
    
    if [ -f "$PACKAGE_ENV_PATH" ]; then
        PKG_ENV_NAME_VALUE=$(grep '^NAME=' "$PACKAGE_ENV_PATH" 2>/dev/null | head -n 1 | cut -d '=' -f 2- | tr -d "'\"")
        
        # --- Tampilkan konten packages/env/.env ---
        echo -e "\n--------------------------------------------------"
        echo "ENVIRONMENT DI PACKAGE (${PACKAGE_ENV_PATH}):"
        cat "$PACKAGE_ENV_PATH"
        echo "--------------------------------------------------"
    fi
    
    # 4. Inisialisasi daftar opsi yang BISA DIPILIH (hanya non-root envs)
    local ENV_OPTIONS=()
    local DISPLAY_OPTIONS=()
    
    # 4a. Tambahkan Named Environments (.env.arh-home, etc.)
    ENV_OPTIONS+=("${NAMED_ENV_OPTIONS[@]}")
    DISPLAY_OPTIONS+=("${NAMED_ENV_OPTIONS[@]}")
    
    # 5. Tentukan Default untuk tombol ENTER
    local DEFAULT_ENV_KEY
    local DEFAULT_DISPLAY
    
    # 5a. Prioritas 1: root/.env
    if [ -f ".env" ]; then
        DEFAULT_ENV_KEY="root_key" 
        DEFAULT_DISPLAY="${ROOT_ENV_NAME_VALUE:-default}"
    
    # 5b. Prioritas 2: packages/env/.env
    elif [ -f "$PACKAGE_ENV_PATH" ]; then
        DEFAULT_ENV_KEY="pkg_env_key" 
        DEFAULT_DISPLAY="${PKG_ENV_NAME_VALUE:-packages_default}"
        
    # 5c. Prioritas 3: Named environment pertama
    elif [ ${#ENV_OPTIONS[@]} -gt 0 ]; then
        DEFAULT_ENV_KEY="${ENV_OPTIONS[0]}"
        DEFAULT_DISPLAY="${DISPLAY_OPTIONS[0]}"
        
    # 5d. Pilihan terakhir: Tidak ada env yang ditemukan
    else
        DEFAULT_ENV_KEY="no-env"
        DEFAULT_DISPLAY="TIDAK ADA"
    fi

    # Handle kasus di mana tidak ada env yang ditemukan sama sekali
    if [ "$DEFAULT_ENV_KEY" == "no-env" ]; then
        echo "[PERINGATAN] Tidak ada file .env yang ditemukan. Tidak ada environment yang akan digunakan."
        ENV_NAME="no-env"
        ENV_FILE=""
        return
    fi
    
    # Tampilkan daftar pilihan bernomor
    echo "Pilih Environment (Tekan ENTER untuk ${DEFAULT_DISPLAY}):"
    for i in "${!DISPLAY_OPTIONS[@]}"; do
        echo "$((i+1))) ${DISPLAY_OPTIONS[$i]}"
    done
    
    local INPUT_CHOICE
    local VALID_CHOICE=false

    # --- LOOP MENGGUNAKAN READ ---
    while [ "$VALID_CHOICE" == false ]; do
        read -r -p "Masukkan nomor pilihan: " INPUT_CHOICE
        
        if [ -z "$INPUT_CHOICE" ]; then
            # 1. Handle default choice (ENTER kosong)
            SELECTED_ENV_KEY="$DEFAULT_ENV_KEY"
            VALID_CHOICE=true
            echo "[PILIHAN] Menggunakan Environment Default: ${DEFAULT_DISPLAY}"
        
        elif [[ "$INPUT_CHOICE" =~ ^[0-9]+$ ]]; then
            # 2. Handle numbered choice
            local SELECTED_INDEX=$((INPUT_CHOICE - 1))
            
            # Validasi input numerik
            if [ "$SELECTED_INDEX" -ge 0 ] && [ "$SELECTED_INDEX" -lt ${#ENV_OPTIONS[@]} ]; then
                SELECTED_ENV_KEY="${ENV_OPTIONS[$SELECTED_INDEX]}"
                VALID_CHOICE=true
            else
                echo "[KESALAHAN] Nomor pilihan tidak valid. Masukkan nomor yang terdaftar atau tekan ENTER."
            fi
        
        else
            # 3. Handle invalid text input
            echo "[KESALAHAN] Input tidak valid. Masukkan nomor yang terdaftar atau tekan ENTER."
        fi
    done
    
    # 6. Tentukan ENV_NAME (untuk build.sh) dan ENV_FILE (untuk dart-define)
    if [ "$SELECTED_ENV_KEY" == "root_key" ]; then
        # Jika memilih root/.env (via ENTER)
        if [ -z "$ROOT_ENV_NAME_VALUE" ]; then
            ENV_NAME="default"
        else
            ENV_NAME="$ROOT_ENV_NAME_VALUE" 
        fi
        ENV_FILE=".env"
    elif [ "$SELECTED_ENV_KEY" == "pkg_env_key" ]; then
        # Jika memilih packages/env/.env (via ENTER jika root/.env tidak ada)
        if [ -z "$PKG_ENV_NAME_VALUE" ]; then
            ENV_NAME="packages_default"
        else
            ENV_NAME="$PKG_ENV_NAME_VALUE"
        fi
        ENV_FILE="$PACKAGE_ENV_PATH"
    else
        # Jika memilih env bernama (arh-home, dev, dll.)
        ENV_NAME="$SELECTED_ENV_KEY"
        ENV_FILE="packages/env/.env.${ENV_NAME}"
    fi

    echo "[PILIHAN] File Env: ${ENV_FILE}. Environment yang digunakan: ${ENV_NAME}"
}

# --- 3. Pengaturan Build Options ---
step_3_select_build_options() {
    # 3a. Platform
    echo -e "\n--- 3a. Pilih Platform ---"
    read -r -p "Pilih Platform (1: android, 2: ios) [Default: 1]: " PLATFORM_CHOICE
    PLATFORM_CHOICE=${PLATFORM_CHOICE:-1}
    
    if [ "$PLATFORM_CHOICE" == "1" ]; then
        PLATFORM="android"
    else
        PLATFORM="ios"
    fi
    echo "[PILIHAN] Platform: ${PLATFORM}"

    # 3b. Build Mode
    echo -e "\n--- 3b. Pilih Mode Build ---"
    read -r -p "Pilih Mode (1: release, 2: debug, 3: profile) [Default: 1]: " MODE_CHOICE
    MODE_CHOICE=${MODE_CHOICE:-1}

    if [ "$MODE_CHOICE" == "1" ]; then
        BUILD_MODE="release"
    elif [ "$MODE_CHOICE" == "2" ]; then
        BUILD_MODE="debug"
    else
        BUILD_MODE="profile"
    fi
    echo "[PILIHAN] Mode Build: ${BUILD_MODE}"

    # 3c. Artifact Type
    echo -e "\n--- 3c. Pilih Tipe Artifact ---"
    
    if [ "$PLATFORM" == "android" ]; then
        read -r -p "Pilih Android Artifact (1: aab, 2: apk) [Default: 1]: " ARTIFACT_CHOICE
        ARTIFACT_CHOICE=${ARTIFACT_CHOICE:-1}
        
        if [ "$ARTIFACT_CHOICE" == "1" ]; then
            ARTIFACT_TYPE="appbundle"
        else
            ARTIFACT_TYPE="apk"
        fi
        echo "[PILIHAN] Artifact: $(echo "$ARTIFACT_TYPE" | tr '[:lower:]' '[:upper:]')"
    elif [ "$PLATFORM" == "ios" ]; then
        # Default untuk iOS adalah ipa
        read -r -p "Pilih iOS Artifact (1: ipa) [Default: 1]: " ARTIFACT_CHOICE
        ARTIFACT_TYPE="ipa"
        echo "[PILIHAN] Artifact: IPA"
    fi
}

# --- FUNGSI MENGUNGGAH ANDROID DENGAN FASTLANE ---
execute_fastlane_android_upload() {
    local track=$1
    
    # 1. Path ke Artifact relatif dari folder 'android'
    AAB_PATH="../build/app/outputs/bundle/release/app-release.aab"
    
    # Verifikasi inisialisasi Fastlane Android
    if [ ! -d "android/fastlane" ]; then
        echo "[KESALAHAN] Fastlane belum diinisialisasi di folder 'android'. Silakan jalankan 'cd android && fastlane init'."
        exit 1
    fi
    
    # Pindah ke direktori android (tempat Fastlane diinisialisasi)
    echo -e "\n[INFO] Berpindah ke direktori 'android' untuk menjalankan Fastlane..."
    cd android
    
    echo -e "\n[COMMAND] bundle exec fastlane run upload_to_play_store..."
    
    # Gunakan 'bundle exec fastlane run [action]' untuk menjalankan Fastlane Action tunggal
    bundle exec fastlane run upload_to_play_store \
      aab:"$AAB_PATH" \
      track:"$track" \
      skip_waiting_for_build_processing:true
      
    # Kembali ke root proyek setelah Fastlane selesai
    cd ..
    
    echo -e "\n=================================================="
    echo "  ✅ Deployment ke ${track^^} Track Android Berhasil!"
    echo "=================================================="
}

# --- FUNGSI MENGUNGGAH iOS DENGAN FASTLANE ---
execute_fastlane_ios_upload() {
    local distribution_type=$1 # testflight atau appstore

    # 1. Path ke Artifact relatif dari folder 'ios' (output flutter build ipa)
    # Jalur standar untuk IPA yang dihasilkan oleh Flutter (di folder app/)
    IPA_PATH="app/build/ios/ipa/Runner.ipa" 
    
    # Verifikasi inisialisasi Fastlane iOS
    if [ ! -d "ios/fastlane" ]; then
        echo "[KESALAHAN] Fastlane belum diinisialisasi di folder 'ios'. Silakan jalankan 'cd ios && fastlane init'."
        exit 1
    fi
    
    # Pindah ke direktori ios (tempat Fastlane diinisialisasi)
    echo -e "\n[INFO] Berpindah ke direktori 'ios' untuk menjalankan Fastlane..."
    cd ios

    # Tentukan aksi Fastlane
    if [ "$distribution_type" == "testflight" ]; then
        echo -e "\n[COMMAND] bundle exec fastlane run upload_to_testflight..."
        bundle exec fastlane run upload_to_testflight \
            ipa:"$IPA_PATH" \
            skip_waiting_for_build_processing:true
            
    elif [ "$distribution_type" == "appstore" ]; then
        echo -e "\n[COMMAND] bundle exec fastlane run deliver..."
        # 'deliver' digunakan untuk deployment ke App Store penuh
        bundle exec fastlane run deliver \
            ipa:"$IPA_PATH" \
            skip_metadata:true \
            skip_screenshots:true \
            force:true # Gunakan force:true jika Anda tidak mengunggah metadata/screenshot
            
    fi
    
    # Kembali ke root proyek setelah Fastlane selesai
    cd ..
    
    echo -e "\n=================================================="
    echo "  ✅ Deployment ke ${distribution_type^^} iOS Berhasil!"
    echo "=================================================="
}

# --- 4. Eksekusi Build ---
step_4_execute_build() {
    
    # PENTING: Perbarui signingConfig Android sebelum build dijalankan
    update_android_signing_config

    echo -e "\n"
    echo "=================================================="
    echo "--- 4. Eksekusi Perintah Build Flutter ---"
    echo "=================================================="

    # Inisialisasi perintah (Menggunakan FVM untuk menjalankan Flutter)
    BUILD_COMMAND=("fvm" "flutter")

    # Tentukan command build berdasarkan platform dan artifact
    if [ "$PLATFORM" == "android" ]; then
        BUILD_COMMAND+=("build")
        BUILD_COMMAND+=("$ARTIFACT_TYPE") # appbundle atau apk
    elif [ "$PLATFORM" == "ios" ]; then
        BUILD_COMMAND+=("build")
        BUILD_COMMAND+=("ipa")
    fi

    # Tambahkan Mode Build
    BUILD_COMMAND+=("--${BUILD_MODE}")

    # Tambahkan Build Name/Number dan Dart Defines (jika ada environment yang dipilih)
    if [ "$ENV_NAME" != "no-env" ]; then
        
        # Pisahkan versi menjadi name dan number
        VERSION_NAME=$(echo "$NEW_VERSION" | cut -d '+' -f 1)
        VERSION_NUMBER=$(echo "$NEW_VERSION" | cut -d '+' -f 2)

        BUILD_COMMAND+=("--build-name=${VERSION_NAME}")
        BUILD_COMMAND+=("--build-number=${VERSION_NUMBER}")

        # Dart Defines untuk Environment File
        BUILD_COMMAND+=("--dart-define=FLAVOR=${ENV_NAME}")
        BUILD_COMMAND+=("--dart-define=ENV_FILE=${ENV_FILE}")
    fi
    
    # Penyesuaian akhir untuk iOS IPA: memaksa mode release
    if [ "$PLATFORM" == "ios" ] && [ "$ARTIFACT_TYPE" == "ipa" ] && [ "$BUILD_MODE" != "release" ]; then
        echo -e "\n[INFO] Build IPA biasanya membutuhkan mode 'release'. Menggunakan mode 'release' secara paksa."
        # Ganti --debug/--profile menjadi --release di array
        BUILD_COMMAND=("${BUILD_COMMAND[@]/--$BUILD_MODE/--release}")
        BUILD_MODE="release"
    fi
    
    # PENTING: Jalankan packages/env/build.sh sebelum build
    if [ "$ENV_NAME" != "no-env" ]; then
        echo -e "\n[INFO] Menyiapkan environment '${ENV_NAME}' dan menjalankan pre-build script..."
        
        # 1. Tulis FLUTTER_FLAVOR ke .flavor_env di root
        echo "FLUTTER_FLAVOR=${ENV_NAME}" > ".flavor_env"
        echo "[INFO] Variabel 'FLUTTER_FLAVOR=${ENV_NAME}' disimpan ke .flavor_env"

        # 2. Jalankan packages/env/build.sh
        BUILD_SCRIPT="packages/env/build.sh"
        if [ ! -f "$BUILD_SCRIPT" ]; then
            echo "[KESALAHAN] Error: build.sh tidak ditemukan di packages/env/. Proses build dibatalkan."
            exit 1
        fi
        
        # Pastikan build.sh memiliki izin eksekusi
        chmod +x "$BUILD_SCRIPT"
        
        echo "[COMMAND]: $BUILD_SCRIPT ${ENV_NAME}"
        # Jalankan build.sh dengan environment name sebagai argumen
        "$BUILD_SCRIPT" "$ENV_NAME"
        
        echo "[INFO] Proses penyiapan environment selesai."
    fi
    
    # Konfirmasi dan Jalankan Flutter Build
    echo -e "\n[FINAL COMMAND]: (cd app && ${BUILD_COMMAND[*]})"
    echo "[DETAIL] Versi: ${NEW_VERSION} (Pubspec Path: app/pubspec.yaml)"
    echo "[DETAIL] Platform: ${PLATFORM^^} (${ARTIFACT_TYPE^^})"
    echo "[DETAIL] Mode: ${BUILD_MODE^^}"
    if [ "$ENV_NAME" != "no-env" ]; then
        echo "[DETAIL] Environment (Env): ${ENV_NAME} (File: ${ENV_FILE})"
    fi
    echo -e "\nMemulai proses build dari folder 'app'. Ini mungkin memakan waktu...\n"

    # Jalankan perintah di sub-shell, berpindah ke direktori 'app'
    (cd app && "${BUILD_COMMAND[@]}")

    echo -e "\n\n=================================================="
    echo "✅ FLUTTER BUILD SELESAI DENGAN SUKSES!"
    echo "=================================================="
    
    # --- PANGGIL FASTLANE UNTUK UPLOAD DI SINI ---
    
    # Pertanyaan: Lanjutkan ke Fastlane Deployment?
    read -r -p "Lanjutkan ke Fastlane Deployment (y/N)? [Default: N]: " FASTLANE_CHOICE
    FASTLANE_CHOICE=$(echo "$FASTLANE_CHOICE" | tr '[:upper:]' '[:lower:]')
    
    if [[ "$FASTLANE_CHOICE" == "y" ]]; then
        echo "[PILIHAN] Melanjutkan ke Fastlane Deployment..."
    
        # 5. Deployment Android
        if [ "$PLATFORM" == "android" ] && [ "$ARTIFACT_TYPE" == "appbundle" ] && [ "$BUILD_MODE" == "release" ]; then
            
            # Tanyakan track deployment Google Play
            echo -e "\n--- Pilih Track Deployment Google Play (Android) ---"
            read -r -p "Pilih Track (1: internal, 2: beta, 3: production) [Default: 1]: " TRACK_CHOICE
            TRACK_CHOICE=${TRACK_CHOICE:-1}
            
            if [ "$TRACK_CHOICE" == "2" ]; then
                DEPLOY_TRACK="beta"
            elif [ "$TRACK_CHOICE" == "3" ]; then
                DEPLOY_TRACK="production"
            else
                DEPLOY_TRACK="internal"
            fi
            
            execute_fastlane_android_upload "$DEPLOY_TRACK"

        # 6. Deployment iOS
        elif [ "$PLATFORM" == "ios" ] && [ "$ARTIFACT_TYPE" == "ipa" ] && [ "$BUILD_MODE" == "release" ]; then
            
            # Tanyakan distribusi iOS
            echo -e "\n--- Pilih Tipe Deployment App Store Connect (iOS) ---"
            read -r -p "Pilih Tipe (1: TestFlight, 2: App Store Production) [Default: 1]: " IOS_DISTRIBUTION_CHOICE
            IOS_DISTRIBUTION_CHOICE=${IOS_DISTRIBUTION_CHOICE:-1}
            
            if [ "$IOS_DISTRIBUTION_CHOICE" == "2" ]; then
                IOS_DISTRIBUTION_TYPE="appstore"
            else
                IOS_DISTRIBUTION_TYPE="testflight"
            fi
            
            execute_fastlane_ios_upload "$IOS_DISTRIBUTION_TYPE"

        else
            echo "[INFO] Melewatkan Fastlane Upload. Hanya build 'release' AAB Android atau IPA iOS yang didukung untuk Fastlane."
        fi
    else
        echo "[INFO] Melewatkan Fastlane Deployment. Proses selesai setelah build."
    fi
}

# --- Program Utama ---
main() {
    prepare_sed
    
    step_1_get_new_version

    if [ "$IS_VERSION_UPDATED" == "true" ]; then
        update_pubspec_version "$NEW_VERSION"
    fi

    step_2_select_env

    step_3_select_build_options
    
    # Eksekusi Build dan Deployment Fastlane
    step_4_execute_build
}

main "$@"
