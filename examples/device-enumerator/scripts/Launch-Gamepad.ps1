Write-Host "Starting gamepad simulation..."
Invoke-Command `
    -VMName "Windows 11 WDK Environment" `
    -Credential (Import-Clixml $env:USERPROFILE\.cert\wdkcert.xml) `
    -ScriptBlock {python.exe $env:USERPROFILE\x_controller.py}