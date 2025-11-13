# Cookie Build Script (PowerShell) - Production Version
# Автоматическая сборка приложения Cookie с загрузкой моделей
 
$ErrorActionPreference = "Stop"
$scriptPath = Split-Path -Parent $MyInvocation.MyCommand.Path
$logFile = Join-Path $scriptPath "build.log"

# URLs для загрузки моделей
$VOSK_MODEL_URL = "https://alphacephei.com/vosk/models/vosk-model-small-ru-0.22.zip"
$VOSK_MODEL_NAME = "vosk-model-small-ru-0.22"

function Write-Log {
    param([string]$Message, [string]$Level = "INFO")
    $timestamp = Get-Date -Format "yyyy-MM-dd HH:MM:ss"
    $logMessage = "[$timestamp] [$Level] $Message"
    Write-Host $logMessage
    Add-Content -Path $logFile -Value $logMessage
}

function Write-Success {
    param([string]$Message)
    Write-Host "[✓] $Message" -ForegroundColor Green
    Write-Log -Message $Message -Level "SUCCESS"
}

function Write-Warning-Custom {
    param([string]$Message)
    Write-Host "[!] $Message" -ForegroundColor Yellow
    Write-Log -Message $Message -Level "WARN"
}

function Write-Error-Custom {
    param([string]$Message)
    Write-Host "[✗] $Message" -ForegroundColor Red
    Write-Log -Message $Message -Level "ERROR"
}

function Write-Progress-Custom {
    param([string]$Message)
    Write-Host "[→] $Message" -ForegroundColor Cyan
    Write-Log -Message $Message -Level "PROGRESS"
}

function Test-Command {
    param([string]$Command)
    $null -ne (Get-Command $Command -ErrorAction SilentlyContinue)
}

function Show-Banner {
    Clear-Host
    Write-Host "╔════════════════════════════════════════╗" -ForegroundColor Cyan
    Write-Host "║                                        ║" -ForegroundColor Cyan
    Write-Host "║     🍪  Cookie Build Script v2.0      ║" -ForegroundColor Cyan
    Write-Host "║                                        ║" -ForegroundColor Cyan
    Write-Host "╚════════════════════════════════════════╝" -ForegroundColor Cyan
    Write-Host ""
}

function Check-Prerequisites {
    Write-Progress-Custom "Проверка необходимых инструментов..."

    # Check Rust
    if (-not (Test-Command "cargo")) {
        Write-Error-Custom "Rust (cargo) не найден"
        Write-Host "Скачайте и установите с: https://rustup.rs/" -ForegroundColor Yellow
        exit 1
    }
    $rustVersion = cargo --version
    Write-Success "Rust найден: $rustVersion"

    # Check MSVC
    if (-not (Test-Command "cl")) {
        Write-Warning-Custom "MSVC Build Tools не найдены в PATH"
        $vsPath = "${env:ProgramFiles(x86)}\Microsoft Visual Studio"
        if (Test-Path $vsPath) {
            Write-Success "Visual Studio найдена"
        } else {
            Write-Error-Custom "MSVC Build Tools не установлены"
            Write-Host "Установите Visual Studio Build Tools с https://visualstudio.microsoft.com/downloads/" -ForegroundColor Yellow
            exit 1
        }
    } else {
        Write-Success "MSVC найден"
    }

    # Check Node.js
    if (-not (Test-Command "npm")) {
        Write-Error-Custom "Node.js (npm) не найден"
        Write-Host "Скачайте и установите с: https://nodejs.org/" -ForegroundColor Yellow
        exit 1
    }
    $nodeVersion = node --version
    Write-Success "Node.js найден: $nodeVersion"

    # Check Internet Connection
    Write-Progress-Custom "Проверка интернет-соединения..."
    try {
        $ping = Test-Connection -ComputerName "8.8.8.8" -Count 1 -Quiet -ErrorAction SilentlyContinue
        if ($ping) {
            Write-Success "Интернет доступен"
            return $true
        } else {
            Write-Warning-Custom "Нет доступа к интернету"
            return $false
        }
    } catch {
        Write-Warning-Custom "Не удалось проверить интернет"
        return $false
    }
}

function Prepare-Directories {
    Write-Progress-Custom "Подготовка директорий..."
    
    $dirs = @(
        "$scriptPath\gui\src-tauri\models",
        "$scriptPath\gui\src-tauri\rustpotter",
        "$scriptPath\gui\src-tauri\keywords",
        "$scriptPath\gui\src-tauri\logs",
        "$scriptPath\gui\src-tauri\commands"
    )

    foreach ($dir in $dirs) {
        if (-not (Test-Path $dir)) {
            New-Item -ItemType Directory -Force -Path $dir | Out-Null
            Write-Log "Создана директория: $dir"
        }
    }

    Write-Success "Директории готовы"
}

function Download-VoskModel {
    param([bool]$HasInternet)

    Write-Progress-Custom "Проверка моделей Vosk..."

    $voskModelPath = "$scriptPath\gui\src-tauri\models\$VOSK_MODEL_NAME"
    
    if (Test-Path $voskModelPath) {
        Write-Success "Модель Vosk уже установлена"
        return $true
    }

    if (-not $HasInternet) {
        Write-Warning-Custom "Модель Vosk отсутствует и нет интернета"
        Write-Host "Скачайте модель вручную с:" -ForegroundColor Yellow
        Write-Host $VOSK_MODEL_URL -ForegroundColor Yellow
        Write-Host "И распакуйте в: $voskModelPath" -ForegroundColor Yellow
        return $false
    }

    Write-Progress-Custom "Загрузка модели Vosk (~35MB)..."
    $voskZip = "$scriptPath\gui\src-tauri\models\vosk-model.zip"
    
    try {
        $ProgressPreference = 'SilentlyContinue'
        Invoke-WebRequest -Uri $VOSK_MODEL_URL -OutFile $voskZip -UseBasicParsing
        $ProgressPreference = 'Continue'
        
        Write-Progress-Custom "Распаковка модели Vosk..."
        Expand-Archive -Path $voskZip -DestinationPath "$scriptPath\gui\src-tauri\models" -Force
        Remove-Item $voskZip -Force
        
        Write-Success "Модель Vosk загружена и установлена"
        return $true
    } catch {
        Write-Error-Custom "Не удалось загрузить модель Vosk: $_"
        return $false
    }
}

function Run-CargoCheck {
    Write-Progress-Custom "Запуск cargo check..."
    Set-Location "$scriptPath\gui\src-tauri"
    
    try {
        $output = cargo check 2>&1
        Add-Content -Path $logFile -Value $output
        
        if ($LASTEXITCODE -ne 0) {
            Write-Error-Custom "Cargo check завершился с ошибками"
            Write-Host "Смотрите детали в $logFile" -ForegroundColor Yellow
            Write-Host $output -ForegroundColor Red
            exit 1
        }
        
        Write-Success "Cargo check успешно"
    } catch {
        Write-Error-Custom "Ошибка при выполнении cargo check: $_"
        exit 1
    }
}

function Build-Backend {
    Write-Progress-Custom "Компиляция backend (Rust)..."
    Write-Host "Это может занять несколько минут при первой сборке..." -ForegroundColor Yellow
    Set-Location "$scriptPath\gui\src-tauri"
    
    try {
        $output = cargo build --release 2>&1
        Add-Content -Path $logFile -Value $output
        
        if ($LASTEXITCODE -ne 0) {
            Write-Error-Custom "Cargo build завершился с ошибками"
            Write-Host $output -ForegroundColor Red
            exit 1
        }
        
        Write-Success "Backend скомпилирован"
    } catch {
        Write-Error-Custom "Ошибка при компиляции backend: $_"
        exit 1
    }
}

function Build-Frontend {
    Write-Progress-Custom "Сборка frontend (Svelte/Vite)..."
    Set-Location "$scriptPath\gui"
    
    try {
        Write-Progress-Custom "Установка npm зависимостей..."
        $output = npm install 2>&1
        Add-Content -Path $logFile -Value $output
        
        if ($LASTEXITCODE -ne 0) {
            Write-Error-Custom "npm install завершился с ошибкой"
            exit 1
        }
        
        Write-Progress-Custom "Сборка frontend..."
        $output = npm run build 2>&1
        Add-Content -Path $logFile -Value $output
        
        if ($LASTEXITCODE -ne 0) {
            Write-Error-Custom "npm run build завершился с ошибкой"
            exit 1
        }
        
        Write-Success "Frontend собран"
    } catch {
        Write-Error-Custom "Ошибка при сборке frontend: $_"
        exit 1
    }
}

function Build-Tauri {
    Write-Progress-Custom "Сборка Tauri приложения..."
    Write-Host "Финальная сборка может занять 5-10 минут..." -ForegroundColor Yellow
    Set-Location "$scriptPath\gui"
    
    try {
        $output = npm run tauri build 2>&1
        Add-Content -Path $logFile -Value $output
        
        if ($LASTEXITCODE -ne 0) {
            Write-Error-Custom "Tauri build завершился с ошибкой"
            exit 1
        }
        
        Write-Success "Tauri приложение собрано"
    } catch {
        Write-Error-Custom "Ошибка при сборке Tauri: $_"
        exit 1
    }
}

function Show-Result {
    Write-Host ""
    Write-Host "╔════════════════════════════════════════╗" -ForegroundColor Green
    Write-Host "║                                        ║" -ForegroundColor Green
    Write-Host "║   ✓  Сборка завершена успешно! 🎉    ║" -ForegroundColor Green
    Write-Host "║                                        ║" -ForegroundColor Green
    Write-Host "╚════════════════════════════════════════╝" -ForegroundColor Green
    Write-Host ""
    
    $exePath = "$scriptPath\gui\src-tauri\target\release\jarvis-app.exe"
    if (Test-Path $exePath) {
        $size = (Get-Item $exePath).Length / 1MB
        Write-Host "📦 Исполняемый файл:" -ForegroundColor Cyan
        Write-Host "   Путь: $exePath" -ForegroundColor White
        Write-Host "   Размер: $([math]::Round($size, 2)) MB" -ForegroundColor White
        Write-Host ""
        
        $launch = Read-Host "Запустить приложение сейчас? (y/n)"
        if ($launch -eq "y" -or $launch -eq "Y" -or $launch -eq "д" -or $launch -eq "Д") {
            Write-Progress-Custom "Запуск Cookie..."
            Start-Process $exePath
        }
    } else {
        Write-Warning-Custom "Исполняемый файл не найден по ожидаемому пути"
        Write-Host "Проверьте: $scriptPath\gui\src-tauri\target\release\" -ForegroundColor Yellow
    }
    
    Write-Host ""
    Write-Host "📋 Логи сборки: $logFile" -ForegroundColor Cyan
    Write-Host ""
}

# Main execution
try {
    Show-Banner
    
    "" | Set-Content $logFile
    Write-Log "==================== Cookie Build Script ====================" 
    Write-Log "Начало сборки: $(Get-Date -Format 'yyyy-MM-dd HH:mm:ss')"
    Write-Log "=============================================================="
    
    $hasInternet = Check-Prerequisites
    Prepare-Directories
    $voskReady = Download-VoskModel -HasInternet $hasInternet
    
    if (-not $voskReady) {
        Write-Warning-Custom "Продолжение без модели Vosk"
        $continue = Read-Host "Продолжить сборку? (y/n)"
        if ($continue -ne "y" -and $continue -ne "Y") {
            Write-Host "Сборка отменена пользователем" -ForegroundColor Yellow
            exit 0
        }
    }
    
    Run-CargoCheck
    Build-Backend
    Build-Frontend
    Build-Tauri
    
    Show-Result
    
    Write-Log "=============================================================="
    Write-Log "Сборка завершена: $(Get-Date -Format 'yyyy-MM-dd HH:mm:ss')"
    Write-Log "=============================================================="
    
} catch {
    Write-Error-Custom "Критическая ошибка: $_"
    Write-Log "CRITICAL ERROR: $_"
    Write-Log "Stack trace: $($_.ScriptStackTrace)"
    Write-Host ""
    Write-Host "Подробности в логе: $logFile" -ForegroundColor Yellow
    exit 1
}
