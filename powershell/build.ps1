Push-Location $PSScriptRoot
#region Init
$ModuleName = 'siquery'

# Check for .NET SDK
Try {
    Get-Command -Name 'dotnet' -CommandType 'Application' -ErrorAction Stop | Out-Null
} Catch {
    Throw 'Missing required build tool. Please install dotnet before attempting to build.'
}

# Check for Rust SDK
Try {
    Get-Command -Name 'cargo' -CommandType 'Application' -ErrorAction Stop | Out-Null
} Catch {
    Throw 'Missing required build tool. Please install rust/cargo before attempting to build.'
}

$PSModuleOutputPath = (Test-Path Env:PSMODULE_OUTPUT_PATH) ?
    $Env:PSMODULE_OUTPUT_PATH :
    (Join-Path -path $PSScriptRoot -childpath 'package')

$NewModuleOutputPath = Join-Path -Path $PSModuleOutputPath -ChildPath $ModuleName
Remove-Item -Path $NewModuleOutputPath -Recurse -Force -ErrorAction SilentlyContinue
#endregion Init


#region BuildModule
New-Item -Path $NewModuleOutputPath -ItemType 'Directory' -Force | Out-Null

@('bin', 'lib', 'schema', 'Public', 'Private') | ForEach-Object {
    New-Item -Path "$NewModuleOutputPath\$_" -ItemType 'Directory' -Force | Out-Null
}

Copy-Item -Path "$PSScriptRoot\schema" -Destination "$NewModuleOutputPath" -Recurse -Force

Copy-Item -Path "$PSScriptRoot\$ModuleName\Private" -Destination $NewModuleOutputPath -Recurse -Force
Copy-Item -Path "$PSScriptRoot\$ModuleName\Public" -Destination $NewModuleOutputPath -Recurse -Force

Copy-Item "$PSScriptRoot\$ModuleName\$ModuleName.psd1" -Destination $NewModuleOutputPath -Force
Copy-Item "$PSScriptRoot\$ModuleName\$ModuleName.psm1" -Destination $NewModuleOutputPath -Force

if ("$PSModuleOutputPath".EndsWith('package')) {
    # Copy tests to package folder for easier testing after build.
    Copy-Item -Path "$PSScriptRoot\pester" -Destination $PSModuleOutputPath -Recurse -Force
}
#endregion BuildModule


#region buildRustComponent
Push-Location
Set-Location '..'

& 'cargo' build --release

$ExecutableName = 'siquery'
if ($IsWindows) { $ExecutableName += '.exe' }

Copy-Item ".\target\release\$ExecutableName" -Destination "$NewModuleOutputPath\bin" -Recurse -Force

Remove-Item -Path '.\target' -Recurse -Force | Out-Null

Pop-Location
#endregion buildRustComponent


#region build.NetComponent
& dotnet nuget add source 'https://api.nuget.org/v3/index.json' -n 'nuget.org' | Out-Null

& dotnet publish "$PSScriptRoot\$ModuleName\src" -f 'netcoreapp3.1' -c 'Release' -o "$NewModuleOutputPath\lib"
#endregion build.NetComponent
Pop-Location
