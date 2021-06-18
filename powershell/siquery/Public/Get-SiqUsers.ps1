function Get-SiqUsers {
	[CmdletBinding()]
	param()

	Get-SiqTableJson 'users'
}
