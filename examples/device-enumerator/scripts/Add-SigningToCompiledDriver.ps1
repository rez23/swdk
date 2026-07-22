param (
    [string]$PkgName = "xmouseinput_sys",
    [string]$CargoTargetDir = "target\debug",
    [string]$CertSubject = "CN=xmouseinput-sys-test"
)

$PKG_SRC_PATH = Join-Path $PWD "${CargoTargetDir}\${PkgName}_package"

# --- 1. CARICAMENTO EWDK AMBIENTE ---
$WdkEnvScript = Join-Path $PSScriptRoot "Set-WdkEnvironment.ps1"
if (Test-Path $WdkEnvScript) {
    Write-Host "[*] Loading EWDK environment from ${PKG_SRC_PATH}\Set-WdkEnvironment.ps1..." -ForegroundColor Cyan
    . $WdkEnvScript
} else {
    throw "[*] Set-WdkEnvironment.ps1 not found in $PSScriptRoot."
}


if (-not (Test-Path $PKG_SRC_PATH)) {
    throw "[*] Package folder not found: $PKG_SRC_PATH. Build the driver first with cargo!"
}

# --- 2. GESTIONE CERTIFICATO DI TEST ---
$CertStore = "Cert:\CurrentUser\My"
$Cert = Get-ChildItem -Path $CertStore | Where-Object { $_.Subject -eq $CertSubject } | Select-Object -First 1

if (-not $Cert) {
    Write-Host "[*] Test certificate '$CertSubject' not found. Creation in progress..." -ForegroundColor Yellow
    # Creiamo un certificato auto-firmato per Code Signing (compatibile con Kernel Test Signing)
    $Cert = New-SelfSignedCertificate -Type CodeSigningCert -Subject $CertSubject -CertStoreLocation $CertStore -KeyAlgorithm RSA -KeyLength 2048 -NotAfter (Get-Date).AddYears(5)
    Write-Host "[+] Test certificate created successfully (Thumbprint: $($Cert.Thumbprint))!" -ForegroundColor Green
} else {
    Write-Host "[*] Use existing cert: $($Cert.Subject) ($($Cert.Thumbprint))" -ForegroundColor Cyan
}

$PathForCat = 'F:\Program Files\Windows Kits\10\bin\10.0.28000.0\x86\Inf2Cat.exe'
# --- 3. GENERAZIONE DEL CATALOGO (.cat) ---
Write-Host "[*] Generating catalog file (.cat) using Inf2Cat..." -ForegroundColor Cyan
$Process = Start-Process $PathForCat -Args "/driver:${PKG_SRC_PATH} /os:10_x64 /verbose" -NoNewWindow -Wait -PassThru
if ($Process.ExitCode -ne 0) {
    throw "[*] Catalog file generation failed with exit code $LASTEXITCODE"
}

# --- 4. FIRMA DI .sys E .cat ---
Write-Host "[*] Signing driver and catalog..." -ForegroundColor Cyan
$SysFile = Join-Path $PKG_SRC_PATH "${PkgName}.sys"
$CatFile = Join-Path $PKG_SRC_PATH "${PkgName}.cat"

if (-not (Test-Path $SysFile)) {
    throw "[*] Driver file not found: $SysFile"
}
if (-not (Test-Path $CatFile)) {
    throw "[*] Catalog file not found: $CatFile"
}

# Firmiamo il driver con fallback offline se il timestamp server remoto non risponde
try {
    Write-Host "[*] Firm $SysFile with timestamp..." -ForegroundColor Cyan
    signtool sign /v /fd SHA256 /sha1 $Cert.Thumbprint /tr http://timestamp.digicert.com /td SHA256 $SysFile
} catch {
    Write-Warning "[*] Firm with timestamp failed. Attempting local signing without timestamp..."
    signtool sign /v /fd SHA256 /sha1 $Cert.Thumbprint $SysFile
}

try {
    Write-Host "[*] Firm di $CatFile with timestamp..." -ForegroundColor Cyan
    signtool sign /v /fd SHA256 /sha1 $Cert.Thumbprint /tr http://timestamp.digicert.com /td SHA256 $CatFile
} catch {
    Write-Warning "[*] Firm with timestamp failed. Attempting local signing without timestamp..."
    signtool sign /v /fd SHA256 /sha1 $Cert.Thumbprint $CatFile
}

# Export public cert (copy on VM)
Write-Host "[*] Export public cert..." -ForegroundColor DarkMagenta
$CertCerPath = Join-Path $PKG_SRC_PATH "${PkgName}.cer"
Export-Certificate -Cert $Cert -FilePath $CertCerPath -Force
Write-Host "[+] Firm success! Public cert exported to: $CertCerPath" -ForegroundColor Green
