function Get-SiqEtcHosts {
	[CmdletBinding()]
	param()

	Get-SiqTableJson 'etc_hosts'
}
