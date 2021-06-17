function Get-SiqEtcServices {
	[CmdletBinding()]
	param()

	Get-SiqTableJson 'etc_services'
}
