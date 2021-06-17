function Get-SiqSystemInfo {
	[CmdletBinding()]
	param()

	Get-SiqTableJson 'system_info'
}
