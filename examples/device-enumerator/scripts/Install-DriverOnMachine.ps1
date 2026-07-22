param ([string]$PkgName = "xmouseinput_sys", [string]$CargoTargetDir = "target\debug", [string]$PkgTargetDestDir = "C:\test", [bool]$CleanOldVersion = $true, [bool]$CleanOldDriverOnTargetMachine = $true, [string]$VMName = "Windows 11 WDK Environment", [string]$CredentialPath = "$env:USERPROFILE\.cert\wdkcert.xml", [switch]$UninstallOnly)

$PkgName = "xmouseinput_sys"
$CurrDate = "$( (Get-Date -Format yyyyMMddHHmmss) )"
$PkgDestPath = "${PkgTargetDestDir}\${PkgName}_${CurrDate}"
$PkgSrcPath = "${PWD}\${CargoTargetDir}\${PkgName}_package"
$Credential = Import-Clixml -Path $CredentialPath -ErrorAction Stop
$ArchiveDestPath = "${PWD}\target\${PkgName}_${CurrDate}.zip"
Write-Host "INFO: Try to starting $( $VMName )" -ForegroundColor Cyan
Start-VM -VMName $VMName -ErrorAction Stop

if (-not $UninstallOnly) {
    # Create archive package
    Write-Host "INFO: Creating archive package" -ForegroundColor Cyan
    Compress-Archive -Path "${PkgSrcPath}\*" -DestinationPath $ArchiveDestPath -Force -ErrorAction Stop

    # Copy the file and install on VM
    Write-Host "INFO: Copying ${PkgSrcPath} to ${PkgDestPath}" -ForegroundColor Cyan
    Copy-VMFile -FileSource Host `
            -SourcePath "${ArchiveDestPath}" `
            -DestinationPath "${PkgDestPath}.zip" `
            -VMName $VMName `
            -CreateFullPath `
            -Force `
            -ErrorAction Stop

    Write-Host "INFO: Installing driver on WDK machine" -ForegroundColor Cyan
}
if ($CleanOldDriverOnTargetMachine) {
    # clean old drivers version on WDK machine
    Invoke-Command -VMName $VMName -Credential $Credential -ScriptBlock {
        pnputil.exe /enum-drivers |
        Select-String "${using:PkgName}.inf" -Context 1 |
        ForEach-Object {
            $_.Context.PreContext[0] -replace "Pubblished name:\s+", ""
        } | ForEach-Object {
                    $DriverInstalledInf = $_.Trim() -replace "Published name:\s+", ""
                    Write-Host "INFO: Found old driver verison named '${DriverInstalledInf}'. Uninstalling..." -ForegroundColor DarkYellow
                    pnputil.exe /delete-driver $DriverInstalledInf /uninstall
        } -ErrorAction Stop
    } -ErrorAction Stop
}

if ($UninstallOnly) {
    return
}

# Get wdk cert on target machine
$WdrCertOnMachine = Invoke-Command `
    -VMName $VMName `
    -Credential $Credential `
    -ScriptBlock {
    Get-ChildItem Cert:\ -Recurse |
         Where-Object Subject -eq "CN=WDRLocalTestCert"
}
#Check if WdkCert is installed
if (-not $WdrCertOnMachine) {
    $InstallCert = Read-Host "INFO: Driver cert not found on $VMName. Install it? (y/n)"
    if ($InstallCert -eq "y") {
        Write-Host "INFO: Copying Wdr certificatesd on VM" -ForegroundColor DarkMagenta
        Copy-VMFile `
            -VMName $VMName `
            -SourcePath "${PWD}\WDRLocalTestCert.cer" `
            -DestinationPath "C:\test\WDRLocalTestCert.cer" `
            -FileSource Host `
            -CreateFullPath `
            -Force
        Invoke-Command -VMName $VMName -Credential $Credential -ScriptBlock {

            Write-Host "INFO: Importing driver certificate on VM" -ForegroundColor DarkMagenta
            Import-Certificate `
                -FilePath "C:\test\WDRLocalTestCert.cer" `
                -CertStoreLocation Cert:\LocalMachine\Root `
                -ErrorAction Stop

            Import-Certificate `
                -FilePath "C:\test\WDRLocalTestCert.cer" `
                -CertStoreLocation Cert:\LocalMachine\TrustedPublisher `
                -ErrorAction Stop
        } -ErrorAction Stop
    }
    else { exit 0 }
}

# install package on VM
Write-Host "INFO: Installing driver on WDK machine" -ForegroundColor DarkCyan
Invoke-Command -VMName $VMName -Credential $Credential -ScriptBlock {
    New-Item -ItemType Directory -Path "${using:PkgTargetDestDir}\${using:PkgName}_${using:CurrDate}" -Force
    Expand-Archive `
        -Path "${using:PkgDestPath}.zip" `
        -DestinationPath "${using:PkgDestPath}" -Force

    Write-Host "INFO: Install driver" -ForegroundColor DarkMagenta
    pnputil.exe /add-driver "${using:PkgDestPath}\${using:PkgName}.inf" /install /force
} -ErrorAction Stop

$Result = Invoke-Command -VMName $VMName -Credential $Credential -ScriptBlock {
    pnputil.exe /enum-drivers | Select-String "${using:PkgName}.inf" -Context 1
}

if (-not $Result) {
    Write-Error "Driver installed succesfully but not found in target machine"
    exit 1
}