param(
    [string]$SourceRoot,
    [string]$Platform = "x64",
    [string]$Configuration = "Release"
)

$ErrorActionPreference = "Stop"

function Get-ExistingPath([string[]]$Candidates, [string]$Description) {
    foreach ($candidate in $Candidates) {
        if ($candidate -and (Test-Path -LiteralPath $candidate)) {
            return (Resolve-Path -LiteralPath $candidate).Path
        }
    }

    throw "Unable to find $Description. Checked: $($Candidates -join '; ')"
}

function Get-PackagePath([string]$PackagesRoot, [string]$Prefix) {
    $packages = Get-ChildItem -LiteralPath $PackagesRoot -Directory |
        Where-Object { $_.Name -like "$Prefix.*" } |
        Sort-Object Name -Descending

    if (!$packages) {
        throw "Unable to find package '$Prefix.*' under $PackagesRoot. Restore CommunityToolkit.WinUI packages first."
    }

    return $packages[0].FullName
}

function Get-ProjectPackagePath([string]$PackagesRoot, [string]$ProjectRoot, [string]$PackageId) {
    $packagesConfig = Join-Path $ProjectRoot "packages.config"
    if (Test-Path -LiteralPath $packagesConfig) {
        [xml]$packages = Get-Content -LiteralPath $packagesConfig -Raw
        $package = $packages.packages.package | Where-Object { $_.id -eq $PackageId } | Select-Object -First 1
        if ($package -and $package.version) {
            $path = Join-Path $PackagesRoot "$PackageId.$($package.version)"
            if (Test-Path -LiteralPath $path) {
                return (Resolve-Path -LiteralPath $path).Path
            }

            throw "Project references $PackageId $($package.version), but package path is missing: $path"
        }
    }

    $projectPath = Join-Path $ProjectRoot "XamlToolkit.WinUI.vcxproj"
    if (Test-Path -LiteralPath $projectPath) {
        $content = Get-Content -LiteralPath $projectPath -Raw
        $escaped = [regex]::Escape($PackageId)
        $match = [regex]::Match($content, "packages\\($escaped\.[^\\']+)\\build\\native")
        if ($match.Success) {
            $path = Join-Path $PackagesRoot $match.Groups[1].Value
            if (Test-Path -LiteralPath $path) {
                return (Resolve-Path -LiteralPath $path).Path
            }

            throw "Project imports $($match.Groups[1].Value), but package path is missing: $path"
        }
    }

    Write-Warning "Unable to infer $PackageId from project files. Falling back to highest package under $PackagesRoot."
    return Get-PackagePath $PackagesRoot $PackageId
}

function Get-ProjectMinVersion([string]$ProjectPath) {
    if (!(Test-Path -LiteralPath $ProjectPath)) {
        return $null
    }

    $content = Get-Content -LiteralPath $ProjectPath -Raw
    $match = [regex]::Match($content, '<WindowsTargetPlatformMinVersion>([^<]+)</WindowsTargetPlatformMinVersion>')
    if ($match.Success) {
        return $match.Groups[1].Value.Trim()
    }

    return $null
}

function Get-InteractiveMetadataTarget([string]$MetadataRoot, [string]$PreferredTarget) {
    $targets = Get-ChildItem -LiteralPath $MetadataRoot -Directory |
        Where-Object {
            (Test-Path -LiteralPath (Join-Path $_.FullName "Microsoft.Foundation.winmd")) -and
            (Test-Path -LiteralPath (Join-Path $_.FullName "Microsoft.UI.winmd"))
        } |
        Sort-Object Name

    if (!$targets) {
        throw "No InteractiveExperiences metadata target containing Microsoft.Foundation.winmd and Microsoft.UI.winmd found under $MetadataRoot."
    }

    if ($PreferredTarget) {
        $match = $targets | Where-Object { $_.Name -eq $PreferredTarget } | Select-Object -First 1
        if ($match) {
            return $match.FullName
        }

        Write-Warning "Preferred InteractiveExperiences target '$PreferredTarget' was not found. Using highest available target '$($targets[-1].Name)'."
    }

    return $targets[-1].FullName
}

function Get-ToolkitOutput([string]$SourceRoot, [string]$Platform, [string]$Configuration) {
    $candidates = @(
        (Join-Path $SourceRoot "$Platform\$Configuration\XamlToolkit.WinUI")
    )

    if ($Platform -eq "Win32") {
        $candidates += (Join-Path $SourceRoot "$Configuration\XamlToolkit.WinUI")
    }

    return Get-ExistingPath $candidates "XamlToolkit.WinUI $Platform|$Configuration build output"
}

function Copy-NativeRuntime([string]$ToolkitOutput, [string]$Destination) {
    New-Item -ItemType Directory -Force $Destination | Out-Null

    foreach ($name in @("XamlToolkit.WinUI.dll", "XamlToolkit.WinUI.pri", "XamlToolkit.WinUI.winmd")) {
        $source = Join-Path $ToolkitOutput $name
        if (!(Test-Path -LiteralPath $source)) {
            throw "Missing native runtime artifact: $source"
        }

        Copy-Item -LiteralPath $source -Destination (Join-Path $Destination $name) -Force
    }
}

$crateRoot = $PSScriptRoot
$workspaceRoot = Split-Path -Parent (Split-Path -Parent $crateRoot)

if (!$SourceRoot) {
    $SourceRoot = Get-ExistingPath @(
        (Join-Path $workspaceRoot "CommunityToolkit.WinUI"),
        (Join-Path (Split-Path -Parent $workspaceRoot) "CommunityToolkit.WinUI")
    ) "CommunityToolkit.WinUI source root"
} elseif (!(Test-Path -LiteralPath $SourceRoot)) {
    throw "SourceRoot does not exist: $SourceRoot"
}

$SourceRoot = (Resolve-Path -LiteralPath $SourceRoot).Path
$toolkitOutput = Get-ToolkitOutput $SourceRoot $Platform $Configuration
$toolkitWinmd = Join-Path $toolkitOutput "XamlToolkit.WinUI.winmd"
$projectRoot = Join-Path $SourceRoot "XamlToolkit.WinUI"
$projectPath = Join-Path $projectRoot "XamlToolkit.WinUI.vcxproj"
$packagesRoot = Join-Path $SourceRoot "packages"
$metadataDir = Join-Path $crateRoot "metadata"
$depsDir = Join-Path $metadataDir "deps"
$nativeDir = Join-Path $metadataDir "native"

if (!(Test-Path -LiteralPath $toolkitWinmd)) {
    throw "Missing $toolkitWinmd. Build XamlToolkit.WinUI.vcxproj for $Platform|$Configuration first."
}

if (!(Test-Path -LiteralPath $packagesRoot)) {
    throw "Missing packages directory: $packagesRoot. Restore CommunityToolkit.WinUI packages first."
}

$winuiPackage = Get-ProjectPackagePath $packagesRoot $projectRoot "Microsoft.WindowsAppSDK.WinUI"
$interactivePackage = Get-ProjectPackagePath $packagesRoot $projectRoot "Microsoft.WindowsAppSDK.InteractiveExperiences"
$interactiveTarget = Get-InteractiveMetadataTarget (Join-Path $interactivePackage "metadata") (Get-ProjectMinVersion $projectPath)

$deps = @(
    (Join-Path $winuiPackage "metadata\Microsoft.UI.Xaml.winmd"),
    (Join-Path $winuiPackage "metadata\Microsoft.UI.Text.winmd"),
    (Join-Path $interactiveTarget "Microsoft.UI.winmd"),
    (Join-Path $interactiveTarget "Microsoft.Foundation.winmd")
)

foreach ($dep in $deps) {
    if (!(Test-Path -LiteralPath $dep)) {
        throw "Missing dependency metadata: $dep"
    }
}

New-Item -ItemType Directory -Force $metadataDir, $depsDir | Out-Null
Copy-Item -LiteralPath $toolkitWinmd -Destination (Join-Path $metadataDir "XamlToolkit.WinUI.winmd") -Force
foreach ($dep in $deps) {
    Copy-Item -LiteralPath $dep -Destination (Join-Path $depsDir (Split-Path -Leaf $dep)) -Force
}
Copy-NativeRuntime $toolkitOutput (Join-Path $nativeDir $Platform)

Write-Host "Synced XamlToolkit.WinUI metadata from $toolkitOutput"
Write-Host "Synced native runtime artifacts to $(Join-Path $nativeDir $Platform)"
Write-Host "Synced dependency metadata from $winuiPackage and $interactiveTarget"

