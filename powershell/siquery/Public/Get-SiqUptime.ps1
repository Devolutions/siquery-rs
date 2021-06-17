function Get-SiqUptime {
	[CmdletBinding()]
	param()

	Get-SiqTableJson 'uptime'
}
