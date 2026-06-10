param(
    [string]$WindowsSdkVersion = "10.0.26100.0",
    [string]$WindowsAppSdkWinUIVersion = "1.8.260224000",
    [string]$InteractiveExperiencesVersion = "1.8.260125001",
    [string]$InteractiveExperiencesTarget = "10.0.18362.0"
)

$ErrorActionPreference = "Stop"

$repoRoot = Split-Path -Parent $PSScriptRoot
$windowsKits = "C:\Program Files (x86)\Windows Kits\10"
$midlrt = Join-Path $windowsKits "bin\$WindowsSdkVersion\arm64\midlrt.exe"
$sdkInclude = Join-Path $windowsKits "Include\$WindowsSdkVersion\winrt"
$sdkRefs = Join-Path $windowsKits "References\$WindowsSdkVersion"
$sdkUnion = Join-Path $windowsKits "UnionMetadata\$WindowsSdkVersion"
$nugetRoot = Join-Path $env:USERPROFILE ".nuget\packages"
$outDir = Join-Path $PSScriptRoot "metadata-gen"
$metadataDir = Join-Path $PSScriptRoot "metadata"
$depsDir = Join-Path $metadataDir "deps"

$references = @(
    (Join-Path $sdkRefs "Windows.Foundation.FoundationContract\4.0.0.0\Windows.Foundation.FoundationContract.winmd"),
    (Join-Path $sdkRefs "Windows.Foundation.UniversalApiContract\19.0.0.0\Windows.Foundation.UniversalApiContract.winmd"),
    (Join-Path $nugetRoot "microsoft.windowsappsdk.interactiveexperiences\$InteractiveExperiencesVersion\metadata\$InteractiveExperiencesTarget\Microsoft.Foundation.winmd"),
    (Join-Path $nugetRoot "microsoft.windowsappsdk.interactiveexperiences\$InteractiveExperiencesVersion\metadata\$InteractiveExperiencesTarget\Microsoft.UI.winmd"),
    (Join-Path $nugetRoot "microsoft.windowsappsdk.winui\$WindowsAppSdkWinUIVersion\metadata\Microsoft.UI.Xaml.winmd"),
    (Join-Path $nugetRoot "microsoft.windowsappsdk.winui\$WindowsAppSdkWinUIVersion\metadata\Microsoft.UI.Text.winmd")
)

$required = @($midlrt, $sdkInclude, $sdkRefs, $sdkUnion) + $references
foreach ($path in $required) {
    if (!(Test-Path -LiteralPath $path)) {
        throw "Missing required path: $path"
    }
}

New-Item -ItemType Directory -Force $outDir, $metadataDir, $depsDir | Out-Null

$args = @(
    "/nologo",
    "/winrt",
    "/nomidl",
    "/no_cpp",
    "/I", $sdkInclude,
    "/metadata_dir", $sdkRefs,
    "/metadata_dir", $sdkUnion
)

foreach ($reference in $references) {
    $args += @("/reference", $reference)
}

$args += @(
    "/out", $outDir,
    "/winmd", "XamlToolkit.WinUI.winmd",
    (Join-Path $repoRoot "XamlToolkit.WinUI\component.idl")
)

& $midlrt @args
if ($LASTEXITCODE -ne 0) {
    throw "midlrt failed with exit code $LASTEXITCODE"
}

Copy-Item -LiteralPath (Join-Path $outDir "XamlToolkit.WinUI.winmd") -Destination (Join-Path $metadataDir "XamlToolkit.WinUI.winmd") -Force
foreach ($reference in $references) {
    Copy-Item -LiteralPath $reference -Destination (Join-Path $depsDir (Split-Path -Leaf $reference)) -Force
}
Copy-Item -LiteralPath (Join-Path $sdkUnion "Windows.winmd") -Destination (Join-Path $depsDir "Windows.winmd") -Force

Write-Host "Generated and copied metadata for xamltoolkit-winui."
