param(
    [string]$Configuration = "Release",
    [string]$Platform = "x64",
    [string]$WindowsAppSdkWinUIVersion = "1.8.260224000"
)

$ErrorActionPreference = "Stop"

$repoRoot = Split-Path -Parent $PSScriptRoot
$toolkitWinmd = Join-Path $repoRoot "$Platform\$Configuration\XamlToolkit.WinUI\XamlToolkit.WinUI.winmd"
$metadataDir = Join-Path $PSScriptRoot "metadata"
$depsDir = Join-Path $metadataDir "deps"
$nugetRoot = Join-Path $env:USERPROFILE ".nuget\packages"
$xamlWinmd = Join-Path $nugetRoot "microsoft.windowsappsdk.winui\$WindowsAppSdkWinUIVersion\metadata\Microsoft.UI.Xaml.winmd"

New-Item -ItemType Directory -Force $metadataDir, $depsDir | Out-Null

if (!(Test-Path -LiteralPath $toolkitWinmd)) {
    throw "Missing $toolkitWinmd. Build XamlToolkit.WinUI.vcxproj for $Platform|$Configuration first."
}

if (!(Test-Path -LiteralPath $xamlWinmd)) {
    throw "Missing $xamlWinmd. Restore/install Microsoft.WindowsAppSDK.WinUI $WindowsAppSdkWinUIVersion, or pass -WindowsAppSdkWinUIVersion."
}

Copy-Item -LiteralPath $toolkitWinmd -Destination (Join-Path $metadataDir "XamlToolkit.WinUI.winmd") -Force
Copy-Item -LiteralPath $xamlWinmd -Destination (Join-Path $depsDir "Microsoft.UI.Xaml.winmd") -Force

Write-Host "Copied metadata into $metadataDir"
