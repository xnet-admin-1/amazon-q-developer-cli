# Windows Build Script for Amazon Q CLI
# This script checks prerequisites and builds the Windows version

$ErrorActionPreference = "Stop"

Write-Host "Amazon Q CLI - Windows Build Script" -ForegroundColor Cyan
Write-Host "=====================================" -ForegroundColor Cyan
Write-Host ""

# Check for Python
Write-Host "Checking for Python..." -ForegroundColor Yellow
try {
    $pythonVersion = python --version 2>&1
    Write-Host "  Found: $pythonVersion" -ForegroundColor Green
} catch {
    Write-Host "  ERROR: Python not found!" -ForegroundColor Red
    Write-Host "  Please install Python 3.8+ from https://www.python.org/downloads/" -ForegroundColor Red
    exit 1
}

# Check for Rust
Write-Host "Checking for Rust..." -ForegroundColor Yellow
try {
    $rustVersion = rustc --version 2>&1
    Write-Host "  Found: $rustVersion" -ForegroundColor Green
} catch {
    Write-Host "  ERROR: Rust not found!" -ForegroundColor Red
    Write-Host "  Please install Rust from https://rustup.rs" -ForegroundColor Red
    exit 1
}

# Check for MSVC target
Write-Host "Checking for x86_64-pc-windows-msvc target..." -ForegroundColor Yellow
$targets = rustup target list --installed
if ($targets -match "x86_64-pc-windows-msvc") {
    Write-Host "  Found: x86_64-pc-windows-msvc" -ForegroundColor Green
} else {
    Write-Host "  WARNING: x86_64-pc-windows-msvc target not installed" -ForegroundColor Yellow
    Write-Host "  Installing target..." -ForegroundColor Yellow
    rustup target add x86_64-pc-windows-msvc
    if ($LASTEXITCODE -ne 0) {
        Write-Host "  ERROR: Failed to install MSVC target" -ForegroundColor Red
        exit 1
    }
    Write-Host "  Installed: x86_64-pc-windows-msvc" -ForegroundColor Green
}

# Check for Visual Studio Build Tools
Write-Host "Checking for Visual Studio Build Tools..." -ForegroundColor Yellow
$vsWhere = "${env:ProgramFiles(x86)}\Microsoft Visual Studio\Installer\vswhere.exe"
if (Test-Path $vsWhere) {
    $vsInstall = & $vsWhere -latest -property installationPath
    if ($vsInstall) {
        Write-Host "  Found: Visual Studio at $vsInstall" -ForegroundColor Green
    } else {
        Write-Host "  WARNING: Visual Studio not found" -ForegroundColor Yellow
        Write-Host "  You may need to install Visual Studio Build Tools" -ForegroundColor Yellow
    }
} else {
    Write-Host "  WARNING: Could not verify Visual Studio installation" -ForegroundColor Yellow
    Write-Host "  If build fails, install from:" -ForegroundColor Yellow
    Write-Host "  https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio-2022" -ForegroundColor Yellow
}

Write-Host ""
Write-Host "Starting build..." -ForegroundColor Cyan
Write-Host ""

# Parse command line arguments
$buildArgs = @("build")
foreach ($arg in $args) {
    $buildArgs += $arg
}

# Run the Python build script
python scripts/main.py @buildArgs

if ($LASTEXITCODE -eq 0) {
    Write-Host ""
    Write-Host "Build completed successfully!" -ForegroundColor Green
    Write-Host "Output files are in the 'build' directory" -ForegroundColor Green
    Write-Host ""
    
    # List build artifacts
    if (Test-Path "build\qchat-windows-x64.zip") {
        Write-Host "Build artifacts:" -ForegroundColor Cyan
        Get-ChildItem "build\qchat-windows-x64.*" | ForEach-Object {
            Write-Host "  - $($_.Name)" -ForegroundColor White
        }
    }
} else {
    Write-Host ""
    Write-Host "Build failed with exit code $LASTEXITCODE" -ForegroundColor Red
    exit $LASTEXITCODE
}
