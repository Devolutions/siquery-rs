$ErrorActionPreference = 'Stop'

# Load Private Functions
$PrivateFunctions = Get-ChildItem "$PSScriptRoot\Private\*.ps1" -Exclude '*.Tests.ps1'
Foreach ($Function in $PrivateFunctions) { . $Function.FullName }

# Load Public Functions
$PublicFunctions = Get-ChildItem "$PSScriptRoot\Public\*.ps1" -Exclude '*.Tests.ps1'
Foreach ($Function in $PublicFunctions) {
    . $Function.FullName
    Export-ModuleMember -Function $Function.BaseName -Alias '*' -Verbose
}

# Detect siquery executable
Find-SiqExecutable
