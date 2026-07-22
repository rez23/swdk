param (
    [string]$PkgName = "xmouseinput_sys",
    [string]$CargoTargetDir = "target\debug",
    [string]$PkgTargetDestDir = "C:\test",
    [string]$VMName = "Windows 11 WDK Environment",
    [bool]$CleanOldVersion = $true,
    [bool]$CleanOldDriverOnTargetMachine = $true,
    [bool]$InstallCertOnTargetMachine = $true,
    [bool]$SignDriver = $true
)

# Source custom hypervisor functions for facility
. $PSScriptRoot\Set-WdkVmManagerFunctions.ps1

# Set needeed stuff
$CurrentTimeStamp = "$( (Get-Date -Format yyyyMMddHHmmss) )"
$PkgDestPath = "${PkgTargetDestDir}\${PkgName}_${CurrentTimeStamp}"
$PkgSourcePath = "${PWD}\${CargoTargetDir}\${PkgName}_package"

if (-not (Test-Path ${PkgSourcePath})) {
    throw "[*] ${PkgName} not found. Run ``cargo wdk build`` to build it."
}

if ($SignDriver) {
    Write-Host "[*] Signing driver:" -ForegroundColor DarkYellow
    & "${PSScriptRoot}\Sign-Driver.ps1" -PkgName $PkgName -CargoTargetDir $CargoTargetDir -CertSubject "CN=xmouseinput-sys-test"
    if (-not $?) {
        throw "[*] Failed to sign ${PkgName}"
    }
}

if ($CleanOldVersion) {
    Write-Host "[*] Removing old version target folder:" -ForegroundColor DarkYellow
    Invoke-WdkGuest { Remove-Item -Path ${using:PkgTargetDestDir} -Recurse -Force }
    Invoke-WdkGuest { New-Item -ItemType Directory -Path ${using:PkgTargetDestDir} -Force}
    if (-not $?) {
        throw "[*] Failed to create target folder: SSH issue"
    }
}

Write-Host "[*] Copyng ${PkgName} to ${VMName}:${PkgDestPath}:" -ForegroundColor DarkGreen

if (-not $?) {
    throw "[*] Failed to copy package to target machine"
}

if ($CleanOldDriverOnTargetMachine) {
    Write-Host "[*] Uninstalling old ${PkgName} version from ${VMName}..." -ForegroundColor DarkYellow

    # Executing remote script via ssh
    # according with user setting
    Invoke-WdkGuest { pnputil /enum-drivers | Select-String "${PkgName}.inf" -Context 1 | ForEach-Object { `$_.Context.PreContext[0] -replace 'Published Name:\s+', '' } | ForEach-Object { pnputil /delete-driver $_ /uninstall /force } }
    if (-not $?) {
        throw "[*] Failed to uninstall old driver: SSH issue"
    }
}

if ($InstallCertOnTargetMachine) {
    if (Test-Path "${PkgSourcePath}\${PkgName}.cer") {
        Write-Host "[*] Installing certificate on target machine:" -ForegroundColor DarkYellow
        Invoke-WdkGuest {Import-Certificate -FilePath "${PkgDestPath}\${PkgName}.cer" -CertStoreLocation "Cert:\LocalMachine\Root"}
        if (-not $?) {
            throw "[*] Failed to install signig cert: SSH issue"
        }

        Invoke-WdkGuest {Import-Certificate -FilePath "${PkgDestPath}\${PkgName}.cer" -CertStoreLocation "Cert:\LocalMachine\TrustedPublisher"}
        if (-not $?) {
            throw "[*] Failed to install signig cert: SSH issue"
        }
    }
    else {
        throw "[*] Certificate file not found (use -SignDriver to generate): ${PkgSourcePath}\${PkgName}.cer"
    }
}

Write-Host "[*] Installing ${PkgName} on ${VMName}:" -ForegroundColor DarkGreen
Invoke-WdkGuest {pnputil /add-driver "${PkgDestPath}\${PkgName}.inf" /install}
if (-not $?) {
    throw "[*] Failed to install driver: SSH issue"
}
