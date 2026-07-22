param(
    [string]$TargetMachine
)
. $PSScriptRoot\\Set-WdkEnvironment.ps1

$PowershellCmd = "powershell.exe -Command"
$SshSourceScript = 'C:\Set-WdkEnvironment.ps1'
$SshScript = "F:\Program Files\Windows Kits\10\Debuggers\x64\kd.exe"
$args = '-kl -c "g"'
$Key="703rcm20hgat.w2fx0vm18tli.2hwy01n37um14.3etihsokzzwk1"

$CmdPath="F:\Program Files\Windows Kits\10\Debuggers\x64\kd.exe"
$Args="-k net:port=50000,key=${Key}"
$Env:_NT_SYMBOL_PATH = "srv*C:\Symbols*https://msdl.microsoft.com/download/symbols;C:\Users\spart\RustroverProjects\xmouseinput-sys\target\debug\xmouseinput_sys_package"
$Process = Start-Process $CmdPath -Args $Args -NoNewWindow -Wait -PassThru
if ($Process.ExitCode -ne 0) {
    throw "[*] Catalog file generation failed with exit code $LASTEXITCODE"
}
