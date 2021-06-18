function Get-SiqGroups {
	[CmdletBinding()]
	param()

	Get-SiqTableJson 'groups'
}
