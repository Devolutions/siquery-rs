function Get-SiqOSVersion {
	[CmdletBinding()]
	param()

	Get-SiqTableJson 'os_version'
}
