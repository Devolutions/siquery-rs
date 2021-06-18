function Get-SiqProxies {
	[CmdletBinding()]
	param()

	Get-SiqTableJson 'proxies'
}
