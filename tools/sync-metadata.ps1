param(
    [string]$SourceRoot,
    [ValidateSet("All", "Root", "Converters", "Helpers", "Controls", "XamlToolkit.WinUI", "XamlToolkit.WinUI.Converters", "XamlToolkit.WinUI.Helpers", "XamlToolkit.WinUI.Controls")]
    [string[]]$Project = @("All"),
    [ValidateSet("All", "x64", "ARM64", "Win32")]
    [string[]]$Platform = @("x64"),
    [ValidateSet("x64", "ARM64", "Win32")]
    [string]$MetadataPlatform = "x64",
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

function Get-ProjectConfigs {
    @(
        [pscustomobject]@{
            Alias = "Root"
            Name = "XamlToolkit.WinUI"
            Crate = "xamltoolkit-winui"
            ProjectDir = "XamlToolkit.WinUI"
            ProjectFile = "XamlToolkit.WinUI.vcxproj"
            Winmd = "XamlToolkit.WinUI.winmd"
            DependencyWinmds = @()
        },
        [pscustomobject]@{
            Alias = "Converters"
            Name = "XamlToolkit.WinUI.Converters"
            Crate = "xamltoolkit-winui-converters"
            ProjectDir = "XamlToolkit.WinUI.Converters"
            ProjectFile = "XamlToolkit.WinUI.Converters.vcxproj"
            Winmd = "XamlToolkit.WinUI.Converters.winmd"
            DependencyWinmds = @()
        },
        [pscustomobject]@{
            Alias = "Helpers"
            Name = "XamlToolkit.WinUI.Helpers"
            Crate = "xamltoolkit-winui-helpers"
            ProjectDir = "XamlToolkit.WinUI.Helpers"
            ProjectFile = "XamlToolkit.WinUI.Helpers.vcxproj"
            Winmd = "XamlToolkit.WinUI.Helpers.winmd"
            DependencyWinmds = @("XamlToolkit.WinUI.winmd")
        },
        [pscustomobject]@{
            Alias = "Controls"
            Name = "XamlToolkit.WinUI.Controls"
            Crate = "xamltoolkit-winui-controls"
            ProjectDir = "XamlToolkit.WinUI.Controls"
            ProjectFile = "XamlToolkit.WinUI.Controls.vcxproj"
            Winmd = "XamlToolkit.WinUI.Controls.winmd"
            DependencyWinmds = @("XamlToolkit.WinUI.winmd", "XamlToolkit.WinUI.Helpers.winmd", "XamlToolkit.WinUI.Converters.winmd")
        }
    )
}

function Resolve-ProjectSelection([string[]]$ProjectNames, $ProjectConfigs) {
    if ($ProjectNames | Where-Object { $_ -eq "All" }) {
        return @($ProjectConfigs)
    }

    $selected = @()
    foreach ($projectName in $ProjectNames) {
        $match = $ProjectConfigs | Where-Object {
            $_.Alias -eq $projectName -or $_.Name -eq $projectName
        } | Select-Object -First 1

        if (!$match) {
            throw "Unknown project '$projectName'."
        }

        if (!($selected | Where-Object { $_.Name -eq $match.Name })) {
            $selected += $match
        }
    }

    return $selected
}

function Resolve-PlatformSelection([string[]]$PlatformNames) {
    if ($PlatformNames | Where-Object { $_ -eq "All" }) {
        return @("x64", "ARM64", "Win32")
    }

    $selected = @()
    foreach ($platformName in $PlatformNames) {
        if (!($selected | Where-Object { $_ -eq $platformName })) {
            $selected += $platformName
        }
    }

    return $selected
}

function Get-ProjectPackagePath([string]$PackagesRoot, [string]$ProjectRoot, [string]$ProjectFile, [string]$PackageId) {
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

    $projectPath = Join-Path $ProjectRoot $ProjectFile
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

function Get-ProjectOutput($ProjectConfig, [string]$SourceRoot, [string]$PlatformName, [string]$ConfigurationName) {
    $candidates = @(
        (Join-Path $SourceRoot "$($ProjectConfig.ProjectDir)\$PlatformName\$ConfigurationName\$($ProjectConfig.Name)"),
        (Join-Path $SourceRoot "$PlatformName\$ConfigurationName\$($ProjectConfig.Name)")
    )

    if ($PlatformName -eq "Win32") {
        $candidates += (Join-Path $SourceRoot "$($ProjectConfig.ProjectDir)\$ConfigurationName\$($ProjectConfig.Name)")
        $candidates += (Join-Path $SourceRoot "$ConfigurationName\$($ProjectConfig.Name)")
    }

    return Get-ExistingPath $candidates "$($ProjectConfig.Name) $PlatformName|$ConfigurationName build output"
}

function Copy-NativeRuntime($ProjectConfig, [string]$ProjectOutput, [string]$Destination) {
    New-Item -ItemType Directory -Force $Destination | Out-Null

    $resourceDestination = Join-Path $Destination $ProjectConfig.Name
    if (Test-Path -LiteralPath $resourceDestination) {
        Remove-Item -LiteralPath $resourceDestination -Recurse -Force
    }

    foreach ($extension in @("dll", "pri", "winmd")) {
        $name = "$($ProjectConfig.Name).$extension"
        $source = Join-Path $ProjectOutput $name
        if (!(Test-Path -LiteralPath $source)) {
            throw "Missing native runtime artifact: $source"
        }

        Copy-Item -LiteralPath $source -Destination (Join-Path $Destination $name) -Force
    }

    $resourceSource = Join-Path $ProjectOutput $ProjectConfig.Name
    if (Test-Path -LiteralPath $resourceSource) {
        Copy-Item -LiteralPath $resourceSource -Destination $resourceDestination -Recurse -Force
    }
}

function Get-WindowsWinmdCandidate([string]$WorkspaceRoot, [string]$LocalWindowsWinmd) {
    $candidates = @(
        $LocalWindowsWinmd,
        (Join-Path $WorkspaceRoot "crates\xamltoolkit-winui\metadata\deps\Windows.winmd")
    )

    foreach ($candidate in $candidates) {
        if ($candidate -and (Test-Path -LiteralPath $candidate)) {
            return (Resolve-Path -LiteralPath $candidate).Path
        }
    }

    $programFilesX86 = [Environment]::GetFolderPath("ProgramFilesX86")
    $unionMetadata = Join-Path $programFilesX86 "Windows Kits\10\UnionMetadata"
    if (Test-Path -LiteralPath $unionMetadata) {
        $windowsWinmd = Get-ChildItem -LiteralPath $unionMetadata -Recurse -Filter "Windows.winmd" -File |
            Sort-Object FullName -Descending |
            Select-Object -First 1
        if ($windowsWinmd) {
            return $windowsWinmd.FullName
        }
    }

    return $null
}

function Copy-ToolkitDependencyMetadata($ProjectConfig, $AllProjectConfigs, [string]$WorkspaceRoot, [string]$SourceRoot, [string]$DepsDir, [string]$MetadataPlatformName, [string]$ConfigurationName) {
    foreach ($dependencyWinmd in $ProjectConfig.DependencyWinmds) {
        $dependencyProject = $AllProjectConfigs | Where-Object { $_.Winmd -eq $dependencyWinmd } | Select-Object -First 1
        if (!$dependencyProject) {
            throw "Unknown Toolkit dependency metadata '$dependencyWinmd' for $($ProjectConfig.Name)."
        }

        $source = Join-Path (Join-Path (Join-Path $WorkspaceRoot "crates") $dependencyProject.Crate) "metadata\$dependencyWinmd"
        if (!(Test-Path -LiteralPath $source)) {
            $dependencyOutput = Get-ProjectOutput $dependencyProject $SourceRoot $MetadataPlatformName $ConfigurationName
            $source = Join-Path $dependencyOutput $dependencyWinmd
        }

        if (!(Test-Path -LiteralPath $source)) {
            throw "Missing Toolkit dependency metadata '$dependencyWinmd'. Sync or build $($dependencyProject.Name) first."
        }

        Copy-Item -LiteralPath $source -Destination (Join-Path $DepsDir $dependencyWinmd) -Force
    }
}

function Sync-DependencyMetadata($ProjectConfig, $AllProjectConfigs, [string]$WorkspaceRoot, [string]$SourceRoot, [string]$PackagesRoot, [string]$DepsDir, [string]$MetadataPlatformName, [string]$ConfigurationName) {
    $projectRoot = Join-Path $SourceRoot $ProjectConfig.ProjectDir
    $projectPath = Join-Path $projectRoot $ProjectConfig.ProjectFile

    $winuiPackage = Get-ProjectPackagePath $PackagesRoot $projectRoot $ProjectConfig.ProjectFile "Microsoft.WindowsAppSDK.WinUI"
    $interactivePackage = Get-ProjectPackagePath $PackagesRoot $projectRoot $ProjectConfig.ProjectFile "Microsoft.WindowsAppSDK.InteractiveExperiences"
    $interactiveTarget = Get-InteractiveMetadataTarget (Join-Path $interactivePackage "metadata") (Get-ProjectMinVersion $projectPath)

    $deps = @(
        (Join-Path $winuiPackage "metadata\Microsoft.UI.Xaml.winmd"),
        (Join-Path $winuiPackage "metadata\Microsoft.UI.Text.winmd"),
        (Join-Path $interactiveTarget "Microsoft.UI.winmd"),
        (Join-Path $interactiveTarget "Microsoft.Foundation.winmd")
    )

    if ($ProjectConfig.Alias -eq "Controls") {
        $foundationPackage = Get-ProjectPackagePath $PackagesRoot $projectRoot $ProjectConfig.ProjectFile "Microsoft.WindowsAppSDK.Foundation"
        $deps += (Join-Path $foundationPackage "metadata\Microsoft.Windows.ApplicationModel.Resources.winmd")
    }

    foreach ($dep in $deps) {
        if (!(Test-Path -LiteralPath $dep)) {
            throw "Missing dependency metadata: $dep"
        }
    }

    New-Item -ItemType Directory -Force $DepsDir | Out-Null
    foreach ($dep in $deps) {
        Copy-Item -LiteralPath $dep -Destination (Join-Path $DepsDir (Split-Path -Leaf $dep)) -Force
    }

    $localWindowsWinmd = Join-Path $DepsDir "Windows.winmd"
    $windowsWinmd = Get-WindowsWinmdCandidate $WorkspaceRoot $localWindowsWinmd
    if ($windowsWinmd -and $windowsWinmd -ne (Resolve-Path -LiteralPath $localWindowsWinmd -ErrorAction SilentlyContinue).Path) {
        Copy-Item -LiteralPath $windowsWinmd -Destination $localWindowsWinmd -Force
    } elseif (!$windowsWinmd) {
        Write-Warning "Windows.winmd was not found. Existing builds may fail for Windows.UI.Xaml.Interop.TypeName."
    }

    Copy-ToolkitDependencyMetadata $ProjectConfig $AllProjectConfigs $WorkspaceRoot $SourceRoot $DepsDir $MetadataPlatformName $ConfigurationName

    $keepToolkitDeps = @($ProjectConfig.DependencyWinmds)
    foreach ($name in @("XamlToolkit.WinUI.winmd", "XamlToolkit.WinUI.Helpers.winmd", "XamlToolkit.WinUI.Converters.winmd")) {
        $staleDep = Join-Path $DepsDir $name
        if ((Test-Path -LiteralPath $staleDep) -and !($keepToolkitDeps | Where-Object { $_ -eq $name })) {
            Remove-Item -LiteralPath $staleDep -Force
        }
    }

    Write-Host "Synced dependency metadata for $($ProjectConfig.Name) from $winuiPackage and $interactiveTarget"
}

function Sync-ProjectMetadata($ProjectConfig, $AllProjectConfigs, [string]$WorkspaceRoot, [string]$SourceRoot, [string]$PackagesRoot, [string[]]$Platforms, [string]$MetadataPlatformName, [string]$ConfigurationName) {
    $crateRoot = Join-Path (Join-Path $WorkspaceRoot "crates") $ProjectConfig.Crate
    $metadataDir = Join-Path $crateRoot "metadata"
    $depsDir = Join-Path $metadataDir "deps"
    $nativeDir = Join-Path $metadataDir "native"

    if (!(Test-Path -LiteralPath $crateRoot)) {
        throw "Crate path does not exist for $($ProjectConfig.Name): $crateRoot"
    }

    New-Item -ItemType Directory -Force $metadataDir, $depsDir | Out-Null

    $topLevelPlatform = if ($Platforms | Where-Object { $_ -eq $MetadataPlatformName }) {
        $MetadataPlatformName
    } else {
        $Platforms[0]
    }

    $metadataOutput = Get-ProjectOutput $ProjectConfig $SourceRoot $topLevelPlatform $ConfigurationName
    $projectWinmd = Join-Path $metadataOutput $ProjectConfig.Winmd
    if (!(Test-Path -LiteralPath $projectWinmd)) {
        throw "Missing $projectWinmd. Build $($ProjectConfig.ProjectFile) for $topLevelPlatform|$ConfigurationName first."
    }

    Copy-Item -LiteralPath $projectWinmd -Destination (Join-Path $metadataDir $ProjectConfig.Winmd) -Force
    Sync-DependencyMetadata $ProjectConfig $AllProjectConfigs $WorkspaceRoot $SourceRoot $PackagesRoot $depsDir $MetadataPlatformName $ConfigurationName

    foreach ($platformName in $Platforms) {
        $projectOutput = Get-ProjectOutput $ProjectConfig $SourceRoot $platformName $ConfigurationName
        Copy-NativeRuntime $ProjectConfig $projectOutput (Join-Path $nativeDir $platformName)
        Write-Host "Synced $($ProjectConfig.Name) native runtime artifacts for $platformName from $projectOutput"
    }

    Write-Host "Synced $($ProjectConfig.Name) metadata from $metadataOutput"
}

$workspaceRoot = Split-Path -Parent $PSScriptRoot

if (!$SourceRoot) {
    $SourceRoot = Get-ExistingPath @(
        (Join-Path $workspaceRoot "submodules\CommunityToolkit.WinUI")
    ) "CommunityToolkit.WinUI source root"
} elseif (!(Test-Path -LiteralPath $SourceRoot)) {
    throw "SourceRoot does not exist: $SourceRoot"
}

$SourceRoot = (Resolve-Path -LiteralPath $SourceRoot).Path
$packagesRoot = Join-Path $SourceRoot "packages"
if (!(Test-Path -LiteralPath $packagesRoot)) {
    throw "Missing packages directory: $packagesRoot. Restore CommunityToolkit.WinUI packages first."
}

$projectConfigs = Get-ProjectConfigs
$selectedProjects = Resolve-ProjectSelection $Project $projectConfigs
$selectedPlatforms = Resolve-PlatformSelection $Platform

foreach ($projectConfig in $selectedProjects) {
    Sync-ProjectMetadata $projectConfig $projectConfigs $workspaceRoot $SourceRoot $packagesRoot $selectedPlatforms $MetadataPlatform $Configuration
}
