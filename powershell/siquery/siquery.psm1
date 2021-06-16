
$ModuleName = $(Get-Item $PSCommandPath).BaseName
$Manifest = Import-PowerShellDataFile -Path $(Join-Path $PSScriptRoot "${ModuleName}.psd1")

if (-Not (Test-Path 'variable:global:IsWindows')) {
    $script:IsWindows = $true; # Windows PowerShell 5.1 or earlier
}

if ($IsWindows) {
    [System.Net.ServicePointManager]::SecurityProtocol = [System.Net.SecurityProtocolType]::Tls12;
}

Export-ModuleMember -Cmdlet @($manifest.CmdletsToExport)

$ScriptFiles = @(Get-ChildItem -Path "$PSScriptRoot/*.ps1" -Recurse)

Foreach ($Import in $ScriptFiles)
{
    Try
    {
        . $Import.FullName
    }
    Catch
    {
        Write-Error -Message "Failed to import $($Import.FullName): $_"
    }
}

Export-ModuleMember -Function @($Manifest.FunctionsToExport)
