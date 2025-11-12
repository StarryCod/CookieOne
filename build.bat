@echo off
setlocal ENABLEDELAYEDEXPANSION

set LOG_FILE=%~dp0build.log
del "%LOG_FILE%" >nul 2>&1

call :log "=============================="
call :log " Cookie Build Script"
call :log "=============================="

where cargo >nul 2>&1
if errorlevel 1 (
    call :error "Rust (cargo) не найден. Установите Rust с https://rustup.rs/"
    exit /b 1
)

call :success "Rust найден"

where cl >nul 2>&1
if errorlevel 1 (
    call :error "MSVC Build Tools не найдены. Установите Visual Studio Build Tools"
    exit /b 1
)

call :success "MSVC найден"

call :log "Проверка интернет-соединения..."
ping -n 1 8.8.8.8 >nul
if errorlevel 1 (
    call :warn "Нет доступа к интернету. Используем оффлайн режим"
) else (
    call :success "Интернет доступен"
)

call :log "Подготовка директорий"
if not exist "%~dp0gui\src-tauri\models" mkdir "%~dp0gui\src-tauri\models"
if not exist "%~dp0gui\src-tauri\rustpotter" mkdir "%~dp0gui\src-tauri\rustpotter"
if not exist "%~dp0gui\src-tauri\logs" mkdir "%~dp0gui\src-tauri\logs"
call :success "Директории готовы"

call :log "Cargo check"
cd /d "%~dp0gui\src-tauri"
cargo check > "%LOG_FILE%" 2>&1
if errorlevel 1 (
    type "%LOG_FILE%"
    call :error "Cargo check завершился с ошибками"
    exit /b 1
)
call :success "Cargo check успешно"

call :log "Cargo build"
cargo build --release >> "%LOG_FILE%" 2>&1
if errorlevel 1 (
    type "%LOG_FILE%"
    call :error "Cargo build завершился с ошибками"
    exit /b 1
)
call :success "Cargo build успешно"

call :log "Сборка фронтенда"
cd /d "%~dp0gui"
call npm install >> "%LOG_FILE%" 2>&1
if errorlevel 1 (
    type "%LOG_FILE%"
    call :error "npm install завершился с ошибкой"
    exit /b 1
)
call npm run build >> "%LOG_FILE%" 2>&1
if errorlevel 1 (
    type "%LOG_FILE%"
    call :error "npm run build завершился с ошибкой"
    exit /b 1
)
call :success "Фронтенд собран"

call :log "Сборка Tauri"
npm run tauri build >> "%LOG_FILE%" 2>&1
if errorlevel 1 (
    type "%LOG_FILE%"
    call :error "Tauri build завершился с ошибкой"
    exit /b 1
)
call :success "Приложение собрано"

call :success "✓ Cookie собран!"
call :log "Файл: %~dp0gui\src-tauri\target\release"

exit /b 0

:log
    echo %~1
    echo %~1 >> "%LOG_FILE%"
    goto :eof

:success
    call :log "[SUCCESS] %~1"
    goto :eof

:warn
    call :log "[WARN] %~1"
    goto :eof

:error
    call :log "[ERROR] %~1"
    color 0C
    echo.
    echo ***************
    echo *    ERROR     *
    echo ***************
    echo %~1
    echo Подробности: %LOG_FILE%
    pause
    goto :eof
