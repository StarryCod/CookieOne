# Cookie Build Script (PowerShell)
$ErrorActionPreference = "Stop"
$scriptPath = Split-Path -Parent $MyInvocation.MyCommand.Path
$logFile = Join-Path $scriptPath "build.log"

function Write-Log {
    param([string]$Message, [string]$Level = "INFO")
    $timestamp = Get-Date -Format "yyyy-MM-dd HH:mm:ss"
    $logMessage = "[$timestamp] [$Level] $Message"
    Write-Host $logMessage
    Add-Content -Path $logFile -Value $logMessage
}

function Write-Success {
    param([string]$Message)
    Write-Host "[SUCCESS] $Message" -ForegroundColor Green
    Write-Log -Message $Message -Level "SUCCESS"
}

function Write-Warning {
    param([string]$Message)
    Write-Host "[WARN] $Message" -ForegroundColor Yellow
    Write-Log -Message $Message -Level "WARN"
}

function Write-Error-Custom {
    param([string]$Message)
    Write-Host "[ERROR] $Message" -ForegroundColor Red
    Write-Log -Message $Message -Level "ERROR"
}

function Test-Command {
    param([string]$Command)
    $null -ne (Get-Command $Command -ErrorAction SilentlyContinue)
}

Clear-Host
Write-Host "==============================" -ForegroundColor Cyan
Write-Host " Cookie Build Script" -ForegroundColor Cyan
Write-Host "==============================" -ForegroundColor Cyan

"" | Set-Content $logFile
Write-Log "Начало сборки Cookie"

Write-Log "Проверка инструментов..."

if (-not (Test-Command "cargo")) {
    Write-Error-Custom "Rust (cargo) не найден. Установите с https://rustup.rs/"
    exit 1
}
Write-Success "Rust найден"

if (-not (Test-Command "cl")) {
    Write-Warning "MSVC Build Tools не найдены в PATH"
    $vsPath = "${env:ProgramFiles(x86)}\Microsoft Visual Studio"
    if (Test-Path $vsPath) {
        Write-Success "Visual Studio найдена"
    } else {
        Write-Error-Custom "MSVC Build Tools не установлены"
        exit 1
    }
} else {
    Write-Success "MSVC найден"
}

if (-not (Test-Command "npm")) {
    Write-Error-Custom "Node.js (npm) не найден. Установите с https://nodejs.org/"
    exit 1
}
Write-Success "Node.js найден"

Write-Log "Проверка интернет-соединения..."
try {
    $ping = Test-Connection -ComputerName "8.8.8.8" -Count 1 -Quiet
    if ($ping) {
        Write-Success "Интернет доступен"
    } else {
        Write-Warning "Нет интернета. Оффлайн режим"
    }
} catch {
    Write-Warning "Ошибка проверки интернета"
}

Write-Log "Подготовка директорий..."
$dirs = @(
    "$scriptPath\gui\src-tauri\models",
    "$scriptPath\gui\src-tauri\rustpotter",
    "$scriptPath\gui\src-tauri\logs"
)

foreach ($dir in $dirs) {
    if (-not (Test-Path $dir)) {
        New-Item -ItemType Directory -Force -Path $dir | Out-Null
        Write-Log "Создана: $dir"
    }
}
Write-Success "Директории готовы"

Write-Log "Cargo check..."
Set-Location "$scriptPath\gui\src-tauri"

try {
    $output = cargo check 2>&1
    Add-Content -Path $logFile -Value $output
    
    if ($LASTEXITCODE -ne 0) {
        Write-Error-Custom "Cargo check завершился с ошибками"
        Write-Host $output
        exit 1
    }
    
    Write-Success "Cargo check успешно"
} catch {
    Write-Error-Custom "Ошибка cargo check: $_"
    exit 1
}

Write-Log "Компиляция backend (Rust)..."
try {
    $output = cargo build --release 2>&1
    Add-Content -Path $logFile -Value $output
    
    if ($LASTEXITCODE -ne 0) {
        Write-Error-Custom "Cargo build ошибка"
        Write-Host $output
        exit 1
    }
    
    Write-Success "Backend скомпилирован"
} catch {
    Write-Error-Custom "Ошибка: $_"
    exit 1
}

Write-Log "Сборка frontend..."
Set-Location "$scriptPath\gui"

try {
    $output = npm install 2>&1
    Add-Content -Path $logFile -Value $output
    
    if ($LASTEXITCODE -ne 0) {
        Write-Error-Custom "npm install ошибка"
        exit 1
    }
    
    $output = npm run build 2>&1
    Add-Content -Path $logFile -Value $output
    
    if ($LASTEXITCODE -ne 0) {
        Write-Error-Custom "npm build ошибка"
        exit 1
    }
    
    Write-Success "Frontend собран"
} catch {
    Write-Error-Custom "Ошибка: $_"
    exit 1
}

Write-Log "Сборка Tauri..."
try {
    $output = npm run tauri build 2>&1
    Add-Content -Path $logFile -Value $output
    
    if ($LASTEXITCODE -ne 0) {
        Write-Error-Custom "Tauri build ошибка"
        exit 1
    }
    
    Write-Success "Tauri собран"
} catch {
    Write-Error-Custom "Ошибка: $_"
    exit 1
}

Write-Host ""
Write-Host "==============================" -ForegroundColor Green
Write-Host " ✓ Сборка завершена!" -ForegroundColor Green
Write-Host "==============================" -ForegroundColor Green
Write-Host ""

$exePath = "$scriptPath\gui\src-tauri\target\release\jarvis-app.exe"
if (Test-Path $exePath) {
    Write-Host "Файл: $exePath" -ForegroundColor Cyan
} else {
    Write-Warning "Исполняемый файл не найден"
}

Write-Host "Логи: $logFile" -ForegroundColor Cyan
