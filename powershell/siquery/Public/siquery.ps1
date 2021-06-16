
function Find-SiqExecutable
{
    [CmdletBinding()]
    param()

    if (Test-Path Env:\SIQUERY) {
        return $Env:SIQUERY
    }

    $command = Get-Command -Name 'siquery' -CommandType 'Application' -ErrorAction SilentlyContinue

    if ($command) {
        return $command.Source
    }

    throw "could not find siquery executable"
}

function Get-SiqTableJson {
    [CmdletBinding()]
    param(
        [string] $Name
    )

    $siq_exe = Find-SiqExecutable
    $(& "$siq_exe" '-a' $Name '--json') | ConvertFrom-Json | Select-Object -ExpandProperty $Name
}

function Get-SiqEtcHosts {
    [CmdletBinding()]
    param()

    Get-SiqTableJson 'etc_hosts'
}

function Get-SiqEtcProtocols {
    [CmdletBinding()]
    param()

    Get-SiqTableJson 'etc_protocols'
}

function Get-SiqEtcServices {
    [CmdletBinding()]
    param()

    Get-SiqTableJson 'etc_services'
}

function Get-SiqSystemInfo {
    [CmdletBinding()]
    param()

    Get-SiqTableJson 'system_info'
}

function Get-SiqOsVersion {
    [CmdletBinding()]
    param()

    Get-SiqTableJson 'os_version'
}

function Get-SiqLogicalDrives {
    [CmdletBinding()]
    param()

    Get-SiqTableJson 'logical_drives'
}

function Get-SiqUptime {
    [CmdletBinding()]
    param()

    Get-SiqTableJson 'uptime'
}

function Get-SiqProcesses {
    [CmdletBinding()]
    param()

    Get-SiqTableJson 'processes'
}

function Get-SiqInterfaceAddress {
    [CmdletBinding()]
    param()

    Get-SiqTableJson 'interface_address'
}

function Get-SiqInterfaceDetails {
    [CmdletBinding()]
    param()

    Get-SiqTableJson 'interface_details'
}

function Get-SiqProcessOpenSockets {
    [CmdletBinding()]
    param()

    Get-SiqTableJson 'process_open_sockets'
}

function Get-SiqProcessMemoryMap {
    [CmdletBinding()]
    param()

    Get-SiqTableJson 'process_memory_map'
}

function Get-SiqProducts {
    [CmdletBinding()]
    param()

    Get-SiqTableJson 'products'
}

function Get-SiqUsers {
    [CmdletBinding()]
    param()

    Get-SiqTableJson 'users'
}

function Get-SiqLoggedInUsers {
    [CmdletBinding()]
    param()

    Get-SiqTableJson 'logged_in_users'
}

function Get-SiqLogonSessions {
    [CmdletBinding()]
    param()

    Get-SiqTableJson 'logon_sessions'
}

function Get-SiqGroups {
    [CmdletBinding()]
    param()

    Get-SiqTableJson 'groups'
}

function Get-SiqProxies {
    [CmdletBinding()]
    param()

    Get-SiqTableJson 'proxies'
}

function Get-SiqTableName {
    [CmdletBinding()]
    param()

    $siq_exe = Find-SiqExecutable
    $tables = $(& "$siq_exe" '-l') -Split '`n'
    return $tables
}
