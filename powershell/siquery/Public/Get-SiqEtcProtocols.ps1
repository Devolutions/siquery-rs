function Get-SiqEtcProtocols {
	[CmdletBinding()]
	param()

	Get-SiqTableJson 'etc_protocols'
}
