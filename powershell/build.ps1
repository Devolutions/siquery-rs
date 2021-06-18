
$ModuleName = 'siquery'
Push-Location $PSScriptRoot

# Check for .NET SDK
Get-Command -Name 'dotnet' -CommandType 'Application' -ErrorAction Stop | Out-Null

# Check for Rust SDK
Get-Command -Name 'cargo' -CommandType 'Application' -ErrorAction Stop | Out-Null

if (Test-Path Env:PSMODULE_OUTPUT_PATH) {
    $PSModuleOutputPath = $Env:PSMODULE_OUTPUT_PATH
} else {
    $PSModuleOutputPath = Join-Path $PSScriptRoot 'package'
}

Remove-Item -Path "$PSModuleOutputPath\$ModuleName" -Recurse -Force -ErrorAction SilentlyContinue
New-Item -Path "$PSModuleOutputPath\$ModuleName" -ItemType 'Directory' -Force | Out-Null

@('bin', 'lib', 'schema', 'Public', 'Private') | % {
    New-Item -Path "$PSModuleOutputPath\$ModuleName\$_" -ItemType 'Directory' -Force | Out-Null
}

# copy schema

Copy-Item "$PSScriptRoot\schema" -Destination "$PSScriptRoot\$ModuleName\schema" -Recurse -Force
Copy-Item "$PSScriptRoot\$ModuleName\schema" -Destination "$PSModuleOutputPath\$ModuleName" -Recurse -Force

# build Rust component

New-Item -Path "$PSScriptRoot\$ModuleName\bin" -ItemType 'Directory' -Force | Out-Null

Push-Location
Set-Location ".."

& 'cargo' 'build' '--release'

$ExecutableName = "siquery"
if ($IsWindows) {
    $ExecutableName += ".exe"
}

Pop-Location
Copy-Item "..\target\release\${ExecutableName}" -Destination "$PSScriptRoot\$ModuleName\bin"

# build .NET component

& dotnet nuget add source "https://api.nuget.org/v3/index.json" -n "nuget.org" | Out-Null

& dotnet publish "$PSScriptRoot\$ModuleName\src" -f 'netcoreapp3.1' -c 'Release' -o "$PSScriptRoot\$ModuleName\lib"

Copy-Item "$PSScriptRoot\$ModuleName\bin" -Destination "$PSModuleOutputPath\$ModuleName" -Recurse -Force
Copy-Item "$PSScriptRoot\$ModuleName\lib" -Destination "$PSModuleOutputPath\$ModuleName" -Recurse -Force

Copy-Item "$PSScriptRoot\$ModuleName\Private" -Destination "$PSModuleOutputPath\$ModuleName" -Recurse -Force
Copy-Item "$PSScriptRoot\$ModuleName\Public" -Destination "$PSModuleOutputPath\$ModuleName" -Recurse -Force

Copy-Item "$PSScriptRoot\$ModuleName\$ModuleName.psd1" -Destination "$PSModuleOutputPath\$ModuleName" -Force
Copy-Item "$PSScriptRoot\$ModuleName\$ModuleName.psm1" -Destination "$PSModuleOutputPath\$ModuleName" -Force

Pop-Location
