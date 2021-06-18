function Get-SiqProcesses {
	[CmdletBinding()]
	param()

	Get-SiqTableJson 'processes'
}
