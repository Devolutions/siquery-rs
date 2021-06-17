$ErrorActionPreference = 'Stop'

if ($IsWindows) {
    [System.Net.ServicePointManager]::SecurityProtocol = [System.Net.SecurityProtocolType]::Tls12
}

# Load Private Functions
$PrivateFunctions = Get-ChildItem "$PSScriptRoot\Private\*.ps1" -Exclude '*.Tests.ps1'
Foreach ($Function in $PrivateFunctions) { . $Function.FullName }

#Region TestCode
# Add siquery executable location to the path. I added this because I couldnt get the .dll working.
# if ($Env:PATH -inotlike '*siquery*') { $env:PATH += ($IsWindows) ? ';C:\siquery-rs' : ':/mnt/c/siquery-rs' }

# Please come up with a better method than this - its just meant as POC!
$AllOsCompatCount = 11
# Check for first run
$PublicFunctions = Get-ChildItem "$PSScriptRoot\Public\" -Exclude '*.Tests.ps1'
if ($PublicFunctions.Count -eq $AllOsCompatCount) { New-SiqOSSpecificFunctions }
#EndRegion TestCode

# Load Public Functions
$PublicFunctions = Get-ChildItem "$PSScriptRoot\Public\*.ps1" -Exclude '*.Tests.ps1'
Foreach ($Function in $PublicFunctions) {
    . $Function.FullName
    Export-ModuleMember -Function $Function.BaseName -Alias '*' -Verbose
}
