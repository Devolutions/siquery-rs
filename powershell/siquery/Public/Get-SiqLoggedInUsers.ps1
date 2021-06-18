function Get-SiqLoggedInUsers {
	[CmdletBinding()]
	param()

	Get-SiqTableJson 'logged_in_users'
}
