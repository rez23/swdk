# Copy and paste this block into your PowerShell window to set all EWDK environment variables:

$ewdkEnv = @{
    WDKContentRoot              = "f:\Program Files\Windows Kits\10\"
    LLVM_HOME                   = "C:\Users\spart\RustroverProjects\xmouseinput-sys\external\LLVM_21.1.2"
    UCRTContentRoot             = "f:\Program Files\Windows Kits\10\"
    BuildBranch                 = "br_release_svc_im"
    BuildLab                    = "br_release_svc_im.28000.1839"
    WindowsSdkDir               = "f:\Program Files\Windows Kits\10\"
    UniversalCRTSdkDir_10       = "f:\Program Files\Windows Kits\10\"
    WDKBinRoot                  = "f:\Program Files\Windows Kits\10\bin\10.0.28000.0\"
    ToolsPath                   = "f:\Program Files\Windows Kits\10\tools"
    WDKToolRoot                 = "f:\Program Files\Windows Kits\10\Tools\10.0.28000.0\"
    WDKBuildBinRoot             = "f:\Program Files\Windows Kits\10\build\10.0.28000.0\bin\"
    EnterpriseWDK               = "True"
    VSINSTALLDIR_180            = "f:\Program Files\Microsoft Visual Studio\18\BuildTools\"
    BuildLabSetupRoot           = "f:\"
    UniversalCRTSdkDir          = "f:\Program Files\Windows Kits\10\"
    BuildLabSetupFilesRoot      = "f:\Program Files"
    LIBCLANG_PATH               = "C:\Users\spart\RustroverProjects\xmouseinput-sys\external\LLVM_21.1.2\bin"
    MSBUILDSDKREFERENCEDIRECTORY= "f:\Program Files"
    VCToolsInstallDir_180       = "f:\Program Files\Microsoft Visual Studio\18\BuildTools\VC\Tools\MSVC\14.50.35717\"
    WDK_CURRENT_KIT_VERSION     = "10"
    WindowsTargetPlatformVersion= "10.0.28000.0"
    DisableRegistryUse          = "True"
    ToolsPathARCH               = "f:\Program Files\Windows Kits\10\tools\10.0.28000.0\x64"
    NETFXKitsDir                = "f:\Program Files\Windows Kits\NETFXSDK\4.8.1\"
    VCTargetsPath              = "f:\Program Files\Microsoft Visual Studio\18\BuildTools\MSBuild\Microsoft\VC\v180\"
    DotNetVersionNumber         = "4.8.1"
    DotNetSDKRoot               = "f:\Program Files\Windows Kits\NETFXSDK\4.8.1\"
    Version_Number              = "10.0.28000.0"
    WindowsSdkVerBinPath        = "f:\Program Files\Windows Kits\10\bin\10.0.28000.0\x64"
    FrameworkPathOverride       = "f:\Program Files\Reference Assemblies\Microsoft\Framework\.NETFramework\v4.8.1\"
    Platform                    = "x64"
}

# Apply all variables to the process environment
foreach ($key in $ewdkEnv.Keys) {
    [System.Environment]::SetEnvironmentVariable($key, $ewdkEnv[$key], [System.EnvironmentVariableTarget]::Process)
}

# Set the updated Path variable
$env:Path = "C:\WINDOWS\system32;C:\WINDOWS;C:\WINDOWS\System32\Wbem;C:\WINDOWS\System32\WindowsPowerShell\v1.0\;C:\WINDOWS\System32\OpenSSH\;C:\Program Files\dotnet\;C:\Program Files\NVIDIA Corporation\NVIDIA App\NvDLISR;C:\Program Files (x86)\NVIDIA Corporation\PhysX\Common;C:\Program Files\Git\cmd;C:\Program Files\Docker\Docker\resources\bin;C:\Program Files\Gpg4win\..\GnuPG\bin;C:\Users\spart\AppData\Local\Programs\Python\Launcher\;C:\Users\spart\.cargo\bin;C:\Users\spart\AppData\Local\Microsoft\WindowsApps;C:\Users\spart\AppData\Local\Microsoft\WinGet\Packages\marlocarlo.OmpManager_Microsoft.Winget.Source_8wekyb3d8bbwe;C:\Users\spart\AppData\Local\JetBrains\Toolbox\scripts;C:\Users\spart\.dotnet\tools;C:\Users\spart\AppData\Local\PowerToys\DSCModules\;C:\Users\spart\AppData\Local\Microsoft\WinGet\Packages\ar51an.iPerf3_Microsoft.Winget.Source_8wekyb3d8bbwe;C:\Users\spart\AppData\Local\Programs\Microsoft VS Code\bin;C:\Users\spart\AppData\Local\Microsoft\WinGet\Links;C:\ghcup\bin;f:\Program Files\Windows Kits\10\bin\10.0.28000.0\x64;f:\Program Files\Windows Kits\10\bin\x64;f:\Program Files\Windows Kits\10\bin;f:\Program Files\Windows Kits\10\tools;f:\Program Files\Windows Kits\10\tools\10.0.28000.0\x64;f:\BuildEnv"

Write-Host "[*] Enterprise WDK Environment successfully loaded into PowerShell!" -ForegroundColor Green