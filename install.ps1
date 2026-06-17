# wow language installer for Windows
# Usage: irm https://raw.githubusercontent.com/wow-language/wow/main/install.ps1 | iex
param(
    [string]$Version = ""
)

$ErrorActionPreference = "Stop"
$Repo = "wow-language/wow"

# ── Fetch latest version if not pinned ───────────────────────────────────────
if (-not $Version) {
    Write-Host "Fetching latest wow version..."
    try {
        $release = Invoke-RestMethod "https://api.github.com/repos/$Repo/releases/latest"
        $Version = $release.tag_name
    } catch {
        Write-Error "Could not fetch latest release. Visit https://github.com/$Repo/releases"
        exit 1
    }
}

Write-Host "Installing wow $Version..."

# ── Build download URL ────────────────────────────────────────────────────────
$Archive = "wow-$Version-x86_64-windows.zip"
$Url = "https://github.com/$Repo/releases/download/$Version/$Archive"

# ── Download ──────────────────────────────────────────────────────────────────
$TmpDir = Join-Path $env:TEMP "wow-install-$(Get-Random)"
New-Item -ItemType Directory -Path $TmpDir | Out-Null

$ArchivePath = Join-Path $TmpDir $Archive
Write-Host "Downloading $Archive..."
try {
    Invoke-WebRequest -Uri $Url -OutFile $ArchivePath -UseBasicParsing
} catch {
    Write-Error "Download failed from $Url`n$_"
    Remove-Item -Recurse -Force $TmpDir
    exit 1
}

# ── Extract ───────────────────────────────────────────────────────────────────
Expand-Archive -Path $ArchivePath -DestinationPath $TmpDir -Force

# ── Install ───────────────────────────────────────────────────────────────────
$InstallDir = Join-Path $env:LOCALAPPDATA "wow\bin"
if (-not (Test-Path $InstallDir)) {
    New-Item -ItemType Directory -Path $InstallDir | Out-Null
}

Copy-Item -Path (Join-Path $TmpDir "wow.exe") -Destination (Join-Path $InstallDir "wow.exe") -Force

# ── Cleanup ───────────────────────────────────────────────────────────────────
Remove-Item -Recurse -Force $TmpDir

# ── Add to user PATH (persists across sessions) ───────────────────────────────
$UserPath = [System.Environment]::GetEnvironmentVariable("PATH", "User")
if ($UserPath -notlike "*$InstallDir*") {
    [System.Environment]::SetEnvironmentVariable(
        "PATH",
        "$UserPath;$InstallDir",
        "User"
    )
    Write-Host ""
    Write-Host "  Added $InstallDir to your PATH."
    Write-Host "  Restart your terminal for 'wow' to be available in new sessions."
}

# Update PATH in the current session immediately
$env:PATH = "$env:PATH;$InstallDir"

# ── Verify ────────────────────────────────────────────────────────────────────
try {
    & "$InstallDir\wow.exe" --version | Out-Null
    Write-Host ""
    Write-Host "  wow $Version installed successfully!"
    Write-Host "  Run: wow run myprog.wow"
    Write-Host ""
} catch {
    Write-Host "Warning: Installed but could not verify. Check $InstallDir\wow.exe"
}
