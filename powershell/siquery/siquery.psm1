$ErrorActionPreference = 'Stop'

$ModuleName = $(Get-Item $PSCommandPath).BaseName
$Manifest = Import-PowerShellDataFile -Path $(Join-Path $PSScriptRoot "${ModuleName}.psd1")

# Load Private Functions
$PrivateFunctions = Get-ChildItem "$PSScriptRoot\Private\*.ps1" -Exclude '*.Tests.ps1'
Foreach ($Function in $PrivateFunctions) { . $Function.FullName }

# Load Public Functions
$PublicFunctions = Get-ChildItem "$PSScriptRoot\Public\*.ps1" -Exclude '*.Tests.ps1'
Foreach ($Function in $PublicFunctions) { . $Function.FullName }

Export-ModuleMember -Cmdlet @($manifest.CmdletsToExport)
Export-ModuleMember -Function @($Manifest.FunctionsToExport)

# Detect siquery executable
Find-SiqExecutable
